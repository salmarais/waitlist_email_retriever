extern crate google_sheets4 as sheets4;

use std::collections::HashMap;

use serde_json::Value;
use sheets4::api::ValueRange;
use sheets4::hyper::client::HttpConnector;
use sheets4::hyper::Client;
use sheets4::hyper_rustls::HttpsConnector;
use sheets4::oauth2::{self};
use sheets4::{hyper, hyper_rustls, Error, Sheets};

pub struct SheetClient {
    client: Client<HttpsConnector<HttpConnector>>,
    secret_file: String,
}

impl SheetClient {
    pub fn new(secret_file: &str) -> Self {
        let client = hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .enable_http2()
                .build(),
        );

        Self {
            client,
            secret_file: secret_file.to_string(),
        }
    }

    pub async fn build_hub(&mut self) -> Sheets<HttpsConnector<HttpConnector>> {
        let secret: oauth2::ServiceAccountKey =
            oauth2::read_service_account_key(self.secret_file.clone())
                .await
                .expect("secret not found");

        let auth = oauth2::ServiceAccountAuthenticator::with_client(secret, self.client.clone())
            .build()
            .await
            .expect("could not create an authenticator");

        Sheets::new(self.client.clone(), auth)
    }

    pub async fn read_data_from_sheet(
        &mut self,
        hub: Sheets<HttpsConnector<HttpConnector>>,
        spreadsheet_id: &str,
        range: &str,
    ) -> Vec<HashMap<String, String>> {
        let read = hub
            .spreadsheets()
            .values_get(spreadsheet_id, range)
            .doit()
            .await;

        match read {
            Err(e) => {
                println!("{}", e);
                vec![HashMap::new()]
            }
            Ok((_, spreadsheet)) => {
                if spreadsheet.values.is_some() {
                    // spreadsheet.values.unwrap();

                    if let Some(values) = spreadsheet.values {
                        let keys = &values[0].clone(); // Assume the first row contains the keys
                        values
                            .into_iter()
                            .skip(1) // Skip the first row since it's used as keys
                            .map(|row| {
                                keys.iter()
                                    .cloned()
                                    .zip(row.into_iter())
                                    .map(|(key, value)| {
                                        let key_str = key.as_str().unwrap_or_default().to_string();
                                        let value_str =
                                            value.as_str().unwrap_or_default().to_string();
                                        (key_str, value_str)
                                    }) // Pair each key with the corresponding value
                                    .collect::<HashMap<String, String>>() // Collect pairs into a HashMap
                            })
                            .collect() // Collect all HashMaps into a Vec
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
        }
    }

    pub async fn write_data_to_sheet(
        &mut self,
        hub: Sheets<HttpsConnector<HttpConnector>>,
        data_list: Vec<HashMap<String, String>>,
        spreadsheet_id: &str,
        range: &str,
    ) {
        let values: Vec<Vec<Value>> = data_list
            .into_iter()
            .map(|hash_map| {
                vec![
                    hash_map
                        .get("Email")
                        .map(|s| Value::String(s.clone()))
                        .unwrap_or(Value::Null),
                    hash_map
                        .get("Type")
                        .map(|s| Value::String(s.clone()))
                        .unwrap_or(Value::Null),
                    hash_map
                        .get("Date")
                        .map(|s| Value::String(s.clone()))
                        .unwrap_or(Value::Null),
                ]
            })
            .collect();
        let req = ValueRange {
            major_dimension: None,
            range: None,
            values: Some(values),
        };

        let write = hub
            .spreadsheets()
            .values_append(req, spreadsheet_id, range)
            .value_input_option("USER_ENTERED")
            .doit()
            .await;

        match write {
            Err(e) => match e {
                // The Error enum provides details about what exactly happened.
                // You can also just use its `Debug`, `Display` or `Error` traits
                Error::HttpError(_)
                | Error::Io(_)
                | Error::MissingAPIKey
                | Error::MissingToken(_)
                | Error::Cancelled
                | Error::UploadSizeLimitExceeded(_, _)
                | Error::Failure(_)
                | Error::BadRequest(_)
                | Error::FieldClash(_)
                | Error::JsonDecodeError(_, _) => println!("{}", e),
            },
            Ok(res) => println!("Success: {:#?}", res),
        }
    }
}
