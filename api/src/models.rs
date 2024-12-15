use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Message {
    pub sender: String,
    pub recipient: String,
    pub content: String,
}
