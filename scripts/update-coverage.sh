#!/usr/bin/env bash
# Run cargo-tarpaulin and update the coverage line in README.md.
# Requires: cargo install cargo-tarpaulin
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

# Run tarpaulin and capture output
OUTPUT=$(cargo tarpaulin --skip-clean --out Stdout 2>&1)

# Extract the coverage percentage (e.g., "42.86% coverage")
PERCENT=$(echo "$OUTPUT" | grep -oP '\d+\.\d+% coverage' | head -1 | grep -oP '\d+' | head -1)

if [ -z "$PERCENT" ]; then
    echo "error: could not extract coverage percentage from tarpaulin output"
    echo "$OUTPUT"
    exit 1
fi

CURRENT=$(grep -oP '^Coverage: \K\d+' README.md 2>/dev/null || echo "")

if [ "$CURRENT" = "$PERCENT" ]; then
    echo "Coverage unchanged at ${PERCENT}%"
else
    # Patch the coverage line in README.md
    sed -i "s/^Coverage: [0-9]*%/Coverage: ${PERCENT}%/" README.md
    echo "Coverage updated: ${CURRENT:-0}% -> ${PERCENT}%"
fi
