use pixels::{Pixels, SurfaceTexture};

use crate::{util::color::Color, map::Map};

pub struct Renderer {
    pixels: Pixels,
    width: u32,
    height: u32,
    map: Map,
}

impl Renderer {
    pub fn new(window: &winit::window::Window, scale: u32, map: Map,) -> Self {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width / scale, window_size.height / scale, surface_texture).unwrap()
        };

        Self {
            pixels,
            width: window.inner_size().width / scale,
            height: window.inner_size().height / scale,
            map,
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

    pub fn draw_vertical_line(&mut self, color: Color, start: u32, end: u32, xpos: u32) {
        let framebuffer = self.pixels.frame_mut();
        for y in start..end {
            let offset = (y * self.width + xpos) as usize;
            let pixel = &mut framebuffer[offset * 4..offset * 4 + 4];
            pixel.copy_from_slice(&color.to_rgba_arr());
        }
    }
}