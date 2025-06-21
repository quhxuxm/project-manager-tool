use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

impl From<Error> for u32 {
    fn from(e: Error) -> u32 {
        match e {
            Error::Db(_) => 1,
        }
    }
}
