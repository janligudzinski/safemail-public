use crate::error::SmError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct OnetimeStampRequest {
    pub stamp_request_id: Uuid,
    pub recipient_id: Uuid,
    pub difficulty: i64,
    pub valid_to: DateTime<Utc>,
    pub solved_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait StampRequestRepository {
    async fn create_stamp_request(
        &self,
        difficulty: i64,
        recipient_id: Uuid,
    ) -> Result<Uuid, SmError>;
    async fn get_stamp_request(
        &self,
        stamp_request_id: Uuid,
    ) -> Result<Option<OnetimeStampRequest>, SmError>;
    async fn mark_solved(&self, stamp_request_id: Uuid) -> Result<(), SmError>;
}
