use domain::{
    crypto::CryptographyService,
    error::{CryptographyError, SmError, StampError, UserError},
    message::{MessageMetadata, MessageRepository},
    onetime_stamp::OneTimeStampTrackerRepository,
    serialize::SerializeService,
    stamp::PeriodicStamp,
    system_key::SystemKeyRepository,
    user::UserRepository,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    stamp::commands::{VerifyOnetimeStampCommand, VerifyPeriodicStampCommand},
    user::queries::GetUserByIdQuery,
};

#[derive(Deserialize)]
pub struct SendMessageWithPeriodicStampCommandDto {
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub content: String,
    pub metadata: String,
    pub signature: String,
    pub stamp: PeriodicStamp,
}
pub struct SendMessageWithPeriodicStampCommand {
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub content: String,
    pub metadata: String,
    pub signature: String,
    pub stamp: PeriodicStamp,
}

impl SendMessageWithPeriodicStampCommand {
    pub async fn handle(
        self,
        user_repository: &impl UserRepository,
        cryptography_service: &impl CryptographyService,
        serialize_service: &impl SerializeService,
        message_repository: &impl MessageRepository,
    ) -> Result<(), SmError> {
        let sender = match (GetUserByIdQuery {
            user_id: self.sender_id,
        })
        .handle(user_repository)
        .await?
        {
            Some(sender) => sender,
            None => return Err(UserError::UserNotFound.into()),
        };

        let stamp_valid = VerifyPeriodicStampCommand(self.stamp)
            .handle(user_repository, cryptography_service, serialize_service)
            .await?;
        if !stamp_valid {
            return Err(StampError::InvalidStamp.into());
        }

        let signature_valid = cryptography_service.validate_signature(
            &format!("{}\n{}", self.metadata, self.content),
            &self.signature,
            &sender.public_verify_key,
        );
        if !signature_valid {
            return Err(CryptographyError::InvalidSignature.into());
        }

        message_repository
            .create_message(
                self.recipient_id,
                domain::message::MessageMetadata(self.metadata),
                self.content,
            )
            .await?;

        Ok(())
    }
}

use domain::stamp::OnetimeStamp;

#[derive(Deserialize)]
pub struct SendMessageWithOnetimeStampCommandDto {
    pub recipient_id: Uuid,
    pub content: String,
    pub metadata: String,
    pub signature: String,
    pub stamp: OnetimeStamp,
}
pub struct SendMessageWithOnetimeStampCommand {
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub content: String,
    pub metadata: String,
    pub signature: String,
    pub stamp: OnetimeStamp,
}

impl SendMessageWithOnetimeStampCommand {
    pub async fn handle(
        self,
        user_repository: &impl UserRepository,
        cryptography_service: &impl CryptographyService,
        serialize_service: &impl SerializeService,
        tracker_repository: &impl OneTimeStampTrackerRepository,
        system_key_repository: &impl SystemKeyRepository,
        message_repository: &impl MessageRepository,
    ) -> Result<(), SmError> {
        let sender = match (GetUserByIdQuery {
            user_id: self.sender_id,
        })
        .handle(user_repository)
        .await?
        {
            Some(sender) => sender,
            None => return Err(UserError::UserNotFound.into()),
        };

        let stamp_valid = VerifyOnetimeStampCommand(self.stamp)
            .handle(
                user_repository,
                cryptography_service,
                serialize_service,
                tracker_repository,
                system_key_repository,
            )
            .await?;
        if !stamp_valid {
            return Err(StampError::InvalidStamp.into());
        }

        let signature_valid = cryptography_service.validate_signature(
            &format!("{}\n{}", self.metadata, self.content),
            &self.signature,
            &sender.public_verify_key,
        );
        if !signature_valid {
            return Err(CryptographyError::InvalidSignature.into());
        }

        message_repository
            .create_message(
                self.recipient_id,
                MessageMetadata(self.metadata),
                self.content,
            )
            .await?;
        Ok(())
    }
}
