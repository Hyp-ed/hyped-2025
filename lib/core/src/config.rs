use config_to_rs::config_to_rs;

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
#[config_to_rs(yaml)]
pub struct Config;

mod test {
    #[test]
    fn test_config() {
        let pod_name = super::CONFIG.pods.poddington.label;
        assert_eq!(pod_name, "Poddington");
    }
}
