use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::error::SmError;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub public_encryption_key: String,
    pub public_verify_key: String,
}

#[async_trait]
pub trait UserRepository {
    async fn create(
        &self,
        username: String,
        public_encryption_key: String,
        public_verify_key: String,
    ) -> Result<User, SmError>;
    async fn find_by_username(&self, username: String) -> Result<Option<User>, SmError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, SmError>;
}

pub trait PasswordService {
    fn hash_password(&self, password: String) -> String;
    fn verify_password(&self, password: String, hashed_password: &str) -> bool;
}
