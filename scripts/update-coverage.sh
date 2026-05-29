#!/usr/bin/env bash
# Run cargo-llvm-cov and update the coverage line in README.md.
# Requires: cargo install cargo-llvm-cov
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

# Find llvm tools if not already set (needed on NixOS where rustup isn't used)
if [ -z "${LLVM_COV:-}" ]; then
    # Search for llvm-cov in nix store via the rustc sysroot's lib directory
    RUSTC_SYSROOT=$(rustc --print sysroot 2>/dev/null || true)
    if [ -n "$RUSTC_SYSROOT" ]; then
        # On NixOS, llvm tools are in a sibling store path
        STORE_PREFIX=$(echo "$RUSTC_SYSROOT" | grep -oP '/nix/store/[^/]+' || true)
        if [ -n "$STORE_PREFIX" ]; then
            # Find llvm-cov in the nix store by looking at the store path references
            for p in /nix/store/*/bin/llvm-cov; do
                if [ -f "$p" ]; then
                    export LLVM_COV="$p"
                    export LLVM_PROFDATA="$(dirname "$p")/llvm-profdata"
                    break
                fi
            done
        fi
    fi
fi

# Run llvm-cov and capture summary output
OUTPUT=$(cargo llvm-cov --skip-functions -- --skip populate_toc 2>&1)

# Extract the coverage percentage from the TOTAL line
PERCENT=$(echo "$OUTPUT" | grep 'TOTAL' | grep -oP '(\d+\.\d+)%' | head -1 | grep -oP '\d+' | head -1)

if [ -z "$PERCENT" ]; then
    echo "error: could not extract coverage percentage from llvm-cov output"
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
