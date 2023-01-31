mod matrix_field;
mod matrix_letter;
mod matrix_strip;
mod post;
mod utils;

use matrix_field::*;
use matrix_letter::*;
use matrix_strip::*;
use post::*;

use bevy::{
    core_pipeline::bloom::{self, BloomSettings},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::{PresentMode, WindowMode, WindowResized},
};
use bevy_editor_pls::prelude::*;
use bevy_tweening::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Matrix".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(TweeningPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MatrixLetterPlugin)
        .add_plugin(MatrixStripPlugin)
        .add_plugin(MatrixFieldPlugin)
        //.add_plugin(PostPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        //.add_plugin(WorldInspectorPlugin::default())
        .add_plugin(EditorPlugin)
        .add_system(update_bloom_settings)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Res<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut post_materials: ResMut<Assets<PostProcessingMaterial>>,
) {
    let window = windows.get_primary().unwrap();

    let size = Extent3d {
        width: 3840,
        height: 2400,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };
    image.resize(size);
    //let image_handle = images.add(image);
    // Camera
    let mut cam = Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        camera: Camera {
            hdr: true,
            //target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        ..default()
    };
    //commands.insert_resource(PostImage(image_handle.clone()));
    cam.projection.scaling_mode = ScalingMode::FixedVertical(16.0);
    commands.spawn((
        cam,
        BloomSettings {
            threshold: 0.2,
            scale: 0.6,
            knee: 0.1,
            intensity: 0.15,
            ..Default::default()
        },
    ));

    /*let post_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        1.0 as f32, 1.0 as f32,
    ))));

    let material_handle = post_materials.add(PostProcessingMaterial {
        source_image: image_handle.clone(),
        time: ShaderTime {
            secs_since_startup: 0.0,
            dt: 0.0,
        },
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                scale: Vec3::new(size.width as f32, size.height as f32, 1.0),
                ..default()
            },
            ..default()
        },
        post_layer,
        PostQuad,
    ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                priority: 1,
                ..default()
            },
            ..default()
        },
        post_layer,
    ));*/
}

fn resize(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        let mut size = Extent3d {
            width: e.width as u32,
            height: e.height as u32,
            ..default()
        };
    }
}

fn update_bloom_settings(
    mut camera: Query<&mut BloomSettings>,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut bloom_settings = camera.single_mut();

    let dt = time.delta_seconds();

    if keycode.pressed(KeyCode::Q) {
        bloom_settings.threshold -= dt;
    }
    if keycode.pressed(KeyCode::W) {
        bloom_settings.threshold += dt;
    }

    if keycode.pressed(KeyCode::R) {
        bloom_settings.knee -= dt;
    }
    if keycode.pressed(KeyCode::T) {
        bloom_settings.knee += dt;
    }

    if keycode.pressed(KeyCode::A) {
        bloom_settings.scale -= dt;
    }
    if keycode.pressed(KeyCode::S) {
        bloom_settings.scale += dt;
    }

    if keycode.pressed(KeyCode::D) {
        bloom_settings.intensity -= dt;
    }
    if keycode.pressed(KeyCode::F) {
        bloom_settings.intensity += dt;
    }
    println!(
        "I: {}, K: {}, S: {}, T: {}",
        bloom_settings.intensity,
        bloom_settings.knee,
        bloom_settings.scale,
        bloom_settings.threshold
    );
}
