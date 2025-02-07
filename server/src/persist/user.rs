use super::{capsule::{self, Capsule}, POOL};


pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub async fn create_capsule(&self, capsule: &Capsule) -> Result<Capsule, sqlx::Error> {
        capsule::create(capsule, &self.id).await
    }
}

pub async fn create(user: &User) -> Result<User, sqlx::Error> {
    match sqlx::query_as!(
        User,
        "
        INSERT INTO users (id, name)
            VALUES ($1, $2) ON CONFLICT DO NOTHING
            RETURNING id, name
        ", user.id, user.name
    ).fetch_one(POOL.get().unwrap()).await {
        Ok(user) => Ok(user),
        Err(sqlx::Error::RowNotFound) => Ok(User {
            id: user.id.clone(),
            name: user.name.clone(),
        }),
        Err(e) => Err(e),
    }
}

pub async fn get_by_id(id: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "
        SELECT id, name
            FROM users
            WHERE id = $1
        ", id
    ).fetch_one(POOL.get().unwrap()).await
}