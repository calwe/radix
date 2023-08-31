use std::rc::Rc;

use super::texture::Texture;

pub struct Sprite {
    pos_x: f64,
    pos_y: f64,
    pos_z: f64, // 0.0 is the floor, 1.0 is the ceiling
    scale_x: f64,
    scale_y: f64,
    texture: Rc<Texture>,
}

impl Sprite {
    pub fn new(
        pos_x: f64,
        pos_y: f64,
        pos_z: f64,
        scale_x: f64,
        scale_y: f64,
        texture: Rc<Texture>,
    ) -> Self {
        Self {
            pos_x,
            pos_y,
            pos_z,
            scale_x,
            scale_y,
            texture,
        }
    }

    pub fn pos_x(&self) -> f64 {
        self.pos_x
    }

    pub fn pos_y(&self) -> f64 {
        self.pos_y
    }

    pub fn pos_z(&self) -> f64 {
        self.pos_z
    }

    pub fn scale_x(&self) -> f64 {
        self.scale_x
    }

    pub fn scale_y(&self) -> f64 {
        self.scale_y
    }

    pub fn width(&self) -> f64 {
        self.texture.width() as f64
    }

    pub fn height(&self) -> f64 {
        self.texture.height() as f64
    }

    pub fn texture(&self) -> Rc<Texture> {
        self.texture.clone()
    }
}
