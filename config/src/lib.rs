use core::fmt;
use std::collections::HashMap;
use yaml_rust::YamlLoader;

pub union MeasurementLimits {
    critical: (f64, f64),
    warning: (f64, f64),
}

impl fmt::Debug for MeasurementLimits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            match self {
                MeasurementLimits {
                    critical: (min, max),
                } => {
                    write!(f, "Critical: ({}, {})", min, max)
                }
                MeasurementLimits {
                    warning: (min, max),
                } => {
                    write!(f, "Warning: ({}, {})", min, max)
                }
            }
        }
    }
}

pub union MeasurementFormat {
    float: (),
    int: (),
}

impl MeasurementLimits {
    fn critical(min: f64, max: f64) -> Self {
        MeasurementLimits {
            critical: (min, max),
        }
    }

    fn warning(min: f64, max: f64) -> Self {
        MeasurementLimits {
            warning: (min, max),
        }
    }
}

impl MeasurementFormat {
    fn float() -> Self {
        MeasurementFormat { float: () }
    }

    fn int() -> Self {
        MeasurementFormat { int: () }
    }
}

impl fmt::Debug for MeasurementFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            match self {
                MeasurementFormat { float: () } => write!(f, "float"),
                MeasurementFormat { int: () } => write!(f, "int"),
            }
        }
    }
}

#[derive(Debug)]
pub struct Measurement {
    pub name: String,
    pub unit: String,
    pub format: MeasurementFormat,
    pub limits: MeasurementLimits,
}

#[derive(Debug)]
pub struct Pod {
    pub name: String,
    pub measurements: HashMap<String, Measurement>,
}

#[derive(Debug)]
pub struct PodConfig {
    pub pods: HashMap<String, Pod>,
    pub pod_ids: Vec<String>,
}

impl PodConfig {
    fn new(config_file: &str) -> Self {
        let file = YamlLoader::load_from_str(config_file).expect("Failed to load pods.yaml");
        let doc = &file[0];

        let mut pods = HashMap::new();
        let mut pod_ids = Vec::new();

        for (pod_name, pod) in doc["pods"].as_hash().unwrap() {
            pod_ids.push(pod_name.as_str().unwrap().to_string());

            let mut measurements = HashMap::new();

            for (measurement_name, measurement) in pod["measurements"].as_hash().unwrap() {
                let name = measurement_name.as_str().unwrap().to_string();
                let unit = measurement["unit"].as_str().unwrap().to_string();
                let format = match measurement["format"].as_str().unwrap() {
                    "float" => MeasurementFormat::float(),
                    "int" => MeasurementFormat::int(),
                    _ => panic!("Invalid format"),
                };
                let limits = match measurement["limits"].as_str().unwrap() {
                    "critical" => MeasurementLimits::critical(
                        measurement["limits"]["critical"][0].as_f64().unwrap(),
                        measurement["limits"]["critical"][1].as_f64().unwrap(),
                    ),
                    "warning" => MeasurementLimits::warning(
                        measurement["limits"]["warning"][0].as_f64().unwrap(),
                        measurement["limits"]["warning"][1].as_f64().unwrap(),
                    ),
                    _ => panic!("Invalid limits"),
                };

                measurements.insert(
                    name.clone(),
                    Measurement {
                        name,
                        unit,
                        format,
                        limits,
                    },
                );
            }

            pods.insert(
                pod_name.as_str().unwrap().to_string(),
                Pod {
                    name: pod_name.as_str().unwrap().to_string(),
                    measurements,
                },
            );
        }

        PodConfig { pods, pod_ids }
    }
}

pub fn get_pod_config() -> PodConfig {
    PodConfig::new("../pods.yaml")
}
