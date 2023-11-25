pub mod routes;
pub mod services;

use axum::response::IntoResponse;
use serde::Serialize;

use self::services::auth::AuthError;
use crate::models::ModelError;

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
        println!("->> {:<12?} - into_response", self);
        match self {
            ApiError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg).into_response(),
            ApiError::UnprocessableEntity(msg) => {
                (axum::http::StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
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
