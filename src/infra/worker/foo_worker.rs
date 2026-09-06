use crate::app::{
  command::FooCommand, gateway::MediaGateway, result::AppResult, worker::Worker,
};

#[derive(Clone)]
pub struct FooWorker<Mg> {
  media_gateway: Mg,
}

impl<Mg> FooWorker<Mg>
where
  Mg: MediaGateway,
{
  const BUCKET_FOLDER: &'static str = "foos";

  pub fn new(media_gateway: Mg) -> Self {
    Self { media_gateway }
  }
}

impl<Mg> Worker<FooCommand> for FooWorker<Mg>
where
  Mg: MediaGateway,
{
  async fn execute(&self, command: FooCommand) -> AppResult<()> {
    self
      .media_gateway
      .upload(
        format!("{}/{}.txt", Self::BUCKET_FOLDER, command.id),
        command.name.into_bytes(),
      )
      .await
  }
}
