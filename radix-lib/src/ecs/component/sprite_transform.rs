use std::{cell::RefCell, rc::Rc};

use super::{Component, ComponentType};

pub struct SpriteTransform {
    pos_x: f64,
    pos_y: f64,
    pos_z: f64,
    scale_x: f64,
    scale_y: f64,
}

impl Component for SpriteTransform {}

impl Into<ComponentType> for SpriteTransform {
    fn into(self) -> ComponentType {
        ComponentType::SpriteTransform(Rc::new(RefCell::new(self)))
    }
}

impl SpriteTransform {
    pub fn new(pos_x: f64, pos_y: f64, pos_z: f64, scale_x: f64, scale_y: f64) -> Self {
        Self {
            pos_x,
            pos_y,
            pos_z,
            scale_x,
            scale_y,
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

    pub fn set_pos_x(&mut self, pos_x: f64) {
        self.pos_x = pos_x;
    }

    pub fn set_pos_y(&mut self, pos_y: f64) {
        self.pos_y = pos_y;
    }

    pub fn set_pos_z(&mut self, pos_z: f64) {
        self.pos_z = pos_z;
    }

    pub fn set_scale_x(&mut self, scale_x: f64) {
        self.scale_x = scale_x;
    }

    pub fn set_scale_y(&mut self, scale_y: f64) {
        self.scale_y = scale_y;
    }
}
