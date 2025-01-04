use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::error::JsonPayloadError;
use serde::Serialize;
use thiserror::Error;
use crate::domain::models::order::{DeleteOrderError, FindOrderError, UpdateOrderError};
use crate::domain::ports::payment_service::PaymentServiceError;

pub mod create_checkout;
pub mod success;
pub mod cancel;
pub mod get_by_id;
pub mod get_all_orders_for_user;
pub mod delete_by_id;
pub mod delete_all_orders;

#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorData {
    message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponseBody<T> {
    status_code: u16,
    data: T,
}

impl<T: Serialize> ApiResponseBody<T> {
    fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl<T: Serialize> Responder for ApiResponseBody<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        match serde_json::to_string(&self) {
            Ok(body) => HttpResponse::build(actix_web::http::StatusCode::from_u16(self.status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))
                .content_type("application/json")
                .body(body),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal server error")]
    InternalServerError(String),
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),
    #[error("Couldn't find {0}")]
    NotFound(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<FindOrderError> for ApiError {
    fn from(e: FindOrderError) -> Self {
        match e {
            FindOrderError::IdNotFound { id } => {
                Self::NotFound(format!("Order ID not found: {id}"))
            }
            FindOrderError::Unknown(_) => {
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<UpdateOrderError> for ApiError {
    fn from(e: UpdateOrderError) -> Self {
        match e {
            UpdateOrderError::NotFound => {
                Self::NotFound("Order not found".to_string())
            }
            UpdateOrderError::Unknown(_) => {
                Self::InternalServerError("Internal server error".to_string()) 
            }
        }
    }
}

impl From<PaymentServiceError> for ApiError {
    fn from(e: PaymentServiceError) -> Self {
        match e {
            PaymentServiceError::Unknown(_) => {
                Self::InternalServerError("Internal server error while processing payment status"
                    .to_string())
            }
            PaymentServiceError::InvalidSessionId(id) => {
                Self::NotFound(format!("Invalid session ID: {id}"))
            }
        }
    }
}

impl From<DeleteOrderError> for ApiError {
    fn from(e: DeleteOrderError) -> Self {
        ApiError::InternalServerError("Internal server error".to_string()) 
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = ApiResponseBody::new(self.status_code(), ApiErrorData {
            message: self.to_string(),
        });
        HttpResponse::build(self.status_code()).json(body)
    }
}

impl From<JsonPayloadError> for ApiError {
    fn from(err: JsonPayloadError) -> Self {
        let message = match err {
            JsonPayloadError::Overflow { limit: _ }   => "JSON payload is too large".to_string(),
            JsonPayloadError::ContentType => "Invalid content type".to_string(),
            JsonPayloadError::Deserialize(ref e) => format!("Deserialization error: {e}"),
            _ => "Invalid JSON payload".to_string(),
        };

        Self::UnprocessableEntity(message)
    }
}
