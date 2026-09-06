use crate::{
  app::{
    bus::CommandBus, command::FooCommand, result::AppResult,
    use_case::FooUseCase as AppFooUseCase,
  },
  domain::{
    model::{CreateFoo, Foo, ListFoo},
    param::FindByCriteriaFooParam,
    repository::FooRepository,
  },
};

#[derive(Clone)]
pub struct FooUseCase<Fr, Cb> {
  foo_repository: Fr,
  foo_command_bus: Cb,
}

impl<Fr, Cb> FooUseCase<Fr, Cb>
where
  Fr: FooRepository,
  Cb: CommandBus<FooCommand>,
{
  pub fn new(foo_repository: Fr, foo_command_bus: Cb) -> Self {
    Self { foo_repository, foo_command_bus }
  }
}

impl<Fr, Cb> AppFooUseCase for FooUseCase<Fr, Cb>
where
  Fr: FooRepository,
  Cb: CommandBus<FooCommand>,
{
  async fn create(&self, foo: CreateFoo) -> AppResult<Foo> {
    let foo = self.foo_repository.create(foo).await?;

    self
      .foo_command_bus
      .send(FooCommand { id: foo.id, name: foo.name.clone() })
      .await?;

    Ok(foo)
  }

  async fn find_by_criteria(
    &self, param: FindByCriteriaFooParam,
  ) -> AppResult<Vec<Foo>> {
    self
      .foo_repository
      .find_by_criteria(param)
      .await
      .map(|ListFoo(foos)| foos)
      .map_err(Into::into)
  }

  async fn find_by_id(&self, id: i64) -> AppResult<Foo> {
    self.foo_repository.find_by_id(id).await.map_err(Into::into)
  }
}
