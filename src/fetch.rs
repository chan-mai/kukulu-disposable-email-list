use anyhow::{Context, Result};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

// 無料プロキシは応答が遅いため直接取得より短いタイムアウトにする
const DIRECT_TIMEOUT: Duration = Duration::from_secs(30);
const PROXY_TIMEOUT: Duration = Duration::from_secs(15);

pub fn build_agent(proxy: Option<&str>) -> Result<ureq::Agent> {
    let builder = match proxy {
        Some(p) => ureq::AgentBuilder::new()
            .proxy(ureq::Proxy::new(p).with_context(|| format!("invalid proxy: {p}"))?)
            .timeout(PROXY_TIMEOUT),
        None => ureq::AgentBuilder::new().timeout(DIRECT_TIMEOUT),
    };
    Ok(builder.build())
}

pub fn fetch_html(agent: &ureq::Agent, url: &str) -> Result<String> {
    // キャッシュ回避のためクエリパラメータを毎回変える
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let response = agent
        .get(&format!("{url}?_={nonce}"))
        .set("User-Agent", USER_AGENT)
        .set(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .set("Accept-Language", "ja,en;q=0.8")
        .call()
        .context("request failed")?;
    response.into_string().context("failed to read body")
}
