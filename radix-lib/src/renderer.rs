use std::{rc::Rc, cell::RefCell};

use log::trace;
use pixels::{Pixels, SurfaceTexture};

use crate::{util::color::Color, map::{colored_map::ColoredMap, textured_map::TexturedMap}, camera::Camera};

pub struct Renderer {
    pixels: Pixels,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(window: &winit::window::Window, scale: u32) -> Self {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width / scale, window_size.height / scale, surface_texture).unwrap()
        };

        Self {
            pixels,
            width: window.inner_size().width / scale,
            height: window.inner_size().height / scale,
        }
    }

    pub fn render(&mut self) {
        self.pixels.render().unwrap();
    }

    pub fn clear(&mut self, color: Color) {
        let framebuffer = self.pixels.frame_mut();
        for pixel in framebuffer.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color.to_rgba_arr());
        }
    }

    pub fn draw_pixel(&mut self, color: Color, x: u32, y: u32) {
        let framebuffer = self.pixels.frame_mut();
        let offset = (y * self.width + x) as usize;
        let pixel = &mut framebuffer[offset * 4..offset * 4 + 4];
        pixel.copy_from_slice(&color.to_rgba_arr());
    }

    pub fn draw_centered_vertical_line(&mut self, color: Color, length: u32, xpos: u32) {
        let start = (self.height as i32 / 2 - length as i32 / 2).max(0);
        let end = (self.height as i32 / 2 + length as i32 / 2).min(self.height as i32);
        self.draw_vertical_line(color, start as u32, end as u32, xpos);
    }

    pub fn draw_vertical_line(&mut self, color: Color, start: u32, end: u32, xpos: u32) {
        let framebuffer = self.pixels.frame_mut();
        for y in start..end {
            let offset = (y * self.width + xpos) as usize;
            let pixel = &mut framebuffer[offset * 4..offset * 4 + 4];
            pixel.copy_from_slice(&color.to_rgba_arr());
        }
    }

    pub fn draw_frame_colored_map(&mut self, camera: &Camera, map: &ColoredMap) {
        // We draw the frame using a method based on DDA.
        // The method used is outlined at https://lodev.org/cgtutor/raycasting.html
        // let mut lines = Vec::new();
        for x in 0..self.width {
            // coverts the x coordinate of the screen to camera space.
            // it simply maps the coordinate to a value between -1 and 1.
            let camera_x = (2 * x) as f64 / self.width as f64 - 1.0;

            // now we calculate the ray direction.
            let ray_dir_x = camera.dir_x + camera.plane_x * camera_x;
            let ray_dir_y = camera.dir_y + camera.plane_y * camera_x;

            // we also need to know which sqaure of the map we are in
            // this is done by truncating the camera's position.
            let mut map_x = camera.pos_x as i32;
            let mut map_y = camera.pos_y as i32;

            // next we need the distance the ray has to travel to go from one x or y side to the next.
            // this is done using the pythagorean theorem; however, this can be simplified, as we only
            // need the ratio between sideDist and deltaDist.
            let delta_dist_x = (1.0 / ray_dir_x).abs();
            let delta_dist_y = (1.0 / ray_dir_y).abs();

            // we also need the distance the ray travels from the camera to the first x or y side.
            // on top of this, we need to know which direction to step in x or y.
            // the step is either a value of 1 or -1
            let mut side_dist_x;
            let mut side_dist_y;
            let step_x;
            let step_y;

            // to calculate these values, we need to know which direction the ray is travelling in.
            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (camera.pos_x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - camera.pos_x) * delta_dist_x;
            }

            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (camera.pos_y - map_y as f64) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - camera.pos_y) * delta_dist_y;
            }
       
            // now that we have all the values we need, we can start the DDA loop.
            // this loop will continue until a wall is hit.
            let mut hit = false; // has the ray hit a wall?
            let mut side = 0; // was a N/S or E/W wall hit: 0 = N/S, 1 = E/W
            while !hit {
                // jump to the next map square, in x or y direction, depending on which is closer.
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                // check if the ray has hit a wall.
                if map.get(map_x as u32, map_y as u32).to_hex() != 0xFFFFFFFF {
                    hit = true;
                }
            }

            // calculate the perpendicular distance between the camera plane and the wall.
            let perp_wall_distance = if side == 0 {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

            // calculate the height of the line to draw on the screen.
            let line_height = (self.height as f64 / perp_wall_distance) as u32;

            // get the color of the wall hit, and darken it if it is a E/W wall.
            let mut color = map.get(map_x as u32, map_y as u32);
            if side == 1 {
                color = color.darken(0.5);
            }

            // draw the vertical line that represents a stripe of the wall.
            self.draw_centered_vertical_line(color, line_height, x);
        }
    }

    // TODO: lots of duplicated code from prev function - fix
    pub fn draw_frame_textured_map(&mut self, camera: &Camera, map: &TexturedMap) {
        self.draw_floor_and_ceiling(camera, map);

        // We draw the frame using a method based on DDA.
        // The method used is outlined at https://lodev.org/cgtutor/raycasting.html
        // let mut lines = Vec::new();
        for x in 0..self.width {
            // coverts the x coordinate of the screen to camera space.
            // it simply maps the coordinate to a value between -1 and 1.
            let camera_x = (2 * x) as f64 / self.width as f64 - 1.0;

            // now we calculate the ray direction.
            let ray_dir_x = camera.dir_x + camera.plane_x * camera_x;
            let ray_dir_y = camera.dir_y + camera.plane_y * camera_x;

            // we also need to know which sqaure of the map we are in
            // this is done by truncating the camera's position.
            let mut map_x = camera.pos_x as i32;
            let mut map_y = camera.pos_y as i32;

            // next we need the distance the ray has to travel to go from one x or y side to the next.
            // this is done using the pythagorean theorem; however, this can be simplified, as we only
            // need the ratio between sideDist and deltaDist.
            let delta_dist_x = (1.0 / ray_dir_x).abs();
            let delta_dist_y = (1.0 / ray_dir_y).abs();

            // we also need the distance the ray travels from the camera to the first x or y side.
            // on top of this, we need to know which direction to step in x or y.
            // the step is either a value of 1 or -1
            let mut side_dist_x;
            let mut side_dist_y;
            let step_x;
            let step_y;

            // to calculate these values, we need to know which direction the ray is travelling in.
            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (camera.pos_x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - camera.pos_x) * delta_dist_x;
            }

            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (camera.pos_y - map_y as f64) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - camera.pos_y) * delta_dist_y;
            }
       
            // now that we have all the values we need, we can start the DDA loop.
            // this loop will continue until a wall is hit.
            let mut hit = false; // has the ray hit a wall?
            let mut side = 0; // was a N/S or E/W wall hit: 0 = N/S, 1 = E/W
            while !hit {
                // jump to the next map square, in x or y direction, depending on which is closer.
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                // check if the ray has hit a wall.
                if map.get(map_x as u32, map_y as u32).is_some() {
                    hit = true;
                }
            }

            // calculate the perpendicular distance between the camera plane and the wall.
            let perp_wall_distance = if side == 0 {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

            // calculate the height of the line to draw on the screen.
            let line_height = (self.height as f64 / perp_wall_distance) as u32;
            let start = (self.height as i32 / 2 - line_height as i32 / 2).max(0);
            let end = (self.height as i32 / 2 + line_height as i32 / 2).min(self.height as i32);

            let texture = map.get(map_x as u32, map_y as u32).unwrap();

            let wall_x = if side == 0 {
                camera.pos_y + perp_wall_distance * ray_dir_y
            } else {
                camera.pos_x + perp_wall_distance * ray_dir_x
            };
            let wall_x = wall_x - wall_x.floor();

            let mut tex_x = (wall_x * texture.width() as f64) as u32;
            if (side == 0 && ray_dir_x > 0.0)
            || (side == 1 && ray_dir_y < 0.0) {
                tex_x = texture.width() - tex_x - 1;
            }

            let step = 1.0 * texture.height() as f64 / line_height as f64;
            let mut tex_pos = (start - self.height as i32 / 2 + line_height as i32) as f64 * step;
            for y in start..end {
                let tex_y = tex_pos as u32 & (texture.height() - 1);
                tex_pos += step;
                let mut color = Color::from_rgba_arr(texture.get(tex_x, tex_y));
                if side == 1 {
                    color = color.darken(0.7);
                }
                self.draw_pixel(color, x, y as u32);
            }
        }
    }

    pub fn draw_floor_and_ceiling(&mut self, camera: &Camera, map: &TexturedMap) {
        for y in 0..self.height {
            // left ray
            let ray_dir_x0 = camera.dir_x - camera.plane_x;
            let ray_dir_y0 = camera.dir_y - camera.plane_y;
            // right ray
            let ray_dir_x1 = camera.dir_x + camera.plane_x;
            let ray_dir_y1 = camera.dir_y + camera.plane_y;

            // current y position compared to the center of the screen (the horizon)
            let pos_y = y as f64 - self.height as f64 / 2.0;

            // vertical position of the camera
            let pos_z = 0.5 * self.height as f64;

            // horizontal distance from the camera to the floor for the current row
            let row_distance = pos_z / pos_y;

            // calculate the real world step vector we have to add for each x (parallel to camera plane)
            // adding step by step avoids multiplications with a weight in the inner loop
            let step_x = row_distance * (ray_dir_x1 - ray_dir_x0) / self.width as f64;
            let step_y = row_distance * (ray_dir_y1 - ray_dir_y0) / self.width as f64;

            // real world coordinates of the leftmost column. This will be updated as we step to the right.
            let mut floor_x = camera.pos_x + row_distance * ray_dir_x0;
            let mut floor_y = camera.pos_y + row_distance * ray_dir_y0;

            for x in 0..self.width {
                // the cell coord is simply got from the integer parts of floor_x and floor_y
                let cell_x = floor_x.floor() as u32;
                let cell_y = floor_y.floor() as u32;

                // get the texture coordinate from the fractional part
                let mut tx = (map.floor.width() as f64 * (floor_x - floor_x.floor())) as u32 & (map.floor.width() - 1);
                let mut ty = (map.floor.height() as f64 * (floor_y - floor_y.floor())) as u32 & (map.floor.height() - 1);

                floor_x += step_x;
                floor_y += step_y;

                let mut color = Color::from_rgba_arr(map.floor.get(tx, ty));
                color = color.darken(0.5);
                self.draw_pixel(color, x, y);

                let mut color = Color::from_rgba_arr(map.ceiling.get(tx, ty));
                color = color.darken(0.5);
                self.draw_pixel(color, x, self.height - y - 1);
            }
        }
    }
}