use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::SmError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub recipient_id: Uuid,
    pub metadata: String,
    pub recipient_metadata: Option<String>,
    pub content: String,
}
pub struct MessageMetadata(pub String);

#[async_trait]
pub trait MessageRepository {
    async fn create_message(
        &self,
        recipient_id: Uuid,
        metadata: MessageMetadata,
        content: String,
    ) -> Result<Message, SmError>;
    async fn get_message(&self, recipient_id: Uuid, id: i64) -> Result<Option<Message>, SmError>;
    async fn update_recipient_metadata(
        &self,
        id: i64,
        recipient_metadata: String,
    ) -> Result<(), SmError>;
    async fn delete_message(&self, id: i64) -> Result<(), SmError>;
    async fn list_messages(
        &self,
        recipient_id: Uuid,
        above_id: Option<i64>,
    ) -> Result<Vec<(i64, MessageMetadata)>, SmError>;
}
