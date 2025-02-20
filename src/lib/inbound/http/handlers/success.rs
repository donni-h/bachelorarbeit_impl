use actix_web::http::StatusCode;
use actix_web::Responder;
use actix_web::web::{Data, Query};
use serde::Deserialize;
use utoipa::IntoParams;
use crate::domain::models::order::UpdateOrderStatusRequest;
use crate::domain::models::order_details::SessionId;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::inbound::http::AppState;
use crate::inbound::http::handlers::{ApiError, ApiResponseBody};
use crate::inbound::http::responses::OrderResponseData;

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct SuccessHttpRequestQuery{
    session_id: String,
}

impl SuccessHttpRequestQuery {
    fn into_domain(self) -> SessionId {
        SessionId::new(&self.session_id)
    }
}

#[utoipa::path(
    get,
    path="/api/payment/success",
    params(
        SuccessHttpRequestQuery
    ),
    responses(
    (status = 200, description = "Order", body = OrderResponseData)
    )
)]
pub async fn success<OS: OrderService, PS: PaymentService>(
    state: Data<AppState<OS, PS>>,
    query: Query<SuccessHttpRequestQuery>
) -> Result<impl Responder, ApiError> {
    let domain_req = query.into_inner().into_domain();
    let order = state
        .order_service
        .find_order_by_session_id(&domain_req)
        .await
        .map_err(ApiError::from)?;
    
    let order_id = *order.details().order_id();
    let new_status = state
        .payment_service
        .retrieve_checkout_status(&domain_req)
        .await
        .map_err(ApiError::from)?;
    
    let update_req = UpdateOrderStatusRequest::new(order_id, new_status);
    
    let updated_order = state
        .order_service
        .update_order_status(update_req)
        .await
        .map_err(ApiError::from)?;
    
    let response_data = OrderResponseData::from(&updated_order);
    
    Ok(ApiResponseBody::new(StatusCode::OK, response_data))
    
}
