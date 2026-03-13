#!/usr/bin/env bash
# Validates that install.sh [rustscanners] config lists exactly the checks
# defined in src/checks/mod.rs (minus mod.rs itself).
# Run as pre-commit hook or manually.
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
MOD="$ROOT/src/checks/mod.rs"
INSTALL="$ROOT/install.sh"

# Extract check names from mod.rs (lines: "pub mod foo;")
checks=$(grep -E '^pub mod ' "$MOD" | sed 's/pub mod //;s/;//' | sort)

# Extract keys from install.sh config block (lines: "key = true/false")
config_keys=$(grep -E '^\w+ = (true|false)' "$INSTALL" | sed 's/ =.*//' | grep -vE '^(enabled|deny)$' | sort)

if [ "$checks" = "$config_keys" ]; then
    echo "[ok] install.sh config matches src/checks/mod.rs"
    exit 0
fi

echo "[FAIL] install.sh config is out of sync with src/checks/mod.rs"
echo ""
echo "In mod.rs but missing from install.sh:"
comm -23 <(echo "$checks") <(echo "$config_keys") | sed 's/^/  + /'
echo ""
echo "In install.sh but not in mod.rs (stale):"
comm -13 <(echo "$checks") <(echo "$config_keys") | sed 's/^/  - /'
echo ""
echo "Fix install.sh default config to match."
exit 1
