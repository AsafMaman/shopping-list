use std::env;

use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;

use crate::app_state::AppState;

mod app_state;
mod error;
mod handlers;
mod models;
mod ports;
mod repositories;
mod routes;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv().ok();

	let port = env::var("PORT").unwrap_or_else(|_| "3000".into());

	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let user_repository = repositories::InMemoryUserRepository::new();
	let user_service = services::UserService::new(user_repository);
	let app_state = AppState::new(user_service);

	let app = routes::UserRoute::create_router(app_state);

	tracing::info!("Listening on port {port}");
	let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await?;
	axum::serve(listener, app).await?;

	tracing::info!("Shutting down gracefully");

	Ok(())
}
