use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header::ContentType;
use derive_more::Display;
use serde::Serialize;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal server error")]
    InternalServerError(String),
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),
    #[error("Couldn't find {0}")]
    NotFound(String),
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