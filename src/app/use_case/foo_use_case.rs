use crate::{
  app::result::AppResult,
  domain::{
    model::{CreateFoo, Foo},
    param::FindByCriteriaFooParam,
  },
};

#[trait_variant::make(Send)]
pub trait FooUseCase: Clone + Sync + 'static {
  async fn create(&self, foo: CreateFoo) -> AppResult<Foo>;

  async fn find_by_criteria(
    &self, param: FindByCriteriaFooParam,
  ) -> AppResult<Vec<Foo>>;

  async fn find_by_id(&self, id: i64) -> AppResult<Foo>;
}
