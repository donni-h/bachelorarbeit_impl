use actix_web::http::StatusCode;
use actix_web::Responder;
use actix_web::web::{Data, Query};
use serde::Deserialize;
use crate::domain::models::order_details::SessionId;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::inbound::http::AppState;
use crate::inbound::http::extractors::auth::KeycloakToken;
use crate::inbound::http::handlers::{ApiError, ApiResponseBody};

#[derive(Debug, Clone, Deserialize)]
pub struct CancelHttpRequestQuery{
    session_id: String,
}

impl CancelHttpRequestQuery {
    fn into_domain(self) -> SessionId {
        SessionId::new(&self.session_id)
    }
}

pub async fn cancel<OS: OrderService, PS: PaymentService>(
    auth: KeycloakToken,
    state: Data<AppState<OS, PS>>,
    query: Query<CancelHttpRequestQuery>
) -> Result<impl Responder, ApiError> {

    println!("{:?}", auth);
    let domain_req = query.into_inner().into_domain();
    let order = state
        .order_service
        .find_order_by_session_id(&domain_req)
        .await
        .map_err(ApiError::from)?;

    let session_id = order.details().session_id();
    state
        .payment_service
        .expire_session(session_id)
        .await
        .map_err(ApiError::from)?;

    let order_id = order.details().order_id().clone();
    state
        .order_service
        .delete_order(order_id)
        .await
        .map_err(ApiError::from)
        .map(|id | ApiResponseBody::new(StatusCode::OK, id))


}