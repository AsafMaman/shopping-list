use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{error::Result, models, ports};

pub struct UserService<T: ports::UserRepository> {
	// users: Arc<tokio::sync::RwLock<HashMap<Uuid, models::User>>>,
	repository: Arc<T>,
}

impl<T: ports::UserRepository> UserService<T> {
	pub fn new(repository: T) -> Self {
		UserService {
			repository: Arc::new(repository),
		}
	}
}

#[async_trait]
impl<T: ports::UserRepository> ports::UserService for UserService<T> {
	async fn fetch_user_by_id(&self, user_id: Uuid) -> Result<models::User> {
		self.repository.fetch_user_by_id(user_id).await
	}

	async fn fetch_all_users(&self) -> Result<Vec<models::User>> {
		self.repository.fetch_all_users().await
	}

	async fn add_user(&self, new_user: models::NewUser) -> Result<models::User> {
		self.repository.add_user(new_user).await
	}

	async fn update_user(&self, user: models::User) -> Result<models::User> {
		self.repository.update_user(user).await
	}

	async fn delete_user(&self, user_id: Uuid) -> Result<()> {
		self.repository.delete_user(user_id).await
	}
}
