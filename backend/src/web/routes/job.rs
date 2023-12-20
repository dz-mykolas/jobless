use crate::models::job::JobModel;
use crate::models::job::{Job, JobForCreate, JobForUpdate};
use crate::web::routes::Role;
use crate::web::services::auth::AuthError;
use crate::web::services::jwt::Claims;
use crate::web::{validate_id, ApiError, Result};

use axum::extract::{Path, State};
use axum::Extension;
use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn routes(controller: JobModel) -> Router {
    Router::new()
        // CRUD
        .route("/jobs/:job_id", get(get_job))
        .route("/jobs", post(create_job))
        .route("/jobs/:job_id", put(update_job))
        .route("/jobs/:job_id", delete(delete_job))
        // Extra
        .route("/jobs", get(get_jobs_by_company))
        // State
        .with_state(controller)
}

// CRUD
async fn create_job(
    Extension(claims): Extension<Claims>,
    Path(company_id): Path<i32>,
    State(controller): State<JobModel>,
    Json(payload): Json<JobForCreate>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - create_job", "HANDLER");

    let user_id = match claims.role {
        Role::Admin | Role::Employer => claims.sub.parse::<i32>().unwrap_or(0),
        _ => return Err(ApiError::AuthError(AuthError::Forbidden)),
    };

    let job = controller.create(payload, company_id, user_id).await?;

    Ok(Json(job))
}

async fn get_job(
    State(controller): State<JobModel>,
    Path((_company_id, job_id)): Path<(i32, i32)>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - get_job", "HANDLER");

    let job = controller.get(job_id).await?;

    Ok(Json(job))
}

async fn update_job(
    Extension(claims): Extension<Claims>,
    State(controller): State<JobModel>,
    Path((_company_id, job_id)): Path<(i32, i32)>,
    Json(payload): Json<JobForUpdate>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - update_job", "HANDLER");

    let user_id = match claims.role {
        Role::Admin => -1,
        Role::Employer => claims.sub.parse::<i32>().unwrap_or(0),
        _ => return Err(ApiError::AuthError(AuthError::Forbidden)),
    };

    validate_id(job_id)?;

    let job = controller.update(job_id, payload, user_id).await?;

    Ok(Json(job))
}

async fn delete_job(
    Extension(claims): Extension<Claims>,
    State(controller): State<JobModel>,
    Path((_company_id, job_id)): Path<(i32, i32)>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - delete_job", "HANDLER");

    let user_id = match claims.role {
        Role::Admin => -1,
        Role::Employer => claims.sub.parse::<i32>().unwrap_or(0),
        _ => return Err(ApiError::AuthError(AuthError::Forbidden)),
    };

    validate_id(job_id)?;

    let job = controller.delete(job_id, user_id).await?;

    Ok(Json(job))
}

// Extra
pub async fn get_all_jobs(State(controller): State<JobModel>) -> Result<Json<Vec<Job>>> {
    println!("->> {:<12} - get_all_jobs", "HANDLER");

    let jobs = controller.get_all().await?;

    Ok(Json(jobs))
}

async fn get_jobs_by_company(
    State(controller): State<JobModel>,
    Path(company_id): Path<i32>,
) -> Result<Json<Vec<Job>>> {
    println!("->> {:<12} - get_jobs_by_company", "HANDLER");

    let jobs = controller.get_by_company(company_id).await?;

    Ok(Json(jobs))
}
