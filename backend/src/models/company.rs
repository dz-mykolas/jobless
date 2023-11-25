use crate::models::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Company {
    id: i32,
    name: String,
    address: String,
}

#[derive(Deserialize)]
pub struct CompanyForCreate {
    pub name: String,
    pub address: String,
}

#[derive(Deserialize)]
pub struct CompanyForUpdate {
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Clone)]
pub struct CompanyModel {
    pub pool: sqlx::PgPool,
}

impl CompanyModel {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl CompanyModel {
    pub async fn create(&self, company: CompanyForCreate) -> Result<Company> {
        let company = sqlx::query_as!(
            Company,
            r#"
                INSERT INTO 
                    company (name, address) 
                VALUES 
                    ($1, $2) 
                RETURNING *
            "#,
            &company.name,
            &company.address
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(company)
    }

    pub async fn get_all(&self) -> Result<Vec<Company>> {
        let companies = sqlx::query_as!(
            Company,
            r#"
                SELECT 
                    * 
                FROM 
                    company
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(companies)
    }

    pub async fn get(&self, id: i32) -> Result<Company> {
        let company = sqlx::query_as!(
            Company,
            r#"
                SELECT 
                    * 
                FROM 
                    company 
                WHERE 
                    id = $1
            "#,
            &id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(company)
    }

    pub async fn update(&self, id: i32, company: CompanyForUpdate) -> Result<Company> {
        let company = sqlx::query_as!(
            Company,
            r#"
                UPDATE 
                    company 
                SET 
                    name = COALESCE($1, name), 
                    address = COALESCE($2, address) 
                WHERE 
                    id = $3 
                RETURNING *
            "#,
            company.name.as_deref(),
            company.address.as_deref(),
            &id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(company)
    }

    pub async fn delete(&self, id: i32) -> Result<Company> {
        let company = sqlx::query_as!(
            Company,
            r#"
                DELETE FROM 
                    company 
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            &id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(company)
    }
}
