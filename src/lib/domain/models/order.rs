use thiserror::Error;
use crate::domain::models::metadata::Metadata;
use crate::domain::models::order_item::OrderItem;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Order {
    id: uuid::Uuid,
    metadata: Metadata,
    items: Vec<OrderItem>,
}

impl Order {
    pub fn new(id: uuid::Uuid, metadata: Metadata, items: Vec<OrderItem>) -> Self {
        Self { id, metadata, items }
    }
}

#[derive(Debug, Error)]
pub enum FindOrderError {
    #[error("cannot find order with id {id}")]
    IdNotFound { id: uuid::Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}