use axum::{middleware, response::{Response, IntoResponse}};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use crate::error::Error;

mod models;
mod web;

mod app;
mod db;
mod error;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let db_pool = db::connect_sqlx().await.unwrap();
    let cors = web::cors::configure_cors();

    let mut app = app::MyApp::new(db_pool, cors).await;

    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(
            app.router()
                .clone()
                .merge(web::routes_login::routes())
                .layer(middleware::map_response(main_response_mapper))
                .layer(CookieManagerLayer::new())
                .into_make_service(),
        )
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();

    let status = res.status();
    let status_error = res.extensions().get::<Error>();

    let error_response = error.as_ref().map(|status_code, error| {
        let body = json!({
            "status": status_code.as_u16(),
            "error": error.to_string(),
        });

        (status_code, Json(body)).into_response()
    });

    println!("->> {:<12} - status: {}", "RES_MAPPER", status);
    println!("->> {:<12} - headers: {:?}", "RES_MAPPER", error_response);

    error_response
}
