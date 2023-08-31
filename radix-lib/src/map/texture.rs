use image::{open, ImageBuffer, RgbaImage};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Texture {
    path: String,
    img: RgbaImage,
}

impl Serialize for Texture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.path)
    }
}

impl<'de> Deserialize<'de> for Texture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = String::deserialize(deserializer)?;
        let img = open(&path).unwrap().into_rgba8();
        Ok(Self { path, img })
    }
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let img = open(path).unwrap().into_rgba8();
        Self {
            path: path.to_string(),
            img,
        }
    }

    pub fn empty() -> Self {
        Self {
            path: String::new(),
            img: ImageBuffer::new(1, 1),
        }
    }

    pub fn get(&self, x: u32, y: u32) -> [u8; 4] {
        let pixel = self.img.get_pixel(x, y);
        [pixel[0], pixel[1], pixel[2], pixel[3]]
    }

    pub fn width(&self) -> u32 {
        self.img.width()
    }

    pub fn height(&self) -> u32 {
        self.img.height()
    }
}
