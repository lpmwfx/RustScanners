//! Child module extraction adviser — rust/modules.md
//!
//! Detects inline `mod name { ... }` blocks and advises extraction to file.
//! Inline modules should be small — large ones reduce readability and hurt AI context.
//!
//! Thresholds:
//!   - warn_at (default 100): "Plan extraction now"
//!   - error_at (default 150): "Extract immediately"

use regex::Regex;
use std::sync::LazyLock;

use crate::context::FileContext;
use crate::issue::Issue;

const RULE: &str = "rust/modules/extract-child";

static MOD_START: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:pub\s+)?mod\s+(\w+)\s*\{").unwrap());

/// Scan for large inline child modules and advise extraction.
pub fn check(
    ctx: &FileContext,
    lines: &[&str],
    issues: &mut Vec<Issue>,
    warn_at: usize,
    error_at: usize,
) {
    // Find all inline mod declarations
    for (start_idx, raw) in lines.iter().enumerate() {
        let Some(caps) = MOD_START.captures(raw) else {
            continue;
        };

        let mod_name = caps.get(1).unwrap().as_str();

        // Find matching closing brace
        if let Some(end_idx) = find_closing_brace(lines, start_idx) {
            let lines_count = end_idx - start_idx;

            // Emit advisory if large
            if lines_count >= error_at {
                let msg = format!(
                    "inline module '{}' has {} lines — EXTRACT IMMEDIATELY to {}.rs.\n\
                     Single file = single responsibility. Move code to:\n\
                     \n  mod {};\n\
                     \n Then create src/{}.rs with module contents.",
                    mod_name, lines_count, mod_name, mod_name, mod_name
                );
                issues.push(Issue::error(ctx.path, start_idx + 1, 1, RULE, msg));
            } else if lines_count >= warn_at {
                let msg = format!(
                    "inline module '{}' has {} lines — plan extraction to {}.rs.\n\
                     Large modules hurt readability and AI context. Structure:\n\
                     \n  mod {};\n\
                     \n  // {}.rs contains: parsing, validation, or domain logic\n\
                     Each file should do one thing well.",
                    mod_name, lines_count, mod_name, mod_name, mod_name
                );
                issues.push(Issue::warning(ctx.path, start_idx + 1, 1, RULE, msg));
            }
        }
    }
}

/// Find matching closing brace for opening brace on line `start_idx`.
/// Returns the line index of the closing brace, or None if not found.
fn find_closing_brace(lines: &[&str], start_idx: usize) -> Option<usize> {
    let start_line = lines[start_idx];
    let open_count = start_line.chars().filter(|&c| c == '{').count();
    let close_count = start_line.chars().filter(|&c| c == '}').count();

    let mut depth = open_count as i32 - close_count as i32;

    // Continue scanning from the line after the mod declaration
    for (idx, line) in lines.iter().enumerate().skip(start_idx + 1) {
        depth += line.chars().filter(|&c| c == '{').count() as i32;
        depth -= line.chars().filter(|&c| c == '}').count() as i32;

        if depth == 0 {
            return Some(idx);
        }

        // Safety: don't scan forever
        if idx - start_idx > 500 {
            return None;
        }
    }

    None
}
