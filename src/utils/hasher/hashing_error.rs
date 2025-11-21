use crate::app_error::AppError;

#[derive(Debug, thiserror::Error, Clone)]
pub enum HashingError {
  #[error("Failed to hash password: {0}")]
  HashingError(String),
  #[error("Failed to verify password: {0}")]
  VerificationError(String),
}

impl From<HashingError> for AppError {
  fn from(value: HashingError) -> Self {
    AppError::Other(anyhow::Error::from(value))
  }
}