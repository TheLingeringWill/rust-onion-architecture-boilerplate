use diesel::result::{
  DatabaseErrorKind as DieselDatabaseErrorKind, Error as DieselError,
};
use diesel_async::pooled_connection::bb8::RunError;

use crate::domain::error::DomainError;

impl From<RunError> for DomainError {
  fn from(error: RunError) -> Self {
    Self::InternalServerError(error.to_string())
  }
}

impl From<DieselError> for DomainError {
  fn from(error: DieselError) -> Self {
    match error {
      DieselError::NotFound => Self::NotFound(error.to_string()),
      DieselError::DatabaseError(kind, info) => {
        let message = info.message().to_string();
        match kind {
          DieselDatabaseErrorKind::UniqueViolation => {
            Self::AlreadyExists(message)
          }
          DieselDatabaseErrorKind::ForeignKeyViolation => {
            Self::Conflict(message)
          }
          DieselDatabaseErrorKind::NotNullViolation
          | DieselDatabaseErrorKind::CheckViolation => {
            Self::BadRequest(message)
          }
          _ => Self::InternalServerError(message),
        }
      }
      _ => Self::InternalServerError(error.to_string()),
    }
  }
}
