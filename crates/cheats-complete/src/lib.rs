use cheats_core::{CheatSheet, Suggestion};

pub fn complete(sheets: &[CheatSheet], prefix: &str, language: Option<&str>, limit: usize) -> Vec<Suggestion> {
    if limit == 0 {
        return Vec::new();
    }

    let prefix = prefix.trim().to_lowercase();
    let mut out = Vec::new();
    for sheet in sheets {
        if language.is_some_and(|l| !sheet.language.eq_ignore_ascii_case(l)) {
            continue;
        }
        let candidates = [
            (sheet.id.as_str(), Some(sheet.description.as_str()), 90.0),
            (sheet.title.as_str(), Some(sheet.language.as_str()), 80.0),
        ];
        for (label, detail, base_score) in candidates {
            let label_lower = label.to_lowercase();
            if prefix.is_empty() || label_lower.starts_with(&prefix) {
                let score = base_score + if label_lower == prefix { 100.0 } else {
                    10.0 / (label.len().max(1) as f64)
                };
                out.push(Suggestion {
                    label: label.to_string(),
                    insert_text: label.to_string(),
                    detail: detail.filter(|s| !s.is_empty()).map(str::to_string),
                    score,
                });
            }
        }
    }
    out.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.label.cmp(&b.label)));
    out.dedup_by(|a, b| a.label == b.label);
    out.truncate(limit);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_prefix_returns_suggestions() {
        let sheets = vec![CheatSheet {
            id: "rust/vec".into(), title: "Rust Vec".into(), language: "rust".into(),
            category: "collections".into(), description: "Vectors".into(),
            tags: vec![], examples: vec![],
        }];
        assert!(!complete(&sheets, "", None, 10).is_empty());
    }

    #[test]
    fn zero_limit_returns_no_results() {
        let sheets = vec![CheatSheet {
            id: "rust/vec".into(), title: "Rust Vec".into(), language: "rust".into(),
            category: "collections".into(), description: "Vectors".into(),
            tags: vec![], examples: vec![],
        }];
        assert!(complete(&sheets, "rust", None, 0).is_empty());
    }

    #[test]
    fn exact_prefix_is_ranked_first() {
        let sheets = vec![CheatSheet {
            id: "rust".into(), title: "Rust Basics".into(), language: "rust".into(),
            category: "basics".into(), description: "Basics".into(),
            tags: vec![], examples: vec![],
        }];
        let results = complete(&sheets, "rust", None, 10);
        assert_eq!(results[0].label, "rust");
    }
}
