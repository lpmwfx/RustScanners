//! Project scanner orchestrator — coordinates gateway, scanner, and checks.
//!
//! scan_project() is the single public entry point for build.rs consumers.
//! Helper functions keep each step focused and below the orchestrator threshold.

use std::io::Write;
use std::path::{Path, PathBuf};

use crate::checks;
use crate::config::Config;
use crate::gateway;
use crate::issue;
use crate::scanner;
use crate::Issue;

const DENY_MSG: &str =
    "cargo:error=rustscanners: {} error(s) found — fix violations or set deny = false in proj/rulestools.toml";

/// Scan the project and emit `cargo:warning` for each violation.
/// Call this from `build.rs`. Returns the total number of errors found.
pub fn scan_project() -> usize {
    let Some((_root, cfg, rs_files)) = collect_context() else {
        return 0;
    };
    let (mut errors, contents) = scan_sources(&rs_files, &cfg);
    errors += run_cross_checks(&contents, &cfg);
    if errors > 0 && cfg.deny {
        deny_build(errors);
    }
    errors
}

fn collect_context() -> Option<(PathBuf, Config, Vec<PathBuf>)> {
    let manifest_dir = resolve_manifest_dir();
    let root = gateway::find_workspace_root(&manifest_dir);
    let cfg = load_config(&root);
    if !cfg.enabled {
        return None;
    }
    let files = gateway::collect_project_files(&root, &manifest_dir, &cfg.topology, &cfg.exclude);
    if files.is_empty() {
        return None;
    }
    Some((root, cfg, files))
}

fn resolve_manifest_dir() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn load_config(root: &PathBuf) -> Config {
    Config::from_content(gateway::read_config(root).as_deref())
}

fn scan_sources(
    rs_files: &[PathBuf],
    cfg: &Config,
) -> (usize, Vec<(PathBuf, String)>) {
    let mut errors = 0;
    let mut contents: Vec<(PathBuf, String)> = Vec::new();
    for path in rs_files {
        let Some(content) = gateway::read_text(path) else {
            continue;
        };
        for iss in scanner::scan_file(path, &content, cfg) {
            emit_issue(&iss);
            errors += 1;
        }
        contents.push((path.clone(), content));
    }
    (errors, contents)
}

fn run_cross_checks(contents: &[(PathBuf, String)], cfg: &Config) -> usize {
    let mut cross: Vec<Issue> = Vec::new();
    if cfg.check_sibling_import {
        checks::sibling_import::check(contents, &mut cross);
    }
    if cfg.check_duplicate_pub_fn {
        checks::duplicate_pub_fn::check(contents, &mut cross);
    }
    let mut errors = 0;
    for iss in &cross {
        emit_issue(iss);
        if matches!(iss.severity, issue::Severity::Error) {
            errors += 1;
        }
    }
    errors
}

fn emit_issue(iss: &Issue) {
    let _ = writeln!(std::io::stdout(), "cargo:warning={}", iss);
}

fn deny_build(count: usize) {
    let _ = writeln!(std::io::stdout(), "{}", DENY_MSG.replace("{}", &count.to_string()));
    std::process::exit(1);
}

/// Scan the project at `root` and return all issues.
///
/// Unlike `scan_project()`, this does not use `CARGO_MANIFEST_DIR` and
/// does not emit `cargo:warning=` lines — suitable for standalone CLI use.
pub fn scan_at(root: &Path) -> Vec<Issue> {
    let Some((cfg, files)) = prepare_scan(root) else { return vec![]; };
    let (mut issues, contents) = scan_files(&files, &cfg);
    run_cross_file_checks(&contents, &cfg, &mut issues);
    issues
}

fn prepare_scan(root: &Path) -> Option<(Config, Vec<PathBuf>)> {
    let actual_root = gateway::find_workspace_root(root);
    let cfg = Config::from_content(gateway::read_config(&actual_root).as_deref());
    if !cfg.enabled {
        return None;
    }
    let files = gateway::collect_project_files(&actual_root, &actual_root, &cfg.topology, &cfg.exclude);
    if files.is_empty() {
        return None;
    }
    Some((cfg, files))
}

fn scan_files(files: &[PathBuf], cfg: &Config) -> (Vec<Issue>, Vec<(PathBuf, String)>) {
    let mut issues: Vec<Issue> = Vec::new();
    let mut contents: Vec<(PathBuf, String)> = Vec::new();
    for path in files {
        let Some(content) = gateway::read_text(path) else { continue; };
        issues.extend(scanner::scan_file(path, &content, cfg));
        contents.push((path.clone(), content));
    }
    (issues, contents)
}

fn run_cross_file_checks(
    contents: &[(PathBuf, String)],
    cfg: &Config,
    issues: &mut Vec<Issue>,
) {
    if cfg.check_sibling_import {
        checks::sibling_import::check(contents, issues);
    }
    if cfg.check_duplicate_pub_fn {
        checks::duplicate_pub_fn::check(contents, issues);
    }
}
