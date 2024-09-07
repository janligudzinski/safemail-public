use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct PeriodicStamp {
    pub issuer_id: Uuid,
    pub recipient_id: Uuid,
    pub sender_id: Uuid,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnetimeStamp {
    pub stamp_id: Uuid,
    pub issuer_id: Uuid,
    pub recipient_id: Uuid,
    pub sender_id: Uuid,
    pub valid_to: Option<DateTime<Utc>>,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimeStampRequest {
    pub stamp_request_id: Uuid,
    pub difficulty: i64,
    pub valid_to: DateTime<Utc>,
    pub solved_at: Option<DateTime<Utc>>,
}
