//! Hardcoded file path checks — rust/constants.md
//!
//! BANNED: String literals ending in .json/.toml/.yaml/.txt/.png/.svg/.wasm
//!         outside const/static definition lines.
//! Exempt: const/static defs, tests, comments, full paths (contain / or \).

use regex::Regex;
use std::sync::LazyLock;

use crate::context::{self, FileContext};
use crate::issue::Issue;

const RULE: &str = "rust/constants/no-hardcoded-path";

static FILE_STRING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#""([^"]*\.(?:json|toml|yaml|yml|txt|png|svg|wasm|ron))""#).unwrap()
});

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

        for cap in FILE_STRING.captures_iter(raw) {
            let filename = &cap[1];
            // Skip full paths — test fixtures or doc examples
            if filename.contains('/') || filename.contains('\\') {
                continue;
            }
            issues.push(Issue::error(
                ctx.path,
                lineno,
                cap.get(0).unwrap().start() + 1,
                RULE,
                format!(
                    "hardcoded path \"{}\" \u{2014} all filenames must be named constants \
                     in paths module, e.g. const NAME: &str = \"{}\";",
                    filename, filename
                ),
            ));
        }
    }
}
