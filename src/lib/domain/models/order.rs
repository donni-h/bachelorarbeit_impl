use derive_more::From;
use getset::Getters;
use thiserror::Error;
use uuid::Uuid;
use crate::domain::models::order_details::{OrderDetails, SessionStatus, UserName};
use crate::domain::models::order_item::{CreateOrderItemRequest, OrderItem};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
#[getset(get = "pub")]
pub struct Order {
    details: OrderDetails,
    items: Vec<OrderItem>,
}

impl Order {
    pub fn new(order_details: OrderDetails, items: Vec<OrderItem>) -> Result<Self, CreateOrderError> {
        if items.is_empty() {
            return Err(CreateOrderError::NoItems);
        }
        
        Ok(Self { details: order_details, items })
    }
}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters)]
#[getset(get = "pub")]
pub struct CreateOrderRequest {
    id: Uuid,
    username: UserName, 
    items: Vec<CreateOrderItemRequest>,
}

impl CreateOrderRequest {
    pub fn new(username: UserName, items: Vec<CreateOrderItemRequest>) -> Self {
        Self {id: Uuid::new_v4(), username, items }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters)]
#[getset(get = "pub")]
pub struct UpdateOrderStatusRequest {
    id: Uuid,
    status: SessionStatus,
}

impl UpdateOrderStatusRequest {
    pub fn new(id: Uuid, status: SessionStatus) -> Self {
        Self { id, status }
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
    #[error("Order must contain items")]
    NoItems,
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

#[derive(Debug, Error)]
pub enum UpdateOrderError {
    #[error("order does not exist")]
    NotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}


#[cfg(test)]
mod tests {
    use chrono::Utc;
    use crate::domain::models::order::{CreateOrderError, Order};
    use crate::domain::models::order_details::{OrderDetails, SessionId, SessionStatus, UserName};
    use crate::domain::models::order_item::{OrderItem, Price, ProductName};

    fn create_order_details() -> OrderDetails {
        let id = uuid::Uuid::new_v4();
        let username = UserName::new("Hannes");
        let status = Some(SessionStatus::Open);
        let session_id = SessionId::new("meine session");
        let created_at = Utc::now();
        
        OrderDetails::new(id, username, status, session_id, created_at)
    }
    
    fn create_order_item() -> OrderItem {
        let id = uuid::Uuid::new_v4();
        let product_name = ProductName::new("Testprodukt");
        let item_id = uuid::Uuid::new_v4();
        let price = Price::new(5.0).unwrap();
        
        OrderItem::new(id, product_name, item_id, price)
    }
    #[test]
    fn order_new() {
        let details = create_order_details();
        let item = create_order_item();
        let order_result = Order::new(details, vec![item]);

        assert!(order_result.is_ok(), "Expected Order::new to return Ok");
        
    }
    
    #[test]
    fn order_no_items() {
        let details = create_order_details();
        let order_result = Order::new(details, vec![]);
        
        matches!(order_result, Err(CreateOrderError::NoItems));
    }
}
