const CONFIG_PATH: &str = "../../config/pods.yaml";

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let config_path = current_dir.join(CONFIG_PATH);
    let config_path = config_path.to_str().unwrap();
    println!("cargo:rerun-if-changed=.");
    println!("cargo:rerun-if-changed={}", config_path);

    println!("cargo:rustc-env=CONFIG_PATH={}", config_path);
}
