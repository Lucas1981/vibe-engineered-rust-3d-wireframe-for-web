# wasm-ui codebase report

Architecture review after steps 1–9. Layers match the planned build-up; **wgpu** is the eframe backend, not a hand-written 3D GPU path.

| | |
|---|---|
| Status | Steps 1–9 complete |
| Target | Wasm-only (`wasm32-unknown-unknown`) |
| Stack | egui + eframe → wgpu |
| Scene | Wireframe torus |

## Verdict

Fit for purpose as a teaching demo: clear `app → engine → graphics → painter` pipeline, faithful JS torus port, intentional simplicity around camera/matrices/clipping. Highest practical wins are color-input rebuild UX, near-camera projection safety, and clarifying unused object offsets.

## Render pipeline

```
canvas (#the_canvas_id)
  → main.rs: eframe::WebRunner + WasmUiApp
    → WasmUiApp::ui (controls + rebuild)
      → Engine::render (optional Y-spin on vlist)
        → Graphics::render → Screen
          → project + draw_line
            → egui::Painter
              → eframe / wgpu present
```

Rotation (when enabled) mutates `vlist` in object space before projection. View distance (`z − 4`) is applied in `project`, then FOV 90° divide by `−z`. There are no custom shaders or mesh vertex buffers for the torus.

## Modules

| File | Role | Notes |
|------|------|-------|
| `main.rs` | Wasm bootstrap | `WebRunner` on `#the_canvas_id`; panics if DOM missing |
| `app.rs` | UI + scene ownership | Controls, rebuild, rotation flag, Reset shape |
| `engine.rs` | Projection + wireframe | FOV 90°, Y-spin mutates `vlist`, view distance at project |
| `graphics.rs` | Screen / lines | egui `Painter` allocate + `line_segment`; not raw wgpu |
| `object3d.rs` | Mesh model | `vlist` + `plist`; color; unused `x`/`y` fields |
| `torus.rs` | Factory | Port of `create-ring.js`; single stroke color |

## Actionable advice

Ranked by impact. Paths are under `src/` unless noted.

### High

1. **UX — defer color rebuild until valid hex (or blur/Enter)**  
   `app.rs` · `parse_hex_color` / `show_controls`  
   Today every keystroke with an incomplete hex rebuilds the mesh as white.

2. **Correctness — guard `project()` when depth ≈ 0**  
   `engine.rs` · `project`  
   Avoid NaN / huge segments if geometry ever sits near the camera plane.

3. **API clarity — apply or remove `Object3D.x` / `y`**  
   `object3d.rs` · `engine.rs`  
   Fields exist but are ignored by the renderer.

### Medium

4. **Architecture — decouple mesh color from `egui::Color32`**  
   `object3d.rs`  
   Store RGB (or similar) and convert at draw time for clearer layering and easier tests.

5. **Performance — deduplicate shared edges before `draw_line`**  
   `engine.rs` · `draw_wireframe`  
   Adjacent quads currently stroke the same edge twice.

6. **API — expose `start_angle` or drop the private path**  
   `torus.rs`  
   Public `create` always passes `0.0`; JS exposes the parameter.

7. **Safety — guard `Torus::create` for `sides`/`turns` < 3**  
   `torus.rs` · `create`  
   UI clamps; the factory API does not.

### Low

8. **Design — keep a base mesh + spin angle instead of mutating `vlist`**  
   `engine.rs` · `app.rs`  
   Cleaner interaction with rebuild/reset; spin becomes reversible without a factory round-trip.

9. **Docs — fix README typo; note `create-ring.js` vs AGENTS name**  
   `README.md` · `AGENTS.md`  
   “Enginereed”; AGENTS cites `create-rings.js`.

10. **Tests — unit-test hex parse, Y-rotate, torus vert/poly counts**  
    New tests module  
    Especially valuable against the JS reference topology.

## Strengths

- Layering mirrors `AGENTS.md`: graphics → objects → engine → torus → UI → rotation.
- `Torus::create` closely matches `resources/create-ring.js` topology (floored angles, Z-rotated tube, seam quads).
- Tooling is solid: Wasm-forced target, Trunk serve, `./check.sh` with fmt + clippy `-D warnings`.

## Intentionally out of scope

Per `AGENTS.md`:

- Native (non-Wasm) builds
- Camera / dynamic FOV / matrices
- Clipping, culling, lighting, solid fill
- Custom wgpu shaders / vertex buffers for the mesh

## Notable quirks

- Hex color rebuilds on every keystroke while incomplete → flashes white.
- Shared quad edges are drawn twice.
- `AGENTS.md` cites `create-rings.js`; file is `create-ring.js`.
- README typo “Enginereed”.
- Reset shape does not stop rotation.

---

*Source: local codebase review · wasm-ui · 2026-09-17*
