use chrono::NaiveDateTime;
use derive_more::From;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use thiserror::Error;
use crate::domain::models::order_details::{OrderDetails, SessionId, SessionStatus, UserName};

#[derive(Debug, FromRow)]
pub struct FetchOrderDetailsEntity {
    pub id: Uuid,
    pub username: String,
    pub status: Option<SessionStatusEntity>,
    pub session_id: String,
    pub created_at: DateTime<Utc>,
}


impl Into<OrderDetails> for FetchOrderDetailsEntity {
    fn into(self) -> OrderDetails {
        let username = UserName::new(self.username.as_str());
        let status: Option<SessionStatus> = self.status.map(SessionStatusEntity::into_domain);
        let session_id = SessionId::new(self.session_id.as_str());
        OrderDetails::new(
            self.id,
            username,
            status,
            session_id,
            self.created_at,
        )
    }
}
#[derive(Debug, sqlx::Type, From, Clone)]
#[sqlx(type_name = "session_status", rename_all = "lowercase")]
pub enum SessionStatusEntity {
    Open,
    Complete,
    Expired,
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

#[derive(Debug)]
pub struct CreateOrderDetailsEntity {
    pub id: Uuid,
    pub username: String,
    pub status: Option<SessionStatusEntity>,
    pub session_id: String,
}

impl CreateOrderDetailsEntity {
    pub fn from_domain(value: OrderDetails) -> Self {
        let status = value.status().clone().map(Into::into);
        Self {
            id: value.id().clone(),
            username: value.username().to_string(),
            status,
            session_id: value.session_id().to_string(),
        }
    }
}
#[derive(Debug, Error)]
pub enum FindOrderDetailsError {
    #[error("Couldn't find user belong to id: {id}")]
    SessionIdNotFound {id: String},
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}