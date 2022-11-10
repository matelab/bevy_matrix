use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{matrix_strip::MatrixStripBundle, utils::exponential_event};
pub struct MatrixFieldPlugin;

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

impl Plugin for MatrixFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_strips);
    }
}
