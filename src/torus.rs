use crate::object3d::{Object3D, Polygon, Vec3};
use egui::Color32;
use std::f32::consts::PI;

const DEG_TO_RAD: f32 = PI / 180.0;

/// Builds a wireframe torus [`Object3D`] from the classic ring parameters
/// (ported from `resources/create-ring.js`).
pub struct Torus;

impl Torus {
    /// `sides` — points around the tube cross-section  
    /// `turns` — points around the major ring  
    /// `thickness` — tube radius  
    /// `reach` — distance from center to tube center  
    /// `color` — single stroke color for the whole mesh
    pub fn create(
        sides: usize,
        turns: usize,
        thickness: f32,
        reach: f32,
        color: Color32,
    ) -> Object3D {
        Self::create_with_start_angle(sides, turns, thickness, reach, 0.0, color)
    }

    fn create_with_start_angle(
        sides: usize,
        turns: usize,
        thickness: f32,
        reach: f32,
        start_angle: f32,
        color: Color32,
    ) -> Object3D {
        let mut object = Object3D::new(color, 0.0, 0.0);
        object.vlist.resize(sides * turns, Vec3::new(0.0, 0.0, 0.0));

        for j in 0..turns {
            let ring_deg = ((360.0 / turns as f32) * j as f32).floor();
            let center = Vec3::new(
                reach * (ring_deg * DEG_TO_RAD).sin(),
                reach * (ring_deg * DEG_TO_RAD).cos(),
                0.0,
            );

            for i in 0..sides {
                let side_deg = ((360.0 / sides as f32) * i as f32).floor() - start_angle;
                let local = Vec3::new(
                    0.0,
                    thickness * (side_deg * DEG_TO_RAD).sin(),
                    thickness * (side_deg * DEG_TO_RAD).cos(),
                );
                let rotated = scale_and_rotate(
                    local.x,
                    local.y,
                    local.z,
                    0.0,
                    0.0,
                    360.0 - (360.0 / turns as f32) * j as f32,
                    1.0,
                );
                object.vlist[j * sides + i] = add(center, rotated);

                if j != 0 && i != 0 {
                    object.add_polygon(Polygon::new([
                        (j - 1) * sides + (i - 1),
                        (j - 1) * sides + i,
                        j * sides + i,
                        j * sides + (i - 1),
                    ]));
                }

                if j != 0 && i == sides - 1 {
                    object.add_polygon(Polygon::new([
                        (j - 1) * sides + i,
                        (j - 1) * sides,
                        j * sides,
                        j * sides + i,
                    ]));
                }
            }

            // Close the torus: last ring back to the first.
            if j == turns - 1 {
                for i in 1..sides {
                    object.add_polygon(Polygon::new([
                        j * sides + (i - 1),
                        j * sides + i,
                        i,
                        i - 1,
                    ]));
                }
                object.add_polygon(Polygon::new([
                    j * sides + sides - 1,
                    j * sides,
                    0,
                    sides - 1,
                ]));
            }
        }

        object
    }
}

fn add(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z)
}

/// Scale, then rotate about X, Y, Z (degrees). Only Z is used by the torus path.
fn scale_and_rotate(
    mut x: f32,
    mut y: f32,
    mut z: f32,
    rot_x_deg: f32,
    rot_y_deg: f32,
    rot_z_deg: f32,
    scale: f32,
) -> Vec3 {
    x *= scale;
    y *= scale;
    z *= scale;

    if rot_x_deg != 0.0 {
        let (s, c) = (rot_x_deg * DEG_TO_RAD).sin_cos();
        let ny = y * c - z * s;
        let nz = y * s + z * c;
        y = ny;
        z = nz;
    }
    if rot_y_deg != 0.0 {
        let (s, c) = (rot_y_deg * DEG_TO_RAD).sin_cos();
        let nx = x * c + z * s;
        let nz = -x * s + z * c;
        x = nx;
        z = nz;
    }
    if rot_z_deg != 0.0 {
        let (s, c) = (rot_z_deg * DEG_TO_RAD).sin_cos();
        let nx = x * c - y * s;
        let ny = x * s + y * c;
        x = nx;
        y = ny;
    }

    Vec3::new(x, y, z)
}
