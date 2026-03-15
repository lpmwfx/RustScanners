//! Per-file scanner — receives pre-loaded content, performs no IO.
//!
//! `scan_file` dispatches to each enabled check and returns all issues found.

use std::path::Path;

use crate::checks;
use crate::config::Config;
use crate::context;
use crate::issue::Issue;

/// Scan a single `.rs` file given its pre-loaded `content`.
/// Returns all issues found according to `cfg`.
pub fn scan_file(path: &Path, content: &str, cfg: &Config) -> Vec<Issue> {
    let lines: Vec<&str> = content.lines().collect();
    let file_ctx = context::FileContext::new(path, &lines);

    let mut issues = Vec::new();

    if cfg.check_magic_numbers {
        checks::magic_numbers::check(&file_ctx, &lines, &mut issues);
    }
    if cfg.check_hardcoded_durations {
        checks::hardcoded_durations::check(&file_ctx, &lines, &mut issues);
    }
    if cfg.check_string_states {
        checks::string_states::check(&file_ctx, &lines, &mut issues);
    }
    if cfg.check_unwrap_panic {
        checks::unwrap_panic::check(&file_ctx, &lines, &mut issues);
    }
    if cfg.check_unsafe_no_comment {
        checks::unsafe_no_comment::check(&file_ctx, &lines, &mut issues);
    }
    if cfg.check_doc_comments {
        checks::doc_comments::check(&file_ctx, &lines, &mut issues);
    }
    if cfg.check_child_module_size {
        checks::child_module_size::check(
            &file_ctx,
            &lines,
            &mut issues,
            cfg.child_module_warn_at,
            cfg.child_module_error_at,
        );
    }
    if cfg.check_shared_guard {
        checks::shared_guard::check(&file_ctx, &lines, &mut issues);
    }

    issues
}
