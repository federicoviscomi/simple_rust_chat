use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Client {
    Join {
        chat_name: Arc<String>,
    },
    Post {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_client() {
    use std::sync::Arc;
    let client: Client = Client::Post {
        chat_name: Arc::new(String::from("Chat")),
        message: Arc::new(String::from("Message sent!")),
    };
    let json: String = serde_json::to_string(&client).unwrap();
    println!("{:?}", json);
    assert_eq!(
        json,
        r#"{"Post":{"chat_name":"Chat","message":"Message sent!"}}"#
    );
    assert_eq!(serde_json::from_str::<Client>(&json).unwrap(), client);
}
