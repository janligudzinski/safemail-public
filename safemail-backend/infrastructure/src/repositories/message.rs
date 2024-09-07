use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use domain::error::{DatabaseError, SmError};
use domain::message::{Message, MessageMetadata, MessageRepository};

#[derive(Clone)]
pub struct PostgresMessageRepository {
    pool: Arc<PgPool>,
}

impl PostgresMessageRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepository for PostgresMessageRepository {
    async fn create_message(
        &self,
        recipient_id: Uuid,
        metadata: MessageMetadata,
        content: String,
    ) -> Result<Message, SmError> {
        let record = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO sm.messages (recipient_id, metadata, content)
            VALUES ($1, $2, $3)
            RETURNING id, recipient_id, metadata, recipient_metadata, content
            "#,
            recipient_id,
            metadata.0,
            content
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|_| SmError::Database(DatabaseError::Arbitrary))?;

        Ok(record)
    }

    async fn get_message(&self, recipient_id: Uuid, id: i64) -> Result<Option<Message>, SmError> {
        let record = sqlx::query_as!(
            Message,
            r#"
            SELECT id, recipient_id, metadata, recipient_metadata, content
            FROM sm.messages
            WHERE recipient_id = $1 AND id = $2
            "#,
            recipient_id,
            id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|_| SmError::Database(DatabaseError::Arbitrary))?;

        Ok(record)
    }

    async fn update_recipient_metadata(
        &self,
        id: i64,
        recipient_metadata: String,
    ) -> Result<(), SmError> {
        sqlx::query!(
            r#"
            UPDATE sm.messages
            SET recipient_metadata = $1
            WHERE id = $2
            "#,
            recipient_metadata,
            id
        )
        .execute(&*self.pool)
        .await
        .map_err(|_| SmError::Database(DatabaseError::Arbitrary))?;

        Ok(())
    }

    async fn delete_message(&self, id: i64) -> Result<(), SmError> {
        sqlx::query!(
            r#"
            DELETE FROM sm.messages
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await
        .map_err(|_| SmError::Database(DatabaseError::Arbitrary))?;

        Ok(())
    }

    async fn list_messages(
        &self,
        recipient_id: Uuid,
        above_id: Option<i64>,
    ) -> Result<Vec<(i64, MessageMetadata)>, SmError> {
        let records = sqlx::query!(
            r#"
            SELECT id, metadata
            FROM sm.messages
            WHERE recipient_id = $1
            AND ($2::bigint IS NULL OR id > $2)
            ORDER BY id
            "#,
            recipient_id,
            above_id
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|_| SmError::Database(DatabaseError::Arbitrary))?;

        Ok(records
            .into_iter()
            .map(|r| (r.id, MessageMetadata(r.metadata)))
            .collect())
    }
}
