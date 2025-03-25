use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use saphyr::Yaml;

#[proc_macro]
pub fn gen_measurement_ids(args: TokenStream) -> TokenStream {
    let args = args
        .to_string()
        .split(",")
        .map(|x| x.to_string().replace(" ", ""))
        .collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("`gen_measurement_ids!` macro requires 2 arguments: yaml_path and pod_id");
    }

    let yaml_path = args[0].clone().replace("\"", "");
    let pod_id = args[1].clone().replace("\"", "");

    let measurement_ids = get_measurement_ids(yaml_path, pod_id);

    // Actual enum
    let mut enum_str =
        String::from("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, defmt::Format)]\n");
    enum_str.push_str("pub enum MeasurementId {\n");
    for id in measurement_ids.clone() {
        enum_str.push_str(&format!("    {},\n", id));
    }
    enum_str.push_str("}\n");

    // to_string and from_string
    enum_str.push_str("\nimpl MeasurementId {\n");
    enum_str.push_str("    pub fn to_string(&self) -> String<50> {\n");
    enum_str.push_str("        match self {\n");
    for id in measurement_ids.clone() {
        enum_str.push_str(&format!(
            "            MeasurementId::{} => String::<50>::from_str(\"{}\").unwrap(),\n",
            id,
            id.to_case(Case::Snake)
        ));
    }
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push('\n');
    enum_str.push_str("    pub fn from_string(enum_str: &str) -> MeasurementId {\n");
    enum_str.push_str("        match enum_str {\n");
    for id in measurement_ids.clone() {
        enum_str.push_str(&format!(
            "            \"{}\" => MeasurementId::{},\n",
            id.to_case(Case::Snake),
            id
        ));
    }
    enum_str.push_str("            _ => panic!(\"Failed to parse enum\"),\n");
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    // Into<u16>
    enum_str.push_str("\nimpl Into<u16> for MeasurementId {\n");
    enum_str.push_str("    fn into(self) -> u16 {\n");
    enum_str.push_str("        match self {\n");
    for (i, id) in measurement_ids.clone().into_iter().enumerate() {
        enum_str.push_str(&format!("            MeasurementId::{} => {},\n", id, i));
    }
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    // From<u16>
    enum_str.push_str("\nimpl From<u16> for MeasurementId {\n");
    enum_str.push_str("    fn from(enum_str: u16) -> MeasurementId {\n");
    enum_str.push_str("        match enum_str {\n");
    for (i, id) in measurement_ids.clone().into_iter().enumerate() {
        enum_str.push_str(&format!("            {} => MeasurementId::{},\n", i, id));
    }
    enum_str.push_str("            _ => panic!(\"Failed to parse enum\"),\n");
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    enum_str.parse().expect("Failed to parse enum")
}

fn get_measurement_ids(yaml_path: String, pod_id: String) -> Vec<String> {
    let yaml =
        get_yaml(yaml_path.clone()).unwrap_or_else(|| panic!("Failed to load yaml: {}", yaml_path));
    let mut measurement_ids = Vec::new();
    let measurements = yaml["pods"][pod_id.clone().as_str()]["measurements"].clone();
    for key in measurements.as_hash().unwrap().keys() {
        measurement_ids.push(key.as_str().unwrap().to_string().to_case(Case::Pascal));
    }
    measurement_ids
}

fn get_yaml(yaml_path: String) -> Option<Yaml> {
    match std::fs::read_to_string(yaml_path.clone()) {
        Ok(file) => Some(Yaml::load_from_str(&file).unwrap()[0].clone()),
        Err(_) => panic!("Failed to read file: {}", yaml_path),
    }
}
