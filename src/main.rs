mod matrix_field;
mod matrix_letter;
mod matrix_strip;
mod utils;

use matrix_field::*;
use matrix_letter::*;
use matrix_strip::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::camera::ScalingMode,
    window::WindowMode,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tweening::*;

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
        .add_plugin(MatrixFieldPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_plugin(WorldInspectorPlugin::default())
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
}
