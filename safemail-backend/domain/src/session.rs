use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::error::SmError;

#[derive(Serialize)]
pub struct Session {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub active: bool,
    pub challenge_string: String,
    pub requested_at_utc: DateTime<Utc>,
    pub activated_at_utc: Option<DateTime<Utc>>,
    pub expires_at_utc: DateTime<Utc>,
}

#[async_trait]
pub trait SessionRepository {
    async fn request_session(&self, user_id: Uuid) -> Result<Session, SmError>;
    async fn activate_session(&self, session_id: Uuid) -> Result<(), SmError>;
    async fn get_session(
        &self,
        session_id: Uuid,
        include_inactive: bool,
    ) -> Result<Option<Session>, SmError>;
    async fn logout_session(&self, session_id: Uuid) -> Result<(), SmError>;
}
