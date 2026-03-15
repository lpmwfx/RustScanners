//! Sibling import detector — rust/modules/mother-child.md
//!
//! A child must not import from a sibling child.
//! Exception: imports from `shared/` or `common/` are always allowed —
//! those are topology-free universal modules.
//!
//! Topology:
//!   child → sibling child  : ERROR — route through mother or extract to shared/
//!   child → shared/        : OK
//!   mother → any           : OK (skipped)

use regex::Regex;
use std::path::{Path, PathBuf};

use crate::issue::Issue;

const RULE: &str = "rust/modules/no-sibling-import";
const DIR_SHARED: &str = "shared";
const DIR_COMMON: &str = "common";
const DIR_SRC: &str = "src";
const MOTHER_LIB: &str = "lib.rs";
const MOTHER_MAIN: &str = "main.rs";
const MOTHER_MOD: &str = "mod.rs";
const PATTERN_USE_CRATE: &str = r"^\s*use\s+crate::([a-zA-Z_][a-zA-Z0-9_]*)";

/// Returns the parent module name if the file is a child in a subdirectory,
/// or `None` if it should be skipped (mother, shared path, or top-level).
fn child_module(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_str()?;
    if name == MOTHER_LIB || name == MOTHER_MAIN || name == MOTHER_MOD {
        return None;
    }
    if path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s == DIR_SHARED || s == DIR_COMMON
    }) {
        return None;
    }
    let parent = path.parent()?.file_name()?.to_str()?.to_string();
    if parent == DIR_SRC {
        return None;
    }
    Some(parent)
}

/// Cross-file check: emit ERROR when a child imports a sibling child.
/// Receives pre-loaded file contents — no IO performed here.
pub fn check(files: &[(PathBuf, String)], issues: &mut Vec<Issue>) {
    let Ok(re) = Regex::new(PATTERN_USE_CRATE) else {
        return;
    };

    for (path, content) in files {
        let Some(parent_mod) = child_module(path) else {
            continue;
        };

        for (idx, line) in content.lines().enumerate() {
            if line.trim_start().starts_with("//") {
                continue;
            }
            let Some(caps) = re.captures(line) else {
                continue;
            };
            let imported_first = &caps[1];
            if imported_first == DIR_SHARED || imported_first == DIR_COMMON {
                continue;
            }
            if imported_first == parent_mod {
                issues.push(Issue::error(
                    path,
                    idx + 1,
                    1,
                    RULE,
                    format!(
                        "sibling import `use crate::{}::...` in child of `{}`.\n  \
                         Route through mother or extract repeated logic to shared/.\n  \
                         If logic is used in multiple children: move to shared/{}.rs",
                        parent_mod, parent_mod, imported_first,
                    ),
                ));
            }
        }
    }
}
