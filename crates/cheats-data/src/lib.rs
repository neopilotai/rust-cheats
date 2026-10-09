use cheats_core::{CheatError, CheatSheet};
use std::{fs, path::Path};
use walkdir::WalkDir;

pub fn load_dir(path: impl AsRef<Path>) -> Result<Vec<CheatSheet>, CheatError> {
    let root = path.as_ref();
    if !root.exists() {
        return Err(CheatError::Data(format!("data directory does not exist: {}", root.display())));
    }

    let mut sheets = Vec::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry.map_err(|e| CheatError::Data(e.to_string()))?;
        if !entry.file_type().is_file() || entry.path().extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        let text = fs::read_to_string(entry.path())
            .map_err(|e| CheatError::Data(format!("{}: {e}", entry.path().display())))?;
        let sheet: CheatSheet = toml::from_str(&text)
            .map_err(|e| CheatError::Data(format!("{}: {e}", entry.path().display())))?;
        sheet.validate()?;
        sheets.push(sheet);
    }
    sheets.sort_by(|a, b| a.id.cmp(&b.id));

    let mut ids = std::collections::HashSet::new();
    for sheet in &sheets {
        if !ids.insert(sheet.id.clone()) {
            return Err(CheatError::Data(format!("duplicate sheet id: {}", sheet.id)));
        }
    }
    Ok(sheets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_missing_directory() {
        assert!(load_dir("definitely-not-a-real-rust-cheats-directory").is_err());
    }
}
