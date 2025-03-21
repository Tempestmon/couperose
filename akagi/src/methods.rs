use crate::proto::{
    messenger_server::Messenger, GetMessagesRequest, GetMessagesResponse, SendMessageRequest,
    SendMessageResponse,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::fs;
use tonic::async_trait;
use tracing::info;

#[derive(Debug, Default)]
pub struct MessengerService {}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    sender: String,
    content: String,
    timestamp: i64,
}

impl Message {
    fn new(sender: String, content: String) -> Message {
        Message {
            sender,
            content,
            timestamp: 0,
        }
    }
}

impl From<Message> for crate::proto::Message {
    fn from(msg: Message) -> Self {
        crate::proto::Message {
            sender: msg.sender,
            content: msg.content,
            timestamp: msg.timestamp,
        }
    }
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[async_trait]
impl Messenger for MessengerService {
    async fn send_message(
        &self,
        request: tonic::Request<SendMessageRequest>,
    ) -> std::result::Result<tonic::Response<SendMessageResponse>, tonic::Status> {
        send_message(request).await
    }

    async fn get_messages(
        &self,
        _request: tonic::Request<GetMessagesRequest>,
    ) -> std::result::Result<tonic::Response<GetMessagesResponse>, tonic::Status> {
        get_messages().await
    }
}

async fn get_messages() -> Result<tonic::Response<GetMessagesResponse>, tonic::Status> {
    info!("Got request to get messages");

    let path = "messages.json";
    let data = fs::read_to_string(path)
        .await
        .expect("Couldn't read data to string");
    let messages: Vec<Message> = if data.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&data).expect("Couldn't parse from string")
    };

    let messages: Vec<crate::proto::Message> = messages.into_iter().map(|msg| msg.into()).collect();

    let response = GetMessagesResponse { messages };
    Ok(tonic::Response::new(response))
}

async fn send_message(
    request: tonic::Request<SendMessageRequest>,
) -> Result<tonic::Response<SendMessageResponse>, tonic::Status> {
    let input = request.get_ref();
    let path = "messages.json";

    info!("Got request from {}", input.sender);

    let response = SendMessageResponse { success: true };

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .expect("Couldn't open file");
    let data = fs::read_to_string(path)
        .await
        .expect("Couldn't read data to string");
    let mut messages: Vec<Message> = if data.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&data).expect("Couldn't parse from string")
    };
    let message = Message::new(input.sender.clone(), input.content.clone());
    messages.push(message);
    serde_json::to_writer_pretty(&mut file, &messages)
        .expect("Couldn't write messages to json file");

    Ok(tonic::Response::new(response))
}
