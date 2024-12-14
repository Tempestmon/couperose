use std::error::Error;

use proto::gateway_client::GatewayClient;

pub mod proto {
    tonic::include_proto!("gateway");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = GatewayClient::connect(url).await?;
    let req = proto::SendMessageRequest {
        sender: "client".to_string(),
        recipient: "server".to_string(),
        content: "Hello from client to server!".to_string(),
    };
    let response = client.send_message(req).await?;

    print!("Got response from server {}", response.get_ref().success);
    Ok(())
}
