use actix_web::http::StatusCode;
use actix_web::Responder;
use actix_web::web::{Data, Query};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::inbound::http::AppState;
use crate::inbound::http::handlers::{ApiError, ApiResponseBody};

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct DeleteByOrderIdHttpRequestQuery{
    order_id: Uuid
}

impl DeleteByOrderIdHttpRequestQuery {
    fn into_domain(self) -> Uuid {
        self.order_id
    }
}


#[utoipa::path(
    delete,
    params(
      DeleteByOrderIdHttpRequestQuery
    ),
    path="/api/payment/order",
    responses(
    (status = 200, description = "Successfully deleted order")
    )
)]
pub async fn delete_order_by_id<OS: OrderService, PS: PaymentService>(
    state: Data<AppState<OS, PS>>,
    query: Query<DeleteByOrderIdHttpRequestQuery>
) -> Result<impl Responder, ApiError> {
    let domain_req = query.into_inner().into_domain();

    state
        .order_service
        .delete_order(domain_req)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiResponseBody::new(StatusCode::OK, ()))
}