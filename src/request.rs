use std::collections::HashMap;

use axum::{
  body::{to_bytes, Body},
  http::Request,
};
use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;
use serde_json::Value;

#[napi(js_name = "Request")]
pub struct VerreRequest(Request<Body>);

#[napi]
impl VerreRequest {
  pub fn from_axum(request: Request<Body>) -> Self {
    Self(request)
  }

  // pub fn to_axum(self) -> Request<Body> {
  //   self.0
  // }

  #[napi]
  pub async unsafe fn bytes(&mut self) -> Option<Uint8Array> {
    let body = std::mem::take(self.0.body_mut());

    match to_bytes(body, usize::MAX).await {
      Ok(bytes) => Some(Uint8Array::new(bytes.to_vec())),
      Err(_) => None,
    }
  }

  #[napi]
  pub async unsafe fn json(&mut self) -> Option<Value> {
    let body = std::mem::take(self.0.body_mut());

    let bytes = match to_bytes(body, usize::MAX).await {
      Ok(bytes) => bytes,
      Err(_) => return None,
    };

    match serde_json::from_slice(&bytes) {
      Ok(json_value) => Some(json_value),
      Err(_) => None,
    }
  }

  #[napi(getter)]
  pub fn url(&mut self) -> String {
    self.0.uri().to_string()
  }

  #[napi(getter)]
  pub fn query(&mut self) -> Option<String> {
    self.0.uri().query().map(|query| query.to_string())
  }

  #[napi(getter)]
  pub fn headers(&mut self) -> HashMap<String, String> {
    self
      .0
      .headers()
      .into_iter()
      .filter_map(|(key, value)| {
        let key = key.to_string();
        let value = value.to_str().unwrap().to_string();
        Some((key, value))
      })
      .collect()
  }
}
