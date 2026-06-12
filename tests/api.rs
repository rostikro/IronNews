use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use iron_news::app;
use tower::ServiceExt;

#[tokio::test]
async fn health_check() {
    let app = app();

    let response = app
        .oneshot(Request::get("/health_check").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}
