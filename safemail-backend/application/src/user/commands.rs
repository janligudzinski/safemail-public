use domain::{
    crypto::CryptographyService,
    error::{CryptographyError, SessionError, SmError, UserError, ValidationError},
    session::{Session, SessionRepository},
    user::{User, UserRepository},
    validate::Validate,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterUserCommand {
    pub username: String,
    pub public_encryption_key: String,
    pub public_verify_key: String,
}
impl RegisterUserCommand {
    pub async fn handle<UR: UserRepository>(self, user_repository: &UR) -> Result<User, SmError> {
        user_repository
            .create(
                self.username,
                self.public_encryption_key,
                self.public_verify_key,
            )
            .await
    }
}

#[derive(Deserialize)]
pub struct RequestSessionCommand {
    pub username: String,
}
impl RequestSessionCommand {
    pub async fn handle<UR: UserRepository, SR: SessionRepository>(
        self,
        user_repository: &UR,
        session_repository: &SR,
    ) -> Result<Session, SmError> {
        let user = user_repository
            .find_by_username(self.username)
            .await?
            .ok_or(UserError::InvalidCredentials)?;
        let session = session_repository.request_session(user.id).await?;
        Ok(session)
    }
}

#[derive(Deserialize)]
pub struct ActivateSessionCommand {
    pub session_id: Uuid,
    pub challenge_signature: String,
}
impl ActivateSessionCommand {
    pub async fn handle<UR: UserRepository, SR: SessionRepository, CS: CryptographyService>(
        self,
        user_repository: &UR,
        session_repository: &SR,
        cryptography_service: &CS,
    ) -> Result<(), SmError> {
        let session = match session_repository
            .get_session(self.session_id, true)
            .await?
        {
            Some(session) => session,
            None => {
                return Err(SessionError::SessionNotFound.into());
            }
        };
        let user = match user_repository.find_by_id(session.user_id).await? {
            Some(user) => user,
            None => {
                return Err(UserError::UserNotFound.into());
            }
        };
        if !cryptography_service.validate_signature(
            &session.challenge_string,
            &self.challenge_signature,
            &user.public_verify_key,
        ) {
            return Err(CryptographyError::InvalidSignature.into());
        }
        session_repository.activate_session(self.session_id).await?;
        Ok(())
    }
}

pub struct UserCommandValidator<'a, CS: CryptographyService>(pub &'a CS);
impl<CS: CryptographyService> Validate<RegisterUserCommand> for UserCommandValidator<'_, CS> {
    fn validate(&self, value: &RegisterUserCommand) -> Result<(), domain::error::ValidationError> {
        if value.username.len() < 3
            || value
                .username
                .chars()
                .any(|c| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
        {
            return Err(ValidationError("Username must be at least 3 characters long and contain only ASCII letters, numbers, underscores, and hyphens".to_string()));
        }
        if !self.0.validate_public_key(&value.public_encryption_key) {
            return Err(ValidationError(
                "Not a valid base64 SPKI public key".to_string(),
            ));
        }
        Ok(())
    }
}
