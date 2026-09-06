use aws_config::SdkConfig;
use aws_sdk_s3::{Client as S3Client, config::Builder};

#[derive(Debug, Clone)]
pub struct AwsS3Config {
  pub client: S3Client,
  pub bucket: String,
  #[allow(dead_code)]
  pub endpoint_url: String,
}

impl AwsS3Config {
  pub fn new(config: &SdkConfig, bucket: String) -> Self {
    match config.endpoint_url() {
      Some(endpoint_url) => {
        tracing::info!("Using local S3 endpoint: {}", endpoint_url);
        Self {
          client: S3Client::from_conf(
            Builder::from(config).force_path_style(true).build(),
          ),
          endpoint_url: format!("{}/{}", endpoint_url, bucket),
          bucket,
        }
      }
      None => Self {
        client: S3Client::new(config),
        endpoint_url: format!("https://{}.s3.amazonaws.com", bucket),
        bucket,
      },
    }
  }
}
