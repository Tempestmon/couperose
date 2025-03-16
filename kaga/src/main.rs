use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use actix_web_prometheus::PrometheusMetricsBuilder;
use grpc_client::{initialize_grpc_pool, AppState};
use methods::get::get_messages;
use methods::send::send_message;
use std::env;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod grpc_client;
mod models;
pub mod messenger {
    tonic::include_proto!("messenger");
}
pub mod methods;

use crate::models::SendMessage;

#[derive(OpenApi)]
#[openapi(
    paths(methods::send::send_message, methods::get::get_messages),
    components(schemas(SendMessage)),
    tags(
        (name = "Messaging", description = "API for messaging operations")
    )
)]
pub struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::fmt().with_target(false).finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber.");
    let openapi = ApiDoc::openapi();
    let grpc_clients = initialize_grpc_pool(5).await;
    let app_state = AppState {
        grpc_clients: grpc_clients.clone(),
    };
    let host = env::var("API_HOST").expect("Couldn't get API_HOST env");
    let port = env::var("API_PORT").expect("Couldn't get API_PORT env");
    let url = format!("{}:{}", host, port);
    let allowed_url = env::var("CORS_URL").expect("Couldnt get CORS_URL env");
    info!("Starting http server {}", url);
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // .allowed_origin(&allowed_url)'
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allow_any_header(),
            )
            .wrap(prometheus.clone())
            .wrap(Logger::default())
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
