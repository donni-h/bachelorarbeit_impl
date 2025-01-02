use actix_web::http::StatusCode;
use actix_web::Responder;
use actix_web::web::{Data, Query};
use serde::Serialize;
use uuid::Uuid;
use crate::domain::models::order::Order;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::inbound::http::AppState;
use crate::inbound::http::handlers::{ApiError, ApiResponseBody};
use crate::inbound::http::responses::OrderResponseData;

pub struct GetByIdHttpRequestQuery(Uuid);

impl GetByIdHttpRequestQuery {
    fn into_domain(self) -> Uuid {
        self.0
    }
}



pub async fn get_order_by_id<OS: OrderService, PS: PaymentService>(
    state: Data<AppState<OS, PS>>,
    query: Query<GetByIdHttpRequestQuery>
) -> Result<impl Responder, ApiError> { 
    let domain_req = query.into_inner().into_domain();
    
    state
        .order_service
        .find_order_by_id(domain_req)
        .await
        .map_err(ApiError::from)
        .map(|order| {
            let response = OrderResponseData::from(&order);
            ApiResponseBody::new(StatusCode::OK, response)
        })
}


