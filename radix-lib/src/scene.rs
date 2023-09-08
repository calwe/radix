use serde::{Deserialize, Serialize};
use winit_input_helper::WinitInputHelper;

use crate::{map::Map, player::Player};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    name: String,
    player: Player,
    map: Map,
}

impl Scene {
    pub fn new(name: &str, player: Player, map: Map) -> Self {
        Self {
            name: name.to_string(),
            player,
            map,
        }
    }

    // TODO: scene saving and loading

    pub fn update(&mut self, input: &WinitInputHelper) {
        self.player.update(input, &self.map);
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn map(&self) -> &Map {
        &self.map
    }
}
