use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TexturedMapBuilder {
    pub walls_path: String,
    pub walls_texture_map: HashMap<u32, String>, // colour -> texture path
    pub floor_path: String,
    pub ceiling_path: String,
}

impl TexturedMapBuilder {
    pub fn load(path: &str) -> Self {
        let contents = std::fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&contents).unwrap()
    }
}