use radix_lib::{app::App, window::Window};

// Entry point of the program
fn main() {
    pretty_env_logger::init();

    App::new()
        .title("Sandbox")
        .window(Window::with_title(1280, 720, 1, "Sandbox Window"))
        .run();
}
