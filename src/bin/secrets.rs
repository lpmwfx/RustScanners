//! Hardcoded secrets checker — port of common/secrets.py
//!
//! Flags credential-like literals assigned directly in source code.
//! Secrets belong in ~/.env/ and must never be copied into project files.
//!
//! Output: path:line:col: error global/secrets: message
//! Exits 1 if any violations found.
//!
//! Usage: secrets <file>...

use regex::Regex;
use std::path::Path;
use std::process;
use std::sync::OnceLock;

const RULE: &str = "global/secrets";

const SKIP_SUFFIXES: &[&str] = &["md", "txt", "rst", "toml", "example", "template"];
const SKIP_PARTS: &[&str] = &["test", "tests", "fixtures", "examples", "docs", "__pycache__"];

fn secret_key_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r#"(?xi)
        \b(
            password | passwd | pwd |
            api[-_]?key | apikey |
            secret[-_]?key | client[-_]?secret |
            access[-_]?token | auth[-_]?token | bearer[-_]?token |
            private[-_]?key | signing[-_]?key |
            database[-_]?url | db[-_]?password |
            aws[-_]?secret | aws[-_]?access | aws[-_]?key
        )\s*[=:]\s*["'][^"']{4,}["']
    "#).unwrap())
}

fn pem_key_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"-----BEGIN\s+(RSA|EC|OPENSSH|PRIVATE)\s+PRIVATE KEY-----").unwrap()
    })
}

fn should_skip(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if SKIP_SUFFIXES.contains(&ext.to_lowercase().as_str()) {
            return true;
        }
    }
    path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .map(|s| SKIP_PARTS.contains(&s.to_lowercase().as_str()))
            .unwrap_or(false)
    })
}

fn check_file(path_str: &str) -> usize {
    let path = Path::new(path_str);
    if should_skip(path) {
        return 0;
    }

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cannot read {path_str}: {e}");
            return 0;
        }
    };

    let secret_re = secret_key_re();
    let pem_re = pem_key_re();
    let mut errors = 0;

    for (idx, line) in content.lines().enumerate() {
        let lineno = idx + 1;
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') || trimmed.starts_with("//") || trimmed.starts_with('*') {
            continue;
        }

        if let Some(caps) = secret_re.captures(line) {
            let key_name = caps.get(1).map(|m| m.as_str()).unwrap_or("credential");
            let col = caps.get(0).map(|m| m.start() + 1).unwrap_or(1);
            println!(
                "{path_str}:{lineno}:{col}: error {RULE}: \
                 hardcoded credential '{key_name}' — \
                 move to ~/.env/ and load via environment variable"
            );
            errors += 1;
        }

        if let Some(m) = pem_re.find(line) {
            println!(
                "{path_str}:{lineno}:{}: error {RULE}: \
                 PEM private key in source file — must never be committed",
                m.start() + 1
            );
            errors += 1;
        }
    }
    errors
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: secrets <file>...");
        process::exit(2);
    }

    let errors: usize = args.iter().map(|p| check_file(p)).sum();
    if errors > 0 {
        process::exit(1);
    }
}
