use std::collections::{hash_map, HashMap};

use crate::{
	error::{Error, Result},
	models,
	ports::UserRepository,
};
use async_trait::async_trait;
use uuid::Uuid;

pub struct InMemoryUserRepository {
	users: std::sync::Mutex<std::collections::HashMap<uuid::Uuid, models::User>>,
}

impl InMemoryUserRepository {
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

		InMemoryUserRepository {
			users: std::sync::Mutex::new(users_map),
		}
	}
}

impl Default for InMemoryUserRepository {
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
	async fn fetch_user_by_id(&self, user_id: Uuid) -> Result<models::User> {
		let users = self.users.lock().unwrap();
		users
			.get(&user_id)
			.cloned()
			.ok_or(Error::NotFound(format!("User {user_id} not found")))
	}

	async fn fetch_all_users(&self) -> Result<Vec<models::User>> {
		let users = self.users.lock().unwrap();
		Ok(users.values().cloned().collect())
	}

	async fn add_user(&self, new_user: models::NewUser) -> Result<models::User> {
		let mut users = self.users.lock().unwrap();
		let user: models::User = new_user.into();
		users.insert(user.id, user.clone());
		Ok(user)
	}

	async fn update_user(&self, user: models::User) -> Result<models::User> {
		let mut users = self.users.lock().unwrap();

		if let hash_map::Entry::Occupied(mut e) = users.entry(user.id) {
			e.insert(user.clone());
		} else {
			return Err(Error::NotFound(format!("User {} not found", user.id)));
		}
		Ok(user)
	}

	async fn delete_user(&self, user_id: Uuid) -> Result<()> {
		let mut users = self.users.lock().unwrap();
		if users.remove(&user_id).is_some() {
			Ok(())
		} else {
			Err(Error::NotFound(format!("User {user_id} not found")))
		}
	}
}
