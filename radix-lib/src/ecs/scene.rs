use std::{cell::RefCell, rc::Rc};

use log::warn;
use winit_input_helper::WinitInputHelper;

use crate::map::Map;

use super::entity::entity::Entity;

pub struct Scene {
    map: Rc<RefCell<Map>>,
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new(map: Map) -> Self {
        Self {
            map: Rc::new(RefCell::new(map)),
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, mut entity: Entity) {
        if self.get_entity(entity.tag()).is_none() {
            entity.start();
            self.entities.push(entity);
        } else {
            warn!(
                "Entity {} already exists in scene: ignoring...",
                entity.tag()
            );
        }
    }

    pub fn get_entity(&self, tag: &str) -> Option<&Entity> {
        for entity in &self.entities {
            if entity.tag() == tag {
                return Some(entity);
            }
        }

        None
    }

    pub fn get_entity_mut(&mut self, tag: &str) -> Option<&mut Entity> {
        for entity in &mut self.entities {
            if entity.tag() == tag {
                return Some(entity);
            }
        }

        None
    }

    pub fn update(&mut self, input: &WinitInputHelper) {
        for entity in &mut self.entities {
            entity.update(input, &self.map.borrow());
        }
    }

    pub fn start(&mut self) {
        for entity in &mut self.entities {
            entity.start();
        }
    }

    pub fn map(&self) -> Rc<RefCell<Map>> {
        self.map.clone()
    }
}
