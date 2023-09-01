use std::fs;

use serde::{Deserialize, Serialize};
use winit_input_helper::WinitInputHelper;

use crate::{map::Map, player::Player};

const SAVE_PATH: &str = "scenes";

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
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_yaml::to_string(&self)?;
        // create save directory if it doesn't exist
        fs::create_dir_all(SAVE_PATH)?;
        fs::write(format!("{}/{}.yaml", SAVE_PATH, self.name), serialized)?;
        Ok(())
    }

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
