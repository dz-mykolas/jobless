use axum::response::IntoResponse;

pub type Result<T> = core::result::Result<T, LoginError>;

#[derive(Debug)]
pub enum LoginError {
    NotFound,
    IncorrectPassword,
    TooManyAttempts,
    InternalServerError(sqlx::Error),
}

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (axum::http::StatusCode::NOT_FOUND, "NOT_FOUND").into_response(),
            Self::IncorrectPassword => (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                "INCORRECT_PASSWORD",
            )
                .into_response(),
            Self::TooManyAttempts => (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                "TOO_MANY_ATTEMPTS",
            )
                .into_response(),
            Self::InternalServerError(err) => {
                println!("->> {:<12} - {err:?}", "INTO_RES");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                )
                    .into_response()
            }
        }
    }
}
