pub mod routes;
pub mod services;

use axum::response::IntoResponse;
use serde::Serialize;

use self::services::auth::AuthError;
use crate::{error::MainErrorResponse, models::ModelError};

pub type Result<T> = core::result::Result<T, ApiError>;

#[derive(Debug, Serialize)]
pub enum ApiError {
    BadRequest(String),
    UnprocessableEntity(String),
    ModelError(ModelError),
    AuthError(AuthError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::BadRequest(msg) => MainErrorResponse::new("BAD_REQUEST", &msg)
                .into_response(axum::http::StatusCode::BAD_REQUEST),
            ApiError::UnprocessableEntity(msg) => {
                MainErrorResponse::new("UNPROCESSABLE_ENTITY", &msg)
                    .into_response(axum::http::StatusCode::UNPROCESSABLE_ENTITY)
            }
            ApiError::ModelError(err) => err.into_response(),
            ApiError::AuthError(err) => err.into_response(),
        }
    }
}

impl From<ModelError> for ApiError {
    fn from(err: ModelError) -> Self {
        Self::ModelError(err)
    }
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        Self::AuthError(err)
    }
}

pub trait Validation {
    fn validate(&self) -> Result<()>;
}

pub fn validate_id(id: i32) -> Result<()> {
    if id < 1 {
        return Err(ApiError::BadRequest(
            "id must be greater than 0".to_string(),
        ));
    } else {
        return Ok(());
    }
}
