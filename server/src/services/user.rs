use crate::persist::user::User;
use crate::persist;



pub async fn get_by_id(id: &str) -> Result<User, sqlx::Error> {
    persist::user::get_by_id(id).await
}