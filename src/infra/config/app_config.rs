use std::env;
use dotenvy::dotenv;
use tokio::sync::OnceCell;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use super::{AwsS3Config, AwsSqsConfig};

#[derive(Debug)]
pub struct AppConfig {
  pub host: String,
  pub port: u16,
  pub db_url: String,
  pub aws_s3_config: AwsS3Config,
  pub aws_sqs_config: AwsSqsConfig,
  pub openapi_enabled: bool,
}

impl AppConfig {
  pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
      .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
          .unwrap_or_else(|_| "RUST_LOG=debug".into()),
      )
      .with(tracing_subscriber::fmt::layer())
      .init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let port = env::var("PORT")
      .unwrap_or_else(|_| "8000".to_string())
      .parse::<u16>()
      .map_err(|_| "PORT must be a number")?;

    let db_url = env::var("DATABASE_URL")
      .map_err(|_| "DATABASE_URL must be set in .env file")?;

    let bucket = env::var("AWS_S3_BUCKET")
      .map_err(|_| "AWS_S3_BUCKET must be set in .env file")?;

    let queue_url = env::var("AWS_SQS_QUEUE_URL")
      .map_err(|_| "AWS_SQS_QUEUE_URL must be set in .env file")?;

    let openapi_enabled = env::var("OPENAPI_ENABLED")
      .unwrap_or_else(|_| "false".to_string())
      .parse::<bool>()
      .unwrap_or(false);

    let sdk_config = aws_config::from_env().load().await;

    Ok(AppConfig {
      host,
      port,
      db_url,
      aws_s3_config: AwsS3Config::new(&sdk_config, bucket),
      aws_sqs_config: AwsSqsConfig::new(&sdk_config, queue_url),
      openapi_enabled,
    })
  }
}

static CONFIG: OnceCell<AppConfig> = OnceCell::const_new();

pub async fn load() -> &'static AppConfig {
  CONFIG
    .get_or_init(|| async {
      AppConfig::from_env().await.expect("Failed to load config")
    })
    .await
}
