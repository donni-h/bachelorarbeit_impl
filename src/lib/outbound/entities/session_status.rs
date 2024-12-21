#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "session_status")]
#[sqlx(rename_all = "lowercase")]
pub enum SessionStatus {
    Open,
    Complete,
    Expired,
}