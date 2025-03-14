use crate::messenger::messenger_client::MessengerClient;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub grpc_clients: Arc<Mutex<Vec<MessengerClient<Channel>>>>,
}

pub async fn create_grpc_client() -> MessengerClient<Channel> {
    let host = env::var("MESSENGER_HOST").expect("Couldn't get MESSENGER_HOST env");
    let port = env::var("MESSENGER_PORT").expect("Couldn't get MESSENGER_PORT env");
    let url = format!("http://{}:{}", host, port);
    info!("Create grpc client for {}", url);
    MessengerClient::connect(url)
        .await
        .expect("Failed to create gRPC client")
}

pub async fn initialize_grpc_pool(pool_size: usize) -> Arc<Mutex<Vec<MessengerClient<Channel>>>> {
    info!("Initializing grpc pool with {} size", pool_size);
    let mut pool = Vec::with_capacity(pool_size);
    for _ in 0..pool_size {
        pool.push(create_grpc_client().await);
    }
    Arc::new(Mutex::new(pool))
}

pub async fn get_grpc_client_from_pool(
    pool: Arc<Mutex<Vec<MessengerClient<Channel>>>>,
) -> Option<MessengerClient<Channel>> {
    let mut pool = pool.lock().await;
    info!("Take client from connection pool");
    pool.pop() // Take a client from the pool
}

pub async fn return_grpc_client_to_pool(
    pool: Arc<Mutex<Vec<MessengerClient<Channel>>>>,
    client: MessengerClient<Channel>,
) {
    let mut pool = pool.lock().await;
    info!("Return connection to pool");
    pool.push(client);
}
