use hyped_core::config::LOCALISATION_CONFIG;

pub const NUM_AXIS: usize = LOCALISATION_CONFIG.num_axis as usize;
pub const NUM_ACCELEROMETERS: usize = LOCALISATION_CONFIG.accelerometers.num_sensors as usize;
pub const NUM_OPTICAL_FLOW_SENSORS: usize = LOCALISATION_CONFIG.optical_flow.num_sensors as usize;
pub const NUM_KEYENCE_SENSORS: usize = LOCALISATION_CONFIG.keyence.num_sensors as usize;
pub const NUM_ALLOWED_ACCELEROMETER_OUTLIERS: usize =
    LOCALISATION_CONFIG.accelerometers.num_allowed_outliers as usize;
