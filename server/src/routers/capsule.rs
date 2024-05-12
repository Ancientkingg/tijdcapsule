
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::persist::capsule::Capsule;


#[derive(Debug, Deserialize)]
pub struct CreateCapsule {
    pub name: String,
    pub content: String,
    pub author: String,
    pub deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublishCapsule {
    pub capsule: Capsule,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    sleutel: String,
}

pub mod handler {

    use axum::{extract::{Path, Query}, http::StatusCode, response::{IntoResponse, Response}, Json};
    use axum_extra::extract::{cookie::Cookie, SignedCookieJar};
    use log::info;
    use serde_json::json;
    
    use crate::utils::idgen::CapsuleId;
    use crate::utils::idgen;
    use crate::services;

    use super::{CreateCapsule, Params};

    fn get_client_id(jar: SignedCookieJar) -> (SignedCookieJar, String) {
        if let Some(client_id) = jar.get("client_id") {
            (jar, client_id.value().to_string())
        } else {
            let client_id = idgen::generate_client_id();
            let jar = jar.add(Cookie::new("client_id", client_id.clone()));
            (jar, client_id)
        }
    }

    pub async fn get(jar: SignedCookieJar, Query(params): Query<Params>, Path(capsule_id): Path<CapsuleId>) -> (SignedCookieJar, Response) {
        let (jar, client_id) = get_client_id(jar);

        info!("Handling GET request to /capsule/{} from {}", capsule_id, client_id);

        let (jar, resp) = match services::capsule::get(&capsule_id, &params.sleutel).await {
            Ok(capsule) => {
                let author = services::user::get_by_id(&capsule.author_id).await.unwrap();

                let capsule = json!({
                    "id": capsule.id,
                    "name": capsule.name,
                    "content": capsule.content,
                    "author": author.name,
                    "deadline": capsule.deadline,
                    "created_at": capsule.created_at,
                });
                (jar, Json(capsule).into_response())
            },

            Err(services::capsule::CapsuleError::Deadline(deadline, created_at)) => {
                (jar, (StatusCode::FORBIDDEN, Json(json!({"deadline": deadline, "created_at": created_at}))).into_response())
            },

            Err(services::capsule::CapsuleError::Sqlx(sqlx::Error::RowNotFound)) => (jar, (StatusCode::NOT_FOUND, format!("Capsule {} not found", capsule_id)).into_response()),
            
            Err(_) => (jar, (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()),
        };
        
        (jar, resp)
    }

    
    pub async fn post(jar: SignedCookieJar, Json(payload): Json<CreateCapsule>) -> (SignedCookieJar, Response) {
        let (jar, client_id) = get_client_id(jar);

        info!("Handling POST request to /capsule from {}", client_id);
        
        let server_capsule = match services::capsule::create(payload, client_id).await {
            Ok(capsule) => capsule,
            Err(_) => return (jar, (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()),
        };
        
        (jar, Json(server_capsule).into_response())
    }
}


