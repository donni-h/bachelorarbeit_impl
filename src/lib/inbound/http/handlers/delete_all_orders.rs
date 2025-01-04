use actix_web::http::StatusCode;
use actix_web::Responder;
use actix_web::web::Data;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::inbound::http::AppState;
use crate::inbound::http::handlers::{ApiError, ApiResponseBody};

pub async fn delete_all_orders<OS: OrderService, PS: PaymentService>(
    state: Data<AppState<OS, PS>>,
) -> Result<impl Responder, ApiError> {
    state
        .order_service
        .delete_all_orders()
        .await
        .map_err(ApiError::from)
        .map(|()| ApiResponseBody::new(StatusCode::OK, ()))
}