use axum::{
    middleware,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
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
    // Set environment variables from .env file.
    dotenv::dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let db_pool = db::connect_sqlx().await.unwrap();
    let cors = web::services::cors::configure_cors();

    let mut app = app::MyApp::new(db_pool, cors).await;

    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(
            app.router()
                .clone()
                .layer(middleware::map_response(main_response_mapper))
                .layer(CookieManagerLayer::new())
                .into_make_service(),
        )
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let status = res.status();
    let status_error = res.extensions().get::<Error>();

    let error_response = match status_error {
        Some(error) => {
            let body = json!({
                "status": status.as_u16(),
                "error": error,
            });

            (status, Json(body)).into_response()
        }
        None => res,
    };

    // let error_response = status_error.as_ref().map(|status_code, error| {
    //     let body = json!({
    //         "status": status_code.as_u16(),
    //         "error": error.to_string(),
    //     });

    //     (status_code, Json(body)).into_response()
    // });

    println!("->> {:<12} - status: {}", "RES_MAPPER", status);
    println!("->> {:<12} - headers: {:?}", "RES_MAPPER", error_response);
    println!(
        "->> {:<12} - current_time: {:?}",
        "RES_MAPPER",
        chrono::Local::now()
    );
    println!();

    error_response
}
