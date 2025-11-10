use axum::http::StatusCode;
use axum_test::TestServer;
use mockall::predicate::eq;
use shopping_list::{
	models::{NewUser, User},
	ports::MockUserRepository,
	routes::UserRoute,
	services::UserService,
	AppState,
};
use uuid::Uuid;

mod helpers {
	use super::*;

	pub fn create_test_server(mock_repo: MockUserRepository) -> TestServer {
		let user_service = UserService::new(mock_repo);
		let app_state = AppState::new(user_service);
		let app = UserRoute::create_router(app_state);
		TestServer::new(app).unwrap()
	}

	// Test data factory
	pub fn create_test_user() -> User {
		User {
			id: Uuid::parse_str(&fakeit::unique::uuid_v4()).unwrap(),
			first_name: fakeit::name::first().to_string(),
			last_name: fakeit::name::last().to_string(),
			email: format!(
				"{}@{}",
				fakeit::name::first().to_lowercase(),
				fakeit::internet::domain_name()
			),
		}
	}

	pub fn create_test_new_user() -> NewUser {
		NewUser {
			first_name: fakeit::name::first().to_string(),
			last_name: fakeit::name::last().to_string(),
			email: format!(
				"{}@{}",
				fakeit::name::first().to_lowercase(),
				fakeit::internet::domain_name()
			),
		}
	}

	pub fn compare_users(actual: &User, expected: &User) {
		assert_eq!(actual.id, expected.id);
		assert_eq!(actual.first_name, expected.first_name);
		assert_eq!(actual.last_name, expected.last_name);
		assert_eq!(actual.email, expected.email);
	}
}

mod fetch_all_users {
	use super::*;

	#[tokio::test]
	async fn should_fetch_all_users_successfully() {
		// Create test users
		let users: Vec<User> = (0..5).map(|_| helpers::create_test_user()).collect();

		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		let expected_users = users.clone();
		mock_repo.expect_fetch_all_users().returning(move || {
			let users = expected_users.clone();
			Box::pin(async move { Ok(users) })
		});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.get("/users").await;

		// Assert response
		response.assert_status_ok();
		let response_users: Vec<User> = response.json();

		// Compare the lengths and individual fields since User doesn't derive PartialEq
		assert_eq!(response_users.len(), users.len());
		for (actual, expected) in response_users.iter().zip(users.iter()) {
			helpers::compare_users(actual, expected);
		}
	}

	#[tokio::test]
	async fn fetch_all_users_failed() {
		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		mock_repo.expect_fetch_all_users().returning(|| {
			Box::pin(async move {
				Err(shopping_list::error::Error::InternalServerError(
					"DB connection failed".to_string(),
				))
			})
		});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.get("/users").await;

		// Assert response
		response.assert_status_internal_server_error();
	}
}

mod get_user_by_id {
	use super::*;

	#[tokio::test]
	async fn should_return_user_successfully() {
		let user = helpers::create_test_user();

		let expected_user = user.clone();
		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		mock_repo.expect_fetch_user_by_id().returning(move |_| {
			let user = expected_user.clone();
			Box::pin(async move { Ok(user) })
		});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.get(&format!("/users/{}", user.id)).await;

		// Assert response
		response.assert_status_ok();
		let response_user: User = response.json();
		helpers::compare_users(&response_user, &user);
	}

	#[tokio::test]
	async fn should_return_user_not_found() {
		let user_id = Uuid::parse_str(&fakeit::unique::uuid_v4()).unwrap();
		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		mock_repo
			.expect_fetch_user_by_id()
			.with(eq(user_id))
			.returning(move |_| {
				Box::pin(async move {
					Err(shopping_list::error::Error::NotFound(format!(
						"User with id {user_id} not found"
					)))
				})
			});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.get(&format!("/users/{user_id}")).await;

		// Assert response
		response.assert_status_not_found();
	}
}

mod add_user {
	use shopping_list::error::Error;

	use super::*;

