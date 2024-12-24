use tonic::transport::Server;

use proto::messenger_server::MessengerServer;

mod methods;

mod proto {
    tonic::include_proto!("messenger");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;

    let messenger = methods::MessengerService::default();

    Server::builder()
        .add_service(MessengerServer::new(messenger))
        .serve(addr)
        .await?;

    Ok(())
}
