use config_to_rs::config_to_rs;
use core::str::FromStr;
use heapless::String;
use hyped_measurement_ids::gen_measurement_ids;

/// Configuration for the pods
/// The configuration is loaded from the `config/pods.yaml` file, and can be read using standard
/// Rust syntax.
///
/// E.g. to get the pod name,
/// ```rust
/// let pod_name = CONFIG.pods.poddington.label;
///
/// assert_eq!(pod_name, "Poddington");
/// ````
#[config_to_rs(yaml, "../../../config/pods.yaml")]
pub struct PodsConfig;

#[config_to_rs(yaml, "../../../config/sensors.yaml")]
pub struct SensorsConfig;

#[config_to_rs(yaml, "../../../config/telemetry.yaml")]
pub struct TelemetryConfig;

#[config_to_rs(yaml, "../../../config/heartbeats.yaml")]
pub struct HeartbeatConfig;

#[config_to_rs(yaml, "../../../config/localisation.yaml")]
pub struct LocalisationConfig;

#[config_to_rs(yaml, "../../../config/control.yaml")]
pub struct ControlConfig;

#[config_to_rs(yaml, "../../../config/levitation.yaml")]
pub struct LevitationConfig;

// TODOLater: this should be in a config
pub static POD_NAME: &str = "poddington";

gen_measurement_ids!("config/pods.yaml", "poddington");

mod test {
    #[test]
    fn test_config() {
        let pod_name = super::PODS_CONFIG.pods.poddington.label;
        assert_eq!(pod_name, "Poddington");
    }
}
