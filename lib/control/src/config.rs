use hyped_core::config::LEVITATION_CONFIG;

pub const MAX_VOLTAGE: f32 = LEVITATION_CONFIG.max_voltage_v;
pub const MAX_CURRENT: f32 = LEVITATION_CONFIG.max_current_a;
pub const TARGET_HEIGHT: f32 = LEVITATION_CONFIG.target_height_mm;
pub const SAMPLING_PERIOD: u64 = LEVITATION_CONFIG.sampling_period;

pub const GAIN_HEIGHT: PidGain = PidGain {
    kp: LEVITATION_CONFIG.gain_height.kp,
    ki: LEVITATION_CONFIG.gain_height.ki,
    kd: LEVITATION_CONFIG.gain_height.kd,
    p_reference_gain: LEVITATION_CONFIG.gain_height.p_reference_gain,
    d_reference_gain: LEVITATION_CONFIG.gain_height.d_reference_gain,
    filter_coefficient: LEVITATION_CONFIG.gain_height.filter_coefficient,
};
pub const GAIN_CURRENT: PiGain = PiGain {
    kp: LEVITATION_CONFIG.gain_current.kp,
    ki: LEVITATION_CONFIG.gain_current.ki,
};
pub const GAIN_VOLTAGE: PiGain = PiGain {
    kp: LEVITATION_CONFIG.gain_voltage.kp,
    ki: LEVITATION_CONFIG.gain_voltage.ki,
};