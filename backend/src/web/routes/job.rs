use crate::models::job::JobModel;
use crate::models::job::{Job, JobForCreate, JobForUpdate};
use crate::web::routes::Role;
use crate::web::services::auth::AuthError;
use crate::web::{ApiError, Result};

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
        .route("/jobs", get(get_jobs))
        // State
        .with_state(controller)
}

// CRUD
async fn create_job(
    Extension(role): Extension<Role>,
    State(controller): State<JobModel>,
    Json(payload): Json<JobForCreate>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - create_job", "HANDLER");

    if role != Role::Employer || role != Role::Admin {
        return Err(ApiError::AuthError(AuthError::Forbidden));
    }

    let job = controller.create(payload).await?;

    Ok(Json(job))
}

async fn get_job(State(controller): State<JobModel>, Path(job_id): Path<i32>) -> Result<Json<Job>> {
    println!("->> {:<12} - get_job", "HANDLER");

    let job = controller.get(job_id).await?;

    Ok(Json(job))
}

async fn update_job(
    Extension(role): Extension<Role>,
    State(controller): State<JobModel>,
    Path(job_id): Path<i32>,
    Json(payload): Json<JobForUpdate>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - update_job", "HANDLER");

    if role != Role::Employer || role != Role::Admin {
        return Err(ApiError::AuthError(AuthError::Forbidden));
    }

    let job = controller.update(job_id, payload).await?;

    Ok(Json(job))
}

async fn delete_job(
    Extension(role): Extension<Role>,
    State(controller): State<JobModel>,
    Path(job_id): Path<i32>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - delete_job", "HANDLER");

    if role != Role::Employer || role != Role::Admin {
        return Err(ApiError::AuthError(AuthError::Forbidden));
    }

    let job = controller.delete(job_id).await?;

    Ok(Json(job))
}

// Extra
async fn get_jobs(State(controller): State<JobModel>) -> Result<Json<Vec<Job>>> {
    println!("->> {:<12} - get_jobs", "HANDLER");

    let jobs = controller.get_all().await?;

    Ok(Json(jobs))
}
