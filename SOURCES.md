# Sources

References that informed this project's Wasm + egui/eframe scaffold and tooling.

## egui / eframe (community)

| Resource | Why it matters |
|----------|----------------|
| [emilk/eframe_template](https://github.com/emilk/eframe_template/) | Canonical starter for egui apps (native + web): `lib` / `app` / `main` split, Trunk, canvas `index.html`, `WebRunner` bootstrap. Closest public boilerplate to our step-1 layout. |
| [eframe docs (docs.rs)](https://docs.rs/eframe/latest/eframe/) | Official crate docs; points at `eframe_template` and documents `App`, `run_native`, and Wasm `WebRunner` usage. |
| [egui](https://github.com/emilk/egui) | UI crate and parent project (Emil Ernerfeldt / community). |

We followed that template's *shape* but authored files by hand and kept a **Wasm-only** variant (no native `run_native` path).

## Tooling

| Resource | Why it matters |
|----------|----------------|
| [Trunk](https://trunkrs.dev/) / [trunk-rs/trunk](https://github.com/trunk-rs/trunk) | Community Wasm bundler: build, wasm-bindgen, and `trunk serve`. Not part of the Rust toolchain itself. |
| [`rustup` Wasm target](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html) | `rustup target add wasm32-unknown-unknown` — the main Rust-project piece of the browser story. |
| [Cargo book](https://doc.rust-lang.org/cargo/) | Generic crate layout (`src/lib.rs`, `src/main.rs`); not egui- or Wasm-specific. |

## In-repo reference

| Resource | Why it matters |
|----------|----------------|
| [`resources/create-ring.js`](resources/create-ring.js) | Original torus mesh generator ported into `src/torus.rs`. |
| [`AGENTS.md`](AGENTS.md) | Build plan and intentional scope for this demo. |
| [`REPORT.md`](REPORT.md) | Post-build architecture notes and follow-ups. |

## What is *not* an official “Rust team” boilerplate

There is no first-party Rust-lang template for “egui + wgpu + Trunk web app.” The language/project supplies the `wasm32-unknown-unknown` target (and Cargo); the egui/eframe + Trunk stack is community-maintained.
