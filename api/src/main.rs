use actix_web::web::Json;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use messenger::messenger_client::MessengerClient;
use models::Message;
use tonic::transport::Channel;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod messenger {
    tonic::include_proto!("messenger");
}

mod models;

async fn create_grpc_client() -> MessengerClient<Channel> {
    MessengerClient::connect("http://[::1]:50051") // TODO:
        // вынести
        // в
        // энвы
        .await
        .unwrap()
}

impl From<models::Message> for messenger::SendMessageRequest {
    fn from(message: models::Message) -> Self {
        Self {
            sender: message.sender,
            recipient: message.recipient,
            content: message.content,
        }
    }
}

#[utoipa::path(
    post,
    path = "/message",
    request_body = Message,
    responses(
        (status = 200, description = "Message sent successfully", body = Message),
        (status = 400, description = "Invalid payload"),
    )
)]
#[post("/message")]
async fn send_message(message: Json<Message>) -> impl Responder {
    println!("Got message");
    let mut client = create_grpc_client().await; //TODO: Сделать пул
                                                 //соединений
    let grpc_request =
        tonic::Request::new(messenger::SendMessageRequest::from(message.into_inner()));
    match client.send_message(grpc_request).await {
        // TODO: Нормальная обработка
        Ok(response) => HttpResponse::Ok(),
        Err(e) => HttpResponse::InternalServerError(),
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
async fn get_messages() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(OpenApi)]
#[openapi(
    paths(send_message, get_messages),
    components(schemas(Message)),
    tags(
        (name = "Messaging", description = "API for messaging operations")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        App::new()
            .service(send_message)
            .service(get_messages)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
