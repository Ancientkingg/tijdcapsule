use chrono::{DateTime, Utc};

use crate::routers::capsule::PublishCapsule;
use crate::utils::idgen::key::to_key;
use crate::{persist::capsule::Capsule, routers::capsule::CreateCapsule, utils::idgen::CapsuleId};
use crate::persist;
use crate::utils::cipher;

pub async fn create(payload: CreateCapsule, ip_addr: String) -> Result<PublishCapsule, sqlx::Error> {

    let (key_fragment, key) = crate::utils::idgen::key::generate();

    let encrypted_content = cipher::encrypt(&payload.content, &key, None).unwrap();

    let capsule = Capsule {
        id: CapsuleId::generate(),
        name: Some(payload.name),
        content: Some(encrypted_content),
        author_id: String::from("NOT_AN_ID___SOMETHING_WENT_WRONG"),
        deadline: payload.deadline,
        created_at: chrono::Utc::now().to_utc(),
    };

    let author = crate::persist::user::User {
        id: ip_addr,
        name: payload.author,
    };

    let server_author = crate::persist::user::create(&author).await.unwrap();
    let server_capsule = server_author.create_capsule(&capsule).await.unwrap();

    if server_capsule.author_id == "NOT_AN_ID___SOMETHING_WENT_WRONG" {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(PublishCapsule { capsule: server_capsule, key: key_fragment })
}

pub enum CapsuleError {
    Sqlx(sqlx::Error),
    Deadline(DateTime<Utc>)
}

pub async fn get(capsule_id: &CapsuleId, sleutel: &str) -> Result<Capsule, CapsuleError> {
    let mut server_capsule = match persist::capsule::get_by_id(capsule_id).await {
        Ok(capsule) => capsule,
        Err(x) => return Err(CapsuleError::Sqlx(x))
    };

    if let Some(deadline) = server_capsule.deadline {
        if deadline > chrono::Utc::now().to_utc() {
            return Err(CapsuleError::Deadline(deadline));
        }
    }

    server_capsule.content = Some(cipher::decrypt(&server_capsule.content.unwrap(), &to_key(sleutel), None).unwrap());

    Ok(server_capsule)
}