//! File size checker — port of common/file_size.py
//!
//! Counts only code lines — comments (// /// //! /* */), blank lines,
//! and pure string literals are excluded from the count.
//!
//! Limits (code lines):
//!   .slint  soft=200  hard=250
//!   .js/.ts soft=200  hard=250
//!   .css    soft=120  hard=150
//!   .py     soft=200  hard=250
//!   .rs     soft=200  hard=250
//!   .cpp/.h soft=280  hard=350
//!
//! Output: path:line:col: severity global/file-limits/ext: message
//! Exits 1 if any hard-limit violations found.
//!
//! Usage: file_size <file>...

use std::path::Path;
use std::process;

const QUOTE_PAIR: usize = 2; // open + close quotes = simple one-liner string literal
const EXIT_USAGE: i32 = 2;   // exit code for bad CLI usage
const EXT_PY: &str = "py";
const EXT_CSS: &str = "css";
const EXT_SCSS: &str = "scss";
const SOFT_STANDARD: usize = 200;
const HARD_STANDARD: usize = 250;
const SOFT_CSS: usize = 120;
const HARD_CSS: usize = 150;
const SOFT_CPP: usize = 280;
const HARD_CPP: usize = 350;

// (soft, hard)
const LIMITS: &[(&str, usize, usize)] = &[
    ("slint", SOFT_STANDARD, HARD_STANDARD),
    ("js",    SOFT_STANDARD, HARD_STANDARD),
    ("ts",    SOFT_STANDARD, HARD_STANDARD),
    ("mjs",   SOFT_STANDARD, HARD_STANDARD),
    ("css",   SOFT_CSS,      HARD_CSS),
    ("scss",  SOFT_CSS,      HARD_CSS),
    ("py",    SOFT_STANDARD, HARD_STANDARD),
    ("rs",    SOFT_STANDARD, HARD_STANDARD),
    ("cpp",   SOFT_CPP,      HARD_CPP),
    ("cc",    SOFT_CPP,      HARD_CPP),
    ("cxx",   SOFT_CPP,      HARD_CPP),
    ("h",     SOFT_CPP,      HARD_CPP),
    ("hpp",   SOFT_CPP,      HARD_CPP),
];

fn limits_for(ext: &str) -> Option<(usize, usize)> {
    LIMITS.iter().find(|(e, _, _)| *e == ext).map(|&(_, s, h)| (s, h))
}

/// C-family extensions that use // and /* */ comments
fn is_c_family(ext: &str) -> bool {
    matches!(ext, "rs" | "slint" | "js" | "ts" | "mjs" | "cpp" | "cc" | "cxx" | "h" | "hpp")
}

/// Count only code lines — skip comments, blanks, and pure string literals.
fn count_code_lines(ext: &str, content: &str) -> usize {
    let mut count = 0;
    let mut is_in_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // blank lines never count
        if trimmed.is_empty() {
            continue;
        }

        if is_c_family(ext) {
            if is_in_block {
                if trimmed.ends_with("*/") {
                    is_in_block = false;
                }
                continue;
            }
            if trimmed.starts_with("/*") {
                if !trimmed.ends_with("*/") {
                    is_in_block = true;
                }
                continue;
            }
            if trimmed.starts_with("//") {
                continue;
            }
            // pure string literal line: "..." or "...";
            if trimmed.starts_with('"') {
                let rest = trimmed.trim_end_matches(';').trim();
                if rest.ends_with('"') && rest.matches('"').count() == QUOTE_PAIR {
                    continue;
                }
            }
        } else if ext == EXT_PY {
            if trimmed.starts_with('#') {
                continue;
            }
            if trimmed.starts_with('"') {
                let rest = trimmed.trim_end_matches(';').trim();
                if rest.ends_with('"') && rest.matches('"').count() == QUOTE_PAIR {
                    continue;
                }
            }
        }
        // css/scss: block comments only
        else if ext == EXT_CSS || ext == EXT_SCSS {
            if is_in_block {
                if trimmed.ends_with("*/") {
                    is_in_block = false;
                }
                continue;
            }
            if trimmed.starts_with("/*") {
                if !trimmed.ends_with("*/") {
                    is_in_block = true;
                }
                continue;
            }
        }

        count += 1;
    }

    count
}

fn check_file(path_str: &str) -> bool {
    let path = Path::new(path_str);
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e.to_lowercase(),
        None => return false,
    };
    let Some((soft, hard)) = limits_for(&ext) else { return false };

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => { eprintln!("cannot read {path_str}: {e}"); return false; }
    };

    let total = content.lines().count();
    let count = count_code_lines(&ext, &content);
    let rule = format!("global/file-limits/{ext}");

    if count >= hard {
        println!(
            "{path_str}:{total}:1: error {rule}: \
             file has {count} code lines (of {total} total) — hard limit is {hard}. \
             Split the module before adding anything."
        );
        return true;
    }
    if count >= soft {
        println!(
            "{path_str}:{total}:1: warning {rule}: \
             file has {count} code lines (of {total} total) — approaching limit of {hard} \
             (soft={soft}). Plan the split now."
        );
    }
    false
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: file_size <file>...");
        process::exit(EXIT_USAGE);
    }

    let errors: usize = args.iter().map(|p| check_file(p) as usize).sum();
    if errors > 0 {
        process::exit(1);
    }
}
