use cheats_core::{CheatSheet, Suggestion};

pub fn complete(sheets: &[CheatSheet], prefix: &str, language: Option<&str>, limit: usize) -> Vec<Suggestion> {
    let prefix = prefix.trim().to_lowercase();
    let mut out = Vec::new();
    for sheet in sheets {
        if language.is_some_and(|l| !sheet.language.eq_ignore_ascii_case(l)) {
            continue;
        }
        let candidates = [
            (sheet.id.as_str(), Some(sheet.description.as_str())),
            (sheet.title.as_str(), Some(sheet.language.as_str())),
        ];
        for (label, detail) in candidates {
            if prefix.is_empty() || label.to_lowercase().starts_with(&prefix) {
                out.push(Suggestion {
                    label: label.to_string(),
                    insert_text: label.to_string(),
                    detail: detail.filter(|s| !s.is_empty()).map(str::to_string),
                    score: if label.eq_ignore_ascii_case(&prefix) { 100.0 } else { 50.0 },
                });
            }
        }
    }
    out.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.label.cmp(&b.label)));
    out.dedup_by(|a, b| a.label == b.label);
    out.truncate(limit.max(1));
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
}
