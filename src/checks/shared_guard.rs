//! Shared module purity guard — rust/modules/shared.md
//!
//! Files in `shared/` or `common/` directories are topology-free.
//! They must not import any internal project module via `use crate::`.
//!
//! Rationale: shared/ modules are universal leaf nodes — no topological home,
//! usable by any mother or child. Internal imports would reintroduce coupling.
//! Only std and external crates are allowed.

use regex::Regex;
use std::path::Path;

use crate::context;
use crate::context::FileContext;
use crate::issue::Issue;

const RULE: &str = "rust/modules/shared-no-internal-import";
const DIR_SHARED: &str = "shared";
const DIR_COMMON: &str = "common";
const PATTERN_USE_CRATE: &str = r"^\s*use\s+crate::";

/// Returns true if the file lives inside a `shared/` or `common/` directory.
fn is_shared_file(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s == DIR_SHARED || s == DIR_COMMON
    })
}

/// Enforce that shared/ files have zero internal project dependencies.
pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>) {
    if !is_shared_file(ctx.path) {
        return;
    }

    let Ok(re) = Regex::new(PATTERN_USE_CRATE) else {
        return;
    };

    for (idx, line) in lines.iter().enumerate() {
        if context::is_comment(line) {
            continue;
        }
        if re.is_match(line) {
            issues.push(Issue::error(
                ctx.path,
                idx + 1,
                1,
                RULE,
                "shared/ module imports internal project code — shared/ must be topology-free.\n  \
                 Remove `use crate::` or move the dependency to the caller.\n  \
                 shared/ allows only: std, extern crates, other shared/."
                    .to_string(),
            ));
        }
    }
}
