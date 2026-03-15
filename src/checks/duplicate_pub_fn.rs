//! Duplicate public function detector — rust/modules/shared.md
//!
//! If the same `pub fn` name appears in 2 or more child files,
//! the logic is a late-abstraction candidate for `shared/`.
//!
//! Design principle: shared/ modules emerge from observed repetition,
//! never from upfront design. This check surfaces that signal early.
//!
//! Before creating a new shared/ module, verify no equivalent exists already.

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::issue::Issue;

const RULE: &str = "rust/modules/extract-to-shared";
const DIR_SHARED: &str = "shared";
const DIR_COMMON: &str = "common";
const MOTHER_LIB: &str = "lib.rs";
const MOTHER_MAIN: &str = "main.rs";
const MOTHER_MOD: &str = "mod.rs";
const MIN_CHILD_COUNT: usize = 2;
const PATTERN_PUB_FN: &str = r"^\s*pub\s+fn\s+([a-zA-Z_][a-zA-Z0-9_]*)";

fn is_child(path: &Path) -> bool {
    let name = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return false,
    };
    if name == MOTHER_LIB || name == MOTHER_MAIN || name == MOTHER_MOD {
        return false;
    }
    !path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s == DIR_SHARED || s == DIR_COMMON
    })
}

/// Cross-file check: warn when the same pub fn name appears in multiple child files.
/// Receives pre-loaded file contents — no IO performed here.
pub fn check(files: &[(PathBuf, String)], issues: &mut Vec<Issue>) {
    let Ok(re) = Regex::new(PATTERN_PUB_FN) else {
        return;
    };

    let mut seen: HashMap<String, Vec<(PathBuf, usize)>> = HashMap::new();

    for (path, content) in files {
        if !is_child(path) {
            continue;
        }
        let mut seen_in_file: HashSet<String> = HashSet::new();
        for (idx, line) in content.lines().enumerate() {
            let Some(caps) = re.captures(line) else {
                continue;
            };
            let fn_name = caps[1].to_string();
            if seen_in_file.insert(fn_name.clone()) {
                seen.entry(fn_name).or_default().push((path.clone(), idx + 1));
            }
        }
    }

    for (fn_name, locations) in &seen {
        let unique_files: HashSet<&PathBuf> = locations.iter().map(|(f, _)| f).collect();

        if unique_files.len() < MIN_CHILD_COUNT {
            continue;
        }

        // Strategy/plugin pattern: all files in the same directory share a
        // function name by convention (e.g. checks/*/pub fn check) — not a
        // candidate for extraction.
        let unique_dirs: HashSet<Option<&Path>> =
            unique_files.iter().map(|f| f.parent()).collect();
        if unique_dirs.len() == 1 {
            continue;
        }

        let mut reported: HashSet<&PathBuf> = HashSet::new();

        for (file, lineno) in locations {
            if !reported.insert(file) {
                continue;
            }
            let others: Vec<String> = unique_files
                .iter()
                .filter(|&&f| f != file)
                .map(|f| f.display().to_string())
                .collect();
            issues.push(Issue::error(
                file,
                *lineno,
                1,
                RULE,
                format!(
                    "pub fn `{}` also defined in: {}\n  \
                     Same name in multiple children — candidate for shared/.\n  \
                     Check shared/ for existing similar logic before extracting.",
                    fn_name,
                    others.join(", "),
                ),
            ));
        }
    }
}
