use application::message::commands::{
    SendMessageWithOnetimeStampCommand, SendMessageWithOnetimeStampCommandDto,
    SendMessageWithPeriodicStampCommand, SendMessageWithPeriodicStampCommandDto,
};
use axum::{http::StatusCode, Extension, Json};

use crate::{error::ApiError, extractors::AuthUser, state::AppState};

#[axum::debug_handler]
pub async fn send_onetime(
    Extension(app_state): Extension<AppState>,
    AuthUser(user): AuthUser,
    Json(command_dto): Json<SendMessageWithOnetimeStampCommandDto>,
) -> Result<StatusCode, ApiError> {
    let command = SendMessageWithOnetimeStampCommand {
        content: command_dto.content,
        metadata: command_dto.metadata,
        signature: command_dto.signature,
        recipient_id: command_dto.recipient_id,
        stamp: command_dto.stamp,
        sender_id: user.id,
    };
    command
        .handle(
            &app_state.user_repository,
            &app_state.cryptography_service,
            &app_state.serialize_service,
            &app_state.tracker_repository,
            &app_state.system_key_repository,
            &app_state.message_repository,
        )
        .await?;
    Ok(StatusCode::OK)
}

#[axum::debug_handler]
pub async fn send_periodic(
    Extension(app_state): Extension<AppState>,
    AuthUser(user): AuthUser,
    Json(command_dto): Json<SendMessageWithPeriodicStampCommandDto>,
) -> Result<StatusCode, ApiError> {
    let command = SendMessageWithPeriodicStampCommand {
        content: command_dto.content,
        metadata: command_dto.metadata,
        signature: command_dto.signature,
        recipient_id: command_dto.recipient_id,
        stamp: command_dto.stamp,
        sender_id: user.id,
    };
    command
        .handle(
            &app_state.user_repository,
            &app_state.cryptography_service,
            &app_state.serialize_service,
            &app_state.message_repository,
        )
        .await?;
    Ok(StatusCode::OK)
}
