use iron_news::app;

#[tokio::test]
async fn health_check() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let _ = axum::serve(listener, app()).await;
    });

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{addr}/health_check"))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
