use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    color: u32,
}

impl Color {
    pub fn from_hex(hex: u32) -> Self {
        Self {
            color: hex,
        }
    }

    pub fn from_rgb_hex(hex: u32) -> Self {
        Self::from_hex(hex << 8 | 0xFF)
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            color: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32),
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }

    pub fn to_hex(&self) -> u32 {
        self.color
    }

    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        let r = (self.color >> 24) as u8;
        let g = (self.color >> 16) as u8;
        let b = (self.color >> 8) as u8;
        let a = self.color as u8;

        (r, g, b, a)
    }

    pub fn to_rgba_arr(&self) -> [u8; 4] {
        let (r, g, b, a) = self.to_rgba();
        [r, g, b, a]
    }

    pub fn darken(&self, value: f64) -> Color {
        let (r, g, b, a) = self.to_rgba();
        let r = (r as f64 * value) as u8;
        let g = (g as f64 * value) as u8;
        let b = (b as f64 * value) as u8;
        Color::from_rgba(r, g, b, a)
    }
}