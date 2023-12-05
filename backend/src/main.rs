use axum::{middleware, response::Response};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

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
    println!("->> {:<12} - status: {}", "RES_MAPPER", &res.status());
    println!(
        "->> {:<12} - current_time: {:?}",
        "RES_MAPPER",
        chrono::Local::now()
    );
    println!();

    res
}
