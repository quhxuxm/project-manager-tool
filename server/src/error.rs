use crate::entity::RoleName;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Db(#[from] sqlx::Error),
    #[error("User not found with username: {0}")]
    UserNotFound(String),
    #[error(transparent)]
    Security(SecurityError),
}

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Expired token: {0}")]
    InvalidToken(String),
    #[error("No authorization header")]
    AuthorizationHeader,
    #[error("Authenticate fail")]
    Authentication,
    #[error("Generate token fail")]
    GenerateToken,
    #[error("Require authorization: {0}")]
    RequireAuthorization(RoleName),
}

impl From<Error> for u32 {
    fn from(e: Error) -> u32 {
        match e {
            Error::Security(_) => 0,
            Error::Db(_) => 1,
            Error::UserNotFound(_) => 2,
            Error::Io(_) => 3,
        }
    }
}
