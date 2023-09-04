use std::{cell::RefCell, rc::Rc};

use crate::map::texture::Texture;

use super::{sprite_transform::SpriteTransform, Component, ComponentType};

pub struct Sprite {
    texture: Rc<RefCell<Texture>>,
}

impl Component for Sprite {}

impl Into<ComponentType> for Sprite {
    fn into(self) -> ComponentType {
        ComponentType::Sprite(Rc::new(RefCell::new(self)))
    }
}

impl Sprite {
    pub fn new(texture: Texture) -> Self {
        Self {
            texture: Rc::new(RefCell::new(texture)),
        }
    }

    pub fn width(&self) -> f64 {
        self.texture.borrow().width() as f64
    }

    pub fn height(&self) -> f64 {
        self.texture.borrow().height() as f64
    }

    pub fn texture(&self) -> Rc<RefCell<Texture>> {
        self.texture.clone()
    }
}
