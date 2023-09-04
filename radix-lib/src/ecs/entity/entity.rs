use std::{cell::RefCell, rc::Rc};

use log::warn;
use winit_input_helper::WinitInputHelper;

use crate::{
    ecs::component::{Component, ComponentType},
    map::Map,
};

pub struct Entity {
    tag: String,
    components: Vec<ComponentType>,
}

impl Entity {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            components: Vec::new(),
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        if self.get_component::<T>().cloned().is_none() {
            self.components.push(component.into());
        } else {
            warn!(
                "Component {} already exists on {}: ignoring...",
                std::any::type_name::<T>(),
                self.tag
            );
        }
    }

    pub fn get_component<T: Component + 'static>(&self) -> Option<&Rc<RefCell<T>>> {
        self.components
            .iter()
            .map(ComponentType::to_any)
            .find_map(|c| c.downcast_ref::<Rc<RefCell<T>>>())
    }

    pub fn has_component<T: Component + 'static>(&self) -> bool {
        self.get_component::<T>().is_some()
    }

    pub fn update(&mut self, input: &WinitInputHelper, map: &Map) {
        for component in &mut self.components {
            component.update(input, map);
        }
    }

    pub fn start(&mut self) {
        for component in &mut self.components {
            component.start();
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }
}
