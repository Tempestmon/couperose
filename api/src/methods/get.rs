use crate::grpc_client::create_grpc_client;
use crate::messenger;
use crate::models::GetMessage;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::methods::helpers::is_proxy_request;

#[utoipa::path(
    get,
    path = "/messages",
    responses(
        (status = 200, description = "Nothing yet")
    )
)]
#[get("/messages")]
pub async fn get_messages(_req: HttpRequest) -> impl Responder {
    println!("Got request to get messages");
    let has_token = is_proxy_request(_req.clone()).await;
    if has_token {
        let mut grpc_client = create_grpc_client().await; // TODO: Use grpc pool
        let grpc_request = tonic::Request::new(messenger::GetMessagesRequest::default());
        let response = grpc_client.get_messages(grpc_request).await.unwrap();
        let response_json: Vec<GetMessage> = response
            .into_inner()
            .messages
            .into_iter()
            .map(GetMessage::from)
            .collect();
        HttpResponse::Ok().json(response_json)
    } else {
        HttpResponse::Forbidden().body("Token is incorrect or was not provided")
    }
}
