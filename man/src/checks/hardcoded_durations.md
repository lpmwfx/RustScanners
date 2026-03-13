# `src/checks/hardcoded_durations.rs`

## `pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>)`
*Line 20 · fn*

Scan for hardcoded Duration literals — emit issues for Duration::from_secs/millis/nanos/etc with non-zero arguments.

---

