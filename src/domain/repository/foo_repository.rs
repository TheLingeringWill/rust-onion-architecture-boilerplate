use crate::domain::{
  model::{CreateFoo, Foo, ListFoo},
  param::FindByCriteriaFooParam,
  result::DomainResult,
};

#[trait_variant::make(Send)]
pub trait FooRepository: Sync + Clone + 'static {
  async fn find_by_id(&self, id: i64) -> DomainResult<Foo>;

  async fn find_by_criteria(
    &self, param: FindByCriteriaFooParam,
  ) -> DomainResult<ListFoo>;

  async fn create(&self, foo: CreateFoo) -> DomainResult<Foo>;
}
