use std::{cell::RefCell, rc::Rc};

use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::map::Map;

use super::{camera::Camera, transform::Transform, Component, ComponentType};

pub struct PlayerController {
    speed: f64,
    turn_speed: f64,
    transform: Rc<RefCell<Transform>>,
    camera: Rc<RefCell<Camera>>,
}

impl Component for PlayerController {
    fn update(&mut self, input: &WinitInputHelper, map: &Map) {
        {
            let mut transform = self.transform.borrow_mut();
            let pos_x = transform.pos_x();
            let pos_y = transform.pos_y();
            let dir = transform.dir();
            if input.key_held(VirtualKeyCode::W) {
                let x_diff = self.speed * dir.cos();
                let y_diff = self.speed * dir.sin();
                // check if we will move into a wall
                if map
                    .get_wall((pos_x + x_diff) as u32, pos_y as u32)
                    .is_none()
                {
                    transform.add_pos_x(x_diff);
                }
                if map
                    .get_wall(pos_x as u32, (pos_y + y_diff) as u32)
                    .is_none()
                {
                    transform.add_pos_y(y_diff);
                }
            }
            if input.key_held(VirtualKeyCode::S) {
                let x_diff = self.speed * dir.cos();
                let y_diff = self.speed * dir.sin();
                // check if we will move into a wall
                if map
                    .get_wall((pos_x - x_diff) as u32, pos_y as u32)
                    .is_none()
                {
                    transform.add_pos_x(-x_diff);
                }
                if map
                    .get_wall(pos_x as u32, (pos_y - y_diff) as u32)
                    .is_none()
                {
                    transform.add_pos_y(-y_diff);
                }
            }
            if input.key_held(VirtualKeyCode::A) {
                let x_diff = self.speed * dir.sin();
                let y_diff = self.speed * dir.cos();
                // check if we will move into a wall
                if map
                    .get_wall((pos_x - x_diff) as u32, pos_y as u32)
                    .is_none()
                {
                    transform.add_pos_x(-x_diff);
                }
                if map
                    .get_wall(pos_x as u32, (pos_y + y_diff) as u32)
                    .is_none()
                {
                    transform.add_pos_y(y_diff);
                }
            }
            if input.key_held(VirtualKeyCode::D) {
                let x_diff = self.speed * dir.sin();
                let y_diff = self.speed * dir.cos();
                // check if we will move into a wall
                if map
                    .get_wall((pos_x + x_diff) as u32, pos_y as u32)
                    .is_none()
                {
                    transform.add_pos_x(x_diff);
                }
                if map
                    .get_wall(pos_x as u32, (pos_y - y_diff) as u32)
                    .is_none()
                {
                    transform.add_pos_y(-y_diff);
                }
            }

            // here is out custom mouse_diff. it just takes the mouse position and subtracts the center of the screen from it.
            // TODO: the 1280 here should be replaced. THE INPUT SYSTEM SHOULD BE EXTRACTED
            #[cfg(target_os = "windows")]
            let mouse_diff = input.mouse().unwrap_or((0.0, 0.0)).0 as f64 - 1280f64 / 2.0;
            #[cfg(target_os = "linux")]
            let mouse_diff = input.mouse_diff().0 as f64;
            transform.set_dir(dir - self.turn_speed * 0.1 * mouse_diff);
        }
        // the camera is always in the same position as our player
        self.camera
            .borrow_mut()
            .set_from_transform(&self.transform.borrow());
    }
    fn start(&mut self) {}
}

impl Into<ComponentType> for PlayerController {
    fn into(self) -> ComponentType {
        ComponentType::PlayerController(Rc::new(RefCell::new(self)))
    }
}

impl PlayerController {
    pub fn new(
        speed: f64,
        turn_speed: f64,
        transform: Rc<RefCell<Transform>>,
        camera: Rc<RefCell<Camera>>,
    ) -> Self {
        Self {
            speed,
            turn_speed,
            transform,
            camera,
        }
    }
}
