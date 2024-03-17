use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn write_data_to_csv(
    data_list: &Vec<HashMap<String, String>>,
    file_path: &str,
) -> io::Result<()> {
    // Open a file in write mode, automatically creating it if it doesn't exist,
    // or truncating it if it does.
    let mut file = File::create(file_path)?;
    let placeholder = "".to_string();

    // Write the CSV header
    writeln!(file, "Email,Type,Date")?;

    // Iterate over the data and write each entry to the file
    for entry in data_list {
        let email = entry.get("Email").unwrap_or(&placeholder);
        let entry_type = entry.get("Type").unwrap_or(&placeholder);
        let date = entry.get("Date").unwrap_or(&placeholder);
        writeln!(file, "{},{},{}", email, entry_type, date)?;
    }

    Ok(())
}

pub fn write_data_to_md_table(
    data_list: &Vec<HashMap<String, String>>,
    file_path: &str,
) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let placeholder = "".to_string();

    // Write the Markdown table header
    writeln!(file, "| Email | Type | Date |")?;
    writeln!(file, "|-------|------|------|")?;

    // Iterate over the data and write each entry as a table row
    for entry in data_list {
        let email = entry.get("Email").unwrap_or(&placeholder);
        let entry_type = entry.get("Type").unwrap_or(&placeholder);
        let date = entry.get("Date").unwrap_or(&placeholder);
        if !email.is_empty() {
            writeln!(file, "| {} | {} | {} |", email, entry_type, date)?;
        }
    }
    Ok(())
}

pub fn write_data_to_json(
    data: &Vec<HashMap<String, String>>,
    file_path: &str,
) -> io::Result<String> {
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
        Ok(_) => Ok(json),
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
        write_data_to_csv(&data, file_path).unwrap();
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
        write_data_to_md_table(&data, file_path).unwrap();
        assert!(Path::new(file_path).exists());
        fs::remove_file(file_path).unwrap(); // Clean up
    }
}
