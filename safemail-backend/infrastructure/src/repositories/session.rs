use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use domain::{
    error::{DatabaseError, SessionError, SmError, UserError},
    session::{Session, SessionRepository},
};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{types::Uuid, PgPool};

#[derive(Clone)]
pub struct PostgresSessionRepository {
    pub db: Arc<PgPool>,
}
impl PostgresSessionRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}
#[async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn request_session(&self, user_id: Uuid) -> Result<Session, SmError> {
        let now = Utc::now();
        let session_end = now.checked_add_signed(chrono::Duration::hours(2)).unwrap();
        // generate a random ASCII string of 24 characters
        let challenge_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(24)
            .collect::<Vec<_>>();
        // collect the bytes into a string
        let challenge_string = String::from_utf8(challenge_string).unwrap();
        let result = sqlx::query_as!
        (Session,
            "INSERT INTO sm.sessions (user_id, challenge_string, requested_at_utc, expires_at_utc) VALUES ($1, $2, $3, $4) RETURNING *",
            user_id,
            challenge_string,
            now,
            session_end
        ).fetch_one(&*self.db).await.map_err(|e| {
            let db = e.as_database_error();
            let err: SmError = if let Some(sqlx_error) = db {
                match sqlx_error.kind() {
                    sqlx::error::ErrorKind::ForeignKeyViolation => UserError::UserNotFound.into(),
                    _ => DatabaseError::Arbitrary.into(),
                }
            } else {
                DatabaseError::Arbitrary.into()
            };
            err
        })?;
        Ok(result)
    }

    async fn activate_session(&self, session_id: Uuid) -> Result<(), SmError> {
        let now = Utc::now();
        let result = sqlx::query!(
            "UPDATE sm.sessions SET active = true, activated_at_utc = $1 WHERE session_id = $2 AND expires_at_utc > $3",
            now,
            session_id,
            now
        )
        .execute(&*self.db)
        .await
        .map_err(|_| DatabaseError::Arbitrary)?;
        if result.rows_affected() == 0 {
            Err(SessionError::SessionNotFound.into())
        } else {
            Ok(())
        }
    }
    async fn get_session(
        &self,
        session_id: Uuid,
        include_inactive: bool,
    ) -> Result<Option<Session>, SmError> {
        let now = Utc::now();
        let result = sqlx::query_as!(Session, "SELECT * FROM sm.sessions WHERE session_id = $1 AND (active = true OR $2) AND expires_at_utc > $3", session_id, include_inactive, now) 
            .fetch_optional(&*self.db)
            .await
            .map_err(|_| DatabaseError::Arbitrary)?;
        Ok(result)
    }
    async fn logout_session(&self, session_id: Uuid) -> Result<(), SmError> {
        sqlx::query!(
            "UPDATE sm.sessions SET active = false WHERE session_id = $1",
            session_id
        )
        .execute(&*self.db)
        .await
        .map_err(|_| DatabaseError::Arbitrary)?;
        Ok(())
    }
}
