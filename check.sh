#!/usr/bin/env bash
# Format check + Clippy for the Wasm target.
set -euo pipefail
cd "$(dirname "$0")"

# --check: fail if rustfmt would change files (CI-friendly).
cargo fmt -- --check

# Target comes from `.cargo/config.toml` (wasm32-unknown-unknown).
# -D warnings: treat Clippy warnings as errors.
cargo clippy -- -D warnings

echo "check.sh: ok"
