use derive_more::Display;
use headers::{Connection, Header, SecWebsocketAccept, SecWebsocketKey, Upgrade};

use futures::future::{ok, Ready};
use hyper::{
  body::HttpBody,
  header::{self, HeaderValue},
  upgrade::Upgraded,
  Request, Response, StatusCode,
};
use routerify::ext::RequestExt;
use std::future::Future;
use tokio_tungstenite::{tungstenite::protocol::Role, WebSocketStream};


type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Display)]
#[display(fmt = "routerify-websocket: {}")]
pub enum WebsocketError {
  /// Websocket upgrade error.
  #[display(fmt = "Websocket upgrade error: {}", _0)]
  Upgrade(BoxError),

  /// Failed to decode the message data as `JSON` in [`message.decode_json()`](./struct.Message.html#method.decode_json) method.
  #[cfg(feature = "json")]
  #[display(fmt = "Failed to decode the message data as JSON: {}", _0)]
  DecodeJson(BoxError),

  /// Failed to convert a struct to `JSON` in [`message.json()`](./struct.Message.html#method.json) method.
  #[cfg(feature = "json")]
  #[display(fmt = "Failed to convert a struct to JSON: {}", _0)]
  EncodeJson(BoxError),

  #[doc(hidden)]
  __Nonexhaustive,
}
pub fn upgrade_ws<H, R, B, D>(
  handler: H,
) -> impl Fn(Request<hyper::Body>) -> Ready<Result<Response<B>, BoxError>> + Send + Sync + 'static
where
  H: Fn(WebSocketStream<Upgraded>, Option<D>) -> R + Copy + Send + Sync + 'static,
  R: Future<Output = ()> + Send + 'static,
  B: From<&'static str> + HttpBody + Send + 'static,
  D: Clone + Send + Sync + 'static,
{
  return move |mut req: Request<hyper::Body>| {
    let sec_key = extract_upgradable_key(&req);
    // let remote_addr = req.remote_addr();

    if sec_key.is_none() {
      return ok(
        Response::builder()
          .status(StatusCode::BAD_REQUEST)
          .body("BAD REQUEST: The request is not websocket".into())
          .unwrap(),
      );
    }

    tokio::spawn(async move {
      match hyper::upgrade::on(&mut req).await {
        Ok(upgraded) => {
          let ws = WebSocketStream::from_raw_socket(upgraded, Role::Server, None).await;
          let data = req.data::<D>().map(Clone::clone);
          handler(ws, data).await;
        }
        Err(err) => log::error!("{}", WebsocketError::Upgrade(err.into())),
      }
    });

    let resp = Response::builder()
      .status(StatusCode::SWITCHING_PROTOCOLS)
      .header(header::CONNECTION, encode_header(Connection::upgrade()))
      .header(header::UPGRADE, encode_header(Upgrade::websocket()))
      .header(
        header::SEC_WEBSOCKET_ACCEPT,
        encode_header(SecWebsocketAccept::from(sec_key.unwrap())),
      )
      .body("".into())
      .unwrap();

    ok(resp)
  };
}

/*
pub fn upgrade_ws<H, R, B, E, D>(
    handler: H,
) -> impl Fn(Request<hyper::Body>) -> Ready<Result<Response<B>, E>> + Send + Sync + 'static
where
    H: Fn(WebSocketStream<Upgraded>, Option<D>) -> R + Copy + Send + Sync + 'static,
    R: Future<Output = ()> + Send + 'static,
    B: From<&'static str> + HttpBody + Send + 'static,
    E: std::error::Error +Sized+ Send + 'static,
    D: Clone + Send + Sync + 'static,
{
    return upgrade_ws_with_config(handler, WebSocketConfig::default());
}
*/

fn extract_upgradable_key(req: &Request<hyper::Body>) -> Option<SecWebsocketKey> {
  let hdrs = req.headers();

  hdrs
    .get(header::CONNECTION)
    .and_then(|val| decode_header::<Connection>(val))
    .and_then(|conn| some(conn.contains("upgrade")))
    .and_then(|_| hdrs.get(header::UPGRADE))
    .and_then(|val| val.to_str().ok())
    .and_then(|val| some(val == "websocket"))
    .and_then(|_| hdrs.get(header::SEC_WEBSOCKET_VERSION))
    .and_then(|val| val.to_str().ok())
    .and_then(|val| some(val == "13"))
    .and_then(|_| hdrs.get(header::SEC_WEBSOCKET_KEY))
    .and_then(|val| decode_header::<SecWebsocketKey>(val))
}

fn encode_header<T: Header>(h: T) -> HeaderValue {
  let mut val = Vec::with_capacity(1);
  h.encode(&mut val);
  val.into_iter().nth(0).unwrap()
}

fn decode_header<T: Header>(val: &HeaderValue) -> Option<T> {
  let values = [val];
  let mut iter = (&values).into_iter().copied();
  T::decode(&mut iter).ok()
}
fn some(cond: bool) -> Option<()> {
  if cond {
    Some(())
  } else {
    None
  }
}
