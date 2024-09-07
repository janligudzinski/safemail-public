use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    error::{DatabaseError, SmError, UserError},
    user::{User, UserRepository},
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pub db: Arc<PgPool>,
}
impl PostgresUserRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}
#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(
        &self,
        username: String,
        public_encryption_key: String,
        public_verify_key: String,
    ) -> Result<User, SmError> {
        let result = sqlx::query!(
            "INSERT INTO sm.users (username, public_encryption_key, public_verify_key) VALUES ($1, $2, $3) RETURNING *",
            username,
            public_encryption_key,
            public_verify_key
        )
        .fetch_one(&*self.db)
        .await
        .map_err(|e| {
            let db = e.as_database_error();
            let err: SmError = if let Some(sqlx_error) = db {
                match sqlx_error.kind() {
                    sqlx::error::ErrorKind::UniqueViolation => UserError::UserAlreadyExists.into(),
                    _ => DatabaseError::Arbitrary.into(),
                }
            } else {
                DatabaseError::Arbitrary.into()
            };
            err
        })?;

        Ok(User {
            id: result.id,
            username,
            public_encryption_key: result.public_encryption_key,
            public_verify_key: result.public_verify_key,
        })
    }
    async fn find_by_username(&self, username: String) -> Result<Option<User>, SmError> {
        let user = sqlx::query_as!(User, "SELECT * FROM sm.users WHERE username = $1", username)
            .fetch_optional(&*self.db)
            .await
            .map_err(|_| DatabaseError::Arbitrary)?;
        Ok(user)
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, SmError> {
        let user = sqlx::query_as!(User, "SELECT * FROM sm.users WHERE id = $1", id)
            .fetch_optional(&*self.db)
            .await
            .map_err(|_| DatabaseError::Arbitrary)?;
        Ok(user)
    }
}
