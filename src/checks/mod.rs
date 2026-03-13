/// Zero-literal enforcement for magic numbers — rust/constants.md
pub mod magic_numbers;
/// Detect hardcoded Duration literals — rust/constants.md
pub mod hardcoded_durations;
/// Detect string literal state matching — rust/types.md
pub mod string_states;
/// Detect unwrap() and panic!() in non-test code — rust/errors.md
pub mod unwrap_panic;
/// Require // SAFETY: comments on unsafe blocks — rust/safety.md
pub mod unsafe_no_comment;
/// Enforce /// doc comments on all pub items — rust/docs.md
pub mod doc_comments;
