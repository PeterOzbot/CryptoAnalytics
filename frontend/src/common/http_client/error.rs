use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq)]
#[error("{msg:?}")]
pub struct Error {
    pub msg: String,
}
