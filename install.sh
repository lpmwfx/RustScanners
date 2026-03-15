#!/usr/bin/env bash
set -euo pipefail

# RustScanners installer — adds zero-literal scanning to cargo build.
#
# Usage:  curl -sSf https://raw.githubusercontent.com/lpmwfx/RustScanners/main/install.sh | bash
#
# What it does:
#   1. Adds rustscanners as a [build-dependencies] entry in Cargo.toml
#   2. Creates or patches build.rs to call rustscanners::scan_project()
#   3. Creates proj/rulestools.toml with default config (if missing)

REPO="https://github.com/lpmwfx/RustScanners"
CRATE_REF='rustscanners = { git = "https://github.com/lpmwfx/RustScanners" }'

echo "=== RustScanners installer ==="

# ── Must be in a Cargo project ──────────────────────────────────────────────
if [ ! -f Cargo.toml ]; then
    echo "ERROR: No Cargo.toml found. Run this from your Rust project root."
    exit 1
fi

# ── 1. Add [build-dependencies] ─────────────────────────────────────────────
if grep -q 'rustscanners' Cargo.toml; then
    echo "[ok] rustscanners already in Cargo.toml"
else
    if grep -q '\[build-dependencies\]' Cargo.toml; then
        # Append under existing [build-dependencies]
        sed -i '/\[build-dependencies\]/a '"$CRATE_REF" Cargo.toml
        echo "[+] Added rustscanners to existing [build-dependencies]"
    else
        # Add new section
        printf '\n[build-dependencies]\n%s\n' "$CRATE_REF" >> Cargo.toml
        echo "[+] Added [build-dependencies] with rustscanners"
    fi
fi

# ── 2. Create or patch build.rs ──────────────────────────────────────────────
SCAN_LINE="    rustscanners::scan_project();"
BUILD_RS_TEMPLATE='fn main() {
    rustscanners::scan_project();
}'

if [ ! -f build.rs ]; then
    echo "$BUILD_RS_TEMPLATE" > build.rs
    echo "[+] Created build.rs"
elif grep -q 'rustscanners' build.rs; then
    echo "[ok] build.rs already calls rustscanners"
else
    # Insert scan_project() as first line inside fn main()
    sed -i '/fn main()/a\'"$SCAN_LINE" build.rs
    echo "[+] Patched build.rs — added rustscanners::scan_project()"
fi

# ── 3. Create default config ────────────────────────────────────────────────
mkdir -p proj
if [ ! -f proj/rulestools.toml ]; then
    cat > proj/rulestools.toml << 'EOF'
[rustscanners]
enabled = true
deny = false                  # true = cargo build fails on violations

# Toggle individual scanners
magic_numbers = true          # integers >= 2, floats except 0.0/1.0
hardcoded_durations = true    # Duration::from_secs(30) etc.
string_states = true          # match "foo" =>, == "kind"
unwrap_panic = true           # .unwrap()/.expect()/panic!()/todo!() outside tests
unsafe_no_comment = true      # unsafe block/fn without // SAFETY: comment
doc_comments = true           # pub items must have /// doc comment
child_module_size = true      # advise extraction of inline modules > 100 lines

# Child module extraction thresholds (lines)
child_module_warn_at = 100    # warning: plan extraction
child_module_error_at = 150   # error: extract immediately

# Mother-child topology checks
shared_guard = true           # ERROR: shared/ files must not use crate:: (topology-free)
sibling_import = true         # WARNING: child must not import sibling child
duplicate_pub_fn = true       # WARNING: same pub fn in 2+ children → extract to shared/
EOF
    echo "[+] Created proj/rulestools.toml"
else
    echo "[ok] proj/rulestools.toml already exists"
fi

echo ""
echo "Done! Run 'cargo build' to see scanner output."
echo "Configure scanners in proj/rulestools.toml"
echo "Set deny = true to make violations break the build."
