use crate::{error::Result, models};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
	async fn fetch_user_by_id(&self, user_id: Uuid) -> Result<models::User>;
	async fn fetch_all_users(&self) -> Result<Vec<models::User>>;
	async fn add_user(&self, new_user: models::NewUser) -> Result<models::User>;
	async fn update_user(&self, user: models::User) -> Result<models::User>;
	async fn delete_user(&self, user_id: Uuid) -> Result<()>;
}
