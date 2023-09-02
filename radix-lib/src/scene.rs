use std::{cell::RefCell, fs, rc::Rc};

use serde::{Deserialize, Serialize};
use winit_input_helper::WinitInputHelper;

use crate::{map::Map, player::Player};

const SAVE_PATH: &str = "scenes";

pub struct Scene {
    name: String,
    player: Rc<RefCell<Player>>,
    map: Map,
}

impl Scene {
    pub fn new(name: &str, player: Player, map: Map) -> Self {
        Self {
            name: name.to_string(),
            player: Rc::new(RefCell::new(player)),
            map,
        }
    }

    pub fn update(&mut self, input: &WinitInputHelper) {
        self.player.borrow_mut().update(input, &self.map);
    }

    pub fn player(&self) -> Rc<RefCell<Player>> {
        self.player.clone()
    }

    pub fn map(&self) -> &Map {
        &self.map
    }
}
