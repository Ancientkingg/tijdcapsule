pub mod capsule;
pub mod user;

use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

#[allow(dead_code)]
pub async fn init() {
    if POOL.get().is_some() {
        panic!("Database connection pool already initialized!");
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => (),
        Err(e) => log::error!("Failed to run migrations: {:?}", e),
    }

    POOL.set(pool).unwrap();
}

#[allow(dead_code)]
pub fn from(pool: Pool<Postgres>) {
    if POOL.get().is_some() {
        panic!("Database connection pool already initialized!");
    }

    POOL.set(pool).unwrap();
}
