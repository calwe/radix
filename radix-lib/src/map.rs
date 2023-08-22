use crate::util::color::Color;

pub struct Map {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<Color>,
}

impl Map {
    pub fn empty(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: Vec::new(),
        }
    }

    pub fn with_data(width: u32, height: u32, data: Vec<Color>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn with_raw_data(width: u32, height: u32, data: Vec<u32>) -> Self {
        let mut map = Self::with_data(width, height, Vec::new());
        for color in data {
            map.data.push(Color::from_hex(color));
        }
        map
    }
}