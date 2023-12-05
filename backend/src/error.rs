use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

use crate::web::ApiError;

#[derive(Serialize)]
pub struct MainErrorResponse {
    error: String,
    message: String,
}

impl MainErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        MainErrorResponse {
            error: error.to_string(),
            message: message.to_string(),
        }
    }

    pub fn into_response(self, status: StatusCode) -> Response {
        let body = json!(self).to_string();
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize)]
pub enum Error {
    ApiError(ApiError),
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Self::ApiError(err)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("Error: {:?}", self);
        match self {
            Error::ApiError(err) => err.into_response(),
        }
    }
}
