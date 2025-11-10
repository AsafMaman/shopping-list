use axum::{
	extract::{Path, State},
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use std::sync::Arc;

use crate::{
	app_state::AppState,
	error::{Error, Result},
	models,
	ports::UserService,
};

#[derive(utoipa::OpenApi)]
#[openapi(
	info(
		title = "User Management API",
		version = "1.0.0",
		description = "API for managing users",
	),
	paths(
		get_users_handler,
		get_user_handler,
		create_user_handler,
		update_user_handler,
		delete_user_handler,
	),
	components(schemas(models::User, models::NewUser)),
	tags(
		(name = "User Management", description = "APIs for managing users")
	)
)]
pub struct ApiDoc;

#[utoipa::path(
		get,
		path = "/users",
		responses(
				(status = 200, description = "List all users", body = [models::User]),
		)
)]
pub async fn get_users_handler<T>(State(app_state): State<Arc<AppState<T>>>) -> Result<Response>
where
	T: UserService + Send + Sync + 'static,
{
	tracing::debug!("Fetching all users");
	let users = app_state.user_service.fetch_all_users().await?;

	Ok((StatusCode::OK, Json(users)).into_response())
}

#[utoipa::path(
		get,
		path = "/users/{id}",
		responses(
				(status = 200, description = "Get user by ID", body = models::User),
				(status = 400, description = "Invalid UUID format"),
				(status = 404, description = "User not found"),
		),
		params(
				("id" = String, Path, description = "UUID of the user to fetch"),
		)
)]
pub async fn get_user_handler<T>(
	State(app_state): State<Arc<AppState<T>>>,
	Path(user_id): Path<String>,
) -> Result<Response>
where
	T: UserService + Send + Sync + 'static,
{
	tracing::debug!("Fetching user with ID: {user_id}");
	let uuid = uuid::Uuid::parse_str(&user_id)
		.map_err(|_| Error::InvalidInput("Invalid UUID format".to_string()))?;

	let user = app_state.user_service.fetch_user_by_id(uuid).await?;
	Ok((StatusCode::OK, Json(user)).into_response())
}

#[utoipa::path(
		post,
		path = "/users",
		request_body = models::NewUser,
		responses(
				(status = 201, description = "User created successfully", body = models::User),
				(status = 400, description = "Invalid input data"),
		),
)]
pub async fn create_user_handler<T>(
	State(app_state): State<Arc<AppState<T>>>,
	Json(payload): Json<models::NewUser>,
) -> Response
where
	T: UserService + Send + Sync + 'static,
{
	tracing::debug!("Creating user");
	match app_state.user_service.add_user(payload).await {
		Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
		Err(err) => err.into_response(),
	}
}

#[utoipa::path(
		put,
		path = "/users/{id}",
		request_body = models::NewUser,
		responses(
				(status = 200, description = "User updated successfully", body = models::User),
				(status = 400, description = "Invalid UUID format or input data"),
				(status = 404, description = "User not found"),
		),
		params(
				("id" = String, Path, description = "UUID of the user to update"),
		)
)]
pub async fn update_user_handler<T>(
	State(app_state): State<Arc<AppState<T>>>,
	Path(user_id): Path<String>,
	Json(payload): Json<models::NewUser>,
) -> Result<Response>
where
	T: UserService + Send + Sync + 'static,
{
	tracing::debug!("Updating user with ID: {user_id}");
	let uuid = uuid::Uuid::parse_str(&user_id)
		.map_err(|_| Error::InvalidInput("Invalid UUID format".to_string()))?;

	let user = models::User {
		id: uuid,
		first_name: payload.first_name,
		last_name: payload.last_name,
		email: payload.email,
	};
	let user = app_state.user_service.update_user(user).await?;

	Ok((StatusCode::OK, Json(user)).into_response())
}

#[utoipa::path(
		delete,
		path = "/users/{id}",
		responses(
				(status = 204, description = "User deleted successfully"),
				(status = 400, description = "Invalid UUID format"),
				(status = 404, description = "User not found"),
		),
		params(
				("id" = String, Path, description = "UUID of the user to delete"),
		)
)]
pub async fn delete_user_handler<T>(
	State(app_state): State<Arc<AppState<T>>>,
	Path(user_id): Path<String>,
) -> Result<Response>
where
	T: UserService + Send + Sync + 'static,
{
	tracing::debug!("Deleting user with ID: {user_id}");
	let uuid = uuid::Uuid::parse_str(&user_id)
		.map_err(|_| Error::InvalidInput("Invalid UUID format".to_string()))?;

	app_state.user_service.delete_user(uuid).await?;
	Ok(StatusCode::NO_CONTENT.into_response())
}
