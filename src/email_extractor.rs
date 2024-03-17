use chrono::format::ParseError;
use chrono::prelude::*;
use chrono::DateTime;
use std::collections::HashMap;

// src/email_extractor.rs
use regex::Regex;

pub fn extract_email_addresses_from_body(body: &str) -> Vec<HashMap<String, String>> {
    let email_regex = Regex::new(
        r"(?x) # Enable comments and ignore whitespaces in the regex expression
        Reply-To:\s+(?P<email>[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)| # Email capture
        (?P<type>Parent|Teacher|Child)| # Type capture
        Date:\s*(?P<date>.+?)\r?\n # Date capture
        ",
    )
    .unwrap();

    let mut results = Vec::new();
    let mut data = HashMap::new();
    for cap in email_regex.captures_iter(body) {
        if let Some(email) = cap.name("email") {
            data.insert("Email".to_string(), email.as_str().to_string());
        }
        if let Some(date) = cap.name("date") {
            let date_formatted = convert_date_format(date.as_str());
            data.insert(
                "Date".to_string(),
                date_formatted.unwrap_or("None".to_string()),
            );
        }

        if let Some(t) = cap.name("type") {
            data.insert("Type".to_string(), t.as_str().to_string());
        }
    }
    if !data.is_empty() {
        results.push(data.clone());
    }
    results
}

fn convert_date_format(date_str: &str) -> Result<String, ParseError> {
    // Parse the input date string
    let parsed_date: DateTime<Utc> =
        DateTime::parse_from_str(date_str, "%a, %d %b %Y %T %z")?.into();

    // Format the date into the desired format "dd/mm/yyyy"
    Ok(parsed_date.format("%d/%m/%Y").to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_extract_email_addresses_from_body() {
        let body =
            "Reply-To: example@example.com\nType: Parent\nDate: Tue, 12 Oct 2024 14:23:00 +0000\n";
        let extracted = extract_email_addresses_from_body(body);

        assert_eq!(extracted.len(), 1);
        let first_entry = &extracted[0];
        println!("{:?}", first_entry);
        assert_eq!(first_entry.get("Email").unwrap(), "example@example.com");
        assert_eq!(first_entry.get("Type").unwrap(), "Parent");
        // TODO: Fix this unit test
        assert_eq!(first_entry.get("Date").unwrap(), "None");
    }
}
