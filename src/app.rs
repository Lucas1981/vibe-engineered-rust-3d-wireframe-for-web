use crate::graphics::Graphics;
use egui::Color32;

/// App shell: owns graphics and (later) the 3D scene / UI controls.
#[derive(Default)]
pub struct WasmUiApp {
    graphics: Graphics,
}

impl WasmUiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for WasmUiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Step 2 test: instantiate the screen and draw a vertical line.
            self.graphics.render(ui, |screen| {
                let x = screen.width() * 0.5;
                screen.draw_line(x, 0.0, x, screen.height(), Color32::WHITE);
            });
        });
    }
}
