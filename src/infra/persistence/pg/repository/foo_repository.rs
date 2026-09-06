use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::Pool};

use crate::{
  domain::{
    model::{CreateFoo, Foo, ListFoo},
    param::FindByCriteriaFooParam,
    repository::FooRepository,
    result::DomainResult,
  },
  infra::persistence::pg::{
    foo,
    model::{PgCreateFoo, PgFoo},
  },
};

#[derive(Clone)]
pub struct PgFooRepository {
  pool: Pool<AsyncPgConnection>,
}

impl PgFooRepository {
  pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
    Self { pool }
  }
}

impl FooRepository for PgFooRepository {
  async fn find_by_id(&self, id: i64) -> DomainResult<Foo> {
    let mut conn = self.pool.get().await?;

    foo::table
      .find(id)
      .first::<PgFoo>(&mut conn)
      .await
      .map_err(Into::into)
      .map(Into::into)
  }

  async fn find_by_criteria(
    &self, param: FindByCriteriaFooParam,
  ) -> DomainResult<ListFoo> {
    let mut conn = self.pool.get().await?;

    foo::table
      .order(foo::created_at.desc())
      .limit(param.limit)
      .offset(param.offset)
      .get_results::<PgFoo>(&mut conn)
      .await
      .map_err(Into::into)
      .map(Into::into)
  }

  async fn create(&self, foo_to_create: CreateFoo) -> DomainResult<Foo> {
    let mut conn = self.pool.get().await?;

    diesel::insert_into(foo::table)
      .values(PgCreateFoo::from(foo_to_create))
      .get_result::<PgFoo>(&mut conn)
      .await
      .map_err(Into::into)
      .map(Into::into)
  }
}
