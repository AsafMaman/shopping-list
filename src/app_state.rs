use crate::ports::UserService;

pub struct AppState<U>
where
	U: UserService + Send + Sync + 'static,
{
	pub user_service: U,
}

impl<U> AppState<U>
where
	U: UserService + Send + Sync + 'static,
{
	pub fn new(user_service: U) -> Self {
		Self { user_service }
	}
}
