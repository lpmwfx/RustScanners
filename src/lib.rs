//! # RustScanners
//!
//! Zero-literal static analysis scanners for Rust projects.
//! Add as a `[build-dependencies]` and call `rustscanners::scan_project()`
//! from `build.rs` to enforce rules during `cargo build`.

/// Collection of RulesTools static analysis checks.
pub mod checks;
mod config;
mod context;
mod gateway;
mod issue;
mod project;
mod scanner;

pub use config::Config;
pub use issue::Issue;
pub use project::scan_project;
