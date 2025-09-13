use crate::models::{ErrorResponse, ValidationErrorResponse};
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    Error(ErrorResponse),
    Validation(ValidationErrorResponse),
    Internal,
}

use std::fmt;

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Error(e) => write!(f, "Api error: {} - {}", e.code, e.message),
            ApiError::Validation(e) => write!(f, "Validation error: {} - {}", e.code, e.message),
            ApiError::Internal => write!(f, "Internal server error"),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Error(err) => HttpResponse::BadRequest().json(err),
            ApiError::Validation(err) => HttpResponse::UnprocessableEntity().json(err),
            ApiError::Internal => HttpResponse::InternalServerError().json(ErrorResponse {
                code: "internal.error".to_string(),
                message: "Internal server error".to_string(),
                description: "An unexpected error occurred".to_string(),
            }),
        }
    }
}
