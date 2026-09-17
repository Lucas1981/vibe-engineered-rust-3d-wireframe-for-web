use crate::graphics::Graphics;
use crate::object3d::{Object3D, Vec3};

/// Minimal wireframe renderer: camera at the origin looking down −Z, FOV 90°.
/// Object `x`/`y` offsets are ignored for now (no transforms yet).
#[derive(Default)]
pub struct Engine {
    graphics: Graphics,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            graphics: Graphics::new(),
        }
    }

    /// Clear the screen and draw every object as a wireframe.
    pub fn render(&self, ui: &mut egui::Ui, objects: &[Object3D]) {
        self.graphics.render(ui, |screen| {
            for object in objects {
                draw_wireframe(screen, object);
            }
        });
    }
}

fn draw_wireframe(screen: &crate::graphics::Screen, object: &Object3D) {
    let w = screen.width();
    let h = screen.height();
    let projected: Vec<(f32, f32)> = object.vlist.iter().map(|v| project(*v, w, h)).collect();

    for polygon in &object.plist {
        let n = polygon.verts.len();
        if n < 2 {
            continue;
        }
        for i in 0..n {
            let a = polygon.verts[i];
            let b = polygon.verts[(i + 1) % n];
            let (x0, y0) = projected[a];
            let (x1, y1) = projected[b];
            screen.draw_line(x0, y0, x1, y1, object.color);
        }
    }
}

/// Object space → screen pixels in one step. FOV 90° ⇒ `tan(45°) = 1`, so
/// perspective is just divide by depth. View looks toward −Z, so depth is `−z`.
fn project(v: Vec3, width: f32, height: f32) -> (f32, f32) {
    let depth = -v.z;
    let u = v.x / depth;
    let v_ndc = v.y / depth;
    let sx = (u + 1.0) * 0.5 * width;
    let sy = (1.0 - v_ndc) * 0.5 * height;
    (sx, sy)
}
