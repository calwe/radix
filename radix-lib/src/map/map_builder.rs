use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use super::{map::Map, sprite::Sprite, texture::Texture};

#[derive(Serialize, Deserialize, Debug)]
pub struct MapBuilder {
    walls_path: String,
    walls_texture_map: HashMap<u32, String>, // colour -> texture path
    floor_path: String,
    floor_texture_map: HashMap<u32, String>, // colour -> texture path
    ceiling_path: String,
    ceiling_texture_map: HashMap<u32, String>, // colour -> texture path
}

impl MapBuilder {
    pub fn load(path: &str) -> Self {
        let contents = std::fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&contents).unwrap()
    }

    pub fn build(&self) -> Map {
        // TODO: change to just rgb?
        let (walls, w1, h1) = self.map_textures(&self.walls_texture_map, &self.walls_path);
        let (floor, w2, h2) = self.map_textures(&self.floor_texture_map, &self.floor_path);
        let (ceiling, w3, h3) = self.map_textures(&self.ceiling_texture_map, &self.ceiling_path);

        let width = if w1 == w2 && w2 == w3 {
            w1
        } else {
            // TODO: better error handling
            panic!("widths of textures do not match")
        };
        let height = if h1 == h2 && h2 == h3 {
            h1
        } else {
            panic!("heights of textures do not match")
        };

        // we can also add our sprites
        // finally we turn this into our TexturedMap
        Map::with_data(width, height, walls, floor, ceiling)
    }

    fn map_textures(
        &self,
        path_map: &HashMap<u32, String>,
        texture_path: &String,
    ) -> (Vec<Option<Rc<Texture>>>, u32, u32) {
        let mut textures = HashMap::new();
        for (color, path) in path_map {
            textures.insert(*color, Rc::new(Texture::new(path)));
        }

        let texture = image::open(texture_path).unwrap().into_rgba8();
        let width = texture.width();
        let height = texture.height();
        let mut mapped_textures = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let pixel = texture.get_pixel(x, y);
                // color in rgba format
                let color = (pixel[0] as u32) << 24
                    | (pixel[1] as u32) << 16
                    | (pixel[2] as u32) << 8
                    | (pixel[3] as u32);
                let out_texture = textures.get(&color);
                mapped_textures.push(out_texture.cloned());
            }
        }
        (mapped_textures, width, height)
    }
}
