#!/usr/bin/env bash
# Build the Wasm target and serve it locally (rebuilds on change).
set -euo pipefail
cd "$(dirname "$0")"

# Trunk 0.21 mishandles NO_COLOR=1 (treats it as --no-color=1).
unset NO_COLOR FORCE_COLOR || true

# serve: compile crate → Wasm, bindgen, then HTTP-serve dist/ (default :8080).
# Open http://127.0.0.1:8080/#dev so browsers skip any SW cache during development.
trunk serve
