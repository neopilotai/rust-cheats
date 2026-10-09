use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Language {
    pub id: String,
    pub display_name: String,
    pub extensions: Vec<String>,
    pub aliases: Vec<String>,
}

pub fn builtins() -> Vec<Language> {
    vec![
        Language { id: "rust".into(), display_name: "Rust".into(), extensions: vec!["rs".into()], aliases: vec!["rs".into()] },
        Language { id: "typescript".into(), display_name: "TypeScript".into(), extensions: vec!["ts".into(), "tsx".into()], aliases: vec!["ts".into()] },
        Language { id: "javascript".into(), display_name: "JavaScript".into(), extensions: vec!["js".into(), "jsx".into(), "mjs".into()], aliases: vec!["js".into(), "node".into()] },
        Language { id: "python".into(), display_name: "Python".into(), extensions: vec!["py".into(), "pyi".into()], aliases: vec!["py".into()] },
        Language { id: "go".into(), display_name: "Go".into(), extensions: vec!["go".into()], aliases: vec!["golang".into()] },
        Language { id: "bash".into(), display_name: "Bash".into(), extensions: vec!["sh".into(), "bash".into()], aliases: vec!["shell".into(), "zsh".into()] },
        Language { id: "c".into(), display_name: "C".into(), extensions: vec!["c".into(), "h".into()], aliases: vec![] },
        Language { id: "cpp".into(), display_name: "C++".into(), extensions: vec!["cc".into(), "cpp".into(), "cxx".into(), "hpp".into()], aliases: vec!["c++".into(), "cxx".into()] },
    ]
}

pub fn normalize(input: &str) -> Option<String> {
    let needle = input.trim().to_lowercase();
    builtins().into_iter().find(|lang| {
        lang.id == needle || lang.display_name.to_lowercase() == needle
            || lang.aliases.iter().any(|a| a == &needle)
    }).map(|l| l.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recognizes_common_aliases() {
        assert_eq!(normalize("rs").as_deref(), Some("rust"));
        assert_eq!(normalize("C++").as_deref(), Some("cpp"));
    }
}
