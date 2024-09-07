use application::message::commands::{
    SendMessageWithOnetimeStampCommand, SendMessageWithOnetimeStampCommandDto,
    SendMessageWithPeriodicStampCommand, SendMessageWithPeriodicStampCommandDto,
};
use application::message::queries::*;
use axum::{http::StatusCode, Extension, Json};
use domain::message::MessageMetadata;

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

pub async fn get_all_messages(
    Extension(app_state): Extension<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<(i64, MessageMetadata)>>, ApiError> {
    let query = GetAllMessagesForUserQuery {
        recipient_id: user.id,
    };

    let messages = query.handle(&app_state.message_repository).await?;

    // Convert MessageMetadata to String for JSON serialization
    let messages = messages
        .into_iter()
        .map(|(id, metadata)| (id, metadata.0))
        .collect();

    Ok(Json(messages))
}

pub async fn get_message_by_id(
    Extension(app_state): Extension<AppState>,
    AuthUser(user): AuthUser,
    Path(message_id): Path<i64>,
) -> Result<Json<Option<Message>>, ApiError> {
    let query = GetMessageByIdQuery {
        recipient_id: user.id,
        message_id,
    };

    let message = query.handle(&app_state.message_repository).await?;

    Ok(Json(message))
}
