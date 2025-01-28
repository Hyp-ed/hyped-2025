#![no_std]

use heapless::{LinearMap, String as HeaplessString, Vec};
use serde::{Deserialize, Serialize};

const DEFAULT_STRING_SIZE: usize = 32;
const NUM_PODS: usize = 2;
const NUM_MEASUREMENTS: usize = 32;

type String = HeaplessString<DEFAULT_STRING_SIZE>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MeasurementFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "integer")]
    Int,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Limits {
    warning: Option<MeasurementLimits>,
    critical: MeasurementLimits,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MeasurementLimits {
    min: f64,
    max: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Measurement {
    pub name: String,
    pub unit: String,
    pub format: MeasurementFormat,
    pub limits: Limits,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pod {
    pub name: String,
    pub measurements: LinearMap<String, Measurement, NUM_MEASUREMENTS>,
    #[serde(skip)]
    pub measurement_ids: Vec<String, NUM_MEASUREMENTS>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodConfig {
    pub pods: LinearMap<String, Pod, NUM_PODS>,
    #[serde(skip)]
    pub pod_ids: Vec<String, NUM_PODS>,
}

impl PodConfig {
    /// Create a new PodConfig from a raw YAML string
    pub fn new(raw_config: &str) -> Result<Self, serde_yml::Error> {
        let config = match serde_yml::from_str::<PodConfig>(raw_config) {
            Ok(mut config) => {
                config.pod_ids = config.pods.keys().cloned().collect();
                for pod in config.pods.values_mut() {
                    pod.measurement_ids = pod.measurements.keys().cloned().collect();
                }
                Ok(config)
            }
            Err(e) => Err(e),
        };
        config
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;
    extern crate std;

    fn get_pod_config() -> Result<PodConfig, serde_yml::Error> {
        let raw_config = std::fs::read_to_string("pods.yaml").unwrap();
        PodConfig::new(&raw_config)
    }

    #[test]
    fn test_pods_yaml_does_not_error() {
        let config = get_pod_config();
        std::println!("{:?}", config);
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
        "#;

        let config = PodConfig::new(raw_config).unwrap();
        let expected: Vec<String, NUM_PODS> =
            Vec::from_slice(&[String::from_str("pod_1").unwrap()]).unwrap();
        assert_eq!(config.pod_ids, expected);

        // Check the pod details
        let pod = config
            .pods
            .get(&String::from_str("pod_1").unwrap())
            .unwrap();
        assert_eq!(pod.name, "Pod 1");
        assert_eq!(pod.measurements.len(), 1);
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
        let config = PodConfig::new(raw_config).unwrap();
        assert!(config.pod_ids.len() == 2);
        assert!(config.pod_ids[0] == "pod_1");
        assert!(config.pod_ids[1] == "pod_2");

        let pod1 = config
            .pods
            .get(&String::from_str("pod_1").unwrap())
            .unwrap();
        assert_eq!(pod1.name, "Pod 1");
        assert_eq!(pod1.measurements.len(), 1);

        let pod2 = config
            .pods
            .get(&String::from_str("pod_2").unwrap())
            .unwrap();
        assert_eq!(pod2.name, "Pod 2");
        assert_eq!(pod2.measurements.len(), 1);
    }

    #[test]
    fn test_measurement_ids() {
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
        let pod = config
            .pods
            .get(&String::from_str("pod_1").unwrap())
            .unwrap();

        // Test that the measurement IDs are correctly stored
        let expected_measurement_ids: Vec<String, NUM_MEASUREMENTS> = Vec::from_slice(&[
            String::from_str("keyence").unwrap(),
            String::from_str("accelerometer_1").unwrap(),
        ])
        .unwrap();
        assert_eq!(pod.measurement_ids, expected_measurement_ids);
    }

    #[test]
    fn test_limit_levels() {
        let raw_config = r#"
        pods:
            pod_1:
                name: 'Pod 1'
                measurements:
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
        let pod = config.pods.iter().next().unwrap().1.clone();

        // Test the case where only critical limits are provided
        let accelerometer = pod
            .measurements
            .get(&String::from_str("accelerometer_1").unwrap())
            .unwrap();
        assert_eq!(
            accelerometer.limits,
            Limits {
                warning: None,
                critical: MeasurementLimits {
                    min: -150.0,
                    max: 150.0
                }
            }
        );

        // Test the case where both warning and critical limits are provided
        let temperature = pod
            .measurements
            .get(&String::from_str("temperature").unwrap())
            .unwrap();
        assert_eq!(
            temperature.limits,
            Limits {
                warning: Some(MeasurementLimits {
                    min: 0.0,
                    max: 50.0
                }),
                critical: MeasurementLimits {
                    min: -20.0,
                    max: 80.0
                }
            }
        );
    }

    #[test]
    fn test_missing_critical_limits() {
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
        "#;
        let config = PodConfig::new(raw_config);
        assert!(config.is_err());
    }

    #[test]
    fn test_missing_limits() {
        let raw_config = r#"
        pods:
            pod_1:
                name: 'Pod 1'
                measurements:
                    keyence:
                        name: 'Keyence'
                        unit: 'number of stripes'
                        format: 'integer'
        "#;
        let config = PodConfig::new(raw_config);
        assert!(config.is_err());
    }
}
