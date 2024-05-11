

pub mod handler {
    use axum::response::{IntoResponse, Response};
    use log::info;

    pub async fn get() -> Response {
        info!("Handling request to /");
        
        String::from("Hello world!").into_response()
    }
}

