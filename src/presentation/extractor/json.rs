use axum::{
  extract::FromRequest,
  response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::presentation::error::ExtractError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ExtractError))]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
  T: Serialize,
  axum::Json<T>: IntoResponse,
{
  fn into_response(self) -> Response {
    axum::Json(self.0).into_response()
  }
}
