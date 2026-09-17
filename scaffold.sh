#!/usr/bin/env bash
# One-time (or rare) setup for this Wasm-only egui/wgpu app.
set -euo pipefail
cd "$(dirname "$0")"

# Add the browser Wasm target (idempotent).
rustup target add wasm32-unknown-unknown

# Trunk: builds Wasm, runs wasm-bindgen, and serves the app.
# --locked: use Trunk's Cargo.lock for a reproducible install.
cargo install --locked trunk

echo "Scaffold ready. Run ./build-and-run.sh"
