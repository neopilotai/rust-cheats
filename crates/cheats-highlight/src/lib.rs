//! Tree-sitter integration boundary.
//!
//! Add reviewed grammar dependencies and `HighlightConfiguration`s per language in a
//! later phase. Keeping grammar registration explicit avoids silently pulling in
//! every grammar and its transitive dependencies.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightToken {
    pub start_byte: usize,
    pub end_byte: usize,
    pub capture: String,
}

/// Reports whether a language has a registered highlighter.
///
/// The initial scaffold intentionally returns false until grammar crates and
/// language query files are added.
pub fn is_supported(_language_id: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsupported_language_degrades_gracefully() {
        assert!(!is_supported("made-up-language"));
    }
}
