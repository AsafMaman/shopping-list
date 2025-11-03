use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	NotFound(String),
	InvalidInput(String),
	InternalServerError(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::NotFound(msg) => write!(f, "{}", msg),
			Error::InvalidInput(msg) => write!(f, "{}", msg),
			Error::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
		}
	}
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		let status_code = match self {
			Error::NotFound(_) => StatusCode::NOT_FOUND,
			Error::InvalidInput(_) => StatusCode::BAD_REQUEST,
			Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
		};

		(status_code, self.to_string()).into_response()
	}
}
