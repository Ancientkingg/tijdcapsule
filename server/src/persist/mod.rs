
pub mod capsule;
pub mod user;

use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn init() {
    if let Some(_) = POOL.get() {
        panic!("Database connection pool already initialized!");
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();

    POOL.set(pool).unwrap();
}