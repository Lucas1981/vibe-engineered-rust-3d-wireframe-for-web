use crate::engine::Engine;
use crate::object3d::Object3D;
use egui::Color32;

/// App shell: owns the 3D engine and (later) the torus / UI controls.
#[derive(Default)]
pub struct WasmUiApp {
    engine: Engine,
    /// Scene objects rendered each frame (torus arrives in a later step).
    objects: Vec<Object3D>,
}

impl WasmUiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for WasmUiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.engine.render(ui, &self.objects, |screen| {
                // Step 2 test retained until the torus replaces it.
                let x = screen.width() * 0.5;
                screen.draw_line(x, 0.0, x, screen.height(), Color32::WHITE);
            });
        });
    }
}
