use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub(crate) pos_x: f64,
    pub(crate) pos_y: f64,
    pub(crate) dir_x: f64,
    pub(crate) dir_y: f64,
    pub(crate) plane_x: f64,
    pub(crate) plane_y: f64,
    pub(crate) aspect_ratio: f64,
}

impl Camera {
    pub fn new(pos_x: f64, pos_y: f64, aspect_ratio: f64) -> Self {
        Self {
            pos_x,
            pos_y,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: aspect_ratio / 2.0,
            aspect_ratio
        }
    }

    pub fn add_position(&mut self, x: f64, y: f64) {
        self.pos_x += x * self.dir_x;
        self.pos_y += y * self.dir_y;
    }

    /// Rotates the camera by the given angle in radians.
    pub fn add_direction(&mut self, a: f64) {
        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * a.cos() - self.dir_y * a.sin();
        self.dir_y = old_dir_x * a.sin() + self.dir_y * a.cos();
        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * a.cos() - self.plane_y * a.sin();
        self.plane_y = old_plane_x * a.sin() + self.plane_y * a.cos();
    }
}