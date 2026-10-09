use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheatSheet {
    pub id: String,
    pub title: String,
    pub language: String,
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub examples: Vec<CodeExample>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeExample {
    pub title: String,
    pub language: String,
    pub code: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub language: Option<String>,
    pub limit: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub language: String,
    pub score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suggestion {
    pub label: String,
    pub insert_text: String,
    pub detail: Option<String>,
    pub score: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum CheatError {
    #[error("invalid cheat sheet: {0}")]
    InvalidSheet(String),
    #[error("data error: {0}")]
    Data(String),
}

impl CheatSheet {
    pub fn validate(&self) -> Result<(), CheatError> {
        if self.id.trim().is_empty() {
            return Err(CheatError::InvalidSheet("id must not be empty".into()));
        }
        if self.title.trim().is_empty() {
            return Err(CheatError::InvalidSheet(format!("{}: title must not be empty", self.id)));
        }
        if self.language.trim().is_empty() {
            return Err(CheatError::InvalidSheet(format!("{}: language must not be empty", self.id)));
        }
        Ok(())
    }

    pub fn searchable_text(&self) -> String {
        let examples = self.examples.iter().map(|e| {
            format!("{} {} {} {}", e.title, e.code, e.description.as_deref().unwrap_or(""), e.language)
        }).collect::<Vec<_>>().join(" ");
        format!("{} {} {} {} {} {}", self.id, self.title, self.description,
            self.language, self.category, self.tags.join(" ") + " " + &examples)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_id() {
        let sheet = CheatSheet {
            id: "".into(), title: "x".into(), language: "rust".into(),
            category: "basics".into(), description: String::new(),
            tags: vec![], examples: vec![],
        };
        assert!(sheet.validate().is_err());
    }
}
