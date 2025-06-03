use crate::controllers::{pi_controller::PiGain, pid_controller::PidGain};
use hyped_core::config::LEVITATION_CONFIG;

pub const MAX_VOLTAGE: f32 = LEVITATION_CONFIG.max_voltage_v as f32;
pub const MAX_CURRENT: f32 = LEVITATION_CONFIG.max_current_a as f32;
pub const TARGET_HEIGHT: f32 = LEVITATION_CONFIG.target_height_mm as f32;
pub const SAMPLING_PERIOD: u64 = LEVITATION_CONFIG.sampling_period as u64;

pub const HEIGHT_PID_CONSTANTS: PidGain = PidGain {
    kp: LEVITATION_CONFIG.height_pid_constants.kp as f32,
    ki: LEVITATION_CONFIG.height_pid_constants.ki as f32,
    kd: LEVITATION_CONFIG.height_pid_constants.kd as f32,
    p_reference_gain: LEVITATION_CONFIG.height_pid_constants.p_reference_gain as f32,
    d_reference_gain: LEVITATION_CONFIG.height_pid_constants.d_reference_gain as f32,
    filter_coefficient: LEVITATION_CONFIG.height_pid_constants.filter_coefficient as f32,
};
pub const CURRENT_PI_CONSTANTS: PiGain = PiGain {
    kp: LEVITATION_CONFIG.current_pi_constants.kp as f32,
    ki: LEVITATION_CONFIG.current_pi_constants.ki as f32,
};
pub const VOLTAGE_PI_CONSTANTS: PiGain = PiGain {
    kp: LEVITATION_CONFIG.voltage_pi_constants.kp as f32,
    ki: LEVITATION_CONFIG.voltage_pi_constants.ki as f32,
};
