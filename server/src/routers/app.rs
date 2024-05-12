
use axum::{http::Method, routing::{ get, post }, Router};
use tower_http::cors::{Any, CorsLayer};
use crate::utils::state::AppState;
use tower_http::services::ServeDir;

use super::root;
use super::capsule;


pub fn new_with_state(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // let middleware = map_request_with_state(state.clone(), client_id::set_and_verify);

    Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/health", get(root::handler::get))
        .route("/capsule", post(capsule::handler::post))
        .route("/capsule/:capsule_id", get(capsule::handler::get))
        .layer(cors)
        .with_state(state)
}