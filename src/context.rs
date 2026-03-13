use std::path::Path;

/// Shared context for a file being scanned.
pub struct FileContext<'a> {
    pub path: &'a Path,
    pub is_test_file: bool,
}

impl<'a> FileContext<'a> {
    pub fn new(path: &'a Path, _lines: &[&str]) -> Self {
        Self {
            path,
            is_test_file: is_test_file(path),
        }
    }
}

fn is_test_file(path: &Path) -> bool {
    let parts: Vec<_> = path.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
    for part in &parts {
        if part == "tests" || part == "test" {
            return true;
        }
    }
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if stem.ends_with("_test") || stem.starts_with("test_") {
            return true;
        }
    }
    false
}

/// Check if a line number is inside a `#[test]` or `#[cfg(test)]` block.
/// Scans backwards up to 60 lines looking for test markers.
pub fn is_test_context(lines: &[&str], lineno: usize) -> bool {
    let start = if lineno >= 60 { lineno - 60 } else { 0 };
    for i in (start..lineno.saturating_sub(1)).rev() {
        let trimmed = lines[i].trim();
        if trimmed.contains("#[test]") || trimmed.contains("#[cfg(test)]") {
            return true;
        }
        if trimmed.starts_with("mod tests") || trimmed.starts_with("mod test") {
            return true;
        }
    }
    false
}

/// Check if line is a `const` or `static` definition.
pub fn is_const_def(line: &str) -> bool {
    let trimmed = line.trim_start();
    // pub const / pub static / const / static
    if trimmed.starts_with("const ") || trimmed.starts_with("static ") {
        return true;
    }
    if trimmed.starts_with("pub ") {
        let rest = trimmed[4..].trim_start();
        if rest.starts_with("const ") || rest.starts_with("static ") {
            return true;
        }
    }
    false
}

/// Check if line is a comment.
pub fn is_comment(line: &str) -> bool {
    line.trim_start().starts_with("//")
}
