use getset::Getters;
use thiserror::Error;
use crate::domain::models::order_details::OrderDetails;
use crate::domain::models::order_item::OrderItem;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
#[getset(get = "pub")]
pub struct Order {
    order_details: OrderDetails,
    items: Vec<OrderItem>,
}

impl Order {
    pub fn new(order_details: OrderDetails, items: Vec<OrderItem>) -> Self {
        Self { order_details, items }
    }
}

#[derive(Debug, Error)]
pub enum FindOrderError {
    #[error("cannot find order with id {id}")]
    IdNotFound { id: uuid::Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum CreateOrderError {
    #[error("order already exists")]
    Duplicate,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    
}

#[derive(Debug, Error)]
pub enum DeleteOrderError {
    #[error("order does not exist")]
    NotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error), 
}