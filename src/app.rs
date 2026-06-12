use crate::api::health_check;
use axum::Router;
use axum::routing::get;

/// Creates and returns a new `Router` instance.
pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}
