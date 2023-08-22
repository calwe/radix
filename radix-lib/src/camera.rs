pub struct Camera {
    pos_x: f64,
    pos_y: f64,
    dir_x: f64,
    dir_y: f64,
    plane_x: f64,
    plane_y: f64,
}

impl Camera {
    pub fn new(pos_x: f64, pos_y: f64, dir_x: f64, dir_y: f64, plane_x: f64, plane_y: f64) -> Self {
        Self {
            pos_x,
            pos_y,
            dir_x,
            dir_y,
            plane_x,
            plane_y,
        }
    }
}