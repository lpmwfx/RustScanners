//! Unsafe block documentation check — rust/safety.md
//!
//! RULE: Every `unsafe` block or `unsafe fn` must be preceded or followed
//! by a `// SAFETY:` comment on the immediately adjacent line.
//!
//! BANNED:
//!   unsafe { ... }           — without // SAFETY: comment
//!   pub unsafe fn foo() ...  — without // SAFETY: comment
//!
//! Exempt: test files, #[cfg(test)] blocks.

use regex::Regex;
use std::sync::LazyLock;

use crate::context::{self, FileContext};
use crate::issue::Issue;

const RULE: &str = "rust/safety/unsafe-needs-comment";

static UNSAFE_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bunsafe\s*(\{|fn\b)").unwrap());

static SAFETY_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"//\s*SAFETY\s*:").unwrap());

fn has_safety_comment(lines: &[&str], idx: usize) -> bool {
    // Check line before
    if idx > 0 && SAFETY_COMMENT.is_match(lines[idx - 1]) {
        return true;
    }
    // Check same line (inline comment after the unsafe keyword)
    if SAFETY_COMMENT.is_match(lines[idx]) {
        return true;
    }
    // Check line after (rare but valid style)
    if idx + 1 < lines.len() && SAFETY_COMMENT.is_match(lines[idx + 1]) {
        return true;
    }
    false
}

pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>) {
    if ctx.is_test_file {
        return;
    }

    for (idx, raw) in lines.iter().enumerate() {
        if context::is_comment(raw) {
            continue;
        }
        if context::is_test_context(lines, idx) {
            continue;
        }

        if let Some(m) = UNSAFE_BLOCK.find(raw) {
            if !has_safety_comment(lines, idx) {
                let kind = if raw[m.start()..].contains("fn") { "unsafe fn" } else { "unsafe block" };
                issues.push(Issue::error(
                    ctx.path,
                    idx + 1,
                    m.start() + 1,
                    RULE,
                    format!("{kind} requires a `// SAFETY:` comment explaining the invariants"),
                ));
            }
        }
    }
}
