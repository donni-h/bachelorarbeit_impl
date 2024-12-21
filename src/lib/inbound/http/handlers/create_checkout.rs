use crate::domain::models::order::CreateOrderError;
use crate::inbound::http::handlers::general::ApiError;

impl From<CreateOrderError> for ApiError {
    fn from(e: CreateOrderError) -> Self {
        match e {

        }
    }
}