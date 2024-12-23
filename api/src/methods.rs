use crate::grpc_client::{
    create_grpc_client, get_grpc_client_from_pool, return_grpc_client_to_pool, AppState,
};
use crate::messenger;
use crate::models::{GetMessage, SendMessage};
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder};
use utoipa::OpenApi;

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
pub async fn send_message(state: Data<AppState>, message: Json<SendMessage>) -> impl Responder {
    println!("Got message");

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
}

#[utoipa::path(
    get,
    path = "/messages",
    responses(
        (status = 200, description = "Nothing yet")
    )
)]
#[get("/messages")]
pub async fn get_messages() -> impl Responder {
    println!("Got request to get messages");
    let mut grpc_client = create_grpc_client().await;
    let grpc_request = tonic::Request::new(messenger::GetMessagesRequest::default());
    let response = grpc_client.get_messages(grpc_request).await.unwrap();
    println!("{:#?}", response);
    let response_json: Vec<GetMessage> = response
        .into_inner()
        .messages
        .into_iter()
        .map(GetMessage::from)
        .collect();
    HttpResponse::Ok().json(response_json)
}

#[derive(OpenApi)]
#[openapi(
    paths(send_message, get_messages),
    components(schemas(SendMessage)),
    tags(
        (name = "Messaging", description = "API for messaging operations")
    )
)]
pub struct ApiDoc;
