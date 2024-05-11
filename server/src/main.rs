mod routers;
mod utils;
mod persist;
mod services;


#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logger
    utils::log::init();

    // Initialize database connection
    persist::init().await;


    // Initialize and serve the app
    routers::app::init("0.0.0.0:3000").await;
}