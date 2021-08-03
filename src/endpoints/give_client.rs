use hyper::{Body, Request, Response};
use crate::options::InputOptions;
use crate::result::Result;

pub async fn handler(req: Request<Body>, options: &InputOptions) -> Result<Response<Body>>{

  let body: String = "<html><head> </head> <body><h1>OK</h1> </body> </html>".into();
  Ok(Response::new(Body::from(body)))
}
