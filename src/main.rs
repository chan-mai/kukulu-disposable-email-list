mod fetch;
mod parse;
mod proxy;
mod store;

use std::collections::BTreeSet;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

const TARGET_URL: &str = "https://m.kuku.lu/ja.php";
const DOMAINS_FILE: &str = "domains.txt";
// 同一IPからの連続リクエストは空のリストが返ることがあるため間隔を空ける
const SAMPLE_INTERVAL: Duration = Duration::from_secs(5);
const DEFAULT_SAMPLES: u32 = 10;

// 無料プロキシは不安定なため、最初の数回は直接取得して成功の下限を確保する
const DIRECT_SAMPLES: u32 = 3;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<()> {
    // 表示されるドメインはIPとリクエストごとに変動するため、複数回取得して和集合を取る
    let samples: u32 = std::env::var("CRAWL_SAMPLES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_SAMPLES);

    let direct_agent = fetch::build_agent(None)?;

    let mut pool = match proxy::fetch_list(&direct_agent) {
        Ok(list) => {
            println!("proxies: {} available", list.len());
            list
        }
        Err(e) => {
            eprintln!("proxy list unavailable: {e:#}");
            Vec::new()
        }
    };

    let mut collected: BTreeSet<String> = BTreeSet::new();
    let mut succeeded = 0u32;

    for i in 1..=samples {
        let chosen: Option<String> = if i > DIRECT_SAMPLES {
            proxy::pick(&pool).map(String::from)
        } else {
            None
        };
        let result = match &chosen {
            Some(p) => {
                fetch::build_agent(Some(p)).and_then(|agent| fetch::fetch_html(&agent, TARGET_URL))
            }
            None => fetch::fetch_html(&direct_agent, TARGET_URL),
        };
        let label = chosen.as_deref().unwrap_or("direct");
        match result {
            Ok(html) => {
                let domains = parse::extract_domains(&html);
                println!("[{i}/{samples}] {} domains via {label}", domains.len());
                collected.extend(domains);
                succeeded += 1;
            }
            Err(e) => {
                eprintln!("[{i}/{samples}] fetch failed via {label}: {e:#}");
                // 失敗したプロキシは同一実行内で再選択しない
                if let Some(p) = &chosen {
                    pool.retain(|x| x != p);
                }
            }
        }
        if i < samples {
            thread::sleep(SAMPLE_INTERVAL);
        }
    }

    anyhow::ensure!(succeeded > 0, "all {samples} fetch attempts failed");
    // 全サンプルで0件はマークアップ変更などの異常として扱う
    anyhow::ensure!(!collected.is_empty(), "no domains extracted");

    let result = store::merge(DOMAINS_FILE, &collected)?;
    for domain in &result.added {
        println!("new: {domain}");
    }
    println!(
        "fetched: {}, added: {}, total: {}, written: {}",
        collected.len(),
        result.added.len(),
        result.total,
        result.written
    );
    Ok(())
}
