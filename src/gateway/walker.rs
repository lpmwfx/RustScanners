//! File system walking — collects `.rs` source files for scanning.

use glob::Pattern;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Topology;

const CARGO_TOML: &str = "Cargo.toml";
const RS_EXT: &str = "rs";
const WORKSPACE_MAX_DEPTH: usize = 5;
const WORKSPACE_MARKER: &str = "[workspace]";

/// Walk up from `start` to find a `Cargo.toml` containing `[workspace]`.
/// Returns the workspace root if found, otherwise `start`.
pub fn find_workspace_root(start: &Path) -> PathBuf {
    let mut dir = start.to_path_buf();
    loop {
        let cargo = dir.join(CARGO_TOML);
        if cargo.is_file() {
            if let Some(content) = std::fs::read_to_string(&cargo).ok() {
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
        .filter(|e| {
            let s = e.path().to_string_lossy();
            !exclude.iter().any(|p| Pattern::new(p).map_or(false, |pat| pat.matches(&s)))
        })
        .filter(|e| e.path().extension().map_or(false, |x| x == RS_EXT))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Collect all `.rs` files for a project based on declared topology.
///
/// - `Flat`: scans only `manifest_dir/src/` — suitable for standalone crates and library members
///   that rely on a single scan-root binary to cover the full workspace.
/// - `Workspace`: scans all `apps/*/src/` and `crates/*/src/` under the workspace root —
///   use this only in the designated scan-root `build.rs` (typically `apps/desktop/`).
pub fn collect_project_files(
    root: &Path,
    manifest_dir: &Path,
    topology: &Topology,
    exclude: &[String],
) -> Vec<PathBuf> {
    match topology {
        Topology::Flat => collect_flat(manifest_dir, exclude),
        Topology::Workspace => collect_workspace(root, exclude),
    }
}

/// Scan only `manifest_dir/src/` — no workspace walk.
fn collect_flat(manifest_dir: &Path, exclude: &[String]) -> Vec<PathBuf> {
    let src = manifest_dir.join("src");
    if src.is_dir() { collect_rs_files(&src, exclude) } else { vec![] }
}

/// Scan all member `src/` directories under the workspace root.
fn collect_workspace(root: &Path, exclude: &[String]) -> Vec<PathBuf> {
    let root_cargo = root.join(CARGO_TOML);
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .max_depth(WORKSPACE_MAX_DEPTH)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == CARGO_TOML && e.path() != root_cargo)
    {
        if let Some(src) = entry.path().parent().map(|p| p.join("src")) {
            if src.is_dir() {
                files.extend(collect_rs_files(&src, exclude));
            }
        }
    }
    files
}
