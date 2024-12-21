use chrono::NaiveDateTime;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
#[derive(Debug, FromRow)]
pub struct MetadataEntity {
    pub id: Uuid,
    pub order_id: Uuid,
    pub username: Option<String>,       // Nullable column
    pub status: Option<SessionStatus>, // Nullable column
    pub session_id: Option<String>,    // Nullable column
    pub created_at: NaiveDateTime,     // Maps to TIMESTAMP
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "session_status", rename_all = "lowercase")]
pub enum SessionStatus {
    Open,
    Complete,
    Expired,
}