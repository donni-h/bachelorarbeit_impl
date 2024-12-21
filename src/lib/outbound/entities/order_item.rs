use rust_decimal::Decimal;
use sqlx::FromRow;
use sqlx::types::{uuid::Uuid};
#[derive(Debug, Clone, FromRow)]
#[sqlx(type_name = "record")]
pub struct OrderItemEntity {
    pub id: Uuid,
    pub product_name: String,
    pub item_id: Uuid,
    pub price: Decimal,
    pub order_id: Uuid,
}