use proc_macro::TokenStream;
use quote::quote;
use saphyr::{LoadableYamlNode, Yaml};

#[proc_macro]
/// Generates a function to calculate the bounds for a given sensor measurement based on the provided YAML file, pod ID, and measurement ID.
pub fn calculate_bounds_function(args: TokenStream) -> TokenStream {
    let (yaml_path, pod_id, measurement_id) = parse_input(args);

    let yaml = get_yaml(yaml_path).expect("Failed to load YAML file");
    let bounds = get_sensor_bounds(yaml, &pod_id, &measurement_id);

    let tokens = if let Some(warning) = &bounds.warning {
        let critical_min = bounds.critical.low;
        let critical_max = bounds.critical.high;
        let warning_min = warning.low;
        let warning_max = warning.high;
        quote! {
            fn calculate_bounds(value: f32) -> SensorValueRange<f32> {
                if value <= #critical_min || value >= #critical_max {
                    SensorValueRange::Critical(value)
                } else if value <= #warning_min || value >= #warning_max {
                    SensorValueRange::Warning(value)
                } else {
                    SensorValueRange::Safe(value)
                }
            }
        }
    } else {
        let critical_min = bounds.critical.low;
        let critical_max = bounds.critical.high;
        quote! {
            fn calculate_bounds(value: f32) -> SensorValueRange<f32> {
                if value <= #critical_min || value >= #critical_max {
                    SensorValueRange::Critical(value)
                } else {
                    SensorValueRange::Safe(value)
                }
            }
        }
    };

    tokens.into()
}

/// Parses the input arguments for the macro.
fn parse_input(args: TokenStream) -> (String, String, String) {
    let args = args
        .to_string()
        .split(",")
        .map(|x| x.to_string().replace(" ", ""))
        .collect::<Vec<String>>();
    if args.len() != 3 {
        panic!("`calculate_bounds_function!` macro requires 3 arguments: yaml_path, pod_id and measurement_id");
    }

    let yaml_path = args[0].clone().replace("\"", "");
    let pod_id = args[1].clone().replace("\"", "");
    let measurement_id = args[2].clone().replace("\"", "");

    (yaml_path, pod_id, measurement_id)
}

/// A struct representing the minimum and maximum limits for a sensor measurement.
struct Limit {
    pub low: f32,
    pub high: f32,
}

impl Limit {
    pub fn new(low: f32, high: f32) -> Self {
        Limit { low, high }
    }
}

/// A struct representing the critical and (optional) warning limits for a sensor measurement.
struct SensorBounds {
    pub critical: Limit,
    pub warning: Option<Limit>,
}

impl SensorBounds {
    pub fn new(critical: Limit, warning: Option<Limit>) -> Self {
        SensorBounds { critical, warning }
    }
}

/// Extract the critical and warning limits from the YAML file for a given pod and measurement.
fn get_sensor_bounds(yaml: Yaml, pod_id: &str, measurement_id: &str) -> SensorBounds {
    // Get the measurement from the pod_id and measurement_id
    let measurement = &yaml["pods"][pod_id]["measurements"][measurement_id];

    let limits = measurement["limits"]
        .as_mapping()
        .expect("Limits not found");

    // Get the critical limit (REQUIRED)
    let critical = limits
        .get(&Yaml::value_from_str("critical"))
        .expect("Critical limit not found");
    let critical_low = critical["low"]
        .as_floating_point()
        .expect("Critical low limit not found") as f32;
    let critical_high = critical["high"]
        .as_floating_point()
        .expect("Critical high limit not found") as f32;
    let critical_limit = Limit::new(critical_low, critical_high);

    // Get the warning limit (OPTIONAL)
    let warning = limits.get(&Yaml::value_from_str("warning"));
    let warning_limit = if let Some(warning) = warning {
        let warning_low = warning["low"]
            .as_floating_point()
            .expect("Warning low limit not found") as f32;
        let warning_high = warning["high"]
            .as_floating_point()
            .expect("Warning high limit not found") as f32;
        Some(Limit::new(warning_low, warning_high))
    } else {
        None
    };

    // Create the SensorBounds struct
    SensorBounds::new(critical_limit, warning_limit)
}

/// Load the YAML file and return the parsed Yaml object.
fn get_yaml(yaml_path: String) -> Option<Yaml<'static>> {
    let path_to_use = match std::env::current_dir().unwrap().ends_with("lib/sensors") {
        true => "../../".to_string() + &yaml_path,
        false => yaml_path,
    };
    match std::fs::read_to_string(path_to_use.clone()) {
        Ok(file) => Some(Yaml::load_from_str(&file).unwrap()[0].clone()),
        Err(_) => panic!("Failed to open file: {path_to_use}"),
    }
}
