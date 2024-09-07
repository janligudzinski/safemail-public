use domain::{message::MessageRepository, onetime_stamp::OneTimeStampTrackerRepository, serialize};
use infrastructure::{
    repositories::{
        PostgresMessageRepository, PostgresOneTimeStampRepository, PostgresSessionRepository,
        PostgresStampRequestRepository, PostgresSystemKeyRepository, PostgresUserRepository,
    },
    services::{
        cryptography::{BcryptPasswordService, OpensslCryptographyService},
        serialize::JsonService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub user_repository: PostgresUserRepository,
    pub session_repository: PostgresSessionRepository,
    pub message_repository: PostgresMessageRepository,
    pub tracker_repository: PostgresOneTimeStampRepository,
    pub stamp_request_repository: PostgresStampRequestRepository,
    pub system_key_repository: PostgresSystemKeyRepository,
    pub password_service: BcryptPasswordService,
    pub cryptography_service: OpensslCryptographyService,
    pub serialize_service: JsonService,
}
impl AppState {
    pub async fn new() -> Self {
        let db = infrastructure::db::get_pool().await;
        let user_repository = PostgresUserRepository::new(db.clone());
        let session_repository = PostgresSessionRepository::new(db.clone());
        let message_repository = PostgresMessageRepository::new(db.clone());
        let tracker_repository = PostgresOneTimeStampRepository::new(db.clone());
        let stamp_request_repository = PostgresStampRequestRepository::new(db.clone());
        let system_key_repository = PostgresSystemKeyRepository::new(db.clone());

        let password_service = BcryptPasswordService;
        let cryptography_service = OpensslCryptographyService;
        let serialize_service = JsonService;

        Self {
            user_repository,
            session_repository,
            message_repository,
            stamp_request_repository,
            system_key_repository,
            tracker_repository,
            password_service,
            cryptography_service,
            serialize_service,
        }
    }
}
