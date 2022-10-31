use bevy::{prelude::*, render::camera::ScalingMode};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    let mut cam = Camera2dBundle {
        ..Default::default()
    };
    cam.projection.scaling_mode = ScalingMode::Auto {
        min_width: 1.0,
        min_height: 1.0,
    };
    commands.spawn_bundle(cam);
}
