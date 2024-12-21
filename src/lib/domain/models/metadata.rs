use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::uuid;
use derive_more::{Display, From};
use getset::Getters;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
#[getset(get = "pub")]
pub struct Metadata {
    id: uuid::Uuid,
    username: UserName,
    status: Option<SessionStatus>,
    session_id: SessionId,
    created_at: DateTime<Utc>,
}

impl Metadata {
    pub fn new(id: uuid::Uuid, username: UserName, status: Option<SessionStatus>, session_id: SessionId, created_at: DateTime<Utc>) -> Self {
        Self {id, username, status, session_id, created_at}
    }
}

// keine extra Errors, da UserName in Spring Boot auch keine Constraints hat...
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct UserName(String);

impl UserName {
    pub fn new(raw: &str) -> Self {
        Self(raw.trim().to_string())
    }
}

#[derive(Display, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, From)]
pub enum SessionStatus {
    Open,
    Complete,
    Expired,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(raw: &str) -> Self {
        Self(raw.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters)]
#[getset(get = "pub")]
pub struct CreateMetadataRequest {

    username: UserName,
    status: Option<SessionStatus>,
    order_id: uuid::Uuid,
    session_id: SessionId,
}

impl CreateMetadataRequest {
    pub fn new(username: UserName, status: Option<SessionStatus>, order_id: uuid::Uuid, session_id: SessionId) -> Self {
        Self { username, status, order_id, session_id }
    }
}