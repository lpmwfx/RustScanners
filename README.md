# RustScanners

Zero-literal static analysis for Rust projects. Enforces the [rust/constants](https://github.com/lpmwfx/Rules/blob/main/rust/constants.md) rule: **no hardcoded values in function bodies**.

Runs automatically during `cargo build` via `build.rs`.

## What it catches

| Scanner | Rule | Example violation |
|---------|------|-------------------|
| **magic_numbers** | All integers >= 2 and floats except 0.0/1.0 | `let x = a + 2;` |
| **hardcoded_durations** | Duration constructors with literal args | `Duration::from_secs(30)` |
| **hardcoded_urls** | URL string literals | `"https://api.example.com"` |
| **hardcoded_paths** | File path string literals | `"config.toml"` |
| **string_states** | String literals as discriminators | `match x { "active" => ... }` |
| **child_module_size** | Inline modules > 100 lines (advisory) | `mod parser { ... }` (150+ lines) |

## 6 exemptions

Bare literals are allowed in these constructs:

1. `0` and `1` (indexing, ranges, arithmetic)
2. `const`/`static` definitions (these ARE the named values)
3. Test code (`#[test]`, `#[cfg(test)]`)
4. Format/log macro strings (`format!`, `println!`, `tracing::info!`, etc.)
5. Derive/attribute macros
6. Enum variant discriminants (`Active = 1`)

## Install

```bash
curl -sSf https://raw.githubusercontent.com/lpmwfx/RustScanners/main/install.sh | bash
```

Or manually:

**1. Add to Cargo.toml:**
```toml
[build-dependencies]
rustscanners = { git = "https://github.com/lpmwfx/RustScanners" }
```

**2. Create or edit build.rs:**
```rust
fn main() {
    rustscanners::scan_project();
}
```

**3. Configure in proj/rulestools.toml:**
```toml
[rustscanners]
enabled = true
deny = false          # true = build fails on violations

magic_numbers = true
hardcoded_durations = true
hardcoded_urls = true
hardcoded_paths = true
string_states = true
child_module_size = true

# Child module extraction thresholds (lines)
child_module_warn_at = 100      # "Plan extraction now" (warning)
child_module_error_at = 150     # "Extract immediately" (error)

# Optional: exclude paths from scanning (glob patterns)
exclude = ["target/*", "**/vendor/*"]
```

## How it works

`rustscanners::scan_project()` is called from `build.rs` during `cargo build`. It:

1. Reads `proj/rulestools.toml` for configuration
2. Walks `src/` for `.rs` files
3. Runs regex-based scanners on each file
4. Emits `cargo:warning` for each violation
5. If `deny = true`, panics to fail the build

## The zero-literal rule

Every value in function bodies must be a named reference:

```rust
// BANNED
let timeout = Duration::from_secs(30);
let buf = Vec::with_capacity(1024);
if retries > 3 { ... }

// CORRECT
const TIMEOUT_SECS: u64 = 30;
const BUF_SIZE: usize = 1024;
const MAX_RETRIES: u32 = 3;

let timeout = Duration::from_secs(TIMEOUT_SECS);
let buf = Vec::with_capacity(BUF_SIZE);
if retries > MAX_RETRIES { ... }
```

Values live in `src/state/` modules (one file per concern) or `_cfg` structs for runtime config.

## Child module extraction adviser

Large inline `mod { ... }` blocks hurt readability and reduce AI context windows. RustScanners advises:

- **100+ lines**: ⚠️ Warning — "Plan extraction to file"
- **150+ lines**: ❌ Error — "Extract immediately"

**Structure:**
```rust
// Before: inline module bloats parent
mod parser {
    // 120 lines of parsing logic...
}

// After: each file = one responsibility
mod parser;

// parser.rs contains the 120-line module
pub fn parse(input: &str) -> Result<Ast, Error> { ... }
```

Each `.rs` file should do **one thing well** (parsing, validation, serialization, domain logic). This keeps code readable and AI context sharp.

## License

MIT


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=2" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
