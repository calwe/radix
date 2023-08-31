use std::rc::Rc;

use radix_lib::{app::App, window::Window, scene::{Scene, self, Map}, map::{colored_map::ColoredMap, texture::Texture, textured_map::TexturedMap}, player::Player};

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

    let a = None;
    let w = Some(Rc::new(Texture::new("assets/Texture45.png")));
    let b = Some(Rc::new(Texture::new("assets/Texture190.png")));
    // fix this mess
    let scene0_map = [
        w.clone(), w.clone(), w.clone(), w.clone(), w.clone(), w.clone(), w.clone(), w.clone(),
        w.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), w.clone(),
        w.clone(), a.clone(), b.clone(), a.clone(), a.clone(), a.clone(), a.clone(), w.clone(),
        w.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), w.clone(),
        w.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), w.clone(),
        w.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), w.clone(),
        w.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), a.clone(), w.clone(),
        w.clone(), w.clone(), w.clone(), w.clone(), w.clone(), w.clone(), w.clone(), w.clone(),
    ];

    let window = Window::with_title(1280, 720, 1, "Sandbox Window");
    let scene0 = Scene::new(
        "scene0",
        Player::new(&window, 5.0, 5.0, 0.1, 0.05),
        Map::Textured(TexturedMap::with_data(8, 8, scene0_map.to_vec())),
    );
    let scene1 = Scene::new(
        "scene1",
        Player::new(&window, 5.0, 5.0, 0.08, 0.1),
        Map::Colored(ColoredMap::with_raw_data(12, 12, SCENE1_MAP.to_vec())),
    );

    App::new()
        .title("Sandbox")
        .window(window)
        .add_scene(scene0)
        .add_scene(scene1)
        .run();
}
