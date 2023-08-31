use serde::{Deserialize, Serialize};

use super::texture::Texture;

#[derive(Serialize, Deserialize)]
pub struct TexturedMap {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<Texture>,
}

impl TexturedMap {
    pub fn empty(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: Vec::new(),
        }
    }

    pub fn with_data(width: u32, height: u32, data: Vec<Texture>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Texture) {
        self.data[(y * self.width + x) as usize] = color;
    }

    pub fn get(&self, x: u32, y: u32) -> Texture {
        self.data[(y * self.width + x) as usize].clone()
    }
}