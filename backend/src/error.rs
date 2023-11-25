use serde::Serialize;

use crate::web::ApiError;

#[derive(Serialize)]
pub enum Error {
    ApiError(ApiError),
}
