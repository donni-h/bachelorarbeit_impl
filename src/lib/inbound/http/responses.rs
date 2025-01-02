use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::order::Order;
use crate::domain::models::order_details::OrderDetails;
use crate::domain::models::order_item::OrderItem;

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseData {
    pub message: String,
}


#[derive(Debug, Clone, Serialize)]
pub struct OrderResponseData {
    id: Uuid,
    items: Vec<CheckoutItemResponse>,
    #[serde(alias = "metadata")]
    details: CheckoutDetailsResponse
}

impl From<&Order> for OrderResponseData {
    fn from(order: &Order) -> Self {
        let id = order.details().order_id().clone();
        let items = order
            .items()
            .iter()
            .map(CheckoutItemResponse::from)
            .collect();
        let details = CheckoutDetailsResponse::from(order.details());

        Self {
            id, items, details
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutItemResponse {
    name: String,
    item_price: i64,
    plant_id: Uuid,
}

impl From<&OrderItem> for CheckoutItemResponse {
    fn from(item: &OrderItem) -> Self {
        Self {
            name: item.product_name().to_string(),
            item_price: item.price().as_cents().unwrap(),
            plant_id: item.id().clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutDetailsResponse {
    username: String,
    status: String,
    session_id: String,
    created_at: DateTime<Utc>
}

impl From<&OrderDetails> for CheckoutDetailsResponse {
    fn from(details: &OrderDetails) -> Self {
        Self {
            username: details.username().to_string(),
            status: details.status().clone()
                .map_or("None".to_string(), |s| s.to_string()),
            session_id: details.session_id().to_string(),
            created_at: details.created_at().clone(),
        }
    }
}
