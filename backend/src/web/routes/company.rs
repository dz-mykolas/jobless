use std::f32::consts::E;

use crate::models::company::{Company, CompanyForCreate, CompanyForUpdate, CompanyModel};
use crate::web::services::auth::AuthError;
use crate::web::{ApiError, Result, Validation};

use axum::extract::{Path, State};
use axum::Extension;
use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};

use super::Role;

pub fn routes(controller: CompanyModel) -> Router {
    Router::new()
        // CRUD
        .route("/companies", post(create_company))
        .route("/companies/:company_id", get(get_company))
        .route("/companies/:company_id", put(update_company))
        .route("/companies/:company_id", delete(delete_company))
        // Extra
        .route("/companies", get(get_companies))
        // State
        .with_state(controller)
}

/* CRUD-MAIN */
async fn create_company(
    Extension(role): Extension<Role>,
    State(controller): State<CompanyModel>,
    Json(payload): Json<CompanyForCreate>,
) -> Result<Json<Company>> {
    println!("->> {:<12} - create_company", "HANDLER");

    if role != Role::Admin {
        return Err(ApiError::AuthError(AuthError::Forbidden));
    }

    payload.validate()?;

    let company = controller.create(payload).await?;

    Ok(Json(company))
}

async fn get_company(
    State(controller): State<CompanyModel>,
    Path(id): Path<i32>,
) -> Result<Json<Company>> {
    println!("->> {:<12} - get_company", "HANDLER");

    validate_id(id)?;

    let company = controller.get(id).await?;

    Ok(Json(company))
}

async fn update_company(
    Extension(role): Extension<Role>,
    State(controller): State<CompanyModel>,
    Path(id): Path<i32>,
    Json(payload): Json<CompanyForUpdate>,
) -> Result<Json<Company>> {
    println!("->> {:<12} - update_company", "HANDLER");

    if role != Role::Admin {
        return Err(ApiError::AuthError(AuthError::Forbidden));
    }

    validate_id(id)?;
    payload.validate()?;

    let company = controller.update(id, payload).await?;

    Ok(Json(company))
}

async fn delete_company(
    Extension(role): Extension<Role>,
    State(controller): State<CompanyModel>,
    Path(id): Path<i32>,
) -> Result<Json<Company>> {
    println!("->> {:<12} - delete_company", "HANDLER");

    if role != Role::Admin {
        return Err(ApiError::AuthError(AuthError::Forbidden));
    }

    validate_id(id)?;

    let company = controller.delete(id).await?;

    Ok(Json(company))
}

/* CRUD-EXTRA */
async fn get_companies(State(controller): State<CompanyModel>) -> Result<Json<Vec<Company>>> {
    println!("->> {:<12} - get_companies", "HANDLER");

    let companies = controller.get_all().await?;

    Ok(Json(companies))
}

/* VALIDATION */
impl Validation for CompanyForCreate {
    fn validate(&self) -> Result<()> {
        validate_company_fields(Some(&self.name), Some(&self.address))?;

        Ok(())
    }
}

impl Validation for CompanyForUpdate {
    fn validate(&self) -> Result<()> {
        validate_company_fields(self.name.as_ref(), self.address.as_ref())?;

        Ok(())
    }
}

fn validate_company_fields(name: Option<&String>, address: Option<&String>) -> Result<()> {
    let mut errors = vec![];

    if let Some(name) = name {
        if name.len() < 3 {
            errors.push("name must be at least 3 characters long".to_string());
        }
    } else {
        errors.push("name is required".to_string());
    }

    if let Some(address) = address {
        if address.len() < 5 {
            errors.push("address must be at least 5 characters long".to_string());
        }
    } else {
        errors.push("address is required".to_string());
    }

    if !errors.is_empty() {
        return Err(ApiError::UnprocessableEntity(errors.join(", ")));
    } else {
        return Ok(());
    }
}

fn validate_id(id: i32) -> Result<()> {
    if id < 1 {
        return Err(ApiError::BadRequest(
            "id must be greater than 0".to_string(),
        ));
    } else {
        return Ok(());
    }
}
