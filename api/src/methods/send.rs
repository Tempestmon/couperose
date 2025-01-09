use crate::grpc_client::{get_grpc_client_from_pool, return_grpc_client_to_pool, AppState};
use crate::messenger;
use crate::methods::helpers::is_proxy_request;
use crate::models::SendMessage;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpRequest, HttpResponse, Responder};

#[utoipa::path(
    post,
    path = "/message",
    request_body = SendMessage,
    responses(
        (status = 200, description = "Message sent successfully", body = SendMessage),
        (status = 400, description = "Invalid payload"),
    )
)]
#[post("/message")]
pub async fn send_message(
    _req: HttpRequest,
    state: Data<AppState>,
    message: Json<SendMessage>,
) -> impl Responder {
    println!("Got message");
    let has_token = is_proxy_request(_req.clone()).await;
    if has_token {
        let client = get_grpc_client_from_pool(state.grpc_clients.clone()).await;
        let mut client = match client {
            Some(client) => client,
            None => return HttpResponse::InternalServerError().body("No available gRPC clients"),
        };
        let grpc_request =
            tonic::Request::new(messenger::SendMessageRequest::from(message.into_inner()));
        let response = client.send_message(grpc_request).await;
        return_grpc_client_to_pool(state.grpc_clients.clone(), client).await;
        match response {
            Ok(_) => HttpResponse::Ok().body("Message sent successfully"),
            Err(e) => HttpResponse::InternalServerError().body(format!("gRPC error: {}", e)),
        }
    } else {
        HttpResponse::Forbidden().body("Token is incorrect or was not provided")
    }
}
