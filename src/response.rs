use std::collections::HashMap;

use axum::http::{header, StatusCode};
use napi_derive::napi;
use serde_json::Value;

#[napi(object)]
pub struct ResponseInner {
  pub headers: Option<HashMap<String, String>>,
  pub status: Option<u16>,
  // TODO: more body types
  pub body: String,
}

#[napi]
pub struct Response {}

#[napi]
impl Response {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }

  // TODO: options
  // https://developer.mozilla.org/en-US/docs/Web/API/Response/Response#options
  #[napi]
  pub fn text(body: String) -> ResponseInner {
    ResponseInner {
      headers: None,
      status: None,
      body,
    }
  }

  #[napi]
  pub fn json(value: Value) -> ResponseInner {
    let mut headers = HashMap::new();

    headers.insert(
      header::CONTENT_TYPE.to_string(),
      String::from("application/json"),
    );

    ResponseInner {
      headers: Some(headers),
      status: Some(StatusCode::OK.as_u16()),
      body: value.to_string(),
    }
  }
}
