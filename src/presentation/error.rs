use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use thiserror::Error;

use crate::{app::error::AppError, domain::error::DomainError};

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ExtractError {
  Json(#[from] JsonRejection),
  Query(#[from] QueryRejection),
  Path(#[from] PathRejection),
}

#[derive(Debug, Error)]
#[allow(clippy::large_enum_variant, dead_code)]
pub enum PresentationError {
  #[error("App error: {0}")]
  App(#[from] AppError),
  #[error("Domain error: {0}")]
  Domain(#[from] DomainError),
  #[error("Bad request: {0}")]
  BadRequest(String),
}
