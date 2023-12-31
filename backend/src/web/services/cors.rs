use axum::http::header;
use axum::http::Method;
use tower_http::cors::CorsLayer;

pub fn configure_cors() -> CorsLayer {
    let origins = ["http://localhost:5173".parse().unwrap()];

    let methods = [Method::GET, Method::POST, Method::DELETE, Method::PUT];

    let headers = [header::CONTENT_TYPE, header::AUTHORIZATION];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods(methods)
        .allow_headers(headers);

    cors
}
