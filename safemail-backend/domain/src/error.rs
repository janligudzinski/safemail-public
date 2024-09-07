use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    #[error("User error: {0}")]
    User(#[from] UserError),
    #[error("Session error: {0}")]
    Session(#[from] SessionError),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    #[error("Cryptography error: {0}")]
    Cryptography(#[from] CryptographyError),
    #[error("Stamp error: {0}")]
    Stamp(#[from] StampError),
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct ValidationError(pub String);

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error")]
    Arbitrary,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Username is invalid")]
    InvalidUsername,
    #[error("Password is invalid")]
    InvalidPassword,
    #[error("Public key is invalid")]
    InvalidPublicKey,
}

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Non-expired session not found")]
    SessionNotFound,
}

#[derive(Error, Debug)]
pub enum CryptographyError {
    #[error("Invalid signature")]
    InvalidSignature,
}

#[derive(Error, Debug)]
pub enum StampError {
    #[error("Invalid stamp")]
    InvalidStamp,
    #[error("Out of time period")]
    InvalidTimePeriod,
    #[error("Invalid proof of work")]
    InvalidProofOfWork,
    #[error("Stamp request not found")]
    StampRequestNotFound,
    #[error("Stamp request expired")]
    StampRequestExpired,
}
