use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, Extent3d, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin},
    window::WindowResized,
};
use rand::distributions::uniform;

#[derive(ShaderType, Clone)]
pub struct ShaderTime {
    pub secs_since_startup: f32,
    pub dt: f32,
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "0958b3e0-5eb8-408e-a739-bdbcc823d30f"]
pub struct PostProcessingMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
    #[uniform(2)]
    pub time: ShaderTime,
}

#[derive(Component)]
pub struct PostQuad;
#[derive(Resource)]
pub struct PostImage(pub Handle<Image>);

pub struct PostPlugin;

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/post.wgsl".into()
    }
}

fn resize(
    ev: Res<Events<WindowResized>>,
    mut query: Query<&mut Transform, With<PostQuad>>,
    mut images: ResMut<Assets<Image>>,
    post_image: Res<PostImage>,
    mut image_events: EventWriter<AssetEvent<Image>>,
    time: Res<Time>,
) {
    let mut reader = ev.get_reader();
    for e in reader.iter(&ev) {
        for mut t in &mut query {
            *t = t.with_scale(Vec3::new(e.width as f32, e.height as f32, 1.0));
        }
        /*if let Some(img) = images.get_mut(&post_image.0) {
            img.resize(Extent3d {
                width: e.width as u32,
                height: e.height as u32,
                depth_or_array_layers: 1,
            });
            image_events.send(AssetEvent::Modified {
                handle: post_image.0.clone(),
            });
        }*/
    }
}

fn update_shader_time(mut post_materials: ResMut<Assets<PostProcessingMaterial>>, time: Res<Time>) {
    for (_, mut mat) in post_materials.iter_mut() {
        mat.time.secs_since_startup = time.elapsed_seconds();
        mat.time.dt = time.delta_seconds();
    }
}

impl Plugin for PostPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
            .add_system(resize)
            .add_system(update_shader_time);
    }
}
