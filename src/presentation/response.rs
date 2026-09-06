use serde::{Serialize, Serializer};
use axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};

use crate::{app::error::AppError, domain::error::DomainError};
use super::error::{ExtractError, PresentationError};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppResponse<T: Serialize> {
  pub message: T,
  #[serde(serialize_with = "serialize_status_code")]
  pub status_code: StatusCode,
}

impl<T: Serialize> AppResponse<T> {
  pub fn new(message: T, status_code: StatusCode) -> Self {
    Self { message, status_code }
  }
}

impl<T: Serialize> IntoResponse for AppResponse<T> {
  fn into_response(self) -> Response {
    (self.status_code, Json(self)).into_response()
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppResponseError {
  pub message: String,
  pub error: String,
  #[serde(serialize_with = "serialize_status_code")]
  pub status_code: StatusCode,
}

impl AppResponseError {
  pub fn new(message: String, error: String, status_code: StatusCode) -> Self {
    Self { message, error, status_code }
  }
}

impl IntoResponse for AppResponseError {
  fn into_response(self) -> Response {
    logger(self.status_code, &self.message);
    (self.status_code, Json(self)).into_response()
  }
}

/// Helper function to map domain errors to status code and message
fn map_domain_error(error: DomainError) -> (StatusCode, String) {
  match error {
    DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
    DomainError::AlreadyExists(msg) => (StatusCode::CONFLICT, msg),
    DomainError::Conflict(msg) => (StatusCode::CONFLICT, msg),
    DomainError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
    DomainError::InternalServerError(msg) => {
      (StatusCode::INTERNAL_SERVER_ERROR, msg)
    }
  }
}

fn map_app_error(error: AppError) -> (StatusCode, String) {
  match error {
    AppError::Domain(domain_error) => map_domain_error(domain_error),
    AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
    AppError::InternalServerError(msg) => {
      (StatusCode::INTERNAL_SERVER_ERROR, msg)
    }
    AppError::S3(s3_error) => {
      (StatusCode::INTERNAL_SERVER_ERROR, s3_error.to_string())
    }
    AppError::Sqs(sqs_error) => {
      (StatusCode::INTERNAL_SERVER_ERROR, sqs_error.to_string())
    }
  }
}

impl IntoResponse for PresentationError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      Self::App(app_error) => map_app_error(app_error),
      Self::Domain(domain_error) => map_domain_error(domain_error),
      Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
    };

    logger(status, &error_message);

    AppResponseError::new(
      status
        .canonical_reason()
        .unwrap_or("Unknown error")
        .to_string(),
      error_message,
      status,
    )
    .into_response()
  }
}

impl IntoResponse for ExtractError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      ExtractError::Json(rejection) => {
        (rejection.status(), rejection.body_text())
      }
      ExtractError::Query(rejection) => {
        (rejection.status(), rejection.body_text())
      }
      ExtractError::Path(rejection) => {
        (rejection.status(), rejection.body_text())
      }
    };

    logger(status, &error_message);

    AppResponseError::new(
      status
        .canonical_reason()
        .unwrap_or("Unknown error")
        .to_string(),
      error_message,
      status,
    )
    .into_response()
  }
}

fn logger(status: StatusCode, error_message: &str) {
  if status.is_server_error() {
    tracing::error!(error_message = %error_message)
  } else {
    tracing::warn!(error_message = %error_message)
  }
}

fn serialize_status_code<S>(
  status: &StatusCode, serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_u16(status.as_u16())
}

pub type PresentationResult<T> = Result<AppResponse<T>, PresentationError>;
