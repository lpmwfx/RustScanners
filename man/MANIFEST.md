# Documentation Index

Generated: 2026-03-13T22:06:28.196029800+01:00  
Project: `RustScanners`  
Coverage: **25/25** items documented (**100%**)

## Files

| Source File | Items | Undocumented |
|---|---|---|
| [src/bin/file_size.rs](man/src/bin/file_size.md) | 0 | — |
| [src/bin/nesting.rs](man/src/bin/nesting.md) | 0 | — |
| [src/bin/secrets.rs](man/src/bin/secrets.md) | 0 | — |
| [src/checks/doc_comments.rs](man/src/checks/doc_comments.md) | 1 | ✓ |
| [src/checks/hardcoded_durations.rs](man/src/checks/hardcoded_durations.md) | 1 | ✓ |
| [src/checks/magic_numbers.rs](man/src/checks/magic_numbers.md) | 1 | ✓ |
| [src/checks/mod.rs](man/src/checks/mod.md) | 6 | ✓ |
| [src/checks/string_states.rs](man/src/checks/string_states.md) | 1 | ✓ |
| [src/checks/unsafe_no_comment.rs](man/src/checks/unsafe_no_comment.md) | 1 | ✓ |
| [src/checks/unwrap_panic.rs](man/src/checks/unwrap_panic.md) | 1 | ✓ |
| [src/config.rs](man/src/config.md) | 2 | ✓ |
| [src/context.rs](man/src/context.md) | 5 | ✓ |
| [src/issue.rs](man/src/issue.md) | 3 | ✓ |
| [src/lib.rs](man/src/lib.md) | 3 | ✓ |

## All Items

| Item | Kind | Source | Line | Documented |
|---|---|---|---|---|
| `check` | fn | src/checks/doc_comments.rs | 31 | ✓ |
| `check` | fn | src/checks/hardcoded_durations.rs | 20 | ✓ |
| `check` | fn | src/checks/magic_numbers.rs | 42 | ✓ |
| `magic_numbers` | mod | src/checks/mod.rs | 2 | ✓ |
| `hardcoded_durations` | mod | src/checks/mod.rs | 4 | ✓ |
| `string_states` | mod | src/checks/mod.rs | 6 | ✓ |
| `unwrap_panic` | mod | src/checks/mod.rs | 8 | ✓ |
| `unsafe_no_comment` | mod | src/checks/mod.rs | 10 | ✓ |
| `doc_comments` | mod | src/checks/mod.rs | 12 | ✓ |
| `check` | fn | src/checks/string_states.rs | 31 | ✓ |
| `check` | fn | src/checks/unsafe_no_comment.rs | 43 | ✓ |
| `check` | fn | src/checks/unwrap_panic.rs | 32 | ✓ |
| `Config` | struct | src/config.rs | 6 | ✓ |
| `load` | fn | src/config.rs | 53 | ✓ |
| `FileContext` | struct | src/context.rs | 4 | ✓ |
| `new` | fn | src/context.rs | 11 | ✓ |
| `is_test_context` | fn | src/context.rs | 36 | ✓ |
| `is_const_def` | fn | src/context.rs | 51 | ✓ |
| `is_comment` | fn | src/context.rs | 67 | ✓ |
| `Severity` | enum | src/issue.rs | 6 | ✓ |
| `Issue` | struct | src/issue.rs | 12 | ✓ |
| `error` | fn | src/issue.rs | 23 | ✓ |
| `checks` | mod | src/lib.rs | 8 | ✓ |
| `scan_project` | fn | src/lib.rs | 43 | ✓ |
| `scan_file` | fn | src/lib.rs | 118 | ✓ |
