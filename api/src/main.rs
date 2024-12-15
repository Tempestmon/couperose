use actix_web::web::Json;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use models::Message;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;

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
    HttpResponse::Ok().json(message)
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
