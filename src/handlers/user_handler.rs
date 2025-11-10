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

pub async fn get_users_handler<T>(State(app_state): State<Arc<AppState<T>>>) -> Result<Response>
where
	T: UserService + Send + Sync + 'static,
{
	tracing::debug!("Fetching all users");
	let users = app_state.user_service.fetch_all_users().await?;

	Ok((StatusCode::OK, Json(users)).into_response())
}

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
