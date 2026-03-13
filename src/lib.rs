//! # RustScanners
//!
//! Zero-literal static analysis scanners for Rust projects.
//! Add as a `[build-dependencies]` and call `rustscanners::scan_project()`
//! from `build.rs` to enforce rules during `cargo build`.

pub mod checks;
mod config;
mod issue;
mod context;

pub use config::Config;
pub use issue::Issue;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Walk up from `start` to find a Cargo.toml containing `[workspace]`.
/// Returns the workspace root if found, otherwise the original `start`.
fn find_workspace_root(start: &Path) -> PathBuf {
    let mut dir = start.to_path_buf();
    loop {
        let cargo = dir.join("Cargo.toml");
        if cargo.is_file() {
            if let Ok(content) = std::fs::read_to_string(&cargo) {
                if content.contains("[workspace]") {
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

/// Scan the project and emit `cargo:warning` for each violation.
/// Call this from `build.rs`.
///
/// Returns the total number of errors found.
pub fn scan_project() -> usize {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().expect("no cwd"));

    let root = find_workspace_root(&manifest_dir);

    let cfg = Config::load(&root);
    if !cfg.enabled {
        return 0;
    }

    // In a workspace, scan all `src/` dirs under the root.
    // In a standalone crate, just scan `root/src/`.
    let mut rs_files = Vec::new();

    // Always scan root/src/ if it exists
    let root_src = root.join("src");
    if root_src.is_dir() {
        collect_rs_files(&root_src, &mut rs_files);
    }

    // In a workspace, also scan member crate src/ directories
    if root != manifest_dir {
        // Walk the workspace looking for crate src/ dirs
        for entry in WalkDir::new(&root)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == "Cargo.toml" && e.path() != root.join("Cargo.toml"))
        {
            if let Some(parent) = entry.path().parent() {
                let src = parent.join("src");
                if src.is_dir() {
                    collect_rs_files(&src, &mut rs_files);
                }
            }
        }
    }

    if rs_files.is_empty() {
        return 0;
    }

    let mut total_errors = 0;

    for path in &rs_files {
        let issues = scan_file(path, &cfg);
        for issue in &issues {
            println!("cargo:warning={}", issue);
            total_errors += 1;
        }
    }

    if total_errors > 0 && cfg.deny {
        panic!(
            "rustscanners: {} error(s) found — fix violations or set deny = false in proj/rulestools.toml",
            total_errors
        );
    }

    total_errors
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        out.push(entry.path().to_path_buf());
    }
}

/// Scan a single `.rs` file and return all issues.
pub fn scan_file(path: &Path, cfg: &Config) -> Vec<Issue> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let lines: Vec<&str> = content.lines().collect();
    let ctx = context::FileContext::new(path, &lines);

    let mut issues = Vec::new();

    if cfg.check_magic_numbers {
        checks::magic_numbers::check(&ctx, &lines, &mut issues);
    }
    if cfg.check_hardcoded_durations {
        checks::hardcoded_durations::check(&ctx, &lines, &mut issues);
    }
    if cfg.check_string_states {
        checks::string_states::check(&ctx, &lines, &mut issues);
    }
    if cfg.check_unwrap_panic {
        checks::unwrap_panic::check(&ctx, &lines, &mut issues);
    }
    if cfg.check_unsafe_no_comment {
        checks::unsafe_no_comment::check(&ctx, &lines, &mut issues);
    }
    if cfg.check_doc_comments {
        checks::doc_comments::check(&ctx, &lines, &mut issues);
    }

    issues
}
