use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooCommand {
  pub id: i64,
  pub name: String,
}
