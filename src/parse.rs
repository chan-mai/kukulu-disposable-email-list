use scraper::{Html, Selector};

// radio入力のvalue属性から抽出、マークアップ変更に備えてb要素のテキストを予備とする
pub fn extract_domains(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);

    let primary = Selector::parse(r#"input[name="input_manualmaildomain"]"#).unwrap();
    let domains: Vec<String> = doc
        .select(&primary)
        .filter_map(|el| el.value().attr("value"))
        .filter_map(normalize)
        .collect();
    if !domains.is_empty() {
        return domains;
    }

    let fallback = Selector::parse(r#"#input_manualmaildomain_list b[dir="auto"]"#).unwrap();
    doc.select(&fallback)
        .map(|el| el.text().collect::<String>())
        .filter_map(|text| normalize(&text))
        .collect()
}

fn normalize(raw: &str) -> Option<String> {
    let domain = raw.trim().trim_start_matches('@').to_ascii_lowercase();
    let valid = domain.contains('.')
        && domain
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        && !domain.starts_with('.')
        && !domain.ends_with('.');
    valid.then_some(domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_from_radio_inputs() {
        let html = r#"
            <input type="radio" name="input_manualmaildomain" value="miho.uk" />
            <input type="radio" name="input_manualmaildomain" value="eay.jp" />
        "#;
        assert_eq!(
            extract_domains(html),
            vec!["miho.uk".to_string(), "eay.jp".to_string()]
        );
    }

    #[test]
    fn falls_back_to_label_text() {
        let html = r#"
            <div id="input_manualmaildomain_list">
                <b dir="auto">@miho.uk</b>
            </div>
        "#;
        assert_eq!(extract_domains(html), vec!["miho.uk".to_string()]);
    }

    #[test]
    fn rejects_invalid_values() {
        assert_eq!(normalize("miho.uk"), Some("miho.uk".to_string()));
        assert_eq!(normalize("@miho.uk"), Some("miho.uk".to_string()));
        assert_eq!(normalize(""), None);
        assert_eq!(normalize("nodot"), None);
        assert_eq!(normalize(".start"), None);
        assert_eq!(normalize("end."), None);
        assert_eq!(normalize("a b.com"), None);
    }
}
