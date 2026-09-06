use axum::extract::FromRequestParts;

use crate::presentation::error::ExtractError;

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ExtractError))]
pub struct Path<T>(pub T);
