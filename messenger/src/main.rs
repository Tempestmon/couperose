use tonic::transport::Server;

use proto::{
    messenger_server::{Messenger, MessengerServer},
    SendMessageResponse,
};

mod proto {
    tonic::include_proto!("messenger");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("messenger_descriptor");
}

#[derive(Debug, Default)]
struct MessengerService {}

#[tonic::async_trait]
impl Messenger for MessengerService {
    async fn send_message(
        &self,
        request: tonic::Request<proto::SendMessageRequest>,
    ) -> std::result::Result<tonic::Response<proto::SendMessageResponse>, tonic::Status> {
        let input = request.get_ref();

        print!("Got request from {}", input.sender);

        let response = SendMessageResponse { success: true };

        Ok(tonic::Response::new(response))
    }

    async fn get_messages(
        &self,
        request: tonic::Request<proto::GetMessagesRequest>,
    ) -> std::result::Result<tonic::Response<proto::GetMessagesResponse>, tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let messenger = MessengerService::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(MessengerServer::new(messenger))
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
