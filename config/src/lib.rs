use serde::{Deserialize, Serialize};
use serde_yml;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MeasurementFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "integer")]
    Int,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LimitLevel {
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementLimits {
    min: f64,
    max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    pub name: String,
    pub unit: String,
    pub format: MeasurementFormat,
    pub limits: HashMap<LimitLevel, MeasurementLimits>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pod {
    pub name: String,
    pub measurements: HashMap<String, Measurement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodConfig {
    pub pods: HashMap<String, Pod>,
    #[serde(skip)]
    pub pod_ids: Vec<String>,
}

impl PodConfig {
    fn new(raw_config: &str) -> Result<Self, serde_yml::Error> {
        let config = match serde_yml::from_str::<PodConfig>(&raw_config) {
            Ok(mut config) => {
                config.pod_ids = config.pods.keys().cloned().collect();
                Ok(config)
            }
            Err(e) => {
                eprintln!("Error parsing config: {}", e);
                Err(e)
            }
        };
        config
    }
}

pub fn get_pod_config() -> Result<PodConfig, serde_yml::Error> {
    let raw_config = std::fs::read_to_string("pods.yaml").unwrap();
    PodConfig::new(&raw_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pods_yaml_does_not_error() {
        let config = get_pod_config();
        assert!(config.is_ok());
    }

    #[test]
    fn test_one_pod() {
        let raw_config = r#"
        pods:
            pod_1:
                name: 'Pod 1'
                measurements:
                    keyence:
                        name: 'Keyence'
                        unit: 'number of stripes'
                        format: 'integer'
                        limits:
                            critical:
                                min: 0
                                max: 16
                    accelerometer_1:
                        name: 'Accelerometer 1'
                        unit: 'm/s^2'
                        format: 'float'
                        limits:
                            critical:
                                min: -150
                                max: 150
        "#;
        let config = PodConfig::new(raw_config).unwrap();
        assert_eq!(config.pod_ids, vec!["pod_1"]);
        let pod = config.pods.get("pod_1").unwrap();
        assert_eq!(pod.name, "Pod 1");
        assert_eq!(pod.measurements.len(), 2);
        let keyence = pod.measurements.get("keyence").unwrap();
        assert_eq!(keyence.name, "Keyence");
        assert_eq!(keyence.unit, "number of stripes");
        assert_eq!(keyence.format, MeasurementFormat::Int);
        assert_eq!(keyence.limits.len(), 1);
        let keyence_limits = keyence.limits.get(&LimitLevel::Critical).unwrap();
        assert_eq!(keyence_limits.min, 0.0);
        assert_eq!(keyence_limits.max, 16.0);
    }

    #[test]
    fn test_multiple_pods() {
        let raw_config = r#"
        pods:
            pod_1:
                name: 'Pod 1'
                measurements:
                    keyence:
                        name: 'Keyence'
                        unit: 'number of stripes'
                        format: 'integer'
                        limits:
                            critical:
                                min: 0
                                max: 16
            pod_2:
                name: 'Pod 2'
                measurements:
                    accelerometer_1:
                        name: 'Accelerometer 1'
                        unit: 'm/s^2'
                        format: 'float'
                        limits:
                            critical:
                                min: -150
                                max: 150
        "#;
        let mut config = PodConfig::new(raw_config).unwrap();
        assert!(config.pod_ids.len() == 2);
        assert!(config.pod_ids.sort() == vec!["pod_1", "pod_2"].sort());
        let pod1 = config.pods.get("pod_1").unwrap();
        let pod2 = config.pods.get("pod_2").unwrap();
        assert_eq!(pod1.name, "Pod 1");
        assert_eq!(pod1.measurements.len(), 1);
        assert_eq!(pod2.name, "Pod 2");
        assert_eq!(pod2.measurements.len(), 1);
    }

    #[test]
    fn test_limit_levels() {
        let raw_config = r#"
        pods:
            pod_1:
                name: 'Pod 1'
                measurements:
                    keyence:
                        name: 'Keyence'
                        unit: 'number of stripes'
                        format: 'integer'
                        limits:
                            warning:
                                min: 0
                                max: 16
                    accelerometer_1:
                        name: 'Accelerometer 1'
                        unit: 'm/s^2'
                        format: 'float'
                        limits:
                            critical:
                                min: -150
                                max: 150
                    temperature:
                        name: 'Temperature'
                        unit: 'C'
                        format: 'float'
                        limits:
                            warning:
                                min: 0
                                max: 50
                            critical:
                                min: -20
                                max: 80
        "#;
        let config = PodConfig::new(raw_config).unwrap();
        let pod = config.pods.get("pod_1").unwrap();
        let keyence = pod.measurements.get("keyence").unwrap();
        assert_eq!(keyence.limits.len(), 1);
        assert_eq!(keyence.limits.get(&LimitLevel::Warning).unwrap().min, 0.0);
        let accelerometer = pod.measurements.get("accelerometer_1").unwrap();
        assert_eq!(accelerometer.limits.len(), 1);
        assert_eq!(
            accelerometer.limits.get(&LimitLevel::Critical).unwrap().min,
            -150.0
        );
        let temperature = pod.measurements.get("temperature").unwrap();
        assert_eq!(temperature.limits.len(), 2);
        assert_eq!(
            temperature.limits.get(&LimitLevel::Warning).unwrap().max,
            50.0
        );
        assert_eq!(
            temperature.limits.get(&LimitLevel::Critical).unwrap().max,
            80.0
        );
    }
}
