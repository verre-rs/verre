use std::collections::HashMap;

use axum::{body::Body, http::Response, response::IntoResponse};
use napi_derive::napi;

#[napi(object, js_name = "Response")]
pub struct VerreResponse {
  pub headers: Option<HashMap<String, String>>,
  pub status: Option<u16>,
  // TODO: more body types
  pub body: String,
}

pub fn into_response(res: VerreResponse) -> Response<Body> {
  let builder = axum::response::Response::builder();

  builder.body(Body::from(res.body)).unwrap().into_response()
}
