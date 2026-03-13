//! Zero-literal enforcement — rust/constants.md
//!
//! BANNED in function bodies:
//!   - ALL integer literals >= 2
//!   - ALL float literals except 0.0 and 1.0
//!
//! 6 exemptions: 0/1, const/static defs, tests, format macros,
//! derive/attributes, enum variant discriminants.

use regex::Regex;
use std::sync::LazyLock;

use crate::context::{self, FileContext};
use crate::issue::Issue;

const RULE: &str = "rust/constants/no-magic-number";

static FLOAT_LIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(\d+\.\d+)\b").unwrap());

static INT_LIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(\d+)\b").unwrap());

static STRING_LIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""[^"]*""#).unwrap());

static ENUM_VARIANT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*\w+\s*=\s*\d+").unwrap());

static FORMAT_MACRO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(?:format|println|eprintln|print|eprint|write|writeln|tracing::info|tracing::debug|tracing::warn|tracing::error|tracing::trace|info|debug|warn|error|trace|log::info|log::debug|log::warn|log::error|panic|todo|unimplemented|unreachable|assert|assert_eq|assert_ne|anyhow::bail|anyhow::anyhow|bail)!\s*\(",
    )
    .unwrap()
});

fn strip_strings(line: &str) -> String {
    STRING_LIT.replace_all(line, r#""""#).to_string()
}

pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>) {
    if ctx.is_test_file {
        return;
    }

    for (idx, raw) in lines.iter().enumerate() {
        let lineno = idx + 1;

        if context::is_comment(raw) {
            continue;
        }
        if context::is_const_def(raw) {
            continue;
        }
        if ENUM_VARIANT.is_match(raw.trim_start()) {
            continue;
        }
        if context::is_test_context(lines, idx) {
            continue;
        }
        if FORMAT_MACRO.is_match(raw) {
            continue;
        }

        let clean = strip_strings(raw);

        // Float literals
        for cap in FLOAT_LIT.captures_iter(&clean) {
            let val = &cap[1];
            if val == "0.0" || val == "1.0" {
                continue;
            }
            issues.push(Issue::error(
                ctx.path,
                lineno,
                cap.get(1).unwrap().start() + 1,
                RULE,
                format!(
                    "magic number {} \u{2014} use named const or _cfg field, e.g. const NAME: f64 = {};",
                    val, val
                ),
            ));
        }

        // Integer literals >= 2
        let float_positions: Vec<usize> = FLOAT_LIT
            .captures_iter(&clean)
            .filter_map(|c| c.get(1).map(|m| m.start()))
            .collect();

        for cap in INT_LIT.captures_iter(&clean) {
            let m = cap.get(1).unwrap();
            let val = m.as_str();
            let start = m.start();

            // Skip if part of a float
            if float_positions
                .iter()
                .any(|&fp| start.abs_diff(fp) <= val.len() + 1)
            {
                continue;
            }

            // Skip if preceded by: # (attribute), . (field), _ or letter (identifier)
            if start > 0 {
                let prev = clean.as_bytes()[start - 1];
                if prev == b'#' || prev == b'.' || prev == b'_'
                    || prev.is_ascii_alphanumeric()
                {
                    continue;
                }
            }

            // Skip if followed by: . (field/method), _ or letter (identifier suffix)
            let end = m.end();
            if end < clean.len() {
                let next = clean.as_bytes()[end];
                if next == b'.' || next == b'_' || next.is_ascii_alphanumeric() {
                    continue;
                }
            }

            let n: u64 = match val.parse() {
                Ok(v) => v,
                Err(_) => continue,
            };

            // 0 and 1 are universal idioms
            if n <= 1 {
                continue;
            }

            issues.push(Issue::error(
                ctx.path,
                lineno,
                m.start() + 1,
                RULE,
                format!(
                    "magic number {} \u{2014} use named const or _cfg field, e.g. const NAME: usize = {};",
                    val, val
                ),
            ));
        }
    }
}
