use axum::{Extension, Router};
use tower_http::cors::CorsLayer;

use sqlx::pool::Pool;
use sqlx::postgres::Postgres;

use crate::web;

pub struct MyApp {
    router: Router,
}

impl MyApp {
    pub async fn new(db_pool: Pool<Postgres>, cors: CorsLayer) -> Self {
        let api_routes = web::routes::configure_routes(db_pool.clone());

        let app = api_routes.layer(Extension(db_pool.clone())).layer(cors);

        MyApp { router: app }
    }

    pub fn router(&mut self) -> &mut Router {
        &mut self.router
    }
}
