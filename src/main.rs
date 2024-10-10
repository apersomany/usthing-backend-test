use std::net::Ipv4Addr;

use axum::{routing::get, Router};
use reqwest::Client;
use routes::schedule;
use tokio::{net::TcpListener, signal::ctrl_c};
use tracing::{error, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let tcp_listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 80))
        .await
        .inspect_err(|error| error!("error while binding tcp listener: {error}"))
        .unwrap();
    let app = Router::new()
        .route("/", get(hello_world))
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
        .nest("/schedule", schedule::router())
        .with_state(Client::new());
    info!("application started");
    axum::serve(tcp_listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .inspect_err(|error| error!("error while serving the application: {error}"))
        .unwrap();
}

#[derive(OpenApi)]
#[openapi(paths(hello_world, routes::schedule::library::get))]
struct ApiDoc;

#[utoipa::path(get, path = "/")]
/// get a hello world string
async fn hello_world() -> String {
    format!("hello world!")
}

async fn shutdown_signal() {
    ctrl_c()
        .await
        .inspect_err(|error| error!("error while waiting for ctrl-c signal: {error}"))
        .unwrap();
    info!("shutdown signal received");
    info!("gracefully shutting down");
}
