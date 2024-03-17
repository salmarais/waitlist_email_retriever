// use reqwest;
// use std::collections::HashMap;
// use tokio;

// // token = "secret_vkQwyfvaNDckButxEnQ4rfV7oBgy9IixUAglIL686wLK"
// use serde_json::json;

// pub struct NotionClient {
//     _notion_base_url: String,
//     _notion_api_version: String,
//     _token: String,
// }

// impl NotionClient {
//     pub fn new() -> Self {
//         let notion_base_url = "https://api.notion.com/v1/".to_string();
//         let notion_api_version = "2022-06-28".to_string();
//         let token = "secret_vkQwyfvaNDckButxEnQ4rfV7oBgy9IiUAglIL686wLK".to_string();
//         Self {
//             _notion_base_url: notion_base_url,
//             _notion_api_version: notion_api_version,
//             _token: token,
//         }
//     }

//     #[tokio::main]
//     pub async fn set_waitlist_database_properties(self) {
//         let _database_id = "8877afbdd55c4136b0fcbc9b1e580a25/";
//         let _url = self._notion_base_url + "databases/" + _database_id;
//         dbg!(&_url);
//         let title = "Waitlist Users";
//         let description = "List of users that join our waitlist ";

//         let data = Self::format_database_properties_json(
//             title,
//             description,
//             vec!["Teacher", "Parent", "Student"],
//         );

//         let client = reqwest::Client::new();

//         let response = client
//             .post(_url)
//             .bearer_auth("secret_vkQwyfvaNDckButxEnQ4rfV7oBgy9IiUAglIL686wLK")
//             // .header("Content-Type", "application/json")
//             .header("Notion-Version", "2022-06-28")
//             .json(&data)
//             .send()
//             .await
//             .expect("Updating columns failed");

//         if response.status().is_success() {
//             let response_text = response.text().await;
//             println!("Response: {:?}", response_text);
//         } else {
//             eprintln!("Failed to send request");
//             println!("Response code: {:?}", response.status());
//             dbg!(&response);
//         }
//     }

//     fn format_database_entry_json(database_id: &str, entries: Vec<HashMap<String, String>>) {
//         for value in entries {
//             let mut option: HashMap<String, String> = HashMap::new();
//         }

//         let data = json!(
//             {
//                 "description": [
//                     {
//                         "text": {
//                             "content": "List of users that join our waitlist ⌛ "
//                         }
//                     }
//                 ],
//                 "properties": {
//                     "Date": {
//                         "id": "WlFr",
//                         "name": "Date",
//                         "type": "date",
//                         "date": {}
//                     },
//                     "ID": {
//                         "id": "%60%3BMW",
//                         "name": "ID",
//                         "type": "unique_id",
//                         "unique_id": {
//                             "prefix": null
//                         }
//                     },
//                     "Type": {
//                         "id": "giH%5C",
//                         "name": "Type",
//                         "type": "rich_text",
//                         "rich_text": {}
//                     },
//                     "Name": {
//                         "id": "title",
//                         "name": "Name",
//                         "type": "title",
//                         "title": {}
//                     }
//                 }
//             }
//         );
//     }

//     fn format_database_properties_json(
//         title: &str,
//         description: &str,
//         types_users_list: Vec<&str>,
//     ) -> serde_json::Value {
//         // Dynamically construct the options part of the JSON payload
//         let mut options_list: Vec<HashMap<String, String>> = Vec::new();
//         for value in types_users_list {
//             let mut option: HashMap<String, String> = HashMap::new();
//             option.insert("name".to_string(), value.to_string());
//             options_list.push(option);
//         }

//         let data = json!(
//             {
//                 "title": [
//                   {
//                     "text": {
//                       "content": title
//                     }
//                   }
//                 ],
//                 "description": [
//                   {
//                     "text": {
//                       "content": description
//                     }
//                   }
//                 ],
//                 "properties": {
//                   "User email": {
//                     "type": "email",
//                     "email": {}
//                   },
//                   "Application date": {
//                     "name": "Application date",
//                     "type": "date",
//                     "date": {}
//                   },
//                 //   "Type": {
//                 //     "id": "flsb",
//                 //     "name": "Type",
//                 //     "type": "select",
//                 //     "select": {
//                 //       "options": options_list
//                 //     }
//                 //   }
//                 }
//               }
//         );
//         println!("{}", &data);
//         data
//     }
// }
