use derive_more::From;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use crate::domain::models::order_details::{OrderDetails, SessionId, SessionStatus, UserName};

#[derive(Debug, FromRow)]
pub struct FetchOrderDetailsEntity {
    pub id: Uuid,
    pub username: String,       // Nullable column
    pub status: Option<SessionStatusEntity>, // Nullable column
    pub session_id: String,    // Nullable column
    pub created_at: DateTime<Utc>,     // Maps to TIMESTAMP
}

impl FetchOrderDetailsEntity {
    pub fn into_domain(self) -> OrderDetails {
        let status = self.status.map(SessionStatusEntity::into_domain);
        let username = UserName::new(&self.username);
        let session_id = SessionId::new(&self.session_id);
        
        OrderDetails::new(
            self.id,
            username,
            status,
            session_id,
            self.created_at,
        )
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "session_status", rename_all = "lowercase")]
pub enum SessionStatusEntity {
    Open,
    Complete,
    Expired,
}
#[derive(Debug)]
pub struct CreateOrderDetailsEntity {
    pub id: Uuid,
    pub username: String,
    pub status: Option<SessionStatusEntity>,
    pub session_id: String,
    pub created_at: DateTime<Utc>,
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

impl SessionStatusEntity {
    fn into_domain(self) -> SessionStatus {
        match self {
            SessionStatusEntity::Open => SessionStatus::Open,
            SessionStatusEntity::Complete => SessionStatus::Complete,
            SessionStatusEntity::Expired => SessionStatus::Expired,
        }
    }
}

impl CreateOrderDetailsEntity {
    pub fn from_domain(value: &OrderDetails) -> Self {
        let status = match value.status().clone() {
            Some(status) => Some(status.into()),
            None => None,
        };
        Self {
            id: value.order_id().clone(),
            username: value.username().to_string(),
            status,
            session_id: value.session_id().to_string(),
            created_at: value.created_at().clone(),
        }
    }
}