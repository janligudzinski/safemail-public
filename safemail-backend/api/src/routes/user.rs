use application::user::commands::{
    ActivateSessionCommand, RegisterUserCommand, RequestSessionCommand, UserCommandValidator,
};
use application::user::queries::GetUserByUsernameQuery;
use axum::extract::Path;
use axum::{Extension, Json};
use domain::error::UserError;
use domain::validate::Validate;
use domain::{session::Session, user::User};

use crate::extractors::AuthUser;
use crate::{error::ApiError, state::AppState};

#[axum::debug_handler]
pub async fn get_user(
    Extension(state): Extension<AppState>,
    Path(username): Path<String>,
) -> Result<Json<User>, ApiError> {
    let user = GetUserByUsernameQuery { username }
        .handle(&state.user_repository)
        .await?
        .ok_or(UserError::UserNotFound)?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn register_user(
    Extension(state): Extension<AppState>,
    Json(command): Json<RegisterUserCommand>,
) -> Result<Json<User>, ApiError> {
    UserCommandValidator(&state.cryptography_service).validate(&command)?;
    let user = command.handle(&state.user_repository).await?;
    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn request_session(
    Extension(state): Extension<AppState>,
    Json(command): Json<RequestSessionCommand>,
) -> Result<Json<Session>, ApiError> {
    let session = command
        .handle(&state.user_repository, &state.session_repository)
        .await?;
    Ok(Json(session))
}

#[axum::debug_handler]
pub async fn activate_session(
    Extension(state): Extension<AppState>,
    Json(command): Json<ActivateSessionCommand>,
) -> Result<(), ApiError> {
    command
        .handle(
            &state.user_repository,
            &state.session_repository,
            &state.cryptography_service,
        )
        .await?;
    Ok(())
}

#[axum::debug_handler]
pub async fn whoami(AuthUser(user): AuthUser) -> Json<User> {
    Json(user)
}
