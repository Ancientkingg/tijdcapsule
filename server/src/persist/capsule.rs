
use crate::utils::idgen::CapsuleId;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use super::POOL;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct Capsule {
    pub id: CapsuleId,
    pub name: Option<String>,
    pub content: Option<String>,
    pub author_id: String,
    pub deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub async fn get_by_id(id: &CapsuleId) -> Result<Capsule, sqlx::Error> {
    sqlx::query_as!(
        Capsule,
        "
        SELECT id, name, content, author_id, deadline, created_at
            FROM capsules
            WHERE id = $1
        ", id.to_string()
    ).fetch_one(POOL.get().unwrap()).await
}

pub async fn create(capsule: &Capsule, author_id: &str) -> Result<Capsule, sqlx::Error> {

    sqlx::query_as!(
        Capsule,
        "
        INSERT INTO capsules (id, name, content, author_id, deadline, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, content, author_id, deadline, created_at
        ", CapsuleId::generate().to_string(), capsule.name, capsule.content, author_id, capsule.deadline, capsule.created_at).fetch_one(POOL.get().unwrap()).await
}