# `src/config.rs`

## `pub struct Config`
*Line 6 · struct*

Scanner configuration — loaded from `proj/rulestools.toml`.

---

## `pub fn load(project_root: &Path) -> Self`
*Line 53 · fn*

Load configuration from `proj/rulestools.toml` in the project root.
Falls back to defaults if the file doesn't exist.

---

