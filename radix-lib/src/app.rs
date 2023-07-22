use log::trace;
use game_loop::game_loop;

/// The base struct for the engine. Uses the 'Builder Pattern' to be constructed
pub struct App {
    /// The title for the app - will be used in window titlebar
    title: String,
    max_frame_time: f64,
    tps: u32,
}

/// A rust trait that specifies the initial state of the app
impl Default for App {
    fn default() -> Self {
        Self { 
            title: "A Radix App".to_string(), 
            max_frame_time: 0.1,
            tps: 60,
        }
    }
}

impl App {
    /// Wrapper for default()
    pub fn new() -> Self {
        Self::default()
    }

    /// The final function called after defining the app.
    pub fn run(&self) {
        // this is the core loop of the engine.
        //   - the second argument defines how many ticks per second the game should be updated at.
        //      it *doesn't* specify how quick the game renders - rendering happens as quickly as possible.
        //   - the third argument is the maximum frame time. this stops the users app from falling behind,
        //      instead less updates are called per second, slowing the game down. this isn't ideal, but is
        //      more desirable than falling behind.
        game_loop(self, self.tps, self.max_frame_time, |g| {
            g.game.update();
        }, |g| {
            g.game.render();
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

    // ---------------------------------------------------
    //  Private functions
    // ---------------------------------------------------
    fn update(&self) {
        trace!("Update: '{}'", self.title);
    }

    fn render(&self) {

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