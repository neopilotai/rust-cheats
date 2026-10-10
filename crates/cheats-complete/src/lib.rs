use cheats_core::{CheatSheet, Suggestion};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

/// Return deterministic prefix suggestions, with fuzzy fallback for useful typo recovery.
pub fn complete(sheets: &[CheatSheet], prefix: &str, language: Option<&str>, limit: usize) -> Vec<Suggestion> {
    if limit == 0 { return Vec::new(); }
    let query = prefix.trim().to_lowercase();
    let matcher = SkimMatcherV2::default();
    let mut out = Vec::new();
    for sheet in sheets {
        if language.is_some_and(|l| !sheet.language.eq_ignore_ascii_case(l)) { continue; }
        let mut candidates = vec![(sheet.id.as_str(), sheet.description.as_str(), 100.0),
            (sheet.title.as_str(), sheet.language.as_str(), 85.0),
            (sheet.language.as_str(), "language", 75.0),
            (sheet.category.as_str(), "topic", 70.0)];
        candidates.extend(sheet.tags.iter().map(|tag| (tag.as_str(), "tag", 65.0)));
        for (label, detail, base) in candidates {
            if label.is_empty() { continue; }
            let lower = label.to_lowercase();
            let score = if query.is_empty() { Some(base) }
                else if lower.starts_with(&query) { Some(base + 30.0 + (query.len() as f64 / label.len() as f64) * 10.0) }
                else { matcher.fuzzy_match(&lower, &query).map(|score| base + score as f64 * 0.1) };
            if let Some(score) = score { out.push(Suggestion { label: label.to_string(), insert_text: label.to_string(), detail: Some(detail.to_string()), score }); }
        }
    }
    out.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.label.cmp(&b.label)));
    out.dedup_by(|a, b| a.label.eq_ignore_ascii_case(&b.label));
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
