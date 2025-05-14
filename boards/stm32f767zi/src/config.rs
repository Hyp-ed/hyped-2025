use config_to_rs::config_to_rs;

#[config_to_rs(yaml, "../../config/telemetry.yaml")]
pub struct Telemetry;
