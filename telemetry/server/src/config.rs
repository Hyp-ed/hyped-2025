use influxdb2::Client;
use rumqttc::MqttOptions;
use yaml_rust::YamlLoader;

pub fn get_influxdb_client() -> Client {
    let config = YamlLoader::load_from_str(include_str!("../influx.yaml")).unwrap();
    let influxdb_config = &config[0]["influxdb_config"];
    let host = influxdb_config["host"].as_str().unwrap();
    let org = influxdb_config["org"].as_str().unwrap();
    let token = influxdb_config["token"].as_str().unwrap();
    Client::new(host.to_string(), org.to_string(), token.to_string())
}

/**
 * Get the MQTT configuration from the yaml file
 */
pub fn get_mqtt_config() -> MqttOptions {
    let config = YamlLoader::load_from_str(include_str!("../mqtt.yaml")).unwrap();
    let mqtt_config = &config[0]["mqtt_options"];
    let client_id = mqtt_config["client_id"].as_str().unwrap();
    let host = mqtt_config["host"].as_str().unwrap();
    let port = mqtt_config["port"].as_i64().unwrap() as u16;
    MqttOptions::new(client_id, host, port)
}
