use application::user::queries::GetUserBySessionQuery;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
    Extension, RequestPartsExt,
};
use domain::uuid::Uuid;
use domain::{error::SmError, user::User};

use crate::state::AppState;

pub struct AuthUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Extension(state) = parts.extract::<Extension<AppState>>().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to extract AppState".to_string(),
            )
        })?;

        let headers = parts.extract::<HeaderMap>().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to extract headers".to_string(),
            )
        })?;

        // Get the Authorization header
        let auth_header = headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing authorization header".to_string(),
            ))?;

        // Check if it's a Bearer token and extract the value
        let bearer_token = auth_header.strip_prefix("Bearer ").ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid authorization header".to_string(),
        ))?;

        // Parse the bearer token as a UUID
        let session_id = Uuid::parse_str(bearer_token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid session ID".to_string()))?;

        // Create and handle the GetUserBySessionQuery
        let query = GetUserBySessionQuery {
            session: session_id,
        };
        let user = query
            .handle(&state.user_repository, &state.session_repository)
            .await
            .map_err(|e| match e {
                SmError::Session(_) => (StatusCode::UNAUTHORIZED, "Invalid session".to_string()),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error".to_string(),
                ),
            })?;

        Ok(AuthUser(user))
    }
}
