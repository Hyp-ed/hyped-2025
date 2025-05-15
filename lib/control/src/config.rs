use config_to_rs::config_to_rs;

#[config_to_rs(yaml, "config/control.yaml")]
pub struct Control;
