use std::env;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));

    tracing::info!("Listening on port {port}");
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    axum::serve(listener, app).await?;

    tracing::info!("Shutting down gracefully");

    Ok(())
}
