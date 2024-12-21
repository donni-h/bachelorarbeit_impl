use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::uuid::Uuid;
use crate::outbound::entities::order_item::OrderItemEntity;
use crate::outbound::entities::session_status::SessionStatus;

#[derive(Debug, FromRow)]
pub struct OrderQueryResult {
    pub id: Uuid,
    pub metadata_id: Uuid,
    pub metadata_order_id: Uuid,
    pub metadata_username: Option<String>,
    pub metadata_status: Option<SessionStatus>,
    pub metadata_session_id: String,
    pub metadata_created_at: NaiveDateTime,
    pub order_items: Vec<OrderItemEntity>,
}

impl OrderQueryResult {
}