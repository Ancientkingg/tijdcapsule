
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::persist::capsule::Capsule;


#[derive(Debug, Deserialize)]
pub struct CreateCapsule {
    pub name: String,
    pub content: String,
    pub author: String,
    pub deadline: Option<NaiveDateTime>,
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

    use axum::{body::Body, extract::{Query, Path}, http::StatusCode, response::{IntoResponse, Response}, Json};
    use axum_client_ip::SecureClientIp;
    use log::info;
    use serde_json::json;
    
    use crate::utils::idgen::CapsuleId;
    use crate::services;

    use super::{CreateCapsule, Params};

    pub async fn get(client_secure_ip: SecureClientIp, Query(params): Query<Params>, Path(capsule_id): Path<CapsuleId>) -> Response {
        let ip_addr = format!("{}", client_secure_ip.0);
        info!("Handling GET request to /capsule/{} from {}", capsule_id, ip_addr);

        let resp: Response = match services::capsule::get(&capsule_id, &params.sleutel).await {
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
                Json(capsule).into_response()
            },

            Err(services::capsule::CapsuleError::Deadline(deadline)) => {
                return Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::from(format!("{}", deadline))).unwrap();
            },

            Err(services::capsule::CapsuleError::Sqlx(sqlx::Error::RowNotFound)) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(format!("Capsule {} not found", capsule_id))).unwrap();
            },
            
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal server error")).unwrap();
            },
        };
        
        resp
    }

    
    pub async fn post(client_secure_ip: SecureClientIp, Json(payload): Json<CreateCapsule>) -> Response {
        let ip_addr = format!("{}", client_secure_ip.0);
        info!("Handling POST request to /capsule from {}", ip_addr);
        
        let server_capsule = match services::capsule::create(payload, ip_addr).await {
            Ok(capsule) => capsule,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal server error")).unwrap();
            }
        };
        
        Json(server_capsule).into_response()
    }
}


