use crate::app::result::AppResult;

#[trait_variant::make(Send)]
pub trait Worker<T>: Sync + 'static
where
  T: Send + 'static,
{
  async fn execute(&self, command: T) -> AppResult<()>;
}
