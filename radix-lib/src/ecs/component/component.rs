use std::{any::Any, cell::RefCell, rc::Rc};

use winit_input_helper::WinitInputHelper;

use super::{player_controller::PlayerController, transform::Transform};

pub trait Component: Into<ComponentType> {
    fn update(&mut self, _input: &WinitInputHelper) {}
    fn start(&mut self) {}
}

pub enum ComponentType {
    Transform(Rc<RefCell<Transform>>),
    PlayerController(Rc<RefCell<PlayerController>>),
}

impl Component for ComponentType {
    // macro all this?
    fn update(&mut self, input: &WinitInputHelper) {
        match self {
            ComponentType::Transform(transform) => transform.borrow_mut().update(input),
            ComponentType::PlayerController(player_controller) => {
                player_controller.borrow_mut().update(input)
            }
        }
    }

    fn start(&mut self) {
        match self {
            ComponentType::Transform(transform) => transform.borrow_mut().start(),
            ComponentType::PlayerController(player_controller) => {
                player_controller.borrow_mut().start()
            }
        }
    }
}

impl ComponentType {
    pub fn to_any(&self) -> &dyn Any {
        match self {
            ComponentType::Transform(transform) => transform,
            ComponentType::PlayerController(player_controller) => player_controller,
        }
    }
}
