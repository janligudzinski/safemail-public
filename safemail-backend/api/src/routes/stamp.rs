use application::stamp::commands::{
    IssueSystemStampCommand, IssueSystemStampCommandDto, RequestSystemStampIssueCommand,
};
use axum::{Extension, Json};
use domain::{stamp::OnetimeStamp, uuid::Uuid};

use crate::{error::ApiError, extractors::AuthUser, state::AppState};

#[axum::debug_handler]
pub async fn request_system_issue(
    Extension(state): Extension<AppState>,
    AuthUser(_user): AuthUser,
    Json(command): Json<RequestSystemStampIssueCommand>,
) -> Result<Json<Uuid>, ApiError> {
    let result = command
        .handle(&state.user_repository, &state.stamp_request_repository)
        .await?;
    Ok(Json(result))
}

#[axum::debug_handler]
pub async fn system_issue(
    Extension(state): Extension<AppState>,
    AuthUser(user): AuthUser,
    Json(command_dto): Json<IssueSystemStampCommandDto>,
) -> Result<Json<OnetimeStamp>, ApiError> {
    let command = IssueSystemStampCommand {
        sender_id: user.id,
        stamp_request_id: command_dto.stamp_request_id,
        proof_of_work: command_dto.proof_of_work,
    };
    let result = command
        .handle(
            &state.user_repository,
            &state.stamp_request_repository,
            &state.tracker_repository,
            &state.system_key_repository,
            &state.cryptography_service,
            &state.serialize_service,
        )
        .await?;
    Ok(Json(result))
}
