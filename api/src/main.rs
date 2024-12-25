use actix_cors::Cors;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use grpc_client::{initialize_grpc_pool, AppState};
use methods::{get_messages, send_message, ApiDoc};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod grpc_client;
mod methods;
mod models;
pub mod messenger {
    tonic::include_proto!("messenger");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting http server");
    let openapi = ApiDoc::openapi();
    let grpc_clients = initialize_grpc_pool(5).await;
    let app_state = AppState {
        grpc_clients: grpc_clients.clone(),
    };
    let host = env::var("API_HOST").expect("Couldn't get API_HOST env");
    let port = env::var("API_PORT").expect("Couldn't get API_PORT env");
    let url = format!("{}:{}", host, port);
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
    .bind(url)?
    .run()
    .await
}
