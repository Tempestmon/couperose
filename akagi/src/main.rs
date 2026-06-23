use proto::messenger_server::MessengerServer;
use rusqlite::Connection;
use std::env;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;
use tracing::info;

mod methods;

mod proto {
    tonic::include_proto!("messenger");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt().with_target(false).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber.");

    let host = env::var("MESSENGER_HOST").expect("Couldn't get MESSENGER_HOST env");
    let port = env::var("MESSENGER_PORT").expect("Couldn't get MESSENGER_PORT env");
    let url = format!("{}:{}", host, port);
    let addr = url.parse()?;

    let conn = Connection::open("messages.db")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS messages (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            sender    TEXT NOT NULL,
            content   TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        );",
    )?;
    let db = Arc::new(Mutex::new(conn));

    let messenger = methods::MessengerService::new(db);

    info!("Starting grpc server {}", url);
    Server::builder()
        .add_service(MessengerServer::new(messenger))
        .serve(addr)
        .await?;

    Ok(())
}
