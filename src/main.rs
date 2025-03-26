mod matrix_field;
mod matrix_letter;
mod matrix_strip;
//mod post;
mod utils;

use matrix_field::*;
use matrix_letter::*;
use matrix_strip::*;
//use post::*;

use bevy::{
    core_pipeline::bloom::{self, BloomPrefilterSettings, BloomSettings},
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
    window::{PresentMode, PrimaryWindow, WindowMode, WindowResized},
};
use bevy_editor_pls::prelude::*;
use bevy_tweening::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Matrix".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(TweeningPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(MatrixLetterPlugin)
        .add_plugins(MatrixStripPlugin)
        .add_plugins(MatrixFieldPlugin)
        //.add_plugin(PostPlugin)
        .add_systems(Update, close_on_esc)
        .add_systems(Startup, setup)
        //.add_plugin(WorldInspectorPlugin::default())
        //.add_plugins(EditorPlugin::default())
        .add_systems(Update, update_bloom_settings)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut post_materials: ResMut<Assets<PostProcessingMaterial>>,
) {
    let window = primary_window.single();

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
            view_formats: &[TextureFormat::Bgra8UnormSrgb],
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
            prefilter_settings: BloomPrefilterSettings {
                threshold_softness: 0.1,
                threshold: 0.2,
            },
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
    for e in reader.read(&resize_event) {
        let mut size = Extent3d {
            width: e.width as u32,
            height: e.height as u32,
            ..default()
        };
    }
}

fn update_bloom_settings(
    mut camera: Query<&mut BloomSettings>,
    keycode: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut bloom_settings = camera.single_mut();

    let dt = time.delta_seconds();

    if keycode.pressed(KeyCode::KeyQ) {
        bloom_settings.prefilter_settings.threshold -= dt;
    }
    if keycode.pressed(KeyCode::KeyW) {
        bloom_settings.prefilter_settings.threshold += dt;
    }

    if keycode.pressed(KeyCode::KeyR) {
        bloom_settings.prefilter_settings.threshold_softness -= dt;
    }
    if keycode.pressed(KeyCode::KeyT) {
        bloom_settings.prefilter_settings.threshold_softness += dt;
    }

    if keycode.pressed(KeyCode::KeyD) {
        bloom_settings.intensity -= dt;
    }
    if keycode.pressed(KeyCode::KeyF) {
        bloom_settings.intensity += dt;
    }
    println!(
        "I: {}, K: {}, T: {}",
        bloom_settings.intensity,
        bloom_settings.prefilter_settings.threshold_softness,
        bloom_settings.prefilter_settings.threshold
    );
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
