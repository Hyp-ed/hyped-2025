use config_to_rs::config_to_rs;

#[config_to_rs(yaml, "../../config/telemetry.yaml")]
pub struct Telemetry;

#[config_to_rs(yaml, "../../config/sensors.yaml")]
pub struct Sensors;

#[config_to_rs(yaml, "../../config/heartbeats.yaml")]
pub struct Heartbeats;
