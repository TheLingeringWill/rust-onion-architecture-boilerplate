use crate::app::result::AppResult;

#[derive(Debug, Clone)]
pub struct Delivery<T> {
  pub command: T,
  pub receipt: String,
}

#[trait_variant::make(Send)]
pub trait CommandConsumer<T>: Clone + Sync + 'static
where
  T: Send + 'static,
{
  async fn receive(&self) -> AppResult<Vec<Delivery<T>>>;

  async fn ack(&self, receipt: String) -> AppResult<()>;
}
