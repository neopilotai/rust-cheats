#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Intent {
    FindExample,
    ExplainConcept,
    ShowSyntax,
    CompareApproaches,
    Search,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedQuery {
    pub intent: Intent,
    pub language: Option<String>,
    pub query: String,
}

pub fn parse(input: &str) -> ParsedQuery {
    let trimmed = input.trim();
    let lower = trimmed.to_lowercase();
    let intent = if lower.contains("how do i") || lower.contains("example") || lower.starts_with("how to") {
        Intent::FindExample
    } else if lower.contains("explain") || lower.starts_with("what is") {
        Intent::ExplainConcept
    } else if lower.contains("syntax") || lower.contains("show me") {
        Intent::ShowSyntax
    } else if lower.contains("compare") || lower.contains("difference between") {
        Intent::CompareApproaches
    } else {
        Intent::Search
    };

    let language = [
        ("typescript", "typescript"), ("javascript", "javascript"),
        ("python", "python"), ("rust", "rust"), ("golang", "go"),
        (" go ", "go"), ("bash", "bash"), ("shell", "bash"),
        ("c++", "cpp"), ("c/c++", "cpp"),
    ].iter().find(|(needle, _)| {
        let padded = format!(" {lower} ");
        padded.contains(needle)
    }).map(|(_, id)| (*id).to_string());

    ParsedQuery { intent, language, query: trimmed.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_how_to_intent_and_language() {
        let q = parse("How do I read JSON in Rust?");
        assert_eq!(q.intent, Intent::FindExample);
        assert_eq!(q.language.as_deref(), Some("rust"));
    }
}
