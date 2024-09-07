use domain::error::SmError;
use domain::message::{Message, MessageMetadata, MessageRepository};
use uuid::Uuid;

pub struct GetAllMessagesForUserQuery {
    pub recipient_id: Uuid,
}

impl GetAllMessagesForUserQuery {
    pub async fn handle(
        &self,
        message_repository: &impl MessageRepository,
    ) -> Result<Vec<(i64, MessageMetadata)>, SmError> {
        message_repository
            .list_messages(self.recipient_id, None)
            .await
    }
}

pub struct GetMessageByIdQuery {
    pub recipient_id: Uuid,
    pub message_id: i64,
}

impl GetMessageByIdQuery {
    pub async fn handle(
        &self,
        message_repository: &impl MessageRepository,
    ) -> Result<Option<Message>, SmError> {
        message_repository
            .get_message(self.recipient_id, self.message_id)
            .await
    }
}
