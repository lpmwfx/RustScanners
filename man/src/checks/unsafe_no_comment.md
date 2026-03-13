# `src/checks/unsafe_no_comment.rs`

## `pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>)`
*Line 43 · fn*

Scan for unsafe blocks and functions — require a // SAFETY: comment on an adjacent line explaining the invariant.

---

