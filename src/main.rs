mod matrix_letter;
mod matrix_strip;
mod utils;

use matrix_letter::*;
use matrix_strip::*;

use utils::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::event::Event,
    prelude::*,
    render::camera::ScalingMode,
    window::{WindowMode, WindowResized},
};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};
use bevy_tweening::*;
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Matrix".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MatrixLetterPlugin)
        .add_plugin(MatrixStripPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_system(spawn_strips)
        .add_startup_system(setup)
        //.add_plugin(WorldInspectorPlugin::default())
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    let mut cam = Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        ..Default::default()
    };
    cam.projection.scaling_mode = ScalingMode::FixedVertical(16.0);
    commands.spawn_bundle(cam);

    // commands.spawn_bundle(Text2dBundle {
    //     text: Text::from_section("a", text_style.clone()).with_alignment(TextAlignment::CENTER),
    //     transform: Transform::from_scale(Vec3::splat(0.0005)),
    //     ..default()
    // });
}

fn spawn_strips(mut commands: Commands, time: Res<Time>) {
    let mut rng = thread_rng();
    if exponential_event(0.05, time.delta_seconds()) {
        commands.spawn_bundle(
            MatrixStripBundle::new(Vec3::new(
                rng.gen_range(-15.0..18.0),
                rng.gen_range(0.0..8.0),
                rng.gen_range(-4.0..1.0),
            ))
            .with_lifetime(rng.gen_range(0.5..2.0))
            .with_spawnrate(rng.gen_range(5.0..15.0)),
        );
    }
}

// commands.spawn_bundle(SpriteBundle {
//     sprite: Sprite {
//         color: Color::NAVY,
//         custom_size: Some(Vec2::splat(0.1)),
//         ..default()
//     },
//     ..Default::default()
// });
