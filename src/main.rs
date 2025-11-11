use std::env;

use dotenvy::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tracing_subscriber::EnvFilter;

use shopping_list::{repositories, routes, services, AppState};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv().ok();

	let port = env::var("PORT").unwrap_or_else(|_| "3000".into());

	setup_logging();
	let db_pool = setup_database().await;

	// let user_repository = repositories::InMemoryUserRepository::new();
	let user_repository = repositories::SqliteUserRepository::new(db_pool);

	let user_service = services::UserService::new(user_repository);
	let app_state = AppState::new(user_service);

	let app = routes::UserRoute::create_router(app_state).merge(SwaggerUi::new("/swagger-ui").url(
		"/api-docs/openapi.json",
		shopping_list::handlers::ApiDoc::openapi(),
	));

	tracing::info!("Listening on port {port}");
	let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await?;
	axum::serve(listener, app).await?;

	tracing::info!("Shutting down gracefully");

	Ok(())
}

fn setup_logging() {
	let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
	tracing_subscriber::fmt().with_env_filter(filter).init();
}

async fn setup_database() -> SqlitePool {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	if !Sqlite::database_exists(&database_url)
		.await
		.expect("Failed to check database existence")
	{
		Sqlite::create_database(&database_url)
			.await
			.expect("Failed to create database");
		tracing::info!("Created database at {}", database_url);
	} else {
		tracing::info!("Database found at {}", database_url);
	}

	let pool = SqlitePool::connect(&database_url)
		.await
		.expect("Failed to connect to the database");

	// Migrate the database
	if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
		tracing::error!("Failed to run database migrations: {}", e);
	} else {
		tracing::info!("Database migrations applied successfully");
	}

	pool
}
