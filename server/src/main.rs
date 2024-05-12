mod routers;
mod utils;
mod persist;
mod services;


#[shuttle_runtime::main]
pub async fn axum(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5433/tijdcapsule"
    )] pool: sqlx::PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore
) -> shuttle_axum::ShuttleAxum {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logger
    // utils::log::init();

    // Initialize database connection from shuttle provided pool
    persist::from(pool);

    // Initialize app state
    let state = utils::state::AppState {
        cookie_key: axum_extra::extract::cookie::Key::from(secrets.get("COOKIE_KEY").expect("COOKIE_KEY not set in env!").as_bytes())
    };

    // Initialize router
    let router = routers::app::new_with_state(state);
    Ok(router.into())
}