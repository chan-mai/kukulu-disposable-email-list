use anyhow::{Context, Result};
use std::time::{SystemTime, UNIX_EPOCH};

const PROXY_LIST_URL: &str = "https://api.proxyscrape.com/v4/free-proxy-list/get?request=display_proxies&country=jp&protocol=http&proxy_format=protocolipport&format=text&timeout=20000";

// 無料JPプロキシの一覧を取得
pub fn fetch_list(agent: &ureq::Agent) -> Result<Vec<String>> {
    let body = agent
        .get(&format!("{PROXY_LIST_URL}&_={}", nanos()))
        .call()
        .context("proxy list request failed")?
        .into_string()
        .context("failed to read proxy list")?;
    Ok(body
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("http://"))
        .map(String::from)
        .collect())
}

// 秒未満のナノ秒を乱数源として選択。暗号用途ではないため十分
pub fn pick(proxies: &[String]) -> Option<&str> {
    if proxies.is_empty() {
        return None;
    }
    Some(&proxies[nanos() as usize % proxies.len()])
}

fn nanos() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_returns_none_for_empty() {
        assert_eq!(pick(&[]), None);
    }

    #[test]
    fn pick_returns_element() {
        let proxies = vec!["http://a:80".to_string(), "http://b:80".to_string()];
        let picked = pick(&proxies).unwrap();
        assert!(proxies.iter().any(|p| p == picked));
    }
}
