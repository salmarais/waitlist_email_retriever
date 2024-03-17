use imap::Session;
use native_tls::{TlsConnector, TlsStream};
use std::{collections::HashSet, net::TcpStream};

pub struct EmailClient {
    imap_session: Session<TlsStream<TcpStream>>,
}
pub trait EmailClientTrait {
    fn fetch_email_messages(&self) -> Result<HashSet<u32>, String>;
    fn retrieve_bodies(&self, messages: HashSet<u32>) -> Vec<String>;
}

impl Drop for EmailClient {
    fn drop(&mut self) {
        let _ = self.imap_session.logout();
        println!("Session closed");
    }
}

impl EmailClient {
    pub fn new(_domain: &str, _port: u16, _username: &str, _password: &str) -> Self {
        let domain =if  _domain.is_empty() {"imap.hostinger.com" } else { _domain};
        let port = _port;
        let username = _username;
        let password = _password;

        
        let tls = TlsConnector::builder().build().unwrap();
        let client = imap::connect((domain, port), domain, &tls).unwrap();
        let imap_session = client.login(username, password).expect("Failed to login");

        Self {
            imap_session: imap_session, // Default value
        }
    }

    pub fn fetch_email_messages(&mut self) -> Result<std::collections::HashSet<u32>, imap::Error> {
        let subject = "Waitlist";
        return retrieve_messages_from_mailbox("INBOX", &mut self.imap_session, subject);
    }

    pub fn retrieve_bodies(mut self, messages: HashSet<u32>) -> Vec<String> {
        let mut bodies = Vec::new();

        if !messages.is_empty() {
            let sequence = messages
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let fetches = self
                .imap_session
                .fetch(sequence, "(RFC822.SIZE ENVELOPE BODY.PEEK[])")
                .expect("Failed to fetch query from sequence");
            for fetch in fetches.iter() {
                if let Some(body) = fetch.body() {
                    if let Ok(body_str) = std::str::from_utf8(body) {
                        bodies.push(body_str.to_string());
                    }
                }
            }
        }
        return bodies;
    }
}

fn retrieve_messages_from_mailbox(
    mailbox_name: &str,
    imap_session: &mut Session<TlsStream<TcpStream>>,
    subject: &str,
) -> Result<std::collections::HashSet<u32>, imap::Error> {
    imap_session.select(mailbox_name)?;

    let messages = imap_session
        .search(format!("SUBJECT \"{}\"", subject))
        .expect("Failed to get messages");

    println!(
        "Number of messages with subject '{}' in '{}': {}",
        subject,
        mailbox_name,
        messages.len()
    );

    return Ok(messages);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    struct MockEmailClient;
    

    impl EmailClientTrait for MockEmailClient {
        fn fetch_email_messages(&self) -> Result<HashSet<u32>, String> {
            // Return a fixed set of message IDs for testing
            Ok(vec![1, 2, 3].into_iter().collect())
        }

        fn retrieve_bodies(&self, _messages: HashSet<u32>) -> Vec<String> {
            // Return a fix_: &Selft of message bodies for testing
            vec![
                "Reply-To: example@example.com\nType: Parent\nDate: Tue, 12 Oct 2024 14:23:00 +0000".to_string(),
                "Reply-To:example@example.com\nType: Parent\nDate: Tue, 12 Oct 2024 14:23:00 +0000".to_string(),
                "Email body for message 3".to_string(),
            ]
        }
    }

    #[test]
    fn test_fetch_and_process_emails() {
        let email_client = MockEmailClient;
        let messages_result = email_client.fetch_email_messages().unwrap();
        assert_eq!(messages_result.len(), 3); // Verify the mocked fetch

        let bodies = email_client.retrieve_bodies(messages_result);
        assert_eq!(bodies.len(), 3); // Verify the mocked retrieve
        // Further assertions to verify processing logic...
    }
    #[test]
    fn test_retrieve_bodies() {
        let email_client = MockEmailClient;
        // Assuming fetch_email_messages() and retrieve_bodies() use the same message IDs for simplicity
        let message_ids = email_client.fetch_email_messages().unwrap();
        let bodies = email_client.retrieve_bodies(message_ids);

        // Verify the length of the returned bodies
        assert_eq!(bodies.len(), 3);

        // Verify the content of the returned bodies if necessary
        assert_eq!(bodies[0], "Reply-To: example@example.com\nType: Parent\nDate: Tue, 12 Oct 2024 14:23:00 +0000");
        assert_eq!(bodies[1], "Reply-To:example@example.com\nType: Parent\nDate: Tue, 12 Oct 2024 14:23:00 +0000");
        assert_eq!(bodies[2], "Email body for message 3");
    }
}