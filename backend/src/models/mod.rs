pub mod application;
pub mod company;
pub mod job;
pub mod user;

use serde::Serialize;

pub type Result<T> = core::result::Result<T, ModelError>;

#[derive(Debug, Serialize)]
pub enum ModelError {
    NotFound,
    #[serde(skip)]
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for ModelError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ModelError::NotFound,
            _ => ModelError::DatabaseError(err),
        }
    }
}
