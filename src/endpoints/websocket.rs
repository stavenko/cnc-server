use crate::options::InputOptions;
use futures::SinkExt;
use futures::StreamExt;
use hyper::upgrade::Upgraded;
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};

pub async fn connected(ws: WebSocketStream<Upgraded>, _opts: Option<InputOptions>) {
  let (mut client_tx, mut client_rx) = ws.split();

  client_tx.send(Message::Text("OK".into())).await.unwrap();
  client_tx.send(Message::Close(None)).await.unwrap();
  let _recv = client_rx.next().await;
}
