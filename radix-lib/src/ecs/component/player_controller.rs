use std::{cell::RefCell, rc::Rc};

use winit_input_helper::WinitInputHelper;

use super::{transform::Transform, Component, ComponentType};

pub struct PlayerController {
    speed: f32,
    turn_speed: f32,
    transform: Rc<RefCell<Transform>>,
}

impl Component for PlayerController {
    fn update(&mut self, _input: &WinitInputHelper) {
        let pos_x = self.transform.borrow().pos_x();
        self.transform.borrow_mut().set_pos_x(pos_x + 1.0);
    }
    fn start(&mut self) {}
}

impl Into<ComponentType> for PlayerController {
    fn into(self) -> ComponentType {
        ComponentType::PlayerController(Rc::new(RefCell::new(self)))
    }
}

impl PlayerController {
    pub fn new(speed: f32, turn_speed: f32, transform: Rc<RefCell<Transform>>) -> Self {
        Self {
            speed,
            turn_speed,
            transform,
        }
    }
}
