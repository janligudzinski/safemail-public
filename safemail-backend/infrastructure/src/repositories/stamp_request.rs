use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    error::{DatabaseError, SmError},
    stamp_request::{OnetimeStampRequest, StampRequestRepository},
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresStampRequestRepository {
    pool: Arc<PgPool>,
}

impl PostgresStampRequestRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StampRequestRepository for PostgresStampRequestRepository {
    async fn create_stamp_request(
        &self,
        difficulty: i64,
        recipient_id: Uuid,
    ) -> Result<Uuid, SmError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO sm.onetime_stamp_requests (difficulty, recipient_id)
            VALUES ($1, $2)
            RETURNING stamp_request_id
            "#,
            difficulty,
            recipient_id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|_| SmError::from(DatabaseError::Arbitrary))?;

        Ok(row.stamp_request_id)
    }

    async fn get_stamp_request(
        &self,
        stamp_request_id: Uuid,
    ) -> Result<Option<OnetimeStampRequest>, SmError> {
        let result = sqlx::query_as!(
            OnetimeStampRequest,
            r#"
            SELECT * FROM sm.onetime_stamp_requests
            WHERE stamp_request_id = $1
            "#,
            stamp_request_id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|_| SmError::from(DatabaseError::Arbitrary))?;

        Ok(result)
    }

    async fn mark_solved(&self, stamp_request_id: Uuid) -> Result<(), SmError> {
        sqlx::query!(
            r#"
            UPDATE sm.onetime_stamp_requests
            SET solved_at = CURRENT_TIMESTAMP AT TIME ZONE 'UTC'
            WHERE stamp_request_id = $1
            "#,
            stamp_request_id,
        )
        .execute(&*self.pool)
        .await
        .map_err(|_| SmError::from(DatabaseError::Arbitrary))?;

        Ok(())
    }
}
