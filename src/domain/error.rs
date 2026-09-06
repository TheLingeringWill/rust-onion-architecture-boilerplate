use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
  #[error("Not Found: {0}")]
  NotFound(String),
  #[error("Already Exists: {0}")]
  AlreadyExists(String),
  #[error("Conflict: {0}")]
  Conflict(String),
  #[error("Bad Request: {0}")]
  BadRequest(String),
  #[error("Internal server error: {0}")]
  InternalServerError(String),
}
