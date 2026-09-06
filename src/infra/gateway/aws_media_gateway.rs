use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};

use crate::{
  app::{error::AwsS3Error, gateway::MediaGateway, result::AppResult},
  infra::config::AwsS3Config,
};

#[derive(Clone)]
pub struct AwsMediaGateway {
  aws_s3_config: AwsS3Config,
}

#[allow(dead_code)]
impl AwsMediaGateway {
  pub fn new(aws_s3_config: AwsS3Config) -> Self {
    Self { aws_s3_config }
  }

  fn bucket(&self) -> &str {
    &self.aws_s3_config.bucket
  }

  fn endpoint_url(&self) -> &str {
    &self.aws_s3_config.endpoint_url
  }

  fn client(&self) -> &S3Client {
    &self.aws_s3_config.client
  }
}

impl MediaGateway for AwsMediaGateway {
  fn get_url(&self, key: String) -> String {
    format!("{}/{}", self.endpoint_url().trim_end_matches('/'), key)
  }

  async fn upload(&self, key: String, data: Vec<u8>) -> AppResult<()> {
    self
      .client()
      .put_object()
      .bucket(self.bucket())
      .key(&key)
      .body(ByteStream::from(data))
      .send()
      .await
      .map(|_| ())
      .map_err(AwsS3Error::from)
      .map_err(Into::into)
  }

  async fn get_all(&self, key: String) -> AppResult<Vec<String>> {
    self
      .client()
      .list_objects_v2()
      .bucket(self.bucket())
      .prefix(&key)
      .send()
      .await
      .map(|response| {
        response
          .contents()
          .iter()
          .filter_map(|object| object.key())
          .map(String::from)
          .collect()
      })
      .map_err(AwsS3Error::from)
      .map_err(Into::into)
  }

  async fn delete(&self, key: String) -> AppResult<()> {
    self
      .client()
      .delete_object()
      .bucket(self.bucket())
      .key(&key)
      .send()
      .await
      .map(|_| ())
      .map_err(AwsS3Error::from)
      .map_err(Into::into)
  }
}
