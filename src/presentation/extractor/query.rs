use axum::extract::FromRequestParts;

use crate::presentation::error::ExtractError;

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ExtractError))]
pub struct Query<T>(pub T);
