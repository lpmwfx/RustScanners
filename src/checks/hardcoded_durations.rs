//! Hardcoded duration checks — rust/constants.md
//!
//! BANNED: Duration::from_secs/millis/nanos/micros/new with literal arguments.
//! Exempt: const/static defs, tests, Duration::from_secs(0).

use regex::Regex;
use std::sync::LazyLock;

use crate::context::{self, FileContext};
use crate::issue::Issue;

const RULE: &str = "rust/constants/no-hardcoded-duration";

static DURATION_LITERAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Duration::(?:from_secs|from_millis|from_nanos|from_micros|new)\s*\(\s*(\d+)")
        .unwrap()
});

/// Scan for hardcoded Duration literals — emit issues for Duration::from_secs/millis/nanos/etc with non-zero arguments.
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

        for cap in DURATION_LITERAL.captures_iter(raw) {
            let val = &cap[1];
            // Duration::from_secs(0) is exempt
            if val == "0" {
                continue;
            }
            issues.push(Issue::error(
                ctx.path,
                lineno,
                cap.get(0).unwrap().start() + 1,
                RULE,
                format!(
                    "hardcoded duration literal {} \u{2014} use named const from state/ module, \
                     e.g. Duration::from_secs(TIMEOUT_SECS)",
                    val
                ),
            ));
        }
    }
}
