use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
	pub id: uuid::Uuid,
	pub first_name: String,
	pub last_name: String,
	pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewUser {
	pub first_name: String,
	pub last_name: String,
	pub email: String,
}

impl From<NewUser> for User {
	fn from(new_user: NewUser) -> Self {
		User {
			id: uuid::Uuid::new_v4(),
			first_name: new_user.first_name,
			last_name: new_user.last_name,
			email: new_user.email,
		}
	}
}