	#[tokio::test]
	async fn should_create_user_successfully() {
		let new_user = helpers::create_test_new_user();
		let created_user = helpers::create_test_user();

		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		let expected_user = created_user.clone();
		mock_repo.expect_add_user().returning(move |_| {
			let user = expected_user.clone();
			Box::pin(async move { Ok(user) })
		});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.post("/users").json(&new_user).await;
		// Assert response
		response.assert_status(StatusCode::CREATED);
		let response_user: shopping_list::models::User = response.json();
		helpers::compare_users(&response_user, &created_user);
	}

	#[tokio::test]
	async fn should_return_invalid_payload() {
		let invalid_user = NewUser {
			first_name: "".to_string(),
			last_name: "".to_string(),
			email: "invalid_email".to_string(),
		};

		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		mock_repo.expect_add_user().returning(move |_| {
			Box::pin(async move { Err(Error::InvalidInput("Invalid user data".to_string())) })
		});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.post("/users").json(&invalid_user).await;

		// Assert response
		response.assert_status(StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn should_handle_internal_server_error() {
		let new_user = helpers::create_test_new_user();

		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		mock_repo.expect_add_user().returning(move |_| {
			Box::pin(async move { Err(Error::InternalServerError("Database error".to_string())) })
		});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.post("/users").json(&new_user).await;

		// Assert response
		response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
	}
}

mod update_user {
	use super::*;

	#[tokio::test]
	async fn should_update_user_successfully() {
		let user_id = Uuid::parse_str(&fakeit::unique::uuid_v4()).unwrap();
		let updated_user = helpers::create_test_user();

		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		let expected_user = updated_user.clone();
		mock_repo.expect_update_user().returning(move |_| {
			let user = expected_user.clone();
			Box::pin(async move { Ok(user) })
		});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server
			.put(&format!("/users/{user_id}"))
			.json(&User {
				id: user_id,
				first_name: updated_user.first_name.clone(),
				last_name: updated_user.last_name.clone(),
				email: updated_user.email.clone(),
			})
			.await;

		// Assert response
		response.assert_status_ok();
		let response_user: shopping_list::models::User = response.json();
		helpers::compare_users(&response_user, &updated_user);
	}
	#[tokio::test]
	async fn should_return_user_not_found_on_update() {
		let user_id = Uuid::parse_str(&fakeit::unique::uuid_v4()).unwrap();
		let updated_user = helpers::create_test_user();

		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		mock_repo.expect_update_user().returning(move |_| {
			Box::pin(async move {
				Err(shopping_list::error::Error::NotFound(format!(
					"User with id {user_id} not found"
				)))
			})
		});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server
			.put(&format!("/users/{user_id}"))
			.json(&updated_user)
			.await;

		// Assert response
		response.assert_status_not_found();
	}
}

mod delete_user {
	use super::*;

	#[tokio::test]
	async fn should_delete_user_successfully() {
		let user_id = Uuid::parse_str(&fakeit::unique::uuid_v4()).unwrap();

		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		mock_repo
			.expect_delete_user()
			.with(eq(user_id))
			.returning(move |_| Box::pin(async move { Ok(()) }));

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.delete(&format!("/users/{user_id}")).await;

		// Assert response
		response.assert_status_no_content();
	}

	#[tokio::test]
	async fn should_return_user_not_found_on_delete() {
		let user_id = Uuid::parse_str(&fakeit::unique::uuid_v4()).unwrap();

		// Setup mock repository
		let mut mock_repo = MockUserRepository::new();
		mock_repo
			.expect_delete_user()
			.with(eq(user_id))
			.returning(move |_| {
				Box::pin(async move {
					Err(shopping_list::error::Error::NotFound(format!(
						"User with id {user_id} not found"
					)))
				})
			});

		// Create test server
		let server = helpers::create_test_server(mock_repo);

		// Make request
		let response = server.delete(&format!("/users/{user_id}")).await;

		// Assert response
		response.assert_status_not_found();
	}
}
