use tokio::task::JoinHandle;

use crate::app::{
  bus::{CommandConsumer, Delivery},
  worker::Worker,
};

pub fn spawn<W, Cc, T>(worker: W, consumer: Cc) -> JoinHandle<()>
where
  W: Worker<T>,
  Cc: CommandConsumer<T>,
  T: Send + 'static,
{
  tokio::spawn(async move {
    loop {
      match consumer.receive().await {
        Err(error) => {
          tracing::error!(%error, "failed to receive commands")
        }
        Ok(deliveries) => {
          for Delivery { command, receipt } in deliveries {
            match worker.execute(command).await {
              Err(error) => tracing::error!(
                %error,
                "worker failed, leaving message for redelivery"
              ),
              Ok(()) => {
                if let Err(error) = consumer.ack(receipt).await {
                  tracing::error!(%error, "failed to ack command")
                }
              }
            }
          }
        }
      }
    }
  })
}
