use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    error::{DatabaseError, SmError},
    system_key::{SystemKeyPair, SystemKeyRepository},
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresSystemKeyRepository {
    pool: Arc<PgPool>,
}

impl PostgresSystemKeyRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SystemKeyRepository for PostgresSystemKeyRepository {
    async fn init_system_keys(&self, system_key: SystemKeyPair) -> Result<(), SmError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO sm.system_sign_keys (private_key, public_key)
            SELECT $1, $2
            WHERE NOT EXISTS (SELECT 1 FROM sm.system_sign_keys)
            "#,
            system_key.private_key,
            system_key.public_key
        )
        .execute(&*self.pool)
        .await
        .map_err(|_| SmError::from(DatabaseError::Arbitrary))?;

        if result.rows_affected() == 0 {
            return Ok(());
        }

        Ok(())
    }

    async fn get_system_keys(&self) -> Result<Option<SystemKeyPair>, SmError> {
        let result = sqlx::query_as!(
            SystemKeyPair,
            r#"
            SELECT private_key, public_key
            FROM sm.system_sign_keys
            LIMIT 1
            "#
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|_| SmError::from(DatabaseError::Arbitrary))?;

        Ok(result)
    }
}
