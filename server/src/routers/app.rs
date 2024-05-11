
use std::net::SocketAddr;

use axum::{routing::{ post, get }, Router};
use axum_client_ip::SecureClientIpSource;

use super::root;
use super::capsule;

fn new() -> Router {
    Router::new()
        .route("/", get(root::handler::get))
        .route("/capsule", post(capsule::handler::post))
        .route("/capsule/:capsule_id", get(capsule::handler::get))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
}

pub async fn init(addr: &'static str) {
    let app = new();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,  app.into_make_service_with_connect_info::<SocketAddr>(),).await.unwrap();
}