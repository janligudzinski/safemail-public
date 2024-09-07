use crate::error::SmError;
use async_trait::async_trait;
use uuid::Uuid;

pub struct OneTimeStampTracker {
    pub stamp_id: Uuid,
    pub recipient_id: Uuid,
    pub used_or_revoked: bool,
}

#[async_trait]
pub trait OneTimeStampTrackerRepository {
    async fn insert(&self, stamp_id: Uuid, recipient_id: Uuid) -> Result<(), SmError>;
    async fn get_by_id(&self, stamp_id: Uuid) -> Result<Option<OneTimeStampTracker>, SmError>;
    async fn set_used_or_revoked(&self, stamp_id: Uuid) -> Result<(), SmError>;
}
