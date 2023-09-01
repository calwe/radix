use serde::{Deserialize, Serialize};

use crate::player::Player;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Camera {
    pos_x: f64,
    pos_y: f64,
    dir_x: f64,
    dir_y: f64,
    plane_x: f64,
    plane_y: f64,
    aspect_ratio: f64,
}

impl Camera {
    pub fn new(pos_x: f64, pos_y: f64, aspect_ratio: f64) -> Self {
        Self {
            pos_x,
            pos_y,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: aspect_ratio / 2.0,
            aspect_ratio,
        }
    }

    pub fn pos_x(&self) -> f64 {
        self.pos_x
    }

    pub fn pos_y(&self) -> f64 {
        self.pos_y
    }

    pub fn dir_x(&self) -> f64 {
        self.dir_x
    }

    pub fn dir_y(&self) -> f64 {
        self.dir_y
    }

    pub fn plane_x(&self) -> f64 {
        self.plane_x
    }

    pub fn plane_y(&self) -> f64 {
        self.plane_y
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn set_from_player(&mut self, player: Player) {
        self.pos_x = player.pos_x();
        self.pos_y = player.pos_y();
        self.dir_x = player.dir().cos();
        self.dir_y = player.dir().sin();
        self.plane_x = player.dir().sin() * self.aspect_ratio() / 2.0;
        self.plane_y = -player.dir().cos() * self.aspect_ratio() / 2.0;
    }
}
