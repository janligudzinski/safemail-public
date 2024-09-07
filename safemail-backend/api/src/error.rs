use std::fmt::Display;

use axum::{body::Body, http::StatusCode, response::IntoResponse};
use domain::error::{SmError, UserError, ValidationError};

#[derive(Debug)]
pub struct ApiError(pub SmError);
impl From<SmError> for ApiError {
    fn from(e: SmError) -> Self {
        Self(e)
    }
}
impl From<ValidationError> for ApiError {
    fn from(e: ValidationError) -> Self {
        Self(SmError::Validation(e))
    }
}
impl From<UserError> for ApiError {
    fn from(e: UserError) -> Self {
        Self(SmError::User(e))
    }
}
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self.0 {
            SmError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SmError::User(e) => match e {
                UserError::UserNotFound => StatusCode::NOT_FOUND,
                UserError::UserAlreadyExists => StatusCode::CONFLICT,
                UserError::InvalidCredentials => StatusCode::UNAUTHORIZED,
                UserError::InvalidUsername => StatusCode::BAD_REQUEST,
                UserError::InvalidPassword => StatusCode::BAD_REQUEST,
                UserError::InvalidPublicKey => StatusCode::BAD_REQUEST,
            },
            SmError::Validation(_) => StatusCode::BAD_REQUEST,
            SmError::Cryptography(_) => StatusCode::BAD_REQUEST,
            SmError::Session(_) => StatusCode::UNAUTHORIZED,
            SmError::Stamp(_) => StatusCode::UNAUTHORIZED,
        };
        axum::response::Response::builder()
            .status(status)
            .body(Body::from(self.0.to_string()))
            .unwrap()
    }
}
