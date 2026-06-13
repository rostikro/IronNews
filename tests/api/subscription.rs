use axum::body::Body;
use axum::http::{Request, header};
use iron_news::app;
use reqwest::StatusCode;
use rstest::rstest;
use tower::ServiceExt;

#[tokio::test]
async fn subscribe_returns_ok_for_valid_form_data() {
    let app = app();

    let body = "name=John%20Doe&email=john.doe%40example.com";
    let response = app
        .oneshot(
            Request::post("/subscribe")
                .header(
                    header::CONTENT_TYPE,
                    mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_success());
}

#[tokio::test]
#[rstest]
#[case::missing_email("name=John%20Doe")]
#[case::missing_name("email=john.doe%40example.com")]
#[case::missing_name_and_email("")]
async fn subscribe_returns_bad_request_for_invalid_form_data(#[case] body: &'static str) {
    let app = app();

    let response = app
        .oneshot(
            Request::post("/subscribe")
                .header(
                    header::CONTENT_TYPE,
                    mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
