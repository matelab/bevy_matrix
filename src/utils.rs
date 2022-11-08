use rand::{thread_rng, Rng};
use std::f32::consts::E;

pub fn exponential_event(t_average: f32, dt: f32) -> bool {
    let mut rng = thread_rng();
    let probability = 1. - E.powf(-dt / t_average);
    rng.gen::<f32>() < probability
}
