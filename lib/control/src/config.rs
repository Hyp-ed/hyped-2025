use crate::lev_controllers::{PiGain, PidGain};
use hyped_core::config::LEVITATION_CONFIG;

pub const MAX_VOLTAGE: f32 = LEVITATION_CONFIG.max_voltage_v as f32;
pub const MAX_CURRENT: f32 = LEVITATION_CONFIG.max_current_a as f32;
pub const TARGET_HEIGHT: f32 = LEVITATION_CONFIG.target_height_mm as f32;
pub const SAMPLING_PERIOD: u64 = LEVITATION_CONFIG.sampling_period as u64;

pub const GAIN_HEIGHT: PidGain = PidGain {
    kp: LEVITATION_CONFIG.gain_height.kp as f32,
    ki: LEVITATION_CONFIG.gain_height.ki as f32,
    kd: LEVITATION_CONFIG.gain_height.kd as f32,
    p_reference_gain: LEVITATION_CONFIG.gain_height.p_reference_gain as f32,
    d_reference_gain: LEVITATION_CONFIG.gain_height.d_reference_gain as f32,
    filter_coefficient: LEVITATION_CONFIG.gain_height.filter_coefficient as f32,
};
pub const GAIN_CURRENT: PiGain = PiGain {
    kp: LEVITATION_CONFIG.gain_current.kp as f32,
    ki: LEVITATION_CONFIG.gain_current.ki as f32,
};
pub const GAIN_VOLTAGE: PiGain = PiGain {
    kp: LEVITATION_CONFIG.gain_voltage.kp as f32,
    ki: LEVITATION_CONFIG.gain_voltage.ki as f32,
};
