use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{camera::Camera, window::Window};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub(crate) camera: Camera,
    pub(crate) pos_x: f64,
    pub(crate) pos_y: f64,
    pub(crate) dir: f64,
    pub(crate) speed: f64,
    pub(crate) turn_speed: f64,
    window_width: u32,
}

impl Player {
    pub fn new(window: &Window, pos_x: f64, pos_y: f64, speed: f64, turn_speed: f64) -> Self {
        Self {
            camera: Camera::new(pos_x, pos_y, window.aspect_ratio()),
            pos_x,
            pos_y,
            dir: 0.0,
            speed,
            turn_speed,
            window_width: window.width,
        }
    }
    
    pub fn update(&mut self, input: &WinitInputHelper) {
        if input.key_held(VirtualKeyCode::W) {
            self.pos_x += self.speed * self.dir.cos();
            self.pos_y += self.speed * self.dir.sin();
        }
        if input.key_held(VirtualKeyCode::S) {
            self.pos_x -= self.speed * self.dir.cos();
            self.pos_y -= self.speed * self.dir.sin();
        }
        if input.key_held(VirtualKeyCode::A) {
            self.pos_x -= self.speed * self.dir.sin();
            self.pos_y += self.speed * self.dir.cos();
        }
        if input.key_held(VirtualKeyCode::D) {
            self.pos_x += self.speed * self.dir.sin();
            self.pos_y -= self.speed * self.dir.cos();
        }

        let mouse_diff = input.mouse().unwrap_or((0.0, 0.0)).0 as f64 - self.window_width as f64 / 2.0;
        self.dir -= self.turn_speed * 0.1 * mouse_diff;

        self.camera.pos_x = self.pos_x;
        self.camera.pos_y = self.pos_y;
        self.camera.dir_x = self.dir.cos();
        self.camera.dir_y = self.dir.sin();
        self.camera.plane_x = self.dir.sin() * self.camera.aspect_ratio / 2.0;
        self.camera.plane_y = -self.dir.cos() * self.camera.aspect_ratio / 2.0;
    }
}