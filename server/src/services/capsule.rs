use chrono::{DateTime, Utc};

use crate::persist;
use crate::routers::capsule::PublishCapsule;
use crate::utils::cipher::{self, CipherError};
use crate::utils::idgen::key::to_key;
use crate::{persist::capsule::Capsule, routers::capsule::CreateCapsule, utils::idgen::CapsuleId};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CapsuleError {
    #[error("An error occurred while interacting with the database: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("The deadline for this capsule has not yet passed. It will be available at {0} and was created at {1}")]
    Deadline(DateTime<Utc>, DateTime<Utc>),
    #[error("An error occurred while encrypting or decrypting the content")]
    Cypher(#[from] CipherError),
    #[error("The payload was not valid")]
    Payload(&'static str),
}

pub async fn create(payload: CreateCapsule, id: String) -> Result<PublishCapsule, CapsuleError> {
    let (key_fragment, key) = crate::utils::idgen::key::generate();

    let encrypted_content = cipher::encrypt(&payload.content, &key, None)?;

    let capsule = Capsule {
        id: CapsuleId::generate(),
        name: Some(payload.name),
        content: Some(encrypted_content),
        author_id: String::from("NOT_AN_ID___SOMETHING_WENT_WRONG"),
        deadline: payload.deadline,
        created_at: chrono::Utc::now().to_utc(),
    };

    let author = crate::persist::user::User {
        id,
        name: payload.author,
    };

    let server_author = crate::persist::user::create(&author).await?;
    let server_capsule = server_author.create_capsule(&capsule).await?;

    if server_capsule.author_id == "NOT_AN_ID___SOMETHING_WENT_WRONG" {
        return Err(CapsuleError::Sqlx(sqlx::Error::RowNotFound));
    }

    Ok(PublishCapsule {
        capsule: server_capsule,
        key: key_fragment,
    })
}

pub async fn get(capsule_id: &CapsuleId, sleutel: &str) -> Result<Capsule, CapsuleError> {
    let mut server_capsule = match persist::capsule::get_by_id(capsule_id).await {
        Ok(capsule) => capsule,
        Err(x) => return Err(CapsuleError::Sqlx(x)),
    };

    if let Some(deadline) = server_capsule.deadline {
        if deadline > chrono::Utc::now().to_utc() {
            return Err(CapsuleError::Deadline(deadline, server_capsule.created_at));
        }
    }

    server_capsule.content = Some(cipher::decrypt(
        &server_capsule.content.ok_or(CapsuleError::Payload("Capsule has no content"))?,
        &to_key(sleutel),
        None,
    )?);

    Ok(server_capsule)
}
