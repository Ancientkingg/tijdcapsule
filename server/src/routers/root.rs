use axum::response::{IntoResponse, Response};
use log::info;


pub async fn handler() -> Response {
    info!("Handling request to /");
    
    String::from("Hello world!").into_response()
}