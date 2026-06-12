use axum::http::StatusCode;

/// Health check endpoint handler.
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
