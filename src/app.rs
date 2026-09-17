use crate::engine::Engine;
use crate::object3d::{Object3D, Vec3};
use crate::torus::Torus;
use egui::Color32;

/// Default torus knobs (UI controls arrive in a later step).
const SIDES: usize = 12;
const TURNS: usize = 24;
const THICKNESS: f32 = 0.35;
const REACH: f32 = 1.2;
/// Hand translation along −Z so the mesh sits in front of the camera.
const VIEW_DISTANCE: f32 = 4.0;

/// App shell: owns the 3D engine and the torus instance.
pub struct WasmUiApp {
    engine: Engine,
    objects: Vec<Object3D>,
}

impl Default for WasmUiApp {
    fn default() -> Self {
        Self {
            engine: Engine::new(),
            objects: vec![place_in_view(Torus::create(
                SIDES,
                TURNS,
                THICKNESS,
                REACH,
                Color32::from_rgb(220, 220, 230),
            ))],
        }
    }
}

impl WasmUiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for WasmUiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.engine.render(ui, &self.objects);
        });
    }
}

/// Bake a view-space offset into the vertices (no matrix transform yet).
fn place_in_view(mut object: Object3D) -> Object3D {
    for v in &mut object.vlist {
        *v = Vec3::new(v.x, v.y, v.z - VIEW_DISTANCE);
    }
    object
}
