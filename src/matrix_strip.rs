use std::time::Duration;

use super::matrix_letter::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_tweening::*;

#[derive(Component, Inspectable, Default)]
pub struct MatrixStrip {
    num_spawned: u32,
    max_length: u32,
    log_scale: f32,
    lifetime: f32,
    last_spawn: Option<Entity>,
}

#[derive(Component, Default)]
pub struct SpawnTimer(Timer);

#[derive(Component, Inspectable, Default)]
pub struct Spawning;

#[derive(Bundle)]
pub struct MatrixStripBundle {
    #[bundle]
    transform: SpatialBundle,
    strip: MatrixStrip,
    spawning: Spawning,
    timer: SpawnTimer,
}
pub struct MatrixStripPlugin;

impl MatrixStripBundle {
    pub fn new(pos: Vec3) -> Self {
        let log_scale = (10.0_f32).powf(pos.z / 10.0);
        Self {
            transform: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, pos.z))
                    .with_scale(Vec3::new(log_scale, log_scale, 1.0)),
                ..Default::default()
            },
            strip: MatrixStrip {
                num_spawned: 0,
                max_length: 40,
                log_scale,
                lifetime: 0.0,
                last_spawn: None,
            },
            spawning: Spawning,
            timer: SpawnTimer(Timer::new(Duration::from_secs_f32(0.1), true)),
        }
    }

    pub fn with_max_length(mut self, max_length: u32) -> Self {
        self.strip.max_length = max_length;
        self
    }

    pub fn with_lifetime(mut self, lifetime: f32) -> Self {
        self.strip.lifetime = lifetime / self.strip.log_scale;
        self
    }

    pub fn with_spawnrate(mut self, spawnrate: f32) -> Self {
        self.timer = SpawnTimer(Timer::new(Duration::from_secs_f32(1.0 / spawnrate), true));
        self
    }
}

fn spawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut MatrixStrip, &mut SpawnTimer), With<Spawning>>,
    time: Res<Time>,
) {
    for (entity, mut strip, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(last) = strip.last_spawn {
                let tween = Tween::new(
                    EaseFunction::QuadraticOut,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.2),
                    MatrixLetterLens {
                        start: Color::WHITE,
                        end: Color::LIME_GREEN,
                    },
                );
                commands.entity(last).insert(Animator::new(tween));
            }
            let pos = Vec3::new(0.0, -(strip.num_spawned as f32), 0.0);
            let letter = commands
                .spawn()
                .insert_bundle(
                    MatrixLetterBundle::new(pos)
                        .with_brightness(strip.log_scale)
                        .with_lifetime(strip.lifetime),
                )
                .id();
            strip.num_spawned += 1;
            commands.entity(entity).add_child(letter);
            strip.last_spawn = Some(letter);
        }
    }
}

fn stop_spawn(mut commands: Commands, query: Query<(Entity, &MatrixStrip), With<Spawning>>) {
    for (entity, strip) in &query {
        if strip.num_spawned >= strip.max_length {
            commands.entity(entity).remove::<Spawning>();
        }
    }
}

fn strip_clean(
    mut commands: Commands,
    query: Query<(Entity, &Children), (Without<Spawning>, With<MatrixStrip>)>,
) {
    for (entity, children) in &query {
        if children.len() == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_strip(mut query: Query<(&MatrixStrip, &mut Transform)>, time: Res<Time>) {
    for (strip, mut transform) in &mut query {
        transform.translation.x -= time.delta_seconds() * strip.log_scale;
        transform.translation.y += 0.3 * time.delta_seconds() * strip.log_scale;
    }
}

impl Plugin for MatrixStripPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn)
            .add_system(stop_spawn)
            .add_system(move_strip)
            .add_system(strip_clean);
    }
}
