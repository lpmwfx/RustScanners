# `src/checks/mod.rs`

## `pub mod magic_numbers;`
*Line 2 · mod*

Zero-literal enforcement for magic numbers — rust/constants.md

---

## `pub mod hardcoded_durations;`
*Line 4 · mod*

Detect hardcoded Duration literals — rust/constants.md

---

## `pub mod string_states;`
*Line 6 · mod*

Detect string literal state matching — rust/types.md

---

## `pub mod unwrap_panic;`
*Line 8 · mod*

Detect unwrap() and panic!() in non-test code — rust/errors.md

---

## `pub mod unsafe_no_comment;`
*Line 10 · mod*

Require // SAFETY: comments on unsafe blocks — rust/safety.md

---

## `pub mod doc_comments;`
*Line 12 · mod*

Enforce /// doc comments on all pub items — rust/docs.md

---

