use crate::engine::Engine;
use crate::object3d::{Object3D, Vec3};
use crate::torus::Torus;
use egui::Color32;

/// Hand translation along −Z so the mesh sits in front of the camera.
const VIEW_DISTANCE: f32 = 4.0;

#[derive(Clone)]
struct TorusControls {
    sides: u32,
    turns: u32,
    thickness: f32,
    reach: f32,
    /// Hex color string, e.g. `c8c8e6` or `#c8c8e6`.
    color: String,
}

impl Default for TorusControls {
    fn default() -> Self {
        Self {
            sides: 12,
            turns: 24,
            thickness: 0.35,
            reach: 1.2,
            color: "dcdce6".to_owned(),
        }
    }
}

/// App shell: owns the 3D engine, UI knobs, and the torus instance.
pub struct WasmUiApp {
    engine: Engine,
    controls: TorusControls,
    objects: Vec<Object3D>,
}

impl Default for WasmUiApp {
    fn default() -> Self {
        let controls = TorusControls::default();
        let torus = build_torus(&controls);
        Self {
            engine: Engine::new(),
            controls,
            objects: vec![torus],
        }
    }
}

impl WasmUiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn show_controls(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Torus");
        ui.separator();

        changed |= ui
            .add(
                egui::DragValue::new(&mut self.controls.sides)
                    .range(3..=64)
                    .prefix("sides: "),
            )
            .changed();
        changed |= ui
            .add(
                egui::DragValue::new(&mut self.controls.turns)
                    .range(3..=128)
                    .prefix("turns: "),
            )
            .changed();
        changed |= ui
            .add(
                egui::DragValue::new(&mut self.controls.thickness)
                    .range(0.05..=2.0)
                    .speed(0.01)
                    .prefix("thickness: "),
            )
            .changed();
        changed |= ui
            .add(
                egui::DragValue::new(&mut self.controls.reach)
                    .range(0.1..=5.0)
                    .speed(0.01)
                    .prefix("reach: "),
            )
            .changed();

        ui.horizontal(|ui| {
            ui.label("color:");
            changed |= ui
                .add(
                    egui::TextEdit::singleline(&mut self.controls.color)
                        .desired_width(80.0)
                        .hint_text("rrggbb"),
                )
                .changed();
        });
        ui.small("6-digit hex; invalid → white");

        changed
    }
}

impl eframe::App for WasmUiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut rebuild = false;

        egui::Panel::left("torus_controls")
            .resizable(false)
            .exact_size(200.0)
            .show(ui, |ui| {
                rebuild = self.show_controls(ui);
            });

        if rebuild {
            // Replace the mesh whenever a control changes (step 8).
            self.objects.clear();
            self.objects.push(build_torus(&self.controls));
        }

        egui::CentralPanel::default().show(ui, |ui| {
            self.engine.render(ui, &self.objects);
        });
    }
}

fn build_torus(controls: &TorusControls) -> Object3D {
    place_in_view(Torus::create(
        controls.sides as usize,
        controls.turns as usize,
        controls.thickness,
        controls.reach,
        parse_hex_color(&controls.color),
    ))
}

/// Accept `#rrggbb` / `rrggbb`; anything else becomes white.
fn parse_hex_color(raw: &str) -> Color32 {
    let hex = raw.trim().trim_start_matches('#');
    if hex.len() == 6
        && let Ok(r) = u8::from_str_radix(&hex[0..2], 16)
        && let Ok(g) = u8::from_str_radix(&hex[2..4], 16)
        && let Ok(b) = u8::from_str_radix(&hex[4..6], 16)
    {
        return Color32::from_rgb(r, g, b);
    }
    Color32::WHITE
}

/// Bake a view-space offset into the vertices (no matrix transform yet).
fn place_in_view(mut object: Object3D) -> Object3D {
    for v in &mut object.vlist {
        *v = Vec3::new(v.x, v.y, v.z - VIEW_DISTANCE);
    }
    object
}
