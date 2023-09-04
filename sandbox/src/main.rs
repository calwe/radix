use radix_lib::{
    app::App,
    ecs::{
        component::{camera::Camera, player_controller::PlayerController, transform::Transform},
        entity::entity::Entity,
        scene::Scene,
    },
    map::map_builder::MapBuilder,
    window::Window,
};

// Entry point of the program
fn main() {
    pretty_env_logger::init();
    let scene0_map = MapBuilder::load("assets/map/map.yaml").build();

    let window = Window::with_title(1280, 720, 1, "Sandbox Window");

    // entities
    let mut player = Entity::new("Player");
    player.add_component(Transform::new(5.0, 5.0, 0.0));
    player.add_component(Camera::new(window.width() as f64 / window.height() as f64));
    player.add_component(PlayerController::new(
        0.1,
        0.1,
        player.get_component::<Transform>().unwrap().clone(),
        player.get_component::<Camera>().unwrap().clone(),
    ));

    let mut scene0 = Scene::new(scene0_map);
    scene0.add_entity(player);

    App::new()
        .title("Sandbox")
        .window(window)
        .add_scene(scene0)
        .run();
}
