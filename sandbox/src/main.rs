use radix_lib::{app::App, window::Window, scene::Scene, map::colored_map::ColoredMap, player::Player};

const R: u32 = 0xFF0000FF;
const G: u32 = 0x00FF00FF;
const B: u32 = 0x0000FFFF;
const A: u32 = 0xFFFFFFFF;
const SCENE0_MAP: [u32; 64] = [
    R, R, R, R, R, R, R, R,
    R, A, A, A, A, A, A, R,
    R, A, G, A, A, A, A, R,
    R, A, A, A, A, A, A, R,
    R, A, A, A, A, A, A, R,
    R, A, A, A, A, A, A, R,
    R, A, A, A, A, A, A, R,
    R, R, R, R, R, R, R, R,
];
const SCENE1_MAP: [u32; 144] = [
    B, B, B, B, B, B, B, B, B, B, B, B,
    B, A, A, A, A, A, A, A, A, A, A, B,
    B, A, R, A, A, A, A, A, A, A, A, B,
    B, A, A, A, A, A, A, A, A, A, A, B,
    B, A, A, A, A, A, R, R, A, A, A, B,
    B, A, A, A, A, A, R, A, A, A, A, B,
    B, A, A, A, A, A, A, A, A, A, G, B,
    B, A, A, A, A, A, A, A, A, A, A, B,
    B, A, A, A, A, A, A, A, A, A, A, B,
    B, A, G, A, A, A, A, G, A, A, A, B,
    B, A, A, A, A, A, A, A, A, A, A, B,
    B, B, B, B, B, B, B, B, B, B, B, B,
];

// Entry point of the program
fn main() {
    pretty_env_logger::init();

    let window = Window::with_title(1280, 720, 1, "Sandbox Window");
    let scene0 = Scene::new(
        "small_scene",
        Player::new(&window, 1.5, 1.5, 0.05, 0.05),
        ColoredMap::with_raw_data(8, 8, SCENE0_MAP.to_vec()),
    );
    let scene1 = Scene::new(
        "larger_scene",
        Player::new(&window, 1.5, 1.5, 0.07, 0.03),
        ColoredMap::with_raw_data(12, 12, SCENE1_MAP.to_vec()),
    );

    App::new()
        .title("Sandbox")
        .window(window)
        .add_scene(scene0)
        .add_scene(scene1)
        .run();
}
