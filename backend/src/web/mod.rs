pub mod cors;
pub mod routes_login;

pub const AUTH_TOKEN: &str = "auth-token";

use axum::{Router, Json};

use axum::response::IntoResponse;
use sqlx::pool::Pool;
use sqlx::postgres::Postgres;

use crate::models::application::ApplicationController;
use crate::models::company::CompanyController;
use crate::models::model_job::JobController;

/* ROUTES */
/* --------------------------------------- */
pub mod routes_application;
pub mod routes_company;
pub mod routes_job;
pub mod routes_user;

pub use routes_application as application;
pub use routes_company as company;
pub use routes_job as job;
pub use routes_user as user;
/* SERVICES */
/* --------------------------------------- */
pub mod service_login;

pub use service_login as login;
/* --------------------------------------- */

pub fn configure_routes(db_pool: Pool<Postgres>) -> Router {
    let api = configure_api_routes(db_pool);
    let routes = Router::new().nest("/api", api);

    routes
}

fn configure_api_routes(db_pool: Pool<Postgres>) -> Router {
    let company_controller = CompanyController::new(db_pool.clone());
    let job_controller = JobController::new(db_pool.clone());
    let application_controller = ApplicationController::new(db_pool.clone());

    let api_routes = Router::new()
        .merge(company::routes(company_controller))
        .nest("/companies/:id", job::routes(job_controller))
        .nest(
            "/companies/:id/jobs/:id",
            application::routes(application_controller),
        );

    api_routes
}

pub type Result<T> = core::result::Result<T, ApiError>;

pub enum ApiError {
    NotFound,
    UnprocessableEntity(String),
    BadRequest(String),
    InternalServerError(sqlx::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::InternalServerError(err),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (axum::http::StatusCode::NOT_FOUND, "NOT_FOUND").into_response(),
            Self::UnprocessableEntity(msg) => {
                (axum::http::StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
            }
            Self::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, Json(msg)).into_response(),
            Self::InternalServerError(err) => {
                println!("->> {:<12} - {err:?}", "INTO_RES");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                )
                    .into_response()
            }
        }
    }
}

pub trait Validation {
    fn validate(&self) -> Result<()>;
}
