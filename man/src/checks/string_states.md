# `src/checks/string_states.rs`

## `pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>)`
*Line 31 · fn*

Scan for string literal state comparisons — detect match arms and == operators on strings that look like identifiers.

---

