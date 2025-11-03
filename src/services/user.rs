use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
	error::{Error, Result},
	models, ports,
};

pub struct UserService {
	users: Arc<tokio::sync::RwLock<HashMap<Uuid, models::User>>>,
}
impl UserService {
	pub fn new() -> Self {
		let user1 = models::User {
			id: Uuid::parse_str("151b219c-7fe6-4622-8bd0-d9ed03031a5f").unwrap(),
			first_name: "Alice".to_string(),
			last_name: "Smith".to_string(),
			email: "alice@example.com".to_string(),
		};
		let user2 = models::User {
			id: Uuid::parse_str("151b219c-7fe6-4622-8bd0-d9ed03031a5e").unwrap(),
			first_name: "Bob".to_string(),
			last_name: "Johnson".to_string(),
			email: "bob@example.com".to_string(),
		};

		let mut users_map = HashMap::new();
		users_map.insert(user1.id, user1);
		users_map.insert(user2.id, user2);

		UserService {
			users: Arc::new(tokio::sync::RwLock::new(users_map)),
		}
	}
}

#[async_trait]
impl ports::UserService for UserService {
	async fn fetch_user_by_id(&self, user_id: Uuid) -> Result<models::User> {
		let user = self.users.read().await.get(&user_id).cloned();
		match user {
			Some(u) => Ok(u),
			None => Err(Error::NotFound(format!(
				"User with id {} not found",
				user_id
			))),
		}
	}

	async fn fetch_all_users(&self) -> Result<Vec<models::User>> {
		let users = self.users.read().await;
		Ok(users.values().cloned().collect())
	}

	async fn add_user(&self, new_user: models::NewUser) -> Result<models::User> {
		let user = models::User::from(new_user);
		self.users.write().await.insert(user.id, user.clone());
		Ok(user)
	}

	async fn update_user(&self, user: models::User) -> Result<models::User> {
		let mut users = self.users.write().await;
		match users.get_mut(&user.id) {
			Some(existing_user) => {
				*existing_user = user.clone();
				Ok(user)
			}
			None => Err(Error::NotFound(format!(
				"User with id {} not found",
				user.id
			))),
		}
	}
	async fn delete_user(&self, user_id: Uuid) -> Result<()> {
		match self.users.write().await.remove(&user_id) {
			Some(_) => Ok(()),
			None => Err(Error::NotFound(format!(
				"User with id {} not found",
				user_id
			))),
		}
	}
}
