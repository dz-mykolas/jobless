use crate::models::application::{
    Application, ApplicationForCreate, ApplicationForUpdate, ApplicationModel,
};
use crate::web::routes::Role;
use crate::web::services::auth::AuthError;
use crate::web::services::jwt::Claims;
use crate::web::{ApiError, Result};

use axum::extract::{Path, State};
use axum::Extension;
use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn routes(controller: ApplicationModel) -> Router {
    Router::new()
        // CRUD
        .route("/applications/:application_id", get(get_application))
        .route("/applications", post(create_application))
        .route("/applications/:application_id", put(update_application))
        .route("/applications/:application_id", delete(delete_application))
        // Extra
        .route("/applications", get(get_applications_by_job))
        .route(
            "/applications/:application_id/reject",
            put(reject_application),
        )
        .route(
            "/applications/:application_id/accept",
            put(accept_application),
        )
        // State
        .with_state(controller)
}

// CRUD
async fn create_application(
    Extension(claims): Extension<Claims>,
    Path((_company_id, job_id)): Path<(i32, i32)>,
    State(controller): State<ApplicationModel>,
    Json(payload): Json<ApplicationForCreate>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - create_application", "HANDLER");

    // BAD: This is a workaround to get the user_id
    let user_id = claims.sub.parse::<i32>().unwrap_or(0);

    let application = controller.create(payload, job_id, user_id).await?;

    Ok(Json(application))
}

async fn get_application(
    Extension(claims): Extension<Claims>,
    State(controller): State<ApplicationModel>,
    Path(id): Path<i32>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - get_application", "HANDLER");

    // BAD: This is a workaround to get the user_id
    let user_id = match claims.role {
        Role::Admin => -1,
        Role::User | Role::Employer => claims.sub.parse::<i32>().unwrap_or(0),
    };

    let application = controller.get(id, user_id).await?;

    Ok(Json(application))
}

async fn update_application(
    Extension(claims): Extension<Claims>,
    State(controller): State<ApplicationModel>,
    Path((_company_id, _job_id, id)): Path<(i32, i32, i32)>,
    Json(payload): Json<ApplicationForUpdate>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - update_application", "HANDLER");

    // BAD: This is a workaround to get the user_id
    let user_id = claims.sub.parse::<i32>().unwrap_or(0);

    let application = controller.update(id, payload, user_id).await?;

    Ok(Json(application))
}

async fn delete_application(
    Extension(claims): Extension<Claims>,
    State(controller): State<ApplicationModel>,
    Path((_company_id, _job_id, id)): Path<(i32, i32, i32)>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - delete_application", "HANDLER");

    // BAD: This is a workaround to get the user_id
    let user_id = claims.sub.parse::<i32>().unwrap_or(0);

    let application = controller.delete(id, user_id).await?;

    Ok(Json(application))
}

pub async fn get_applications_by_user(
    Extension(claims): Extension<Claims>,
    State(controller): State<ApplicationModel>,
) -> Result<Json<Vec<Application>>> {
    println!("->> {:<12} - get_applications_by_user", "HANDLER");

    // BAD: This is a workaround to get the user_id
    let user_id = claims.sub.parse::<i32>().unwrap_or(0);

    let applications = controller.get_by_user(user_id).await?;

    Ok(Json(applications))
}

// Extra
async fn get_applications(
    Extension(claims): Extension<Claims>,
    State(controller): State<ApplicationModel>,
) -> Result<Json<Vec<Application>>> {
    println!("->> {:<12} - get_applications", "HANDLER");

    if claims.role != Role::Admin {
        return Err(ApiError::AuthError(AuthError::Forbidden));
    }

    let applications = controller.get_all().await?;

    Ok(Json(applications))
}

async fn get_applications_by_job(
    Extension(claims): Extension<Claims>,
    State(controller): State<ApplicationModel>,
    Path((_company_id, job_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<Application>>> {
    println!("->> {:<12} - get_applications_by_job", "HANDLER");

    let user_id = match claims.role {
        Role::Admin => -1,
        Role::Employer => claims.sub.parse::<i32>().unwrap_or(0),
        _ => return Err(ApiError::AuthError(AuthError::Forbidden)),
    };

    let applications = controller.get_by_job(job_id, user_id).await?;

    Ok(Json(applications))
}

async fn reject_application(
    Extension(claims): Extension<Claims>,
    State(controller): State<ApplicationModel>,
    Path((_company_id, _job_id, application_id)): Path<(i32, i32, i32)>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - reject_application", "HANDLER");

    let user_id = match claims.role {
        Role::Admin => -1,
        Role::Employer => claims.sub.parse::<i32>().unwrap_or(0),
        _ => return Err(ApiError::AuthError(AuthError::Forbidden)),
    };

    let application = controller.reject(application_id, user_id).await?;

    Ok(Json(application))
}

async fn accept_application(
    Extension(claims): Extension<Claims>,
    State(controller): State<ApplicationModel>,
    Path((_company_id, _job_id, application_id)): Path<(i32, i32, i32)>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - accept_application", "HANDLER");

    let user_id = match claims.role {
        Role::Admin => -1,
        Role::Employer => claims.sub.parse::<i32>().unwrap_or(0),
        _ => return Err(ApiError::AuthError(AuthError::Forbidden)),
    };

    let application = controller.accept(application_id, user_id).await?;

    Ok(Json(application))
}
