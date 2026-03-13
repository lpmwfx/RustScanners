//! File size checker — port of common/file_size.py
//!
//! Limits (lines):
//!   .slint  soft=160  hard=200   (strictest — AI loses context)
//!   .js/.ts soft=200  hard=250
//!   .css    soft=120  hard=150
//!   .py     soft=200  hard=250
//!   .rs     soft=240  hard=300
//!   .cpp/.h soft=280  hard=350
//!
//! Output: path:line:col: severity global/file-limits/ext: message
//! Exits 1 if any hard-limit violations found.
//!
//! Usage: file_size <file>...

use std::path::Path;
use std::process;

// (soft, hard)
const LIMITS: &[(&str, usize, usize)] = &[
    ("slint", 160, 200),
    ("js",    200, 250),
    ("ts",    200, 250),
    ("mjs",   200, 250),
    ("css",   120, 150),
    ("scss",  120, 150),
    ("py",    200, 250),
    ("rs",    240, 300),
    ("cpp",   280, 350),
    ("cc",    280, 350),
    ("cxx",   280, 350),
    ("h",     280, 350),
    ("hpp",   280, 350),
];

fn limits_for(ext: &str) -> Option<(usize, usize)> {
    LIMITS.iter().find(|(e, _, _)| *e == ext).map(|&(_, s, h)| (s, h))
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

    let count = content.lines().count();
    let rule = format!("global/file-limits/{ext}");

    if count >= hard {
        println!(
            "{path_str}:{count}:1: error {rule}: \
             file has {count} lines — hard limit is {hard}. \
             Split the module before adding anything."
        );
        return true;
    }
    if count >= soft {
        println!(
            "{path_str}:{count}:1: warning {rule}: \
             file has {count} lines — approaching limit of {hard} \
             (soft={soft}). Plan the split now."
        );
    }
    false
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: file_size <file>...");
        process::exit(2);
    }

    let errors: usize = args.iter().map(|p| check_file(p) as usize).sum();
    if errors > 0 {
        process::exit(1);
    }
}
