//! `rustscanners` — standalone CLI scanner for Rust projects.
//!
//! Usage: rustscanners [PATH]
//!
//! Scans the project at PATH (or current directory) and prints violations
//! in `file:line:col: severity rule: message` format.
//! Exits 1 if any error-level issues are found.

use std::path::PathBuf;

use rustscanners::{Severity, scan_at};

fn main() {
    let root = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or(PathBuf::from(".")));

    let issues = scan_at(&root);
    let has_errors = issues.iter().any(|i| matches!(i.severity, Severity::Error));
    for iss in &issues {
        println!("{}", iss);
    }
    if has_errors {
        std::process::exit(1);
    }
}
