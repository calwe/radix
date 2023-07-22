use radix_lib::app::App;

// Entry point of the program
fn main() {
    pretty_env_logger::init();

    App::new()
        .title("Sandbox")
        .run();
}
