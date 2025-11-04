use crate::ports::UserService;

pub struct AppState<T>
where
	T: UserService + Send + Sync + 'static,
{
	pub user_service: T,
}

impl<T> AppState<T>
where
	T: UserService + Send + Sync + 'static,
{
	pub fn new(user_service: T) -> Self {
		Self { user_service }
	}
}
