use futures::future::{ok, Ready};
use headers::{Connection, Header, SecWebsocketAccept, SecWebsocketKey, Upgrade};
use hyper::{
  body::HttpBody,
  header::{self, HeaderValue},
  upgrade::Upgraded,
  Request, Response, StatusCode,
};
use std::future::Future;
use tokio_tungstenite::{tungstenite::protocol::Role, WebSocketStream};
use super::websocketerror::WebsocketError;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn upgrade<H, R, B>(
  handler: H,
) -> impl Fn(Request<hyper::Body>) -> Ready<Result<Response<B>>> + Send + Sync + 'static
where
  H: Fn(Request<hyper::Body>, WebSocketStream<Upgraded>) -> R + Copy + Send + Sync + 'static,
  R: Future<Output = ()> + Send + 'static,
  B: From<&'static str> + HttpBody + Send + 'static,
{
  move |mut req: Request<hyper::Body>| {
    let sec_key = extract_upgradable_key(&req);

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
          handler(req, ws).await;
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
  }
}

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
  val.into_iter().next().unwrap()
}

fn decode_header<T: Header>(val: &HeaderValue) -> Option<T> {
  let values = [val];
  let mut iter = (&values).iter().copied();
  T::decode(&mut iter).ok()
}
fn some(cond: bool) -> Option<()> {
  if cond {
    Some(())
  } else {
    None
  }
}
