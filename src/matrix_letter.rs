use std::time::Duration;

use super::utils::*;
use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, *};
use rand::{thread_rng, Rng};

#[derive(Component, Default)]
pub struct MatrixLetter {
    mul_color: Color,
    color: Color,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct MatrixLetterSpawnRequest {
    pos: Vec3,
    mul_color: Color,
    color: Color,
    lifetime: f32,
}

pub struct MatrixLetterPlugin;
#[derive(Bundle)]
pub struct MatrixLetterBundle {
    request: MatrixLetterSpawnRequest,
}

#[derive(Resource)]
struct MatrixLetterData {
    font: Handle<Font>,
    font_size: f32,
}

#[derive(Component)]
pub struct LetterDeath(Timer);

impl Default for LetterDeath {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(3.0), TimerMode::Once))
    }
}

pub struct MatrixLetterLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<MatrixLetter> for MatrixLetterLens {
    fn lerp(&mut self, target: &mut dyn Targetable<MatrixLetter>, ratio: f32) {
        let (s_start, s_end) = (Srgba::from(self.start), Srgba::from(self.end));
        let col = s_end * ratio + s_start * (1.0 - ratio);
        target.color = Color::from(col);
    }
}

impl MatrixLetterBundle {
    pub fn new(pos: Vec3) -> Self {
        Self {
            request: MatrixLetterSpawnRequest {
                pos,
                mul_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                color: Color::WHITE,
                lifetime: 10.0,
            },
        }
    }

    pub fn with_brightness(mut self, brightness: f32) -> Self {
        let smc = Srgba::from(self.request.mul_color);
        self.request.mul_color = Color::from(
            smc.with_red(brightness)
                .with_green(brightness)
                .with_blue(brightness),
        );
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.request.color = color;
        self
    }

    pub fn with_lifetime(mut self, lifetime: f32) -> Self {
        self.request.lifetime = lifetime;
        self
    }
}

fn make_matrix_character() -> String {
    let mut rng = thread_rng();
    let r: u8 = rng.gen_range(0..=58);
    let c = match r {
        0..=9 => '0' as u8 + r,
        10..=33 => 'a' as u8 + r - 10,
        _ => 'A' as u8 + r - 34,
    } as char;
    return c.to_string();
}

fn change_text(mut query: Query<(&mut Text), With<MatrixLetter>>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for mut t in query.iter_mut() {
        if exponential_event(2.0, dt) {
            t.sections[0].value = make_matrix_character();
        }
    }
}

fn spawn_request_handler(
    mut commands: Commands,
    query: Query<(Entity, &MatrixLetterSpawnRequest)>,
    data: Res<MatrixLetterData>,
) {
    for (entity, request) in &query {
        let text_style = TextStyle {
            font: data.font.clone(),
            font_size: data.font_size,
            color: Color::from(Srgba::from_vec4(
                Srgba::new(1.0, 1.0, 1.0, 0.0).to_vec4() * Srgba::from(request.mul_color).to_vec4(),
            )),
        };
        let fade_in = commands
            .entity(entity)
            .insert(MatrixLetter {
                color: request.color,
                mul_color: request.mul_color,
            })
            .insert(Text2dBundle {
                transform: Transform::from_scale(Vec3::splat(1.0 / data.font_size))
                    .with_translation(request.pos),
                text: Text::from_section(make_matrix_character(), text_style.clone())
                    .with_justify(JustifyText::Center),
                ..Default::default()
            })
            .insert(LetterDeath(Timer::new(
                Duration::from_secs_f32(request.lifetime),
                TimerMode::Once,
            )))
            .remove::<MatrixLetterSpawnRequest>();
    }
}

fn letter_death(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        Option<&Parent>,
        &mut MatrixLetter,
        &mut LetterDeath,
        &Transform,
    )>,
    time: Res<Time>,
) {
    for (entity, parent, mut letter, mut letter_death, transform) in &mut query {
        letter_death.0.tick(time.delta());
        if letter_death.0.just_finished() {
            let tween = Tween::new(
                EaseFunction::QuadraticOut,
                //TweeningType::Once,
                Duration::from_secs_f32(0.5),
                TransformScaleLens {
                    start: transform.scale.clone(),
                    end: Vec3::new(0.0, 0.0, 1.0),
                },
            )
            .with_repeat_count(RepeatCount::Finite(1));
            commands.entity(entity).insert(Animator::new(tween));
        }
    }
}

fn letter_despawn(
    mut commands: Commands,
    query: Query<(Entity, Option<&Parent>, &Animator<Transform>)>,
) {
    for (entity, parent, animator) in &query {
        if animator.tweenable().progress() > 0.999 {
            if let Some(parent) = parent {
                commands.entity(parent.get()).remove_children(&[entity]);
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_color(mut query: Query<(&mut Text, &MatrixLetter), Changed<MatrixLetter>>) {
    for (mut text, letter) in &mut query {
        text.sections[0].style.color = Color::from(Srgba::from_vec4(
            Srgba::from(letter.color).to_vec4() * Srgba::from(letter.mul_color).to_vec4(),
        ))
    }
}

impl Plugin for MatrixLetterPlugin {
    fn build(&self, app: &mut App) {
        let asset_server = app.world().get_resource::<AssetServer>().unwrap();
        let font = asset_server.load("fonts/matrix.ttf");

        app.insert_resource(MatrixLetterData {
            font,
            font_size: 64.0,
        })
        .add_systems(Update, change_text)
        .add_systems(Update, spawn_request_handler)
        .add_systems(Update, letter_death)
        .add_systems(Update, update_color)
        .add_systems(Update, letter_despawn)
        .add_systems(Update, component_animator_system::<MatrixLetter>);
    }
}
