use crate::result::Result;
use hyper::{Body, Request, Response};

pub async fn handler(_req: Request<Body>) -> Result<Response<Body>> {
  Ok(Response::new(Body::empty()))
}
