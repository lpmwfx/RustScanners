# `src/checks/magic_numbers.rs`

## `pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>)`
*Line 42 · fn*

Scan for magic number literals in function bodies — emit issues for all numbers except 0 and 1.

---

