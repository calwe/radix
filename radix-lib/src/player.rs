use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{camera::Camera, map::Map, window::Window};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Player {
    camera: Camera,
    pos_x: f64,
    pos_y: f64,
    dir: f64,
    speed: f64,
    turn_speed: f64,
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
            window_width: window.width(),
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn pos_x(&self) -> f64 {
        self.pos_x
    }

    pub fn pos_y(&self) -> f64 {
        self.pos_y
    }

    pub fn dir(&self) -> f64 {
        self.dir
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    pub fn turn_speed(&self) -> f64 {
        self.turn_speed
    }

    pub fn update(&mut self, input: &WinitInputHelper, map: &Map) {
        if input.key_held(VirtualKeyCode::W) {
            let x_diff = self.speed * self.dir.cos();
            let y_diff = self.speed * self.dir.sin();
            // check if we will move into a wall
            if map
                .get_wall((self.pos_x + x_diff) as u32, self.pos_y as u32)
                .is_none()
            {
                self.pos_x += x_diff;
            }
            if map
                .get_wall(self.pos_x as u32, (self.pos_y + y_diff) as u32)
                .is_none()
            {
                self.pos_y += y_diff;
            }
        }
        if input.key_held(VirtualKeyCode::S) {
            let x_diff = self.speed * self.dir.cos();
            let y_diff = self.speed * self.dir.sin();
            // check if we will move into a wall
            if map
                .get_wall((self.pos_x - x_diff) as u32, self.pos_y as u32)
                .is_none()
            {
                self.pos_x -= x_diff;
            }
            if map
                .get_wall(self.pos_x as u32, (self.pos_y - y_diff) as u32)
                .is_none()
            {
                self.pos_y -= y_diff;
            }
        }
        if input.key_held(VirtualKeyCode::A) {
            let x_diff = self.speed * self.dir.sin();
            let y_diff = self.speed * self.dir.cos();
            // check if we will move into a wall
            if map
                .get_wall((self.pos_x - x_diff) as u32, self.pos_y as u32)
                .is_none()
            {
                self.pos_x -= x_diff;
            }
            if map
                .get_wall(self.pos_x as u32, (self.pos_y + y_diff) as u32)
                .is_none()
            {
                self.pos_y += y_diff;
            }
        }
        if input.key_held(VirtualKeyCode::D) {
            let x_diff = self.speed * self.dir.sin();
            let y_diff = self.speed * self.dir.cos();
            // check if we will move into a wall
            if map
                .get_wall((self.pos_x + x_diff) as u32, self.pos_y as u32)
                .is_none()
            {
                self.pos_x += x_diff;
            }
            if map
                .get_wall(self.pos_x as u32, (self.pos_y - y_diff) as u32)
                .is_none()
            {
                self.pos_y -= y_diff;
            }
        }

        // here is out custom mouse_diff. it just takes the mouse position and subtracts the center of the screen from it.
        #[cfg(target_os = "windows")]
        let mouse_diff =
            input.cursor().unwrap_or((0.0, 0.0)).0 as f64 - self.window_width as f64 / 2.0;
        #[cfg(target_os = "linux")]
        let mouse_diff = input.mouse_diff().0 as f64;
        self.dir -= self.turn_speed * 0.1 * mouse_diff;

        // the camera is always in the same position as our player
        self.camera.set_from_player(*self);
    }
}
