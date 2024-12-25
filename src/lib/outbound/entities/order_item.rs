use rust_decimal::Decimal;
use sqlx::FromRow;
use sqlx::types::{uuid::Uuid};
use crate::domain::models::order_item::OrderItem;

#[derive(Debug, Clone, FromRow)]
pub struct CreateOrderItemEntity {
    pub id: Uuid,
    pub product_name: String,
    pub item_id: Uuid,
    pub price: Decimal,
    pub order_id: Uuid,
}

impl CreateOrderItemEntity {
    pub(crate) fn from_domain(item: &OrderItem, order_id: &Uuid) -> Self {
        Self {
            id: item.id().clone(),
            product_name: item.product_name().to_string(),
            item_id: item.item_id().clone(),
            price: item.price().as_ref().clone(),
            order_id: order_id.clone(),
        }
    }
}