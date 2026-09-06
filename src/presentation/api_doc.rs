use utoipa::OpenApi;

use crate::presentation::{
  controller::foo_controller,
  dto::{CreateFooRequestDto, CreateFooResponseDto, FindFooResponseDto},
};

/// API documentation for the entire app
#[derive(OpenApi)]
#[openapi(
    paths(
        foo_controller::create,
        foo_controller::find_by_criteria,
        foo_controller::find_by_id
    ),
    components(
        schemas(
            CreateFooRequestDto,
            CreateFooResponseDto,
            FindFooResponseDto
        )
    ),
    tags((name = "foo", description = "Foo API"))
)]
pub struct ApiDoc;
