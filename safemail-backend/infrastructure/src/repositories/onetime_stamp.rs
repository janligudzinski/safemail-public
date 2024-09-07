use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    error::{DatabaseError, SmError},
    onetime_stamp::{OneTimeStampTracker, OneTimeStampTrackerRepository},
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresOneTimeStampRepository {
    pool: Arc<PgPool>,
}

impl PostgresOneTimeStampRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OneTimeStampTrackerRepository for PostgresOneTimeStampRepository {
    async fn insert(&self, stamp_id: Uuid, recipient_id: Uuid) -> Result<(), SmError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO sm.onetime_stamps (stamp_id, recipient_id)
            VALUES ($1, $2)
            "#,
            stamp_id,
            recipient_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|_| SmError::from(DatabaseError::Arbitrary))?;

        Ok(())
    }

    async fn get_by_id(&self, stamp_id: Uuid) -> Result<Option<OneTimeStampTracker>, SmError> {
        let result = sqlx::query_as!(
            OneTimeStampTracker,
            r#"
            SELECT stamp_id, recipient_id, used_or_revoked
            FROM sm.onetime_stamps
            WHERE stamp_id = $1
            "#,
            stamp_id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|_| SmError::from(DatabaseError::Arbitrary))?;

        Ok(result)
    }

    async fn set_used_or_revoked(&self, stamp_id: Uuid) -> Result<(), SmError> {
        sqlx::query!(
            r#"
            UPDATE sm.onetime_stamps
            SET used_or_revoked = true
            WHERE stamp_id = $1
            "#,
            stamp_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|_| SmError::from(DatabaseError::Arbitrary))?;

        Ok(())
    }
}
