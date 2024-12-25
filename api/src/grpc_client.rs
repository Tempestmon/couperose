use crate::messenger::messenger_client::MessengerClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct AppState {
    pub grpc_clients: Arc<Mutex<Vec<MessengerClient<Channel>>>>,
}

pub async fn create_grpc_client() -> MessengerClient<Channel> {
    MessengerClient::connect("http://[::]:50051") // TODO:
        // вынести
        // в
        // энвы
        .await
        .expect("Faield to create gRPC client")
}

pub async fn initialize_grpc_pool(pool_size: usize) -> Arc<Mutex<Vec<MessengerClient<Channel>>>> {
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
    pool.pop() // Take a client from the pool
}

pub async fn return_grpc_client_to_pool(
    pool: Arc<Mutex<Vec<MessengerClient<Channel>>>>,
    client: MessengerClient<Channel>,
) {
    let mut pool = pool.lock().await;
    pool.push(client);
}
