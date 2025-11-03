use std::sync::Arc;

use crate::{
	app_state::AppState,
	handlers::{
		create_user_handler, delete_user_handler, get_user_handler, get_users_handler,
		update_user_handler,
	},
	services,
};
use axum::{routing::get, Router};

pub struct UserRoute {}

impl UserRoute {
	pub fn create_router(app_state: AppState<services::UserService>) -> Router {
		Router::new()
			.route("/users", get(get_users_handler).post(create_user_handler))
			.route(
				"/users/{id}",
				get(get_user_handler)
					.put(update_user_handler)
					.delete(delete_user_handler),
			)
			.with_state(Arc::new(app_state))
	}
}
