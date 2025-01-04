use std::ops::Deref;
use actix_web::{web, Responder};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use serde::Deserialize;
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::domain::models::order::{CreateOrderError, CreateOrderRequest};
use crate::domain::models::order_details::UserName;
use crate::domain::models::order_item::{CreateOrderItemRequest, Price, PriceError, ProductName};
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::inbound::http::AppState;
use crate::inbound::http::extractors::auth::KeycloakToken;
use crate::inbound::http::handlers::{ApiError, ApiResponseBody};

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderHttpRequestBody {
    items: Vec<CreateOrderItemHttpRequestBody>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderItemHttpRequestBody {
    name: String,
    item_price: f64,
    plant_id: Uuid,
}

impl TryFrom<CreateOrderItemHttpRequestBody> for CreateOrderItemRequest {
    type Error = PriceError;

    fn try_from(item: CreateOrderItemHttpRequestBody) -> Result<Self, Self::Error> {
        let price = Price::new(item.item_price)?;
        let product_name = ProductName::new(&item.name);

        Ok(Self::new(product_name, item.plant_id, price))
    }
}

#[derive(Debug, Clone, Error)]
enum ParseCreateOrderHttpRequestError {
    #[error(transparent)]
    Price(#[from] PriceError),
}

impl From<ParseCreateOrderHttpRequestError> for ApiError {
    fn from(e: ParseCreateOrderHttpRequestError) -> Self {
        let message = match e {
            ParseCreateOrderHttpRequestError::Price(e) =>
                "Price sent is invalid.".to_string(),
        };

        Self::UnprocessableEntity(message)
    }
}

impl From<CreateOrderError> for ApiError {
    fn from(e: CreateOrderError) -> Self {
        match e {
            CreateOrderError::NoItems => {
                Self::UnprocessableEntity("No items were supplied".to_string())
            }
            CreateOrderError::Unknown(e) => {
                println!("{}", e);
                Self::UnprocessableEntity("Internal server error".to_string())
            }
        }
    }
}

impl CreateOrderHttpRequestBody {
    fn try_into_domain(self, token: &KeycloakToken) -> Result<CreateOrderRequest, ParseCreateOrderHttpRequestError> {
        let username = UserName::new(token.claims().preferred_username());
        let items = self
            .items
            .into_iter()
            .map(CreateOrderItemRequest::try_from)
            .collect::<Result<_, _>>()?;


        Ok(CreateOrderRequest::new(username, items))
    }
}
#[utoipa::path(
  post,
  path="/api/payment/create-checkout-session",
  request_body=CreateOrderHttpRequestBody,
  responses(
    (status = 201, description = "Successfully created session", body = String)
  )
)]
pub async fn create_checkout<OS: OrderService, PS: PaymentService>(
    state: web::Data<AppState<OS, PS>>,
    body: Json<CreateOrderHttpRequestBody>,
    token: KeycloakToken,
) -> Result<ApiResponseBody<String>, ApiError> {
    println!("{body:#?}");
    let domain_req = body.into_inner().try_into_domain(&token)?;

    state
        .order_service
        .create_order(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref checkout_url| ApiResponseBody::new(StatusCode::CREATED, checkout_url.to_string()))
}