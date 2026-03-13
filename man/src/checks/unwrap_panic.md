# `src/checks/unwrap_panic.rs`

## `pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>)`
*Line 32 · fn*

Scan for unwrap(), expect(), panic!(), todo!(), and unimplemented!() calls outside test code.

---

