use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub data_dir: PathBuf,
    pub quiet: bool,
    pub private: bool,
    pub max_results: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("data"),
            quiet: false,
            private: false,
            max_results: 10,
        }
    }
}

impl Config {
    pub fn effective_quiet(&self) -> bool {
        self.quiet || self.private
    }

    pub fn history_enabled(&self) -> bool {
        !self.private
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn private_implies_quiet_and_disables_history() {
        let cfg = Config { private: true, ..Config::default() };
        assert!(cfg.effective_quiet());
        assert!(!cfg.history_enabled());
    }
}
