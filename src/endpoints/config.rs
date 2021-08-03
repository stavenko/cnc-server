use crate::client_config::ClientConfig;
use crate::options::InputOptions;
use crate::result::Result;
use hyper::{Body, Request, Response};

use routerify::ext::RequestExt;

pub async fn handler(req: Request<Body>) -> Result<Response<Body>> {
  if let Some(options) = req.data::<InputOptions>() {
    let config = ClientConfig::from_options(options);
    Ok(Response::new(Body::from(
      serde_json::to_string(&config).unwrap(),
    )))
  } else {
    let config = ClientConfig::default();
    Ok(Response::new(Body::from(
      serde_json::to_string(&config).unwrap(),
    )))
  }
}
