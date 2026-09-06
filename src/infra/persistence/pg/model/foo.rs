use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable, pg::Pg};

use crate::{
  domain::model::{CreateFoo, Foo, ListFoo},
  infra::persistence::pg,
};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = pg::foo)]
#[diesel(check_for_backend(Pg))]
pub struct PgFoo {
  pub id: i64,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
}

impl From<PgFoo> for Foo {
  fn from(foo: PgFoo) -> Self {
    Self {
      id: foo.id,
      name: foo.name,
      description: foo.description,
      created_at: foo.created_at,
      updated_at: foo.updated_at,
    }
  }
}

impl From<Vec<PgFoo>> for ListFoo {
  fn from(foos: Vec<PgFoo>) -> Self {
    Self(foos.into_iter().map(Into::into).collect())
  }
}

#[derive(Insertable)]
#[diesel(table_name = pg::foo)]
#[diesel(check_for_backend(Pg))]
pub struct PgCreateFoo {
  pub name: String,
  pub description: Option<String>,
}

impl From<CreateFoo> for PgCreateFoo {
  fn from(foo: CreateFoo) -> Self {
    Self { name: foo.name, description: foo.description }
  }
}
