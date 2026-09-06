use axum::{Router, extract::State, http::StatusCode, routing};

use crate::{
  app::use_case::FooUseCase,
  presentation::{
    dto::{
      CreateFooRequestDto, CreateFooResponseDto, FindFooRequestDto,
      FindFooResponseDto,
    },
    extractor::{Json, Path, Query},
    response::{AppResponse, PresentationResult},
  },
};

/// Create a foo
///
/// Persists a foo and publishes a command for the worker to pick up
#[utoipa::path(
    post,
    path = "",
    context_path = "/foo",
    tag = "foo",
    request_body = CreateFooRequestDto,
    responses(
        (status = 201, description = "Foo created", body = CreateFooResponseDto),
        (status = 400, description = "Bad request")
    )
)]
async fn create<Uc>(
  State(foo_use_case): State<Uc>, Json(payload): Json<CreateFooRequestDto>,
) -> PresentationResult<CreateFooResponseDto>
where
  Uc: FooUseCase,
{
  foo_use_case
    .create(payload.into())
    .await
    .map(|result| AppResponse::new(result.into(), StatusCode::CREATED))
    .map_err(Into::into)
}

/// List foos
///
/// Returns foos ordered by creation date, most recent first
#[utoipa::path(
    get,
    path = "",
    context_path = "/foo",
    tag = "foo",
    params(FindFooRequestDto),
    responses(
        (status = 200, description = "Foos found", body = Vec<FindFooResponseDto>)
    )
)]
async fn find_by_criteria<Uc>(
  State(foo_use_case): State<Uc>, Query(params): Query<FindFooRequestDto>,
) -> PresentationResult<Vec<FindFooResponseDto>>
where
  Uc: FooUseCase,
{
  foo_use_case
    .find_by_criteria(params.into())
    .await
    .map(|results| {
      AppResponse::new(
        results.into_iter().map(Into::into).collect(),
        StatusCode::OK,
      )
    })
    .map_err(Into::into)
}

/// Get a foo by id
#[utoipa::path(
    get,
    path = "/{id}",
    context_path = "/foo",
    tag = "foo",
    params(("id" = i64, Path, description = "Foo id")),
    responses(
        (status = 200, description = "Foo found", body = FindFooResponseDto),
        (status = 404, description = "Foo not found")
    )
)]
async fn find_by_id<Uc>(
  State(foo_use_case): State<Uc>, Path(id): Path<i64>,
) -> PresentationResult<FindFooResponseDto>
where
  Uc: FooUseCase,
{
  foo_use_case
    .find_by_id(id)
    .await
    .map(|result| AppResponse::new(result.into(), StatusCode::OK))
    .map_err(Into::into)
}

pub fn routes<Uc>(foo_use_case: Uc) -> Router
where
  Uc: FooUseCase,
{
  Router::new()
    .route("/", routing::post(create::<Uc>).get(find_by_criteria::<Uc>))
    .route("/{id}", routing::get(find_by_id::<Uc>))
    .with_state(foo_use_case)
}
