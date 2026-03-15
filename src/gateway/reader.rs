//! File reading — all `std::fs::read_to_string` calls live here.

use std::path::Path;

const CONFIG_DIR: &str = "proj";
const CONFIG_FILENAME: &str = "rulestools.toml";

/// Read a file to `String`. Returns `None` on any IO error.
pub fn read_text(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// Read `proj/rulestools.toml` from the project root. Returns `None` if absent.
pub fn read_config(project_root: &Path) -> Option<String> {
    read_text(&project_root.join(CONFIG_DIR).join(CONFIG_FILENAME))
}
