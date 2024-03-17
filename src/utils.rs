use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn remove_duplicate_emails(
    data_list: Vec<HashMap<String, String>>,
) -> Vec<HashMap<String, String>> {
    let mut seen_emails = HashSet::new();
    let mut result = Vec::new();

    for data in data_list.into_iter() {
        if let Some(email) = data.get("Email") {
            if seen_emails.insert(email.clone()) {
                // Clone the email String to store in the HashSet
                result.push(data);
            }
        }
    }

    result
}

pub fn read_config_from_file(file_path: &str) -> HashMap<String, String> {
    let file_contents = fs::read_to_string(file_path).expect("Failed to retrieve config file");
    let json: Value =
        serde_json::from_str(&file_contents).expect("Failed to parse config json file.");
    let config_map: HashMap<String, String> = json
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
        .collect();

    config_map
}
