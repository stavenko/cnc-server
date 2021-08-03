use hyper::{Body, Request, Response, StatusCode};
use crate::result::Result;
use log::error;

pub async fn handler(req: Request<Body>) -> Result<Response<Body>> {
  let mut response = Response::new(Body::empty());
  *response.status_mut() = StatusCode::NOT_FOUND;
  error!("Resource not found {}", req.uri());
  Ok(response)
}
