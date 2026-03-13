# `src/context.rs`

## `pub struct FileContext<'a>`
*Line 4 · struct*

Shared context for a file being scanned.

---

## `pub fn new(path: &'a Path, _lines: &[&str]) -> Self`
*Line 11 · fn*

Create a new FileContext for a file being scanned, detecting if it's a test file.

---

## `pub fn is_test_context(lines: &[&str], lineno: usize) -> bool`
*Line 36 · fn*

Check if a line number is inside a `#[test]` or `#[cfg(test)]` block.
Scans backwards up to 60 lines looking for test markers.

---

## `pub fn is_const_def(line: &str) -> bool`
*Line 51 · fn*

Check if line is a `const` or `static` definition.

---

## `pub fn is_comment(line: &str) -> bool`
*Line 67 · fn*

Check if line is a comment.

---

