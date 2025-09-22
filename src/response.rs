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
  let mut builder = axum::response::Response::builder();

  if let Some(status) = res.status {
    builder = builder.status(status);
  }

  if let Some(headers) = res.headers {
    for (key, value) in headers {
      builder = builder.header(key, value);
    }
  }

  builder.body(Body::from(res.body)).unwrap().into_response()
}
