use domain::{
    error::{SessionError, SmError},
    session::SessionRepository,
    user::{User, UserRepository},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GetUserByUsernameQuery {
    pub username: String,
}
impl GetUserByUsernameQuery {
    pub async fn handle<UR: UserRepository>(
        self,
        user_repository: &UR,
    ) -> Result<Option<User>, SmError> {
        let user = user_repository.find_by_username(self.username).await?;
        Ok(user)
    }
}

#[derive(Deserialize)]
pub struct GetUserByIdQuery {
    pub user_id: Uuid,
}

impl GetUserByIdQuery {
    pub async fn handle<UR: UserRepository>(
        self,
        user_repository: &UR,
    ) -> Result<Option<User>, SmError> {
        let user = user_repository.find_by_id(self.user_id).await?;
        Ok(user)
    }
}

#[derive(Deserialize)]
pub struct GetUserBySessionQuery {
    pub session: Uuid,
}
impl GetUserBySessionQuery {
    pub async fn handle<UR: UserRepository, SR: SessionRepository>(
        self,
        user_repository: &UR,
        session_repository: &SR,
    ) -> Result<User, SmError> {
        let session =
            if let Some(session) = session_repository.get_session(self.session, false).await? {
                session
            } else {
                return Err(SmError::Session(SessionError::SessionNotFound));
            };
        let user = user_repository
            .find_by_id(session.user_id)
            .await?
            .expect("A session should not be issued to a nonexistent user");
        Ok(user)
    }
}
