#![no_std]

use heapless::{LinearMap, String, Vec};
use serde::{Deserialize, Serialize};

const DEFAULT_STRING_SIZE: usize = 32;
const NUM_PODS: usize = 2;
const NUM_MEASUREMENTS: usize = 32;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MeasurementFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "integer")]
    Int,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LimitLevel {
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeasurementLimits {
    min: f64,
    max: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Measurement {
    pub name: String<DEFAULT_STRING_SIZE>,
    pub unit: String<DEFAULT_STRING_SIZE>,
    pub format: MeasurementFormat,
    #[serde(default)]
    pub limits: LinearMap<LimitLevel, MeasurementLimits, 2>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pod {
    pub name: String<DEFAULT_STRING_SIZE>,
    pub measurements: LinearMap<String<DEFAULT_STRING_SIZE>, Measurement, NUM_MEASUREMENTS>,
    #[serde(skip)]
    pub measurement_ids: Vec<String<DEFAULT_STRING_SIZE>, NUM_MEASUREMENTS>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodConfig {
    pub pods: LinearMap<String<DEFAULT_STRING_SIZE>, Pod, NUM_PODS>,
    #[serde(skip)]
    pub pod_ids: Vec<String<DEFAULT_STRING_SIZE>, NUM_PODS>,
}

impl PodConfig {
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
        let expected: Vec<String<DEFAULT_STRING_SIZE>, NUM_PODS> =
            Vec::from_slice(&[String::from_str("pod_1").unwrap()]).unwrap();
        assert_eq!(config.pod_ids, expected);

        let pod = config
            .pods
            .get(&String::from_str("pod_1").unwrap())
            .unwrap();
        assert_eq!(pod.name, "Pod 1");
        assert_eq!(pod.measurements.len(), 2);

        let keyence = pod
            .measurements
            .get(&String::from_str("keyence").unwrap())
            .unwrap();
        assert_eq!(keyence.name, "Keyence");
        assert_eq!(keyence.unit, "number of stripes");
        assert_eq!(keyence.format, MeasurementFormat::Int);
        assert_eq!(keyence.limits.len(), 1);

        let keyence_limits = keyence.limits.get(&LimitLevel::Critical).unwrap();
        assert_eq!(keyence_limits.min, 0.0);
        assert_eq!(keyence_limits.max, 16.0);
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
        let pod = config.pods.iter().next().unwrap().1.clone();
        let expected: Vec<String<DEFAULT_STRING_SIZE>, NUM_MEASUREMENTS> = Vec::from_slice(&[
            String::from_str("keyence").unwrap(),
            String::from_str("accelerometer_1").unwrap(),
        ])
        .unwrap();
        assert_eq!(pod.measurement_ids, expected);
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
        assert!(config.pod_ids[0] == "pod_1" || config.pod_ids[1] == "pod_1");
        assert!(config.pod_ids[0] == "pod_2" || config.pod_ids[1] == "pod_2");

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
        let pod = config.pods.iter().next().unwrap().1.clone();

        let keyence = pod.measurements.iter().next().unwrap().1.clone();
        assert_eq!(keyence.limits.len(), 1);
        assert_eq!(keyence.limits.get(&LimitLevel::Warning).unwrap().min, 0.0);
        assert_eq!(keyence.limits.get(&LimitLevel::Warning).unwrap().max, 16.0);

        let accelerometer = pod
            .measurements
            .get(&String::from_str("accelerometer_1").unwrap())
            .unwrap();
        assert_eq!(accelerometer.limits.len(), 1);
        assert_eq!(
            accelerometer.limits.get(&LimitLevel::Critical).unwrap().min,
            -150.0
        );
        assert_eq!(
            accelerometer.limits.get(&LimitLevel::Critical).unwrap().max,
            150.0
        );

        let temperature = pod
            .measurements
            .get(&String::from_str("temperature").unwrap())
            .unwrap();
        assert_eq!(temperature.limits.len(), 2);
        assert_eq!(
            temperature.limits.get(&LimitLevel::Warning).unwrap().min,
            0.0
        );
        assert_eq!(
            temperature.limits.get(&LimitLevel::Warning).unwrap().max,
            50.0
        );
        assert_eq!(
            temperature.limits.get(&LimitLevel::Critical).unwrap().min,
            -20.0
        );
        assert_eq!(
            temperature.limits.get(&LimitLevel::Critical).unwrap().max,
            80.0
        );
    }

    #[test]
    fn test_no_limits() {
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
        let config = PodConfig::new(raw_config).unwrap();
        let pod = config.pods.iter().next().unwrap().1.clone();
        let keyence = pod
            .measurements
            .get(&String::from_str("keyence").unwrap())
            .unwrap();
        assert_eq!(keyence.limits.len(), 0);
    }
}
