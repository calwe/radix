use radix_lib::{
    app::App,
    map::{
        colored_map::ColoredMap, texture::Texture, textured_map::TexturedMap,
        textured_map_builder::TexturedMapBuilder,
    },
    player::Player,
    scene::{self, Map, Scene},
    window::Window,
};

// Entry point of the program
fn main() {
    pretty_env_logger::init();
    let scene0_map = TexturedMapBuilder::load("assets/map/map.yaml").build();

    let window = Window::with_title(1280, 720, 1, "Sandbox Window");
    let scene0 = Scene::new(
        "scene0",
        Player::new(&window, 5.0, 5.0, 0.1, 0.03),
        Map::Textured(scene0_map),
    );

    App::new()
        .title("Sandbox")
        .window(window)
        .add_scene(scene0)
        .run();
}
