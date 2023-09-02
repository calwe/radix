use radix_lib::{
    app::App, map::map_builder::MapBuilder, player::Player, scene::Scene, window::Window,
};

// Entry point of the program
fn main() {
    let scene0_map = MapBuilder::load("assets/map/map.yaml").build();

    let window = Window::with_title(1280, 720, 1, "Sandbox Window");
    let scene0 = Scene::new(
        "scene0",
        Player::new(&window, 5.0, 5.0, 0.1, 0.03),
        scene0_map,
    );

    App::new()
        .title("Sandbox")
        .window(window)
        .script_path("assets/scripts/main.lua")
        .add_scene(scene0)
        .run();
}
