use crate::models::job::JobController;
use crate::models::model_job::{Job, JobForCreate, JobForUpdate};
use crate::web::Result;

use axum::extract::{Path, State};
use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn routes(controller: JobController) -> Router {
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
    State(controller): State<JobController>,
    Json(payload): Json<JobForCreate>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - create_job", "HANDLER");

    let job = controller.create(payload).await?;

    Ok(Json(job))
}

async fn get_job(
    State(controller): State<JobController>,
    Path(job_id): Path<i32>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - get_job", "HANDLER");

    let job = controller.get(job_id).await?;

    Ok(Json(job))
}

async fn update_job(
    State(controller): State<JobController>,
    Path(job_id): Path<i32>,
    Json(payload): Json<JobForUpdate>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - update_job", "HANDLER");

    let job = controller.update(job_id, payload).await?;

    Ok(Json(job))
}

async fn delete_job(
    State(controller): State<JobController>,
    Path(job_id): Path<i32>,
) -> Result<Json<Job>> {
    println!("->> {:<12} - delete_job", "HANDLER");

    let job = controller.delete(job_id).await?;

    Ok(Json(job))
}

// Extra
async fn get_jobs(State(controller): State<JobController>) -> Result<Json<Vec<Job>>> {
    println!("->> {:<12} - get_jobs", "HANDLER");

    let jobs = controller.get_all().await?;

    Ok(Json(jobs))
}
