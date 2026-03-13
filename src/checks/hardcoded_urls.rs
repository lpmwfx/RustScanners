//! Hardcoded URL checks — rust/constants.md
//!
//! BANNED: "http://..." or "https://..." string literals outside const/static.
//! Exempt: const/static defs, tests, comments.

use regex::Regex;
use std::sync::LazyLock;

use crate::context::{self, FileContext};
use crate::issue::Issue;

const RULE: &str = "rust/constants/no-hardcoded-url";

static URL_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""(https?://[^"]+)""#).unwrap());

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

        for cap in URL_STRING.captures_iter(raw) {
            let url = &cap[1];
            let display = if url.len() > 40 {
                format!("{}...", &url[..37])
            } else {
                url.to_string()
            };
            issues.push(Issue::error(
                ctx.path,
                lineno,
                cap.get(0).unwrap().start() + 1,
                RULE,
                format!(
                    "hardcoded URL \"{}\" \u{2014} use named const from state/ module or _cfg field",
                    display
                ),
            ));
        }
    }
}
