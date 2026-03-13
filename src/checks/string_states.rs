//! String state checks — rust/types.md
//!
//! BANNED:
//!   - match expr { "foo" => ... } — match on string literal (use enum)
//!   - if x == "foo" where the string looks like an identifier/kind value
//!
//! Exempt: tests, const/static defs, comments, messages (contain spaces/punctuation).

use regex::Regex;
use std::sync::LazyLock;

use crate::context::{self, FileContext};
use crate::issue::Issue;

const RULE_MATCH: &str = "rust/types/no-string-match";
const RULE_COMPARE: &str = "rust/types/no-string-compare";

// match arm on a string literal: "foo" =>
static MATCH_STR_ARM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""([^"]{2,}?)"\s*=>"#).unwrap());

// == "identifier-like" or != "identifier-like"
static EQ_STR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[!=]=\s*"([^"]{2,}?)""#).unwrap());

// Strings that are messages/paths, not identifiers
static SKIP_VALUES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:\s|[/\\.]|[A-Z]{2}|[?!,])").unwrap());

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
        if context::is_test_context(lines, idx) {
            continue;
        }

        // match "foo" =>
        for cap in MATCH_STR_ARM.captures_iter(raw) {
            let val = &cap[1];
            if SKIP_VALUES.is_match(val) {
                continue;
            }
            issues.push(Issue::error(
                ctx.path,
                lineno,
                cap.get(0).unwrap().start() + 1,
                RULE_MATCH,
                format!(
                    "stringly-typed match \"{}\" \u{2014} define an enum variant \
                     instead of a raw string literal in match arms",
                    val
                ),
            ));
        }

        // == "identifier"
        for cap in EQ_STR.captures_iter(raw) {
            let val = &cap[1];
            if SKIP_VALUES.is_match(val) {
                continue;
            }
            issues.push(Issue::error(
                ctx.path,
                lineno,
                cap.get(0).unwrap().start() + 1,
                RULE_COMPARE,
                format!(
                    "stringly-typed comparison == \"{}\" \u{2014} discriminators must be \
                     enums or named consts, not raw string literals",
                    val
                ),
            ));
        }
    }
}
