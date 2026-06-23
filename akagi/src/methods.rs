use crate::proto::{
    messenger_server::Messenger, GetMessagesRequest, GetMessagesResponse,
    Message as ProtoMessage, SendMessageRequest, SendMessageResponse,
};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use tonic::async_trait;
use tracing::info;

pub struct MessengerService {
    db: Arc<Mutex<Connection>>,
}

impl MessengerService {
    pub fn new(db: Arc<Mutex<Connection>>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Messenger for MessengerService {
    async fn send_message(
        &self,
        request: tonic::Request<SendMessageRequest>,
    ) -> Result<tonic::Response<SendMessageResponse>, tonic::Status> {
        let input = request.get_ref();
        let sender = input.sender.clone();
        let content = input.content.clone();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        info!("Got message from {}", sender);

        let db = Arc::clone(&self.db);
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            conn.execute(
                "INSERT INTO messages (sender, content, timestamp) VALUES (?1, ?2, ?3)",
                params![sender, content, timestamp],
            )
        })
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(SendMessageResponse { success: true }))
    }

    async fn get_messages(
        &self,
        _request: tonic::Request<GetMessagesRequest>,
    ) -> Result<tonic::Response<GetMessagesResponse>, tonic::Status> {
        info!("Got request to get messages");

        let db = Arc::clone(&self.db);
        let messages = tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT sender, content, timestamp FROM messages ORDER BY timestamp ASC",
            )?;
            let messages: Vec<ProtoMessage> = stmt
                .query_map([], |row: &rusqlite::Row| {
                    Ok(ProtoMessage {
                        sender: row.get(0)?,
                        content: row.get(1)?,
                        timestamp: row.get(2)?,
                    })
                })?
                .filter_map(|r| r.ok())
                .collect();
            rusqlite::Result::Ok(messages)
        })
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?
        .map_err(|e: rusqlite::Error| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(GetMessagesResponse { messages }))
    }
}
