use log::warn;
use winit_input_helper::WinitInputHelper;

use super::entity::entity::Entity;

pub struct Scene {
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
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

    pub fn get_entity(&mut self, tag: &str) -> Option<&mut Entity> {
        for entity in &mut self.entities {
            if entity.tag() == tag {
                return Some(entity);
            }
        }

        None
    }

    pub fn update(&mut self, input: &WinitInputHelper) {
        for entity in &mut self.entities {
            entity.update(input);
        }
    }

    pub fn start(&mut self) {
        for entity in &mut self.entities {
            entity.start();
        }
    }
}
