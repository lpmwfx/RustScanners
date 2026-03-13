//! Unwrap/panic checks — rust/errors.md
//!
//! BANNED outside tests:
//!   - .unwrap()         — use `?` or match
//!   - .expect("...")    — use `?` or proper error propagation
//!   - panic!(...)       — only acceptable in main or test
//!   - todo!() / unimplemented!() — must not reach production
//!
//! Exempt: test files, #[test] blocks, #[cfg(test)] blocks.

use regex::Regex;
use std::sync::LazyLock;

use crate::context::{self, FileContext};
use crate::issue::Issue;

const RULE: &str = "rust/errors/no-unwrap-panic";

static UNWRAP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bunwrap\s*\(\s*\)").unwrap());

static EXPECT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bexpect\s*\(").unwrap());

static PANIC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bpanic!\s*[(\[]").unwrap());

static TODO_UNIMPL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:todo|unimplemented)!\s*[(\[]").unwrap());

pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>) {
    if ctx.is_test_file {
        return;
    }

    for (idx, raw) in lines.iter().enumerate() {
        let lineno = idx + 1;

        if context::is_comment(raw) {
            continue;
        }
        if context::is_test_context(lines, idx) {
            continue;
        }

        if let Some(m) = UNWRAP.find(raw) {
            issues.push(Issue::error(
                ctx.path, lineno, m.start() + 1, RULE,
                ".unwrap() — use `?` or handle the error explicitly".to_string(),
            ));
        }

        if let Some(m) = EXPECT.find(raw) {
            // Allow .expect() inside const/static (e.g. LazyLock regex compilation)
            if !context::is_const_def(raw) {
                issues.push(Issue::error(
                    ctx.path, lineno, m.start() + 1, RULE,
                    ".expect() — use `?` or return a typed error".to_string(),
                ));
            }
        }

        if let Some(m) = PANIC.find(raw) {
            issues.push(Issue::error(
                ctx.path, lineno, m.start() + 1, RULE,
                "panic!() — panics must not reach production code outside main".to_string(),
            ));
        }

        if let Some(m) = TODO_UNIMPL.find(raw) {
            issues.push(Issue::error(
                ctx.path, lineno, m.start() + 1, RULE,
                "todo!()/unimplemented!() — remove before shipping".to_string(),
            ));
        }
    }
}
