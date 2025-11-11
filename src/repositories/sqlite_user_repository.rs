use crate::{
	error::{Error, Result},
	models,
	ports::UserRepository,
};
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct SqliteUserRepository {
	db: SqlitePool, // users: std::sync::Mutex<std::collections::HashMap<uuid::Uuid, models::User>>,
}

impl SqliteUserRepository {
	pub fn new(db: SqlitePool) -> Self {
		Self { db }
	}
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
	async fn fetch_user_by_id(&self, user_id: Uuid) -> Result<models::User> {
		let row = sqlx::query("SELECT id, first_name, last_name, email FROM users WHERE id = ?")
			.bind(user_id)
			.fetch_one(&self.db)
			.await
			.map_err(|_| Error::NotFound(format!("User {user_id} not found")))?;

		Ok(models::User {
			id: row.get("id"),
			first_name: row.get("first_name"),
			last_name: row.get("last_name"),
			email: row.get("email"),
		})
	}

	async fn fetch_all_users(&self) -> Result<Vec<models::User>> {
		let users = sqlx::query("SELECT id, first_name, last_name, email FROM users")
			.fetch_all(&self.db)
			.await
			.map_err(|_| Error::NotFound("No users found".to_string()))?;
		Ok(
			users
				.into_iter()
				.map(|row| models::User {
					id: row.get("id"),
					first_name: row.get("first_name"),
					last_name: row.get("last_name"),
					email: row.get("email"),
				})
				.collect(),
		)
	}

	async fn add_user(&self, new_user: models::NewUser) -> Result<models::User> {
		let user_id = uuid::Uuid::new_v4();
		sqlx::query("INSERT INTO users (id, first_name, last_name, email) VALUES (?, ?, ?, ?)")
			.bind(user_id)
			.bind(&new_user.first_name)
			.bind(&new_user.last_name)
			.bind(&new_user.email)
			.execute(&self.db)
			.await
			.map_err(|e| Error::NotFound(format!("Failed to insert user: {}", e)))?;

		Ok(models::User {
			id: user_id,
			first_name: new_user.first_name,
			last_name: new_user.last_name,
			email: new_user.email,
		})
	}

	async fn update_user(&self, user: models::User) -> Result<models::User> {
		sqlx::query("UPDATE users SET first_name = ?, last_name = ?, email = ? WHERE id = ?")
			.bind(&user.first_name)
			.bind(&user.last_name)
			.bind(&user.email)
			.bind(user.id)
			.execute(&self.db)
			.await
			.map_err(|e| Error::NotFound(format!("Failed to update user: {}", e)))?;
		Ok(user)
	}

	async fn delete_user(&self, user_id: Uuid) -> Result<()> {
		sqlx::query("DELETE FROM users WHERE id = ?")
			.bind(user_id)
			.execute(&self.db)
			.await
			.map_err(|e| Error::NotFound(format!("Failed to delete user: {}", e)))?;
		Ok(())
	}
}
