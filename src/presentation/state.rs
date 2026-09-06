use diesel_async::{
  AsyncPgConnection,
  pooled_connection::{AsyncDieselConnectionManager, PoolError, bb8::Pool},
};
use thiserror::Error;

use crate::infra::{
  bus::FooCommandBus,
  config::{AppConfig, AwsS3Config},
};

#[derive(Clone)]
pub struct AppState {
  pub pool: Pool<AsyncPgConnection>,
  pub aws_s3_config: AwsS3Config,
  pub foo_command_bus: FooCommandBus,
  pub openapi_enabled: bool,
}

#[derive(Debug, Error)]
pub enum AppStateError {
  #[error("failed to create Diesel connection pool: {0}")]
  PoolBuild(#[from] PoolError),
}

impl AppState {
  pub async fn new(config: &AppConfig) -> Result<Self, AppStateError> {
    Pool::builder()
      .build(AsyncDieselConnectionManager::<AsyncPgConnection>::new(
        &config.db_url,
      ))
      .await
      .map(|pool| Self {
        pool,
        aws_s3_config: config.aws_s3_config.clone(),
        foo_command_bus: FooCommandBus::new(config.aws_sqs_config.clone()),
        openapi_enabled: config.openapi_enabled,
      })
      .map_err(Into::into)
  }
}
