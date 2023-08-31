use std::{collections::HashMap, rc::Rc};

use serde::{Serialize, Deserialize};

use super::{textured_map::TexturedMap, texture::Texture};

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

    pub fn build(&self) -> TexturedMap {
        // first we will turn our walls texture hashmap into one of <u32, Rc<Texture>>
        let mut wall_textures = HashMap::new();
        for (color, path) in &self.walls_texture_map {
            wall_textures.insert(*color, Rc::new(Texture::new(path)));
        }

        // now we can build our textured map
        // TODO: change to just rgb?
        let walls_texture = image::open(&self.walls_path).unwrap().into_rgba8();
        // TODO: once we have floor maps, we need to check that the heights and widths match
        let width = walls_texture.width();
        let height = walls_texture.height();
        let mut walls = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let pixel = walls_texture.get_pixel(x, y);
                // color in rgba format
                let color = (pixel[0] as u32) << 24 | (pixel[1] as u32) << 16 | (pixel[2] as u32) << 8 | (pixel[3] as u32);
                let texture = wall_textures.get(&color);
                walls.push(texture.cloned());
            }
        }

        // now we can just load our floor and ceiling
        let floor = Rc::new(Texture::new(&self.floor_path));
        let ceiling = Rc::new(Texture::new(&self.ceiling_path));

        // finally we turn this into our TexturedMap
        TexturedMap::with_data(width, height, walls, floor, ceiling)
    }
}