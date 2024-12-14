use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Message {
    sender: String,
    recepient: String,
    content: String,
}
