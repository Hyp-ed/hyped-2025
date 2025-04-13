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

    // impl Display for MeasurementId
    enum_str.push_str("\nimpl core::fmt::Display for MeasurementId {\n");
    enum_str
        .push_str("    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {\n");
    enum_str.push_str("        match self {\n");
    for id in measurement_ids.clone() {
        enum_str.push_str(&format!(
            "            MeasurementId::{} => write!(f, \"{}\"),\n",
            id,
            id.to_case(Case::Snake),
        ));
    }
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    // From<MeasurementId> for String<50>
    enum_str.push_str("\nimpl From<MeasurementId> for String<50> {\n");
    enum_str.push_str("    fn from(measurement_id: MeasurementId) -> String<50> {\n");
    enum_str.push_str("        match measurement_id {\n");
    for id in measurement_ids.clone() {
        enum_str.push_str(&format!(
            "            MeasurementId::{} => String::<50>::from_str(\"{}\").unwrap(),\n",
            id,
            id.to_case(Case::Snake),
        ));
    }
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    // Into<MeasurementId> for &str
    enum_str.push_str("\nimpl Into<MeasurementId> for &str {\n");
    enum_str.push_str("    fn into(self) -> MeasurementId {\n");
    enum_str.push_str("        match self {\n");
    for id in measurement_ids.clone() {
        enum_str.push_str(&format!(
            "            \"{}\" => MeasurementId::{},\n",
            id.to_case(Case::Snake),
            id,
        ));
    }
    enum_str.push_str("            _ => panic!(\"Failed to parse enum Into<&str>, {}\", self),\n");
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    // From<MeasurementId> for &str
    enum_str.push_str("\nimpl From<MeasurementId> for &str {\n");
    enum_str.push_str("    fn from(measurement_id: MeasurementId) -> &'static str {\n");
    enum_str.push_str("        match measurement_id {\n");
    for id in measurement_ids.clone() {
        enum_str.push_str(&format!(
            "            MeasurementId::{} => \"{}\",\n",
            id,
            id.to_case(Case::Snake),
        ));
    }
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    // From<String<50>> for MeasurementId
    enum_str.push_str("\nimpl From<String<50>> for MeasurementId {\n");
    enum_str.push_str("    fn from(measurement_id: String<50>) -> MeasurementId {\n");
    enum_str.push_str("        match measurement_id.as_str() {\n");
    for id in measurement_ids.clone() {
        enum_str.push_str(&format!(
            "            \"{}\" => MeasurementId::{},\n",
            id.to_case(Case::Snake),
            id,
        ));
    }
    enum_str.push_str("            _ => panic!(\"Failed to parse enum From<String<50>>\"),\n");
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    // From<MeasurementId> for u16
    enum_str.push_str("\nimpl From<MeasurementId> for u16 {\n");
    enum_str.push_str("    fn from(measurement_id: MeasurementId) -> u16 {\n");
    enum_str.push_str("        match measurement_id {\n");
    for (i, id) in measurement_ids.clone().into_iter().enumerate() {
        enum_str.push_str(&format!("            MeasurementId::{} => {},\n", id, i));
    }
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    // From<u16> for MeasurementId
    enum_str.push_str("\nimpl From<u16> for MeasurementId {\n");
    enum_str.push_str("    fn from(enum_str: u16) -> MeasurementId {\n");
    enum_str.push_str("        match enum_str {\n");
    for (i, id) in measurement_ids.clone().into_iter().enumerate() {
        enum_str.push_str(&format!("            {} => MeasurementId::{},\n", i, id));
    }
    enum_str.push_str("            _ => panic!(\"Failed to parse enum From<u16>\"),\n");
    enum_str.push_str("        }\n");
    enum_str.push_str("    }\n");
    enum_str.push_str("}\n");

    enum_str.parse().expect("Failed to parse enum END")
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
    let path_to_use = match std::env::current_dir().unwrap().ends_with("lib/core") {
        true => "../../".to_string() + &yaml_path,
        false => yaml_path,
    };
    match std::fs::read_to_string(path_to_use.clone()) {
        Ok(file) => Some(Yaml::load_from_str(&file).unwrap()[0].clone()),
        Err(_) => panic!("Failed to open file: {}", path_to_use),
    }
}
