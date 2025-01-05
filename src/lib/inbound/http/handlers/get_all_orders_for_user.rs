use actix_web::http::StatusCode;
use actix_web::Responder;
use actix_web::web::Data;
use crate::domain::models::order_details::UserName;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::inbound::http::AppState;
use crate::inbound::http::extractors::auth::KeycloakToken;
use crate::inbound::http::handlers::{ApiError, ApiResponseBody};
use crate::inbound::http::responses::OrderResponseData;


#[utoipa::path(
    get,
    path="/api/payment/allordersforuser",

    responses(
    (status = 200, description = "order", body = Vec<OrderResponseData>)
    )
)]
pub async fn get_all_orders_for_user<OS: OrderService, PS: PaymentService>(
    token: KeycloakToken,
    state: Data<AppState<OS, PS>>
) -> Result<impl Responder, ApiError>{
    let username_request = token.claims().preferred_username();
    let username = UserName::new(username_request);

    state.order_service
        .find_orders_by_username(&username)
        .await
        .map_err(ApiError::from)
        .map(|orders| {
            let response_data: Vec<OrderResponseData> = orders
            .iter()
            .map(OrderResponseData::from)
            .collect();

            ApiResponseBody::new(StatusCode::OK, response_data)
        })
}