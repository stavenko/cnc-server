use hyper::{Body, Request, Response};
use crate::result::Result;
use crate::options::InputOptions;

pub async fn handler(req: Request<Body>, options: &InputOptions) -> Result<Response<Body>> {

  Ok(Response::new(Body::empty()))
}
