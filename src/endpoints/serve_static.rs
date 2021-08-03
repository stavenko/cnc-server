use crate::options::InputOptions;
use crate::result::Result;
use hyper::{Body, Request, Response, StatusCode};
use routerify::ext::RequestExt;
use std::path::Path;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn handler(req: Request<Body>) -> Result<Response<Body>> {
  let root_path = if let Some(input_options) = req.data::<InputOptions>() {
    Path::new(&input_options.path_to_compiled_client)
  } else {
    Path::new(".")
  };

  let requested_path = req.uri().path();
  let file_path = root_path.join(&requested_path[1..requested_path.len()]);
  println!("/req {}", requested_path);
  println!("req {}", &requested_path[1..requested_path.len()]);

  if file_path.exists() {
    if let Ok(file) = File::open(file_path.clone()).await {
      let stream = FramedRead::new(file, BytesCodec::new());

      let body = Body::wrap_stream(stream);

      return Ok(Response::new(body));
    }
  }

  let mut resp = Response::new(Body::from(format!(
    "File within path `{}` is not found\nroot_path={}",
    file_path.to_str().unwrap(),
    root_path.to_str().unwrap()
  )));
  *resp.status_mut() = StatusCode::NOT_FOUND;
  return Ok(resp);
}
