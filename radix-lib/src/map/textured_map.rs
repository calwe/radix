use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use super::{sprite::Sprite, texture::Texture};

pub struct TexturedMap {
    width: u32,
    height: u32,
    walls: Vec<Option<Rc<Texture>>>,
    floor: Vec<Option<Rc<Texture>>>,
    ceiling: Vec<Option<Rc<Texture>>>,
    sprites: Vec<Rc<RefCell<Sprite>>>,
}

// TODO: Write deserlaize and serialize for TexturedMap
impl Serialize for TexturedMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for TexturedMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl TexturedMap {
    pub fn with_data(
        width: u32,
        height: u32,
        data: Vec<Option<Rc<Texture>>>,
        floor: Vec<Option<Rc<Texture>>>,
        ceiling: Vec<Option<Rc<Texture>>>,
        sprites: Vec<Rc<RefCell<Sprite>>>,
    ) -> Self {
        Self {
            width,
            height,
            walls: data,
            floor,
            ceiling,
            sprites,
        }
    }

    pub fn set(&mut self, x: u32, y: u32, texture: Rc<Texture>) {
        self.walls[(y * self.width + x) as usize] = Some(texture);
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Rc<Texture>> {
        self.walls
            .get((y * self.width + x) as usize)
            .unwrap()
            .clone()
    }

    pub fn get_floor(&self, x: u32, y: u32) -> Option<Rc<Texture>> {
        self.floor[(y.min(self.height - 1) * self.width + x.min(self.width - 1)) as usize].clone()
    }

    pub fn get_ceiling(&self, x: u32, y: u32) -> Option<Rc<Texture>> {
        self.ceiling[(y.min(self.height - 1) * self.width + x.min(self.width - 1)) as usize].clone()
    }

    pub fn sprites(&self) -> Vec<Rc<RefCell<Sprite>>> {
        self.sprites.clone()
    }
}
