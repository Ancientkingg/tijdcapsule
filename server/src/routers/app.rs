
use axum::{routing::get, Router};

use super::root;

fn new() -> Router {
    Router::new()
        .route("/", get(root::handler))
}

pub async fn init(addr: &'static str) {
    let app = new();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}