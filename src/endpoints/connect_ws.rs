use hyper::{ StatusCode, Body, Request, Response };
use bytes::Bytes;
use crate::options::InputOptions;
use crate::result::Result;
use hyper::header::{ HeaderValue, SEC_WEBSOCKET_ACCEPT, CONNECTION, UPGRADE, SEC_WEBSOCKET_KEY };
use log::{info, error};
use sha1::{Digest, Sha1};

pub async fn handler(mut req: Request<Body>, options: &InputOptions) -> Result<Response<Body>>{

  let mut res = Response::new(Body::empty());
  info!("go");
  if !req.headers().contains_key(UPGRADE) {
    *res.status_mut() = StatusCode::BAD_REQUEST;
      error!( "No upgrade header in request, supposed to be websocket"
      );
    return Ok(res);
  }

  let key = match req.headers().get(SEC_WEBSOCKET_KEY) {
    Some(key) => key.clone(),
    None => {
      error!( "websocket request contains no SEC_WEBSOCKET_KEY header");
      *res.status_mut() = StatusCode::BAD_REQUEST;
      return Ok(res);
    }
  };

  tokio::task::spawn(async move {
    let uri = req.uri().to_owned();
    match hyper::upgrade::on(&mut req).await {
      Ok(upgraded) => {
        info!("Upgrade websocket successfull {:?}", upgraded);
        crate::endpoints::on_ws_connect(
          upgraded,
        )
        .await;
        info!("websocket routine finished");
      }
      Err(e) => error!("{}", e),
    }
  });

  *res.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
  let mut sha1 = Sha1::default();
  sha1.input(key);
  sha1.input(&b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"[..]);
  let b64 = Bytes::from(base64::encode(&sha1.result()));

  res
    .headers_mut()
    .insert(UPGRADE, HeaderValue::from_static("websocket"));
  res
    .headers_mut()
    .insert(CONNECTION, HeaderValue::from_static("Upgrade"));
  res
    .headers_mut()
    .insert(SEC_WEBSOCKET_ACCEPT, HeaderValue::from_bytes(&b64).unwrap());

  info!("Upgrade");
  Ok(res)
}
