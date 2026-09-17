use crate::graphics::Graphics;
use crate::object3d::{Object3D, Vec3};

/// Hand translation along −Z applied at projection time (object space stays
/// centered on the origin so Y-rotation spins the mesh in place).
const VIEW_DISTANCE: f32 = 4.0;

/// Mild spin rate while rotation is enabled (radians per second).
const Y_ROTATION_SPEED: f32 = 0.9;

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
    /// When `rotating` is set, nudge each object's `vlist` around Y in object
    /// space (by hand, no matrices) before projecting.
    pub fn render(&self, ui: &mut egui::Ui, objects: &mut [Object3D], rotating: bool) {
        if rotating {
            let dt = ui.ctx().input(|i| i.stable_dt);
            let angle = Y_ROTATION_SPEED * dt;
            for object in objects.iter_mut() {
                rotate_y_object_space(&mut object.vlist, angle);
            }
            ui.ctx().request_repaint();
        }

        self.graphics.render(ui, |screen| {
            for object in objects.iter() {
                draw_wireframe(screen, object);
            }
        });
    }
}

/// Rotate around the Y axis through the origin: `x' = x c − z s`, `z' = x s + z c`.
fn rotate_y_object_space(vlist: &mut [Vec3], angle_rad: f32) {
    let (sin_a, cos_a) = angle_rad.sin_cos();
    for v in vlist.iter_mut() {
        let x = v.x * cos_a - v.z * sin_a;
        let z = v.x * sin_a + v.z * cos_a;
        v.x = x;
        v.z = z;
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

/// Object space → screen pixels. Hand-apply view distance, then FOV 90° divide.
fn project(v: Vec3, width: f32, height: f32) -> (f32, f32) {
    let z = v.z - VIEW_DISTANCE;
    let depth = -z;
    let u = v.x / depth;
    let v_ndc = v.y / depth;
    let sx = (u + 1.0) * 0.5 * width;
    let sy = (1.0 - v_ndc) * 0.5 * height;
    (sx, sy)
}
