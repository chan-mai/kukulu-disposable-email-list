use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub struct MergeResult {
    pub added: Vec<String>,
    pub total: usize,
    pub written: bool,
}

// 既存リストと統合し、ソート済み重複なしで保存。内容が同一なら書き込まない
pub fn merge(path: &str, fetched: &BTreeSet<String>) -> Result<MergeResult> {
    let original = if Path::new(path).exists() {
        Some(fs::read_to_string(path).with_context(|| format!("failed to read {path}"))?)
    } else {
        None
    };

    let current: BTreeSet<String> = original
        .as_deref()
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();

    let added: Vec<String> = fetched.difference(&current).cloned().collect();
    let merged: BTreeSet<String> = current.union(fetched).cloned().collect();

    let mut output = merged.iter().cloned().collect::<Vec<_>>().join("\n");
    output.push('\n');

    let written = original.as_deref() != Some(output.as_str());
    if written {
        fs::write(path, &output).with_context(|| format!("failed to write {path}"))?;
    }

    Ok(MergeResult {
        added,
        total: merged.len(),
        written,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("kukulu-test-{name}-{}", std::process::id()))
    }

    #[test]
    fn merges_and_sorts() {
        let path = tmp_path("merge");
        fs::write(&path, "b.com\na.com\n").unwrap();
        let fetched: BTreeSet<String> = ["c.com".to_string()].into();
        let result = merge(path.to_str().unwrap(), &fetched).unwrap();
        assert_eq!(result.added, vec!["c.com"]);
        assert_eq!(result.total, 3);
        assert!(result.written);
        assert_eq!(fs::read_to_string(&path).unwrap(), "a.com\nb.com\nc.com\n");
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn skips_write_when_unchanged() {
        let path = tmp_path("unchanged");
        fs::write(&path, "a.com\nb.com\n").unwrap();
        let fetched: BTreeSet<String> = ["a.com".to_string()].into();
        let result = merge(path.to_str().unwrap(), &fetched).unwrap();
        assert!(result.added.is_empty());
        assert!(!result.written);
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn creates_file_when_missing() {
        let path = tmp_path("missing");
        let _ = fs::remove_file(&path);
        let fetched: BTreeSet<String> = ["a.com".to_string()].into();
        let result = merge(path.to_str().unwrap(), &fetched).unwrap();
        assert_eq!(result.total, 1);
        assert!(result.written);
        fs::remove_file(&path).unwrap();
    }
}
