use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn get_account_info() -> Value {
    let json_path = Path::new("src/config/accounts_data.json");
    // Read JSON data from a file
    let mut file = File::open(json_path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read file");

    // Parse JSON data into a serde_json::Value
    serde_json::from_str(&json_data).expect("Failed to parse JSON")
}

pub fn get_accounts_from_ext() -> Value {
    let json_path = Path::new("src/config/accounts_extension.json");
    let mut file = File::open(json_path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read file");

    // Parse JSON data into a serde_json::Value
    let data: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // println!("{:?}", data);

    data["accounts_by_seed"]["accounts"].clone()
}
