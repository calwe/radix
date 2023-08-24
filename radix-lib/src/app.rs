use log::{trace, info};
use game_loop::game_loop;
use winit::{event_loop::EventLoop, event::{Event, WindowEvent, VirtualKeyCode}, dpi::{LogicalSize, LogicalPosition}, window::CursorGrabMode};
use winit_input_helper::WinitInputHelper;

use crate::{window::Window, renderer::Renderer, util::color::Color, map::Map, camera::Camera, player::Player, scene::Scene};

const R: u32 = 0xFF0000FF;
const G: u32 = 0x00FF00FF;
const A: u32 = 0xFFFFFFFF;
const DEFAULT_MAP: [u32; 64] = [
    R, R, R, R, R, R, R, R,
    R, A, A, A, A, A, A, R,
    R, A, G, A, A, A, A, R,
    R, A, A, A, A, A, A, R,
    R, A, A, A, A, A, A, R,
    R, A, A, A, A, A, A, R,
    R, A, A, A, A, A, A, R,
    R, R, R, R, R, R, R, R,
];

/// The base struct for the engine. Uses the 'Builder Pattern' to be constructed
pub struct App {
    /// The title for the app - will be used in window titlebar
    title: String,
    max_frame_time: f64,
    tps: u32,
    window: Window,
    renderer: Option<Renderer>,
    input: WinitInputHelper,
    scenes: Vec<Scene>,
    current_scene: usize,
}

/// A rust trait that specifies the initial state of the app
impl Default for App {
    fn default() -> Self {
        Self { 
            title: "A Radix App".to_string(), 
            max_frame_time: 0.1,
            tps: 60,
            window: Window::new(800, 600, 1),
            renderer: None,
            input: WinitInputHelper::new(),
            scenes: Vec::new(),
            current_scene: 0,
        }
    }
}

impl App {
    /// Wrapper for default()
    pub fn new() -> Self {
        Self::default()
    }

    /// The final function called after defining the app.
    pub fn run(mut self) {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(self.window.title.as_ref().unwrap_or(&self.title)) 
            .with_inner_size(LogicalSize::new(self.window.width, self.window.height))
            .with_min_inner_size(LogicalSize::new(self.window.width / self.window.scale, self.window.height / self.window.scale))
            .build(&event_loop)
            .unwrap();
        self.renderer = Some(Renderer::new(&window, self.window.scale, 
            Map::with_raw_data(8, 8, DEFAULT_MAP.to_vec()),
        ));
        
        // TEST SCENE
        let map = Map::with_raw_data(8, 8, DEFAULT_MAP.to_vec());
        let camera = Camera::new(1.5, 1.5, self.window.width as f64 / self.window.height as f64);
        let player = Player::new(camera, 1.5, 1.5, 0.1, 0.1);
        let scene = Scene::new("test", player, map);
        scene.save().unwrap();
        self.scenes.push(scene);

        // this is the core loop of the engine.
        //   - the second argument defines how many ticks per second the game should be updated at.
        //      it *doesn't* specify how quick the game renders - rendering happens as quickly as possible.
        //   - the third argument is the maximum frame time. this stops the users app from falling behind,
        //      instead less updates are called per second, slowing the game down. this isn't ideal, but is
        //      more desirable than falling behind.
        let tps = self.tps;
        let max_frame_time = self.max_frame_time;
        game_loop(event_loop, window, self, tps, max_frame_time, |g| {
            g.game.update();
            
            // lock cursor - winit currently doesn't support this on windows, so we have to use a hacky workaround.
            let _ = g.window.set_cursor_position(LogicalPosition::new(g.game.window.width as f64 / 2.0, g.game.window.height as f64 / 2.0));
            g.window.set_cursor_visible(false);
        }, |g| {
            g.game.render();
        }, |g, event| {
            if g.game.input.update(event) {
                // Close events
                if g.game.input.key_pressed(VirtualKeyCode::Escape) || g.game.input.close_requested() || g.game.input.destroyed() {
                    g.exit();
                    return;
                }

            }
        });
    }

    // ---------------------------------------------------
    //  Builder functions
    // ---------------------------------------------------

    /// Sets the title, takes a `&str` rather than a `String` for ergonomics
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets a custom max_frame_time
    pub fn max_frame_time(mut self, max_frame_time: f64) -> Self {
        self.max_frame_time = max_frame_time;
        self
    }

    /// Sets a custom max_frame_time
    pub fn tps(mut self, tps: u32) -> Self {
        self.tps = tps;
        self
    }

    /// Sets a custom window - and gives it the title of our app if not specified.
    pub fn window(mut self, window: Window) -> Self {
        self.window = window;
        self
    }

    // ---------------------------------------------------
    //  Private functions
    // ---------------------------------------------------
    fn update(&mut self) {
        // self.camera.add_position(0.01, 0.01);
        let current_scene = &mut self.scenes[self.current_scene];
        current_scene.player.update(&self.input);
    }

    fn render(&mut self) {
        let renderer = self.renderer.as_mut().unwrap();
        let current_scene = &mut self.scenes[self.current_scene];

        renderer.clear(Color::from_rgb_hex(0xe1a2ef));

        // DI
        renderer.draw_frame(&current_scene.player.camera);

        renderer.render();
    }

    fn handle_event(&self, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    info!("Close requested, exiting...");
                    return false;
                },
                _ => {},
            },
            _ => {},
        }

        true
    }
}

#[cfg(test)]
// Unit tests for the App struct
mod tests {
    use super::*;

    #[test]
    fn set_title() {
        let app = App::new()
            .title("Test App");

        assert_eq!(app.title, "Test App");
    }

    #[test]
    fn set_max_frame_time() {
        let app = App::new()
            .max_frame_time(0.1);

        assert_eq!(app.max_frame_time, 0.1);
    }

    #[test]
    fn set_tps() {
        let app = App::new()
            .tps(60);

        assert_eq!(app.tps, 60);
    }
}