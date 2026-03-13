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
pub use issue::{Issue, Severity};

use std::path::Path;
use walkdir::WalkDir;

/// Scan the project and emit `cargo:warning` for each violation.
/// Call this from `build.rs`.
///
/// Returns the total number of errors found.
pub fn scan_project() -> usize {
    let root = std::env::var("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().expect("no cwd"));

    let cfg = Config::load(&root);
    if !cfg.enabled {
        return 0;
    }

    let src_dir = root.join("src");
    if !src_dir.is_dir() {
        return 0;
    }

    let mut total_errors = 0;

    for entry in WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext == "rs")
        })
    {
        let path = entry.path();
        let issues = scan_file(path, &cfg);
        for issue in &issues {
            // cargo:warning is the only way build scripts communicate diagnostics
            println!("cargo:warning={}", issue);
            if issue.severity == Severity::Error {
                total_errors += 1;
            }
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
    if cfg.check_hardcoded_urls {
        checks::hardcoded_urls::check(&ctx, &lines, &mut issues);
    }
    if cfg.check_hardcoded_paths {
        checks::hardcoded_paths::check(&ctx, &lines, &mut issues);
    }
    if cfg.check_string_states {
        checks::string_states::check(&ctx, &lines, &mut issues);
    }

    issues
}
