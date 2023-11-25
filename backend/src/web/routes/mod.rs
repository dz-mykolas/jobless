pub mod application;
pub mod auth;
pub mod company;
pub mod job;

use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};
use tower_cookies::Cookies;

use crate::models::{
    application::ApplicationModel, company::CompanyModel, job::JobModel, ModelError,
};
use axum::{
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
};

use super::services::{
    auth::{AuthController, AuthError},
    jwt::get_claims,
};

pub type Result<T> = core::result::Result<T, AuthError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Employeer,
    User,
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "admin" => Role::Admin,
            "employeer" => Role::Employeer,
            "user" => Role::User,
            _ => Role::User,
        }
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Role::Admin, Role::Admin) => true,
            (Role::Employeer, Role::Employeer) => true,
            (Role::User, Role::User) => true,
            _ => false,
        }
    }
}

impl IntoResponse for ModelError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ModelError::NotFound => {
                (axum::http::StatusCode::NOT_FOUND, "NOT_FOUND").into_response()
            }
            ModelError::DatabaseError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
            )
                .into_response(),
        }
    }
}

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    // Extract cookies directly within the scope
    let jwt_cookie_value = if let Some(cookies) = req.extensions_mut().get::<Cookies>() {
        // Retrieve the JWT cookie value
        if let Some(cookie) = cookies.get("token") {
            cookie.value().to_string() // Get the value and convert to String
        } else {
            return Err(AuthError::Unauthorized);
        }
    } else {
        return Err(AuthError::Unauthorized);
    };

    // Process the JWT using the value
    let claims = get_claims(&jwt_cookie_value)?;
    let role = claims.user_role;

    // Insert role into request extensions
    req.extensions_mut().insert(role);

    // Continue processing the request
    Ok(next.run(req).await)
}

pub fn configure_routes(db_pool: Pool<Postgres>) -> Router {
    let api = configure_api_routes(db_pool.clone());
    let auth = configure_auth_routes(db_pool.clone());
    let routes = Router::new().nest("/api", api).nest("/auth", auth);

    routes
}

fn configure_api_routes(db_pool: Pool<Postgres>) -> Router {
    let company_controller = CompanyModel::new(db_pool.clone());
    let job_controller = JobModel::new(db_pool.clone());
    let application_controller = ApplicationModel::new(db_pool.clone());

    let api_routes = Router::new()
        .merge(company::routes(company_controller))
        .nest("/companies/:id", job::routes(job_controller))
        .nest(
            "/companies/:id/jobs/:id",
            application::routes(application_controller),
        )
        .route_layer(middleware::from_fn(auth));

    api_routes
}

fn configure_auth_routes(db_pool: Pool<Postgres>) -> Router {
    let auth_controller = AuthController::new(db_pool.clone());
    let auth_routes = Router::new().merge(auth::routes(auth_controller));

    auth_routes
}
