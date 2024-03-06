use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utilities;
pub mod client;
pub mod chats_map;
pub mod chats;
pub mod connection;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Client {
    Join {
        chat_name: Arc<String>,
    },
    Post {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
}

pub enum Server {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String)
}

#[test]
fn test_client() {
    use std::sync::Arc;

    let client:Client = Client::Post {
        chat_name: Arc::new(String::from("MyName")),
        message: Arc::new(String::from("My Message !")),
    };

    let json:String = serde_json::to_string(&client).unwrap(); // Convert Enum to JSON String
    println!("{:?}", serde_json::from_str::<Client>(&json).unwrap()); // Print the Enum Object

}