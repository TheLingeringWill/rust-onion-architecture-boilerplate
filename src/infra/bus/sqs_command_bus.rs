use std::marker::PhantomData;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
  app::{
    bus::{CommandBus, CommandConsumer, Delivery},
    error::AwsSqsError,
    result::AppResult,
  },
  infra::config::AwsSqsConfig,
};

const WAIT_TIME_SECONDS: i32 = 20;
const MAX_NUMBER_OF_MESSAGES: i32 = 10;

pub struct SqsCommandBus<T> {
  aws_sqs_config: AwsSqsConfig,
  command: PhantomData<fn() -> T>,
}

impl<T> Clone for SqsCommandBus<T> {
  fn clone(&self) -> Self {
    Self {
      aws_sqs_config: self.aws_sqs_config.clone(),
      command: PhantomData,
    }
  }
}

impl<T> SqsCommandBus<T> {
  pub fn new(aws_sqs_config: AwsSqsConfig) -> Self {
    Self { aws_sqs_config, command: PhantomData }
  }

  fn queue_url(&self) -> &str {
    &self.aws_sqs_config.queue_url
  }
}

impl<T> CommandBus<T> for SqsCommandBus<T>
where
  T: Serialize + Clone + Send + Sync + 'static,
{
  async fn send(&self, command: T) -> AppResult<()> {
    self
      .aws_sqs_config
      .client
      .send_message()
      .queue_url(self.queue_url())
      .message_body(serde_json::to_string(&command).map_err(AwsSqsError::from)?)
      .send()
      .await
      .map(|_| ())
      .map_err(AwsSqsError::from)
      .map_err(Into::into)
  }
}

impl<T> CommandConsumer<T> for SqsCommandBus<T>
where
  T: DeserializeOwned + Send + Sync + 'static,
{
  async fn receive(&self) -> AppResult<Vec<Delivery<T>>> {
    self
      .aws_sqs_config
      .client
      .receive_message()
      .queue_url(self.queue_url())
      .wait_time_seconds(WAIT_TIME_SECONDS)
      .max_number_of_messages(MAX_NUMBER_OF_MESSAGES)
      .send()
      .await
      .map_err(AwsSqsError::from)
      .map_err(Into::into)
      .map(|output| {
        output
          .messages
          .unwrap_or_default()
          .into_iter()
          .filter_map(|message| {
            message.receipt_handle.zip(message.body).and_then(
              |(receipt, body)| {
                serde_json::from_str(&body)
                  .inspect_err(|error| {
                    tracing::error!(
                      %error,
                      "discarding unparseable command payload"
                    )
                  })
                  .ok()
                  .map(|command| Delivery { command, receipt })
              },
            )
          })
          .collect()
      })
  }

  async fn ack(&self, receipt: String) -> AppResult<()> {
    self
      .aws_sqs_config
      .client
      .delete_message()
      .queue_url(self.queue_url())
      .receipt_handle(receipt)
      .send()
      .await
      .map(|_| ())
      .map_err(AwsSqsError::from)
      .map_err(Into::into)
  }
}
