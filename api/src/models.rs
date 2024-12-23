use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
