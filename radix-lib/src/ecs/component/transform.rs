use std::{cell::RefCell, rc::Rc};

use log::trace;
use winit_input_helper::WinitInputHelper;

use super::{Component, ComponentType};

pub struct Transform {
    pos_x: f32,
    pos_y: f32,
    dir: f32,
}

impl Component for Transform {
    fn update(&mut self, _input: &WinitInputHelper) {
        trace!("pos_x: {}", self.pos_x);
    }
}

impl Into<ComponentType> for Transform {
    fn into(self) -> ComponentType {
        ComponentType::Transform(Rc::new(RefCell::new(self)))
    }
}

impl Transform {
    pub fn new() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            dir: 0.0,
        }
    }

    pub fn pos_x(&self) -> f32 {
        self.pos_x
    }

    pub fn pos_y(&self) -> f32 {
        self.pos_y
    }

    pub fn dir(&self) -> f32 {
        self.dir
    }

    pub fn set_pos_x(&mut self, pos_x: f32) {
        self.pos_x = pos_x;
    }

    pub fn set_pos_y(&mut self, pos_y: f32) {
        self.pos_y = pos_y;
    }

    pub fn set_dir(&mut self, dir: f32) {
        self.dir = dir;
    }
}
