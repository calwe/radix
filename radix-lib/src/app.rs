use std::{cell::RefCell, rc::Rc};

use game_loop::game_loop;
use log::info;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, VirtualKeyCode},
    event_loop::EventLoop,
};
use winit_input_helper::WinitInputHelper;

use crate::{
    renderer::Renderer, scene::Scene, scripting::engine::Engine, util::color::Color, window::Window,
};

/// The base struct for the engine. Uses the 'Builder Pattern' to be constructed
pub struct App {
    /// The title for the app - will be used in window titlebar
    title: String,
    max_frame_time: f64,
    tps: u32,
    window: Window,
    renderer: Option<Renderer>,
    input: WinitInputHelper,
    script_engine: Option<Engine>,
    scenes: Vec<Rc<RefCell<Scene>>>,
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
            script_engine: None,
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
        env_logger::init();

        // we must first create a window, and then pass it to the renderer.
        // this uses the infomation provided from our own Window wrapper struct.
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(self.window.title().unwrap_or(&self.title))
            .with_inner_size(LogicalSize::new(self.window.width(), self.window.height()))
            .with_min_inner_size(LogicalSize::new(
                self.window.width() / self.window.scale(),
                self.window.height() / self.window.scale(),
            ))
            .build(&event_loop)
            .unwrap();
        self.renderer = Some(Renderer::new(&window, self.window.scale()));

        // load the script engines globals, and call start
        if let Some(engine) = &mut self.script_engine {
            engine.set_scene(self.scenes[self.current_scene].clone());
            engine.load_globals();
            engine.start();
        }

        // this is the core loop of the engine.
        //   - the second argument defines how many ticks per second the game should be updated at.
        //      it *doesn't* specify how quick the game renders - rendering happens as quickly as possible.
        //   - the third argument is the maximum frame time. this stops the users app from falling behind,
        //      instead less updates are called per second, slowing the game down. this isn't ideal, but is
        //      more desirable than falling behind.
        let tps = self.tps;
        let max_frame_time = self.max_frame_time;
        game_loop(
            event_loop,
            window,
            self,
            tps,
            max_frame_time,
            |g| {
                g.game.update();

                // lock cursor - winit currently doesn't support this on windows, so we have to use a hacky workaround.
                //               another side affect of this is that we cannot use input.mouse_diff, as our cursor constantly
                //               gets reset to the center of the screen. the solution is shown in the player class.
                let _ = g.window.set_cursor_position(LogicalPosition::new(
                    g.game.window.width() as f64 / 2.0,
                    g.game.window.height() as f64 / 2.0,
                ));
                g.window.set_cursor_visible(false);
            },
            |g| {
                g.game.render();
            },
            |g, event| {
                if !g.game.handle_event(event) {
                    g.exit();
                };
            },
        );
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

    /// Sets a main script for the app
    pub fn script_path(mut self, script_path: &str) -> Self {
        let mut engine = Engine::new();
        engine.load_script(script_path);
        self.script_engine = Some(engine);
        self
    }

    /// Adds a scene to the app
    pub fn add_scene(mut self, scene: Scene) -> Self {
        self.scenes.push(Rc::new(RefCell::new(scene)));
        self
    }

    // ---------------------------------------------------
    //  Private functions
    // ---------------------------------------------------
    fn update(&mut self) {
        // self.camera.add_position(0.01, 0.01);
        let current_scene = &mut self.scenes[self.current_scene].borrow_mut();
        current_scene.update(&self.input);
    }

    fn render(&mut self) {
        let renderer = self.renderer.as_mut().unwrap();
        let current_scene = &mut self.scenes[self.current_scene].borrow();

        renderer.clear(Color::from_rgb_hex(0xe1a2ef));

        // DI
        renderer.draw_frame_textured_map(
            current_scene.player().borrow().camera(),
            current_scene.map(),
        );

        renderer.render();
    }

    fn handle_event(&mut self, event: &Event<()>) -> bool {
        if let Some(engine) = &self.script_engine {
            if engine.update_input(event) {
                engine.update();
            }
        }

        if self.input.update(event) {
            // Close events
            if self.input.key_pressed(VirtualKeyCode::Escape)
                || self.input.close_requested()
                || self.input.destroyed()
            {
                return false;
            }

            // TODO: add a better way to switch scenes (probs with scripting, when thats a thing)
            if self.input.key_pressed(VirtualKeyCode::Space) {
                let current_scene = self.current_scene;
                let next_scene = (current_scene + 1) % self.scenes.len();
                info!(
                    "Switching from scene {} to scene {}",
                    current_scene, next_scene
                );
                self.current_scene = next_scene;
            }
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
        let app = App::new().title("Test App");

        assert_eq!(app.title, "Test App");
    }

    #[test]
    fn set_max_frame_time() {
        let app = App::new().max_frame_time(0.1);

        assert_eq!(app.max_frame_time, 0.1);
    }

    #[test]
    fn set_tps() {
        let app = App::new().tps(60);

        assert_eq!(app.tps, 60);
    }
}
