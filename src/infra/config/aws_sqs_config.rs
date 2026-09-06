use aws_config::SdkConfig;
use aws_sdk_sqs::Client as SqsClient;

#[derive(Debug, Clone)]
pub struct AwsSqsConfig {
  pub client: SqsClient,
  pub queue_url: String,
}

impl AwsSqsConfig {
  pub fn new(config: &SdkConfig, queue_url: String) -> Self {
    Self { client: SqsClient::new(config), queue_url }
  }
}
