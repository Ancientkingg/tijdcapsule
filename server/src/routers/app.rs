
use std::net::SocketAddr;

use axum::{http, routing::{ get, post }, Router};
use axum_client_ip::SecureClientIpSource;
use tower_http::cors::{Any, CorsLayer};
use http::Method;

use super::root;
use super::capsule;



fn new() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(root::handler::get))
        .route("/capsule", post(capsule::handler::post))
        .route("/capsule/:capsule_id", get(capsule::handler::get))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(cors)
}

pub async fn init(addr: &'static str) {
    let app = new();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,  app.into_make_service_with_connect_info::<SocketAddr>(),).await.unwrap();
}