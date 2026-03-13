# `src/checks/doc_comments.rs`

## `pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>)`
*Line 31 · fn*

Scan for all pub items without /// doc comments — emit issues for any undocumented public function, struct, enum, trait, type, mod, const, or static.

---

