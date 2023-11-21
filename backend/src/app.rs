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
        let api_routes = web::configure_routes(db_pool.clone());

        let app = api_routes.layer(Extension(db_pool.clone())).layer(cors);

        MyApp { router: app }

        // let auth_routes = Router::new()
        //     .route("/login", axum::handler::post(controllers::auth::login))
        //     .route("/register", axum::handler::post(controllers::auth::register));
    }

    pub fn router(&mut self) -> &mut Router {
        &mut self.router
    }
}
