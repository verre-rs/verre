use axum::{
  body::{to_bytes, Body},
  http::Request,
};
use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;

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

  #[napi(getter)]
  pub fn url(&mut self) -> String {
    self.0.uri().to_string()
  }
}
