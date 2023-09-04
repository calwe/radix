use std::{cell::RefCell, rc::Rc};

use log::trace;
use winit_input_helper::WinitInputHelper;

use crate::map::Map;

use super::{Component, ComponentType};

pub struct Transform {
    pos_x: f64,
    pos_y: f64,
    dir: f64,
}

impl Component for Transform {
    fn update(&mut self, _input: &WinitInputHelper, _map: &Map) {
        trace!("pos_x: {}", self.pos_x);
    }
}

impl Into<ComponentType> for Transform {
    fn into(self) -> ComponentType {
        ComponentType::Transform(Rc::new(RefCell::new(self)))
    }
}

impl Transform {
    pub fn new(pos_x: f64, pos_y: f64, dir: f64) -> Self {
        Self { pos_x, pos_y, dir }
    }

    pub fn pos_x(&self) -> f64 {
        self.pos_x
    }

    pub fn pos_y(&self) -> f64 {
        self.pos_y
    }

    pub fn dir(&self) -> f64 {
        self.dir
    }

    pub fn set_pos_x(&mut self, pos_x: f64) {
        self.pos_x = pos_x;
    }

    pub fn set_pos_y(&mut self, pos_y: f64) {
        self.pos_y = pos_y;
    }

    pub fn set_dir(&mut self, dir: f64) {
        self.dir = dir;
    }

    pub fn add_pos_x(&mut self, pos_x: f64) {
        self.pos_x += pos_x;
    }

    pub fn add_pos_y(&mut self, pos_y: f64) {
        self.pos_y += pos_y;
    }

    pub fn add_dir(&mut self, dir: f64) {
        self.dir += dir;
    }
}
