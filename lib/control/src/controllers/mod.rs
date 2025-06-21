pub mod pi_controller;
pub mod pid_controller;

pub use pi_controller::{PiController, PiGain};
pub use pid_controller::{PidController, PidGain};

pub use crate::config;

pub trait Controller {
    fn update(&mut self, set_point: f32, actual: f32, dt: u64) -> f32;
}
