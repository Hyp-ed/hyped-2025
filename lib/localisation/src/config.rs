use config_to_rs::config_to_rs;

#[config_to_rs(yaml, "config/localisation.yaml")]
pub struct Localisation;

pub const NUM_AXIS: usize = LOCALISATION.num_axis as usize;
pub const NUM_ACCELEROMETERS: usize = LOCALISATION.accelerometers.num_sensors as usize;
pub const NUM_OPTICAL_FLOW_SENSORS: usize = LOCALISATION.optical_flow.num_sensors as usize;
pub const NUM_KEYENCE_SENSORS: usize = LOCALISATION.keyence.num_sensors as usize;
pub const NUM_ALLOWED_ACCELEROMETER_OUTLIERS: usize =
    LOCALISATION.accelerometers.num_allowed_outliers as usize;
