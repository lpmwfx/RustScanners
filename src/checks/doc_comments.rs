//! Enforce /// doc comments on all public items — rust/docs.md
//!
//! REQUIRED on:
//!   pub fn, pub struct, pub enum, pub trait, pub type, pub mod, pub const/static
//!   (including pub(crate) and pub(super))
//!
//! EXEMPT:
//!   - test files
//!   - #[test] / #[cfg(test)] contexts
//!   - pub use re-exports

use regex::Regex;
use std::sync::LazyLock;

use crate::context::FileContext;
use crate::issue::Issue;

const RULE: &str = "rust/docs/doc-required";

static PUB_ITEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^\s*pub(?:\([^)]+\))?\s+(?:async\s+)?(?:fn|struct|enum|trait|type|mod|const|static)\s+(\w+)",
    )
    .unwrap()
});

static ATTRIBUTE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*#\[").unwrap());

pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>) {
    if ctx.is_test_file {
        return;
    }

    for (idx, line) in lines.iter().enumerate() {
        let Some(cap) = PUB_ITEM.captures(line) else {
            continue;
        };
        let name = &cap[1];

        // Skip pub use re-exports
        let trimmed = line.trim_start();
        if trimmed.contains(" use ") {
            continue;
        }

        if !has_doc_comment(lines, idx) {
            issues.push(Issue::error(
                ctx.path,
                idx + 1,
                1,
                RULE,
                format!(
                    "pub item `{}` is missing a `///` doc comment — add one above the declaration",
                    name
                ),
            ));
        }
    }
}

/// Walk backwards from the line above `item_idx`.
/// Skip `#[...]` attribute lines.
/// Return true if we hit a `///` line before hitting a non-comment, non-blank line.
fn has_doc_comment(lines: &[&str], item_idx: usize) -> bool {
    let mut i = item_idx;
    while i > 0 {
        i -= 1;
        let trimmed = lines[i].trim();
        if trimmed.is_empty() {
            return false;
        }
        if trimmed.starts_with("///") {
            return true;
        }
        if ATTRIBUTE.is_match(lines[i]) {
            // Keep scanning upward through attribute stack
            continue;
        }
        return false;
    }
    false
}
