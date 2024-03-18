use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

pub fn write_data_to_csv(data_list: &Vec<HashMap<String, String>>, file_path: &str) {
    // Open a file in write mode, automatically creating it if it doesn't exist,
    // or truncating it if it does.
    let mut file = File::create(file_path).expect("Failed to create CSV file");
    let placeholder = "".to_string();
    let mut result: Result<(), Error> = Ok(());

    // Write the CSV header
    writeln!(file, "Email,Type,Date").expect("Failed to write to CSV file");

    // Iterate over the data and write each entry to the file
    for entry in data_list {
        let email = entry.get("Email").unwrap_or(&placeholder);
        let entry_type = entry.get("Type").unwrap_or(&placeholder);
        let date = entry.get("Date").unwrap_or(&placeholder);
        result = writeln!(file, "{},{},{}", email, entry_type, date);
    }

    match result {
        Err(why) => panic!("couldn't write to {}: {}", file_path, why),
        Ok(_) => println!("File {} creatd successfully", file_path),
    }
}

pub fn write_data_to_md_table(data_list: &Vec<HashMap<String, String>>, file_path: &str) {
    let mut file = File::create(file_path).expect("Failed to create MD file");
    let placeholder = "".to_string();
    let mut result: Result<(), Error> = Ok(());

    // Write the Markdown table header
    writeln!(file, "| Email | Type | Date |").expect("Failed to write to MD file");
    writeln!(file, "|-------|------|------|").expect("Failed to write to MD file");

    // Iterate over the data and write each entry as a table row
    for entry in data_list {
        let email = entry.get("Email").unwrap_or(&placeholder);
        let entry_type = entry.get("Type").unwrap_or(&placeholder);
        let date = entry.get("Date").unwrap_or(&placeholder);
        if !email.is_empty() {
            result = writeln!(file, "| {} | {} | {} |", email, entry_type, date);
        }
    }
    match result {
        Err(why) => panic!("couldn't write to {}: {}", file_path, why),
        Ok(_) => println!("File {} creatd successfully", file_path),
    }
}

pub fn write_data_to_json(data: &Vec<HashMap<String, String>>, file_path: &str) {
    // Open a file in write mode
    let path = Path::new(file_path);
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Serialize the data to JSON
    let json = serde_json::to_string_pretty(&data).expect("Error serializing data to JSON");

    // Write the JSON data to the file
    match file.write_all(json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("File {} creatd successfully", file_path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_write_data_to_csv() {
        let data = vec![HashMap::from([
            ("Email".to_string(), "example@example.com".to_string()),
            ("Type".to_string(), "Parent".to_string()),
            (
                "Date".to_string(),
                "Tue, 12 Oct 2024 14:23:00 +0000".to_string(),
            ),
        ])];
        let file_path = "test_output.csv";
        write_data_to_csv(&data, file_path);
        assert!(Path::new(file_path).exists());
        fs::remove_file(file_path).unwrap(); // Clean up
    }

    #[test]
    fn test_write_data_to_md_table() {
        let data = vec![HashMap::from([
            ("Email".to_string(), "example@example.com".to_string()),
            ("Type".to_string(), "Parent".to_string()),
            (
                "Date".to_string(),
                "Tue, 12 Oct 2024 14:23:00 +0000".to_string(),
            ),
        ])];
        let file_path = "test_output.md";
        write_data_to_md_table(&data, file_path);
        assert!(Path::new(file_path).exists());
        fs::remove_file(file_path).unwrap(); // Clean up
    }
}
