use egui::Color32;

/// A point in object space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// A polygon defined by indices into an [`Object3D`]'s shared `vlist`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polygon {
    pub verts: Vec<usize>,
}

impl Polygon {
    pub fn new(verts: impl Into<Vec<usize>>) -> Self {
        Self {
            verts: verts.into(),
        }
    }
}

/// A 3D mesh: one vertex list, many polygons that index into it, plus color and
/// screen-plane offsets `x` / `y`.
#[derive(Clone, Debug)]
pub struct Object3D {
    pub vlist: Vec<Vec3>,
    pub plist: Vec<Polygon>,
    pub color: Color32,
    pub x: f32,
    pub y: f32,
}

impl Object3D {
    pub fn new(color: Color32, x: f32, y: f32) -> Self {
        Self {
            vlist: Vec::new(),
            plist: Vec::new(),
            color,
            x,
            y,
        }
    }

    /// Append a vertex and return its index in `vlist`.
    pub fn add_vertex(&mut self, vertex: Vec3) -> usize {
        let index = self.vlist.len();
        self.vlist.push(vertex);
        index
    }

    /// Append a polygon whose `verts` refer into this object's `vlist`.
    pub fn add_polygon(&mut self, polygon: Polygon) {
        self.plist.push(polygon);
    }
}
