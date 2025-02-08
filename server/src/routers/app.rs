use crate::utils::state::AppState;
use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use tower_http::services::ServeDir;

use super::capsule;
use super::root;

pub fn new_with_state(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any);

    let trace = TraceLayer::new_for_http();

    let front_end = ServeDir::new("public");

    Router::new()
        .nest_service("/", front_end.clone())
        .nest_service("/capsule/:capsule_id", front_end)
        .route("/api/health", get(root::handler::get))
        .route("/api/capsule", post(capsule::handler::post))
        .route("/api/capsule/:capsule_id", get(capsule::handler::get))
        .with_state(state)
        .layer(cors)
        .layer(trace)
}
