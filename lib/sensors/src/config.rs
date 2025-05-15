use config_to_rs::config_to_rs;

#[config_to_rs(yaml, "config/sensors.yaml")]
pub struct Sensors;
