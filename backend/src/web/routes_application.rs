use crate::models::application::{
    Application, ApplicationController, ApplicationForCreate, ApplicationForUpdate,
};
use crate::web::Result;

use axum::extract::{Path, State};
use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn routes(controller: ApplicationController) -> Router {
    Router::new()
        .route("/applications/:application_id", get(get_application))
        .route("/applications", post(create_application))
        .route("/applications/:application_id", put(update_application))
        .route("/applications/:application_id", delete(delete_application))
        // Extra
        .route("/applications", get(get_applications))
        // State
        .with_state(controller)
}

// CRUD
async fn create_application(
    State(controller): State<ApplicationController>,
    Json(payload): Json<ApplicationForCreate>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - create_application", "HANDLER");

    let application = controller.create(payload).await?;

    Ok(Json(application))
}

async fn get_application(
    State(controller): State<ApplicationController>,
    Path(id): Path<i32>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - get_application", "HANDLER");

    let application = controller.get(id).await?;

    Ok(Json(application))
}

async fn update_application(
    State(controller): State<ApplicationController>,
    Path(id): Path<i32>,
    Json(payload): Json<ApplicationForUpdate>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - update_application", "HANDLER");

    let application = controller.update(id, payload).await?;

    Ok(Json(application))
}

async fn delete_application(
    State(controller): State<ApplicationController>,
    Path(id): Path<i32>,
) -> Result<Json<Application>> {
    println!("->> {:<12} - delete_application", "HANDLER");

    let application = controller.delete(id).await?;

    Ok(Json(application))
}

// Extra
async fn get_applications(
    State(controller): State<ApplicationController>,
) -> Result<Json<Vec<Application>>> {
    println!("->> {:<12} - get_applications", "HANDLER");

    let applications = controller.get_all().await?;

    Ok(Json(applications))
}
