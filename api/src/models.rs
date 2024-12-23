use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::messenger::{Message, SendMessageRequest};

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SendMessage {
    pub sender: String,
    pub recipient: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: i64,
}

impl From<SendMessage> for SendMessageRequest {
    fn from(message: SendMessage) -> Self {
        Self {
            sender: message.sender,
            recipient: message.recipient,
            content: message.content,
        }
    }
}

impl From<Message> for GetMessage {
    fn from(message: Message) -> Self {
        Self {
            sender: message.sender,
            content: message.content,
            timestamp: message.timestamp,
        }
    }
}
