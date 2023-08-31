use std::fs;

use serde::{Serialize, Deserialize};

use crate::{player::Player, map::colored_map::ColoredMap, map::textured_map::TexturedMap};

const SAVE_PATH: &str = "scenes";

#[derive(Serialize, Deserialize)]
pub struct Scene {
    name: String,
    pub(crate) player: Player,
    // pub(crate) map: ColoredMap,
    pub(crate) map: TexturedMap,
}

impl Scene {
    pub fn new(name: &str, player: Player, map: TexturedMap) -> Self {
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
}