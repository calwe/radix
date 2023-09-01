use std::fs;

use serde::{Deserialize, Serialize};
use winit_input_helper::WinitInputHelper;

use crate::{
    map::colored_map::ColoredMap, map::textured_map::TexturedMap, player::Player,
    util::color::Color,
};

const SAVE_PATH: &str = "scenes";

#[derive(Serialize, Deserialize)]
pub enum Map {
    Colored(ColoredMap),
    Textured(TexturedMap),
}

impl Map {
    pub fn get_is_none(&self, x: u32, y: u32) -> bool {
        match self {
            Map::Colored(map) => map.get(x, y) == Color::from_rgb_hex(0),
            Map::Textured(map) => map.get(x, y).is_none(),
        }
    }
}

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
