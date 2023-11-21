
use crate::web::ApiError;

pub enum Error {
    ApiError(ApiError),
}