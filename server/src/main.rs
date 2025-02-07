mod persist;
mod routers;
mod services;
mod utils;

async fn app() -> axum::Router {
    // Load environment variables
    dotenvy::dotenv().ok();

    //? This is not needed anymore since shuttle provides the logger
    // Initialize logger
    utils::log::init();

    // Initialize database connection using env var DATABASE_URL
    persist::init().await;

    // Initialize app state
    let state = utils::state::AppState {
        cookie_key: axum_extra::extract::cookie::Key::from(
            std::env::var("COOKIE_KEY")
                .expect("COOKIE_KEY not set in env!")
                .as_bytes(),
        ),
    };

    // Initialize router
    let router = routers::app::new_with_state(state);
    router.into()
}

#[tokio::main]
async fn main() {
    let app = app().await;

    let url = std::env::var("SERVER_URL").expect("Server URL not set in env!");
    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();

    log::info!("Server listening on: {}", url);

    axum::serve(listener, app).await.unwrap();
}
