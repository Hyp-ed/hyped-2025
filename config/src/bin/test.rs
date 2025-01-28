#![no_std]

const CONFIG: &str = include_str!("../../pods.yaml");

fn main() {
    // Parse the configuration file
    let pods = hyped_config::PodConfig::new(CONFIG);
    assert!(pods.is_ok());
}
