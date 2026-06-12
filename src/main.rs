use anyhow::Context;
use iron_news::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .context("failed to create a TCP listener")?;

    axum::serve(listener, app())
        .await
        .context("failed to serve the service")
}
