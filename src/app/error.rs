use thiserror::Error;
use aws_sdk_s3::{
  error::{BuildError, SdkError},
  operation::{
    delete_object::DeleteObjectError, delete_objects::DeleteObjectsError,
    list_objects_v2::ListObjectsV2Error, put_object::PutObjectError,
  },
};
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_sqs::operation::{
  delete_message::DeleteMessageError, receive_message::ReceiveMessageError,
  send_message::SendMessageError,
};

use crate::domain::error::DomainError;

#[derive(Error, Debug)]
pub enum AwsS3Error {
  #[error("Failed to build key for object: {0}")]
  BuildError(#[from] BuildError),

  #[error("Failed to put object to S3: {0}")]
  PutObject(#[from] SdkError<PutObjectError, HttpResponse>),

  #[error("Failed to delete object from S3: {0}")]
  DeleteObject(#[from] SdkError<DeleteObjectError, HttpResponse>),

  #[error("Failed to delete objects from S3: {0}")]
  DeleteObjects(#[from] SdkError<DeleteObjectsError, HttpResponse>),

  #[error("Failed to list objects from S3: {0}")]
  ListObjects(#[from] SdkError<ListObjectsV2Error, HttpResponse>),
}

#[derive(Error, Debug)]
pub enum AwsSqsError {
  #[error("Failed to send message to SQS: {0}")]
  SendMessage(#[from] SdkError<SendMessageError, HttpResponse>),

  #[error("Failed to receive messages from SQS: {0}")]
  ReceiveMessage(#[from] SdkError<ReceiveMessageError, HttpResponse>),

  #[error("Failed to delete message from SQS: {0}")]
  DeleteMessage(#[from] SdkError<DeleteMessageError, HttpResponse>),

  #[error("Failed to serialize command payload: {0}")]
  Payload(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
#[allow(clippy::large_enum_variant, dead_code)]
pub enum AppError {
  #[error("Domain error: {0}")]
  Domain(#[from] DomainError),

  #[error("Not found: {0}")]
  NotFound(String),

  #[error("Internal server error: {0}")]
  InternalServerError(String),

  #[error("S3 error: {0}")]
  S3(#[from] AwsS3Error),

  #[error("SQS error: {0}")]
  Sqs(#[from] AwsSqsError),
}
