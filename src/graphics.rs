use egui::{Color32, Pos2, Rect, Sense, Stroke};

/// Owns screen setup and per-frame rendering for 2D line drawing.
pub struct Graphics {
    clear_color: Color32,
    stroke_width: f32,
}

impl Default for Graphics {
    fn default() -> Self {
        Self::new()
    }
}

impl Graphics {
    pub fn new() -> Self {
        Self {
            clear_color: Color32::from_rgb(20, 20, 24),
            stroke_width: 1.5,
        }
    }

    /// Allocate the remaining UI region as a screen, clear it, then run `draw`.
    pub fn render(&self, ui: &mut egui::Ui, draw: impl FnOnce(&Screen)) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
        let screen = Screen {
            painter,
            rect: response.rect,
            stroke_width: self.stroke_width,
        };
        screen.clear(self.clear_color);
        draw(&screen);
    }
}

/// A single frame's drawable surface. Coordinates are screen-local: (0, 0) at
/// the top-left of the allocated region, +y downward.
pub struct Screen {
    painter: egui::Painter,
    rect: Rect,
    stroke_width: f32,
}

impl Screen {
    pub fn width(&self) -> f32 {
        self.rect.width()
    }

    pub fn height(&self) -> f32 {
        self.rect.height()
    }

    pub fn clear(&self, color: Color32) {
        self.painter.rect_filled(self.rect, 0.0, color);
    }

    pub fn draw_line(&self, x0: f32, y0: f32, x1: f32, y1: f32, color: Color32) {
        let a = self.to_pos(x0, y0);
        let b = self.to_pos(x1, y1);
        self.painter
            .line_segment([a, b], Stroke::new(self.stroke_width, color));
    }

    fn to_pos(&self, x: f32, y: f32) -> Pos2 {
        Pos2::new(self.rect.min.x + x, self.rect.min.y + y)
    }
}
