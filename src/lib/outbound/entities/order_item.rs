use anyhow::anyhow;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sqlx::FromRow;
use sqlx::types::{uuid::Uuid};
use crate::domain::models::order::FindOrderError;
use crate::domain::models::order_item::{OrderItem, Price, ProductName};

#[derive(Debug, Clone, FromRow)]
pub struct CreateOrderItemEntity {
    pub id: Uuid,
    pub product_name: String,
    pub item_id: Uuid,
    pub price: Decimal,
    pub order_id: Uuid,
}

impl CreateOrderItemEntity {
    pub fn from_domain(item: &OrderItem, order_id: &Uuid) -> Self {
        Self {
            id: item.id().clone(),
            product_name: item.product_name().to_string(),
            item_id: item.item_id().clone(),
            price: item.price().as_ref().clone(),
            order_id: order_id.clone(),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct FetchOrderItemEntity {
    pub id: Uuid,
    pub product_name: String,
    pub price: Decimal,
    pub item_id: Uuid,
    pub order_id: Uuid,
}

impl FetchOrderItemEntity {
    pub fn try_into_domain(self) -> Result<OrderItem, FindOrderError> {
        let product_name = ProductName::new(&self.product_name);
        let price = Price::new(self.price.to_f64().unwrap())
            .map_err(|e| FindOrderError::Unknown(anyhow!(e)))?;
        
        Ok(OrderItem::new(
            self.id,
            product_name,
            self.item_id,
            price,
        ))
    }
}