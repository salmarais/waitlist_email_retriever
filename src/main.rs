use std::collections::HashMap;
mod email_client;
mod email_extractor;
mod io_manager;
mod notion_client;
mod utils;

fn main() {
    let config: HashMap<String, String> = utils::read_config_from_file("local_config.json");
    let output_path: &str = "output/";
    std::fs::create_dir_all(output_path).unwrap();
    get_waitlist_data_from_email_server(config, output_path);
}

fn get_waitlist_data_from_email_server(config: HashMap<String, String>, output_path: &str) {
    let _port: u16 = config
        .get("port")
        .get_or_insert(&"993".to_string())
        .parse()
        .expect("failed to parse port");
    let mut email_client_service = email_client::EmailClient::new(
        config.get("domain").get_or_insert(&"".to_string()),
        _port,
        config.get("username").get_or_insert(&"".to_string()),
        config.get("password").get_or_insert(&"".to_string()),
    );

    let _messages = email_client_service.fetch_email_messages().unwrap();
    let _bodies = email_client_service.retrieve_bodies(_messages);
    let mut data_list: Vec<HashMap<String, String>> = _bodies
        .iter()
        .flat_map(|content| email_extractor::extract_email_addresses_from_body(content))
        .collect::<Vec<HashMap<String, String>>>();
    data_list = utils::remove_duplicate_emails(data_list);
    println!("Found {} unique email", data_list.len());

    let csv_file_path = format!("{output_path}waitlist.csv");
    let md_file_path = format!("{output_path}waitlist.md");
    let json_file_path = format!("{output_path}waitlist.json");

    // Call the function to write the emails to the CSV file
    match io_manager::write_data_to_csv(&data_list, csv_file_path.as_str()) {
        Ok(_) => println!("Emails have been written to {}", csv_file_path),
        Err(e) => eprintln!("Failed to write emails to CSV: {}", e),
    }

    // Call the function to write the emails to the Markdown file
    match io_manager::write_data_to_md_table(&data_list, md_file_path.as_str()) {
        Ok(_) => println!("Emails have been written to {}", md_file_path),
        Err(e) => eprintln!("Failed to write emails to CSV: {}", e),
    }

    match io_manager::write_data_to_json(&data_list, json_file_path.as_str()) {
        Ok(_json_string) => {
            println!("Serialized JSON String to file: {}", json_file_path);
            // let notion_client_service = notion_client::NotionClient::new();
            // notion_client_service.set_waitlist_database_properties();
            // You can now use json_string for your JSON request
        }
        Err(e) => eprintln!("Failed to serialize data to JSON: {}", e),
    }
}
