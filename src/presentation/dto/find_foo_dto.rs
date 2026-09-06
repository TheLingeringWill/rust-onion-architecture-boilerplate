use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::domain::{model::Foo, param::FindByCriteriaFooParam};

const DEFAULT_LIMIT: i64 = 10;
const DEFAULT_OFFSET: i64 = 0;

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "snake_case")]
pub struct FindFooRequestDto {
  pub limit: Option<i64>,
  pub offset: Option<i64>,
}

impl From<FindFooRequestDto> for FindByCriteriaFooParam {
  fn from(dto: FindFooRequestDto) -> Self {
    Self {
      limit: dto.limit.unwrap_or(DEFAULT_LIMIT),
      offset: dto.offset.unwrap_or(DEFAULT_OFFSET),
    }
  }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FindFooResponseDto {
  pub id: i64,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
}

impl From<Foo> for FindFooResponseDto {
  fn from(foo: Foo) -> Self {
    Self {
      id: foo.id,
      name: foo.name,
      description: foo.description,
      created_at: foo.created_at,
      updated_at: foo.updated_at,
    }
  }
}
