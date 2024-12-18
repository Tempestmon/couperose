use actix_cors::Cors;
use actix_web::web::{self, Data, Json};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use messenger::messenger_client::MessengerClient;
use models::Message;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
pub mod messenger {
    tonic::include_proto!("messenger");
}

#[derive(Clone)]
struct AppState {
    grpc_clients: Arc<Mutex<Vec<MessengerClient<Channel>>>>,
}

async fn create_grpc_client() -> MessengerClient<Channel> {
    MessengerClient::connect("http://[::1]:50051") // TODO:
        // вынести
        // в
        // энвы
        .await
        .expect("Faield to create gRPC client")
}

async fn initialize_grpc_pool(pool_size: usize) -> Arc<Mutex<Vec<MessengerClient<Channel>>>> {
    let mut pool = Vec::with_capacity(pool_size);
    for _ in 0..pool_size {
        pool.push(create_grpc_client().await);
    }
    Arc::new(Mutex::new(pool))
}

async fn get_grpc_client_from_pool(
    pool: Arc<Mutex<Vec<MessengerClient<Channel>>>>,
) -> Option<MessengerClient<Channel>> {
    let mut pool = pool.lock().await;
    pool.pop() // Take a client from the pool
}

async fn return_grpc_client_to_pool(
    pool: Arc<Mutex<Vec<MessengerClient<Channel>>>>,
    client: MessengerClient<Channel>,
) {
    let mut pool = pool.lock().await;
    pool.push(client);
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
async fn send_message(state: Data<AppState>, message: Json<Message>) -> impl Responder {
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
    let grpc_clients = initialize_grpc_pool(5).await;
    let app_state = AppState {
        grpc_clients: grpc_clients.clone(),
    };
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // Разрешает запросы с любых источников
                    .allow_any_method() // Разрешает любые методы (GET, POST и т.д.)
                    .allow_any_header(), // Разрешает любые заголовки
            )
            .app_data(web::Data::new(app_state.clone()))
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
