use std::{cell::RefCell, rc::Rc};

use super::{transform::Transform, Component, ComponentType};

pub struct Camera {
    pos_x: f64,
    pos_y: f64,
    dir_x: f64,
    dir_y: f64,
    plane_x: f64,
    plane_y: f64,
    aspect_ratio: f64,
}

impl Component for Camera {}

impl Into<ComponentType> for Camera {
    fn into(self) -> ComponentType {
        ComponentType::Camera(Rc::new(RefCell::new(self)))
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: aspect_ratio / 2.0,
            aspect_ratio,
        }
    }

    pub fn pos_x(&self) -> f64 {
        self.pos_x
    }

    pub fn pos_y(&self) -> f64 {
        self.pos_y
    }

    pub fn dir_x(&self) -> f64 {
        self.dir_x
    }

    pub fn dir_y(&self) -> f64 {
        self.dir_y
    }

    pub fn plane_x(&self) -> f64 {
        self.plane_x
    }

    pub fn plane_y(&self) -> f64 {
        self.plane_y
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn set_from_transform(&mut self, transform: &Transform) {
        self.pos_x = transform.pos_x();
        self.pos_y = transform.pos_y();
        self.dir_x = transform.dir().cos();
        self.dir_y = transform.dir().sin();
        self.plane_x = transform.dir().sin() * self.aspect_ratio() / 2.0;
        self.plane_y = -transform.dir().cos() * self.aspect_ratio() / 2.0;
    }
}
