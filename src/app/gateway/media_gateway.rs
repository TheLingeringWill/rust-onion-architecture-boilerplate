use crate::app::result::AppResult;

#[trait_variant::make(Send)]
#[allow(dead_code)]
pub trait MediaGateway: Clone + Sync + 'static {
  fn get_url(&self, key: String) -> String;

  async fn upload(&self, key: String, data: Vec<u8>) -> AppResult<()>;

  async fn get_all(&self, key: String) -> AppResult<Vec<String>>;

  async fn delete(&self, key: String) -> AppResult<()>;
}
