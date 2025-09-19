use std::sync::Arc;

use axum::body::Body;
use axum::response::{IntoResponse, Response as AxumResponse};
use axum::{routing, Router};
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

use crate::Request;
use crate::ResponseInner;

#[napi]
pub struct Verre(Router);

#[napi]
impl Verre {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self(Router::new())
  }

  async fn use_handler(
    req: axum::http::Request<axum::body::Body>,
    handler: Arc<ThreadsafeFunction<Request, ResponseInner>>,
  ) -> AxumResponse {
    let handler = Arc::clone(&handler);

    let req = Request::from_axum(req);

    let res = handler.call_async(Ok(req)).await.unwrap();

    let builder = axum::response::Response::builder();

    builder.body(Body::from(res.body)).unwrap().into_response()
  }

  #[napi]
  pub fn get(&mut self, path: String, handler: ThreadsafeFunction<Request, ResponseInner>) {
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
