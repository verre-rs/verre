use std::sync::Arc;

use axum::response::Response;
use axum::{body::Body, http::Request};
use axum::{routing, Router};
use napi::bindgen_prelude::Promise;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::Either;
use napi_derive::napi;

use crate::{into_response, VerreRequest, VerreResponse};

#[napi]
pub struct Verre(Router);

#[napi]
impl Verre {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self(Router::new())
  }

  async fn use_handler(
    req: Request<Body>,
    handler: Arc<ThreadsafeFunction<VerreRequest, Either<VerreResponse, Promise<VerreResponse>>>>,
  ) -> Response {
    let handler = Arc::clone(&handler);

    let req = VerreRequest::from_axum(req);

    let res = match handler.call_async(Ok(req)).await.unwrap() {
      Either::A(res) => res,
      Either::B(promise) => promise.await.unwrap(),
    };

    into_response(res)
  }

  #[napi]
  pub fn get(
    &mut self,
    path: String,
    handler: ThreadsafeFunction<VerreRequest, Either<VerreResponse, Promise<VerreResponse>>>,
  ) {
    let handler = Arc::new(handler);

    self.0 = self.0.clone().route(
      &path,
      routing::get(|req| async move { Self::use_handler(req, handler).await }),
    );
  }

  #[napi]
  pub async unsafe fn serve(&mut self) {
    // TODO: custom port, host
    let listener = napi::tokio::net::TcpListener::bind("127.0.0.1:3000")
      .await
      .unwrap();

    axum::serve(listener, self.0.clone()).await.unwrap()
  }
}

impl Default for Verre {
  fn default() -> Self {
    Self::new()
  }
}
