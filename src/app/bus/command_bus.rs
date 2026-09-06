use crate::app::result::AppResult;

#[trait_variant::make(Send)]
pub trait CommandBus<T>: Clone + Sync + 'static
where
  T: Clone + Sync + 'static,
{
  async fn send(&self, command: T) -> AppResult<()>;
}
