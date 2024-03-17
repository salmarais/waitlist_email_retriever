# Waitlist email retriever
This is a Rust project for retrieving waitlist data sent by email by WPForm plugin, to an IMAP server. It generated md, json and csv files.

## Getting Started
These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites
What things you need to install the software and how to install them:

- Rust Programming Language
- Cargo (Rust's package manager, comes with Rust)
- An accessible IMAP email server

### Installation
A step-by-step series of examples that tell you how to get a development environment running:

Clone the repository:
```BASH
bash
git clone https://github.com/salmarais/waitlist_email_retriever.git
cd waitlist_email_retriever
```

Build the project:

```BASH
cargo build
```

## Configuration
Before running the application, you need to set up a configuration JSON file that includes details about your IMAP server and authentication credentials. The structure of the configuration file is as follows:

```JSON
{
    "domain": "imap.yourserver.com",
    "port": "993",
    "username": "your-email@yourdomain.com",
    "password": "yourpassword"
}
```
- domain: The domain of your IMAP server.
port: The port your IMAP server uses for connections (usually 993 for IMAP over SSL).
- username: Your email address used for authentication.
- password: Your password used for authentication.
Please create this JSON file and ensure it's located at [Specify the expected location/path of the JSON config file within your project or system].

## Running the Application
To run the application, use the following command:

```BASH
cargo run
```

The files are generated under folder `output` in the project directory.