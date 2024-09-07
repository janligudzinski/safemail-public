use domain::{
    chrono,
    crypto::CryptographyService,
    error::{SmError, StampError, UserError},
    onetime_stamp::OneTimeStampTrackerRepository,
    pow::Pow,
    serialize::SerializeService,
    stamp::{OnetimeStamp, PeriodicStamp},
    stamp_request::StampRequestRepository,
    system_key::SystemKeyRepository,
    user::UserRepository,
};
use serde::Deserialize;
use uuid::{uuid, Uuid};

use crate::user::queries::GetUserByIdQuery;

const STAMP_SYSTEM_ISSUED: Uuid = uuid!("00000000-0000-0000-0000-000000000000");
const BASE_STAMP_DIFFICULTY: i64 = 50_000;

pub struct VerifyPeriodicStampCommand(pub PeriodicStamp);
impl VerifyPeriodicStampCommand {
    pub async fn handle(
        self,
        user_repository: &impl UserRepository,
        cryptography_service: &impl CryptographyService,
        serialize_service: &impl SerializeService,
    ) -> Result<bool, SmError> {
        let stamp = self.0;

        let issuer = GetUserByIdQuery {
            user_id: stamp.issuer_id.clone(),
        }
        .handle(user_repository)
        .await?
        .ok_or(StampError::InvalidStamp)?;

        let recipient = GetUserByIdQuery {
            user_id: stamp.recipient_id.clone(),
        }
        .handle(user_repository)
        .await?
        .ok_or(StampError::InvalidStamp)?;

        let signature_plaintext = {
            let issuer_id = serialize_service.serialize(&issuer.id);
            let recipient_id = serialize_service.serialize(&recipient.id);
            let sender_id = serialize_service.serialize(&stamp.sender_id);
            let valid_from = serialize_service.serialize(&stamp.valid_from);
            let valid_to = serialize_service.serialize(&stamp.valid_to);
            format!(
                "{}\n{}\n{}\n{}\n{}",
                issuer_id, recipient_id, sender_id, valid_from, valid_to
            )
        };

        let validation = cryptography_service.validate_signature(
            &signature_plaintext,
            &stamp.signature,
            &issuer.public_verify_key,
        ) && (issuer.id == recipient.id || issuer.id == STAMP_SYSTEM_ISSUED);

        Ok(validation)
    }
}
pub struct VerifyOnetimeStampCommand(pub OnetimeStamp);

impl VerifyOnetimeStampCommand {
    pub async fn handle(
        self,
        user_repository: &impl UserRepository,
        cryptography_service: &impl CryptographyService,
        serialize_service: &impl SerializeService,
        tracker_repository: &impl OneTimeStampTrackerRepository,
        system_key_repository: &impl SystemKeyRepository,
    ) -> Result<bool, SmError> {
        let stamp = self.0;

        // Check if the stamp has been used before
        if let Some(tracker) = tracker_repository.get_by_id(stamp.stamp_id).await? {
            if tracker.used_or_revoked {
                return Ok(false);
            }
        }

        // Verify issuer and recipient exist
        let issuer_key = if stamp.issuer_id == STAMP_SYSTEM_ISSUED {
            let system_keys = system_key_repository.get_system_keys().await?.unwrap();
            system_keys.public_key
        } else {
            let issuer = GetUserByIdQuery {
                user_id: stamp.issuer_id,
            }
            .handle(user_repository)
            .await?;
            if issuer.is_none() {
                return Err(UserError::UserNotFound)?;
            }
            issuer.unwrap().public_verify_key
        };

        let recipient = GetUserByIdQuery {
            user_id: stamp.recipient_id,
        }
        .handle(user_repository)
        .await?
        .ok_or(UserError::UserNotFound)?;

        // Check if the stamp has expired
        if let Some(valid_to) = stamp.valid_to {
            if valid_to < chrono::Utc::now() {
                return Err(StampError::InvalidTimePeriod)?;
            }
        }

        // Prepare signature plaintext
        let signature_plaintext = {
            let stamp_id = serialize_service.serialize(&stamp.stamp_id);
            let issuer_id = serialize_service.serialize(&stamp.issuer_id);
            let recipient_id = serialize_service.serialize(&stamp.recipient_id);
            let sender_id = serialize_service.serialize(&stamp.sender_id);
            let valid_to = serialize_service.serialize(&stamp.valid_to);
            format!(
                "{}\n{}\n{}\n{}\n{}",
                stamp_id, issuer_id, recipient_id, sender_id, valid_to
            )
        };

        // Validate signature
        let validation = cryptography_service.validate_signature(
            &signature_plaintext,
            &stamp.signature,
            &issuer_key,
        ) && (stamp.issuer_id == recipient.id
            || stamp.issuer_id == STAMP_SYSTEM_ISSUED);
        Ok(validation)
    }
}

