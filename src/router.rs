use crate::options::InputOptions;
use hyper::{Body, Request, Response};
use regex::Regex;
use log::{ info };
use crate::endpoints;
use crate::result::Result;

pub fn to_segments(path: &str, url_patterns: Vec<Regex>) -> Option<Vec<&'_ str>> {
  for pattern in url_patterns {
    if let Some(captures) = pattern.captures(path) {
      return Some(
        captures
          .iter()
          .skip(1)
          .flatten()
          .map(|c| c.as_str())
          .collect::<Vec<_>>(),
      );
    }
  }
  None
}

pub async fn router(
  req: Request<Body>,
  input_options: InputOptions
) -> Result<Response<Body>> {
  info!("Request in router: {}", req.uri().path());
  let url_patterns = vec![
    Regex::new(r"/(index.html)").unwrap(),
    Regex::new(r"/(ngc)").unwrap(),
    Regex::new(r"/(ws)").unwrap(),
  ];

  match to_segments(req.uri().path(), url_patterns).as_deref() {
    Some(["index.html"]) => endpoints::give_client(req, &input_options).await, 
    Some(["ngc"]) => endpoints::post_ngc_file(req, &input_options).await,
    Some(["ws"]) => endpoints::connect_ws(req, &input_options).await,
    _ => endpoints::not_found(req).await,
  }
}
