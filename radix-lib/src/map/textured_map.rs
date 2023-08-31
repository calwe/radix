use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::texture::Texture;

pub struct TexturedMap {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<Option<Rc<Texture>>>,
    pub(crate) floor: Rc<Texture>,
    pub(crate) ceiling: Rc<Texture>,
}

// TODO: Write deserlaize and serialize for TexturedMap
impl Serialize for TexturedMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            todo!()
        }
}

impl<'de> Deserialize<'de> for TexturedMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        todo!()
    }
}

impl TexturedMap {
    pub fn with_data(width: u32, height: u32, data: Vec<Option<Rc<Texture>>>, floor: Rc<Texture>, ceiling: Rc<Texture>) -> Self {
        Self {
            width,
            height,
            data,
            floor,
            ceiling
        }
    }

    pub fn set(&mut self, x: u32, y: u32, texture: Rc<Texture>) {
        self.data[(y * self.width + x) as usize] = Some(texture);
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Rc<Texture>> {
        self.data.get((y * self.width + x) as usize).unwrap().clone()
    }
}