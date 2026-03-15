//! IO layer — only this module may perform `std::fs` operations.
//!
//! All file reading for RustScanners is centralised here.
//! Callers receive `String` content or `Vec<PathBuf>` — never raw IO handles.

use glob::Pattern;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const CARGO_TOML: &str = "Cargo.toml";
const RS_EXT: &str = "rs";
const CONFIG_DIR: &str = "proj";
const CONFIG_FILENAME: &str = "rulestools.toml";
const WORKSPACE_MAX_DEPTH: usize = 5;
const WORKSPACE_MARKER: &str = "[workspace]";

/// Read a file to `String`. Returns `None` on any IO error.
pub fn read_text(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// Read `proj/rulestools.toml` from the project root. Returns `None` if absent.
pub fn read_config(project_root: &Path) -> Option<String> {
    read_text(&project_root.join(CONFIG_DIR).join(CONFIG_FILENAME))
}

/// Walk up from `start` to find a `Cargo.toml` containing `[workspace]`.
/// Returns the workspace root if found, otherwise `start`.
pub fn find_workspace_root(start: &Path) -> PathBuf {
    let mut dir = start.to_path_buf();
    loop {
        let cargo = dir.join(CARGO_TOML);
        if cargo.is_file() {
            if let Some(content) = read_text(&cargo) {
                if content.contains(WORKSPACE_MARKER) {
                    return dir;
                }
            }
        }
        if !dir.pop() {
            break;
        }
    }
    start.to_path_buf()
}

/// Collect all `.rs` files under `dir`, respecting exclude glob patterns.
pub fn collect_rs_files(dir: &Path, exclude: &[String]) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !matches_exclude(e.path(), exclude))
        .filter(|e| e.path().extension().map_or(false, |x| x == RS_EXT))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Collect all `.rs` files for a project or workspace rooted at `root`.
pub fn collect_project_files(
    root: &Path,
    manifest_dir: &Path,
    exclude: &[String],
) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let root_src = root.join("src");
    if root_src.is_dir() {
        files.extend(collect_rs_files(&root_src, exclude));
    }

    if root != manifest_dir {
        let root_cargo = root.join(CARGO_TOML);
        for entry in WalkDir::new(root)
            .max_depth(WORKSPACE_MAX_DEPTH)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == CARGO_TOML && e.path() != root_cargo)
        {
            if let Some(parent) = entry.path().parent() {
                let src = parent.join("src");
                if src.is_dir() {
                    files.extend(collect_rs_files(&src, exclude));
                }
            }
        }
    }

    files
}

fn matches_exclude(path: &Path, patterns: &[String]) -> bool {
    let s = path.to_string_lossy();
    patterns.iter().any(|p| {
        Pattern::new(p).map_or(false, |pat| pat.matches(&s))
    })
}
