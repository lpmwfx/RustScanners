use serde::Deserialize;
use std::path::Path;

/// Scanner configuration — loaded from `proj/rulestools.toml`.
#[derive(Debug, Clone)]
pub struct Config {
    pub enabled: bool,
    pub deny: bool,
    pub check_magic_numbers: bool,
    pub check_hardcoded_durations: bool,
    pub check_hardcoded_urls: bool,
    pub check_hardcoded_paths: bool,
    pub check_string_states: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            deny: false,
            check_magic_numbers: true,
            check_hardcoded_durations: true,
            check_hardcoded_urls: true,
            check_hardcoded_paths: true,
            check_string_states: true,
        }
    }
}

#[derive(Deserialize, Default)]
struct TomlRoot {
    #[serde(default)]
    rustscanners: Option<TomlScanners>,
}

#[derive(Deserialize, Default)]
struct TomlScanners {
    enabled: Option<bool>,
    deny: Option<bool>,
    magic_numbers: Option<bool>,
    hardcoded_durations: Option<bool>,
    hardcoded_urls: Option<bool>,
    hardcoded_paths: Option<bool>,
    string_states: Option<bool>,
}

impl Config {
    /// Load configuration from `proj/rulestools.toml` in the project root.
    /// Falls back to defaults if the file doesn't exist.
    pub fn load(project_root: &Path) -> Self {
        let toml_path = project_root.join("proj").join("rulestools.toml");
        let mut cfg = Self::default();

        if let Ok(content) = std::fs::read_to_string(&toml_path) {
            if let Ok(parsed) = toml::from_str::<TomlRoot>(&content) {
                if let Some(s) = parsed.rustscanners {
                    if let Some(v) = s.enabled {
                        cfg.enabled = v;
                    }
                    if let Some(v) = s.deny {
                        cfg.deny = v;
                    }
                    if let Some(v) = s.magic_numbers {
                        cfg.check_magic_numbers = v;
                    }
                    if let Some(v) = s.hardcoded_durations {
                        cfg.check_hardcoded_durations = v;
                    }
                    if let Some(v) = s.hardcoded_urls {
                        cfg.check_hardcoded_urls = v;
                    }
                    if let Some(v) = s.hardcoded_paths {
                        cfg.check_hardcoded_paths = v;
                    }
                    if let Some(v) = s.string_states {
                        cfg.check_string_states = v;
                    }
                }
            }
        }

        cfg
    }
}
