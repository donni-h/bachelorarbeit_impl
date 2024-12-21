use chrono::NaiveDateTime;
use derive_more::From;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use crate::domain::models::metadata::{Metadata, SessionStatus};

#[derive(Debug, FromRow)]
pub struct FetchMetadataEntity {
    pub id: Uuid,
    pub order_id: Uuid,
    pub username: Option<String>,       // Nullable column
    pub status: Option<SessionStatusEntity>, // Nullable column
    pub session_id: Option<String>,    // Nullable column
    pub created_at: DateTime<Utc>,     // Maps to TIMESTAMP
}

#[derive(Debug, sqlx::Type, From)]
#[sqlx(type_name = "session_status", rename_all = "lowercase")]
pub enum SessionStatusEntity {
    Open,
    Complete,
    Expired,
}
#[derive(Debug)]
pub struct CreateMetadataEntity {
    pub order_id: Uuid,
    pub username: String,
    pub status: Option<SessionStatusEntity>,
    pub session_id: String,
}

impl From<SessionStatus> for SessionStatusEntity {
    fn from(value: SessionStatus) -> Self {
        match value {
            SessionStatus::Open => SessionStatusEntity::Open,
            SessionStatus::Complete => SessionStatusEntity::Complete,
            SessionStatus::Expired => SessionStatusEntity::Expired,
        }
    }
}

impl CreateMetadataEntity {
    pub fn from_metadata(value: Metadata, order_id: Uuid) -> Self {
        let status = match value.status().clone() {
            Some(status) => Some(status.into()),
            None => None,
        };
        Self {
            order_id,
            username: value.username().to_string(),
            status,
            session_id: value.session_id().to_string(),
        }
    }
}