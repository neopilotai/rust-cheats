use cheats_core::{CheatSheet, SearchRequest, SearchResult};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub fn search(sheets: &[CheatSheet], request: &SearchRequest) -> Vec<SearchResult> {
    let query = request.query.trim().to_lowercase();
    if query.is_empty() {
        return Vec::new();
    }
    let matcher = SkimMatcherV2::default();
    let mut results = Vec::new();

    for sheet in sheets {
        if request.language.as_ref().is_some_and(|lang| !sheet.language.eq_ignore_ascii_case(lang)) {
            continue;
        }

        let id = sheet.id.to_lowercase();
        let title = sheet.title.to_lowercase();
        let description = sheet.description.to_lowercase();
        let tags = sheet.tags.join(" ").to_lowercase();
        let haystack = sheet.searchable_text().to_lowercase();

        let score = if id == query || title == query {
            1_000.0
        } else if title.contains(&query) || id.contains(&query) {
            700.0
        } else if tags.contains(&query) {
            500.0
        } else if description.contains(&query) || haystack.contains(&query) {
            300.0
        } else if let Some(score) = matcher.fuzzy_match(&haystack, &query) {
            score as f64
        } else {
            continue;
        };

        results.push(SearchResult {
            id: sheet.id.clone(),
            title: sheet.title.clone(),
            description: sheet.description.clone(),
            language: sheet.language.clone(),
            score,
        });
    }

    results.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.title.cmp(&b.title)));
    results.truncate(request.limit.max(1));
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use cheats_core::{CheatSheet, CodeExample};

    fn sheet() -> CheatSheet {
        CheatSheet {
            id: "rust/iterators".into(), title: "Rust iterators".into(),
            language: "rust".into(), category: "collections".into(),
            description: "Map and collect values".into(),
            tags: vec!["iterator".into(), "collect".into()],
            examples: vec![CodeExample {
                title: "Map".into(), language: "rust".into(),
                code: "items.iter().map(|x| x * 2).collect()".into(),
                description: None,
            }],
        }
    }

    #[test]
    fn exact_title_ranks_first() {
        let results = search(&[sheet()], &SearchRequest {
            query: "Rust iterators".into(), language: None, limit: 10,
        });
        assert_eq!(results[0].id, "rust/iterators");
        assert_eq!(results[0].score, 1000.0);
    }

    #[test]
    fn filters_by_language() {
        let results = search(&[sheet()], &SearchRequest {
            query: "iterators".into(), language: Some("python".into()), limit: 10,
        });
        assert!(results.is_empty());
    }
}