#[derive(Deserialize)]
pub struct RequestSystemStampIssueCommand {
    pub recipient_id: Uuid,
    pub sender_id: Uuid,
}
impl RequestSystemStampIssueCommand {
    pub async fn handle(
        self,
        user_repository: &impl UserRepository,
        stamp_request_repository: &impl StampRequestRepository,
    ) -> Result<Uuid, SmError> {
        // ensure sender and recipient exist
        let recipient = GetUserByIdQuery {
            user_id: self.recipient_id,
        }
        .handle(user_repository)
        .await?
        .ok_or(SmError::from(UserError::UserNotFound))?;
        let _sender = GetUserByIdQuery {
            user_id: self.sender_id,
        }
        .handle(user_repository)
        .await?
        .ok_or(SmError::from(UserError::UserNotFound))?;

        let id = stamp_request_repository
            .create_stamp_request(BASE_STAMP_DIFFICULTY, recipient.id)
            .await?;

        Ok(id)
    }
}

#[derive(Deserialize)]
pub struct IssueSystemStampCommandDto {
    pub stamp_request_id: Uuid,
    pub proof_of_work: Pow<Uuid>,
}

pub struct IssueSystemStampCommand {
    pub stamp_request_id: Uuid,
    pub sender_id: Uuid,
    pub proof_of_work: Pow<Uuid>,
}

impl IssueSystemStampCommand {
    pub async fn handle(
        self,
        user_repository: &impl UserRepository,
        stamp_request_repo: &impl StampRequestRepository,
        tracker_repo: &impl OneTimeStampTrackerRepository,
        system_key_repo: &impl SystemKeyRepository,
        crypto_service: &impl CryptographyService,
        serialize_service: &impl SerializeService,
    ) -> Result<OnetimeStamp, SmError> {
        // Retrieve the stamp request
        let stamp_request = stamp_request_repo
            .get_stamp_request(self.stamp_request_id)
            .await?
            .ok_or(StampError::StampRequestNotFound)?;

        // Check if the stamp request has expired
        let current_time = chrono::Utc::now();
        if stamp_request.valid_to <= current_time {
            return Err(StampError::StampRequestExpired.into());
        }

        // Verify the proof of work
        if !(self
            .proof_of_work
            .score(&self.stamp_request_id)
            .unwrap_or(0)
            >= stamp_request.difficulty as u128)
        {
            return Err(StampError::InvalidProofOfWork.into());
        }

        let system_keys = system_key_repo
            .get_system_keys()
            .await?
            .expect("System keys have not been set");

        // Get the recipient
        let recipient = GetUserByIdQuery {
            user_id: stamp_request.recipient_id,
        }
        .handle(user_repository)
        .await?
        .ok_or(SmError::from(UserError::UserNotFound))?;

        // Create the OneTimeStamp
        let stamp = OnetimeStamp {
            stamp_id: Uuid::new_v4(),
            issuer_id: STAMP_SYSTEM_ISSUED,
            recipient_id: recipient.id,
            sender_id: self.sender_id,
            valid_to: Some(chrono::Utc::now() + chrono::Duration::minutes(15)),
            signature: String::new(), // This will be filled in later
        };

        // Create the signature
        let signature = {
            let signature_stamp = {
                let stamp_id = serialize_service.serialize(&stamp.stamp_id);
                let issuer_id = serialize_service.serialize(&stamp.issuer_id);
                let recipient_id = serialize_service.serialize(&stamp.recipient_id);
                let sender_id = serialize_service.serialize(&stamp.sender_id);
                let valid_to = serialize_service.serialize(&stamp.valid_to);
                format!(
                    "{}\n{}\n{}\n{}\n{}",
                    stamp_id, issuer_id, recipient_id, sender_id, valid_to
                )
            };
            crypto_service.produce_signature(&signature_stamp, &system_keys.private_key)
        }
        .expect("System keys were invalid for signing");

        // Create the final stamp with the signature
        let final_stamp = OnetimeStamp { signature, ..stamp };

        // Set stamp request as completed
        stamp_request_repo
            .mark_solved(self.stamp_request_id)
            .await?;

        tracker_repo
            .insert(final_stamp.stamp_id, final_stamp.recipient_id)
            .await?;

        Ok(final_stamp)
    }
}
