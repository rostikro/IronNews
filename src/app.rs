use crate::api::{health_check, subscribe};
use axum::Router;
use axum::routing::{get, post};

/// Creates and returns a new `Router` instance.
pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
}
