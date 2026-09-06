use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::model::{CreateFoo, Foo};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateFooRequestDto {
  pub name: String,
  pub description: Option<String>,
}

impl From<CreateFooRequestDto> for CreateFoo {
  fn from(dto: CreateFooRequestDto) -> Self {
    Self { name: dto.name, description: dto.description }
  }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateFooResponseDto {
  pub id: i64,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
}

impl From<Foo> for CreateFooResponseDto {
  fn from(foo: Foo) -> Self {
    Self {
      id: foo.id,
      name: foo.name,
      description: foo.description,
      created_at: foo.created_at,
    }
  }
}
