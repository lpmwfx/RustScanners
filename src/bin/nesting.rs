//! Nesting depth checker — port of common/nesting.py
//!
//! Brace-depth tracking with string/comment stripping.
//! Works for Rust, JS/TS, Slint, C#, CSS, Kotlin.
//!
//! Depth thresholds (max_abs_depth) per language:
//!   .rs              5   (impl + fn + 3 logic levels)
//!   .js/.ts/.mjs     4   (fn + 3 logic levels)
//!   .slint           6
//!   .cs              7   (namespace + class + method + 3 levels)
//!   .css/.scss       4
//!   .kt/.kts         6
//!
//! depth == max_abs_depth  → warning
//! depth >  max_abs_depth  → error
//!
//! Usage: nesting <file>...

use regex::Regex;
use std::path::Path;
use std::process;
use std::sync::OnceLock;

const RULE: &str = "global/nesting";

// (extension, has_backtick_strings, max_abs_depth)
const LANG_MAP: &[(&str, bool, usize)] = &[
    ("rs",    false, 5),
    ("js",    true,  4),
    ("ts",    true,  4),
    ("tsx",   true,  4),
    ("mjs",   true,  4),
    ("slint", false, 6),
    ("cs",    false, 7),
    ("css",   false, 4),
    ("scss",  false, 4),
    ("kt",    false, 6),
    ("kts",   false, 6),
];

fn lang_config(ext: &str) -> Option<(bool, usize)> {
    LANG_MAP.iter()
        .find(|(e, _, _)| *e == ext)
        .map(|&(_, backticks, max)| (backticks, max))
}

fn re_dq() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r#""(?:[^"\\]|\\.)*""#).unwrap())
}
fn re_sq() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"'(?:[^'\\]|\\.)*'").unwrap())
}
fn re_bt() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"`(?:[^`\\]|\\.)*`").unwrap())
}

fn strip_strings_and_comments(line: &str, has_backticks: bool) -> String {
    let mut s = re_dq().replace_all(line, r#""""#).into_owned();
    s = re_sq().replace_all(&s, "''").into_owned();
    if has_backticks {
        s = re_bt().replace_all(&s, "``").into_owned();
    }
    if let Some(pos) = s.find("//") {
        s.truncate(pos);
    }
    s
}

fn check_file(path_str: &str) -> usize {
    let path = Path::new(path_str);
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e.to_lowercase(),
        None => return 0,
    };
    let Some((has_backticks, max_abs_depth)) = lang_config(&ext) else { return 0 };

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => { eprintln!("cannot read {path_str}: {e}"); return 0; }
    };

    let mut depth: usize = 0;
    let mut in_block_comment = false;
    let mut errors = 0;

    for (idx, raw_line) in content.lines().enumerate() {
        let lineno = idx + 1;
        let mut work = raw_line.trim_end().to_owned();

        if in_block_comment {
            if let Some(pos) = work.find("*/") {
                work = work[pos + 2..].to_owned();
                in_block_comment = false;
            } else {
                continue;
            }
        }

        loop {
            match work.find("/*") {
                None => break,
                Some(start) => match work[start..].find("*/") {
                    Some(rel) => {
                        let end = start + rel + 2;
                        work = format!("{}{}", &work[..start], &work[end..]);
                    }
                    None => {
                        work.truncate(start);
                        in_block_comment = true;
                        break;
                    }
                },
            }
        }

        let clean = strip_strings_and_comments(&work, has_backticks);

        let opens = clean.chars().filter(|&c| c == '{').count();
        let closes = clean.chars().filter(|&c| c == '}').count();

        let net = opens as isize - closes as isize;
        if opens > 0 && net == 0 {
            continue;
        }

        depth = (depth as isize + net).max(0) as usize;

        if opens > 0 && depth >= max_abs_depth {
            let col = raw_line.find('{').map(|p| p + 1).unwrap_or(1);
            let severity = if depth > max_abs_depth { "error" } else { "warning" };
            println!(
                "{path_str}:{lineno}:{col}: {severity} {RULE}: \
                 nesting depth {depth} exceeds limit of {} logic levels — \
                 extract a helper function",
                max_abs_depth - 1
            );
            if depth > max_abs_depth {
                errors += 1;
            }
        }
    }
    errors
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: nesting <file>...");
        process::exit(2);
    }

    let errors: usize = args.iter().map(|p| check_file(p)).sum();
    if errors > 0 {
        process::exit(1);
    }
}
