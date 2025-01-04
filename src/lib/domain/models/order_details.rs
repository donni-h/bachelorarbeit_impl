use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::uuid;
use derive_more::{Display, From};
use getset::Getters;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
#[getset(get = "pub")]
pub struct OrderDetails {
    order_id: Uuid,
    username: UserName,
    status: Option<SessionStatus>,
    session_id: SessionId,
    created_at: DateTime<Utc>,
}

impl OrderDetails {
    pub fn new(id: Uuid, username: UserName, status: Option<SessionStatus>, session_id: SessionId, created_at: DateTime<Utc>) -> Self {
        Self { order_id: id, username, status, session_id, created_at}
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

#[cfg(test)]
mod tests {
    use crate::domain::models::order_details::{SessionId, UserName};

    #[test]
    fn new_username_trim() {
        let username = UserName::new(" Hannes ");

        assert_eq!(username.to_string(), "Hannes");
    }

    #[test]
    fn new_session_id() {
        let session_id = SessionId::new("abc");
        assert_eq!(session_id.to_string(), "abc");
    }
}