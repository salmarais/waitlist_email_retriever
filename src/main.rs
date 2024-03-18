use clap::Parser;
use std::collections::HashMap;

mod email_client;
mod email_extractor;
mod io_manager;
mod notion_client;
mod sheets_client;
mod utils;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    write_sheet_content: bool,

    #[arg(short, long, default_value_t = String::from("local_config.json"))]
    config_file_path: String,

    #[arg(short, long, default_value_t = String::from("output"))]
    output_dir: String,

    #[arg(long, default_value_t = false)]
    csv: bool,

    #[arg(long, default_value_t = false)]
    json: bool,

    #[arg(long, default_value_t = false)]
    md: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    

    // Setting arguments
    let config: HashMap<String, String> =
        utils::read_config_from_file(args.config_file_path.as_str());

    
    let mut data_list = get_waitlist_data_from_email_server(config.clone());



    // Saving content into files
    let output_path: &str = args.output_dir.as_str();
    std::fs::create_dir_all(output_path).unwrap();
    if args.csv {
        io_manager::write_data_to_csv(&data_list, format!("{output_path}/waitlist.csv").as_str());
    }
    if args.md {
        io_manager::write_data_to_md_table(
            &data_list,
            format!("{output_path}/waitlist.md").as_str(),
        );
    }
    if args.json {
        io_manager::write_data_to_json(&data_list, format!("{output_path}/waitlist.json").as_str());
    }

    let mut sheets_client_service =
        sheets_client::SheetClient::new(config.get("secret_file").get_or_insert(&"".to_string()));
    let _hub = sheets_client_service.build_hub().await;

    let current_sheet_content = sheets_client_service
        .read_data_from_sheet(
            _hub.clone(),
            config
                .get("spreadsheet_id")
                .get_or_insert(&"".to_string())
                .as_str(),
            "A:C",
        )
        .await;

    data_list = utils::subtract_by_email(data_list, current_sheet_content);
    dbg!("Lines to add:");
    dbg!(data_list.clone());

    if args.write_sheet_content && !data_list.is_empty() {
        sheets_client_service
            .write_data_to_sheet(
                _hub.clone(),
                data_list,
                config
                    .get("spreadsheet_id")
                    .get_or_insert(&"".to_string())
                    .as_str(),
                config
                    .get("spreadsheet_start_range")
                    .get_or_insert(&"A2:A2".to_string()),
            )
            .await;
    }
}

fn get_waitlist_data_from_email_server(
    config: HashMap<String, String>,
) -> Vec<HashMap<String, String>> {
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

    data_list
}
