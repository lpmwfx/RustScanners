# `src/lib.rs`

## `pub mod checks;`
*Line 8 · mod*

Collection of RulesTools static analysis checks — zero-literals, unwrap, doc comments, and more.

---

## `pub fn scan_project() -> usize`
*Line 43 · fn*

Scan the project and emit `cargo:warning` for each violation.
Call this from `build.rs`.

Returns the total number of errors found.

---

## `pub fn scan_file(path: &Path, cfg: &Config) -> Vec<Issue>`
*Line 118 · fn*

Scan a single `.rs` file and return all issues.

---

