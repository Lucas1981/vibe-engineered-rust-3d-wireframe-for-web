/// Minimal scaffold app — graphics and torus controls come in later steps.
pub struct WasmUiApp;

impl Default for WasmUiApp {
    fn default() -> Self {
        Self
    }
}

impl WasmUiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for WasmUiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("wasm-ui");
            ui.label("Wasm scaffold is running (egui + wgpu via eframe).");
        });
    }
}
