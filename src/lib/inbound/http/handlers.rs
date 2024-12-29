use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::error::JsonPayloadError;
use derive_more::Display;
use serde::Serialize;
use thiserror::Error;

pub mod create_checkout;

#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorData {
    message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponseBody<T: Serialize> {
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

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
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
            JsonPayloadError::Deserialize(ref e) => format!("Deserialization error: {}", e),
            _ => "Invalid JSON payload".to_string(),
        };

        ApiError::UnprocessableEntity(message)
    }
}