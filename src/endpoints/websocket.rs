use hyper::upgrade::Upgraded;
use futures::SinkExt;
use futures::StreamExt;
use tokio_tungstenite::{
  tungstenite::protocol::{Message, Role}, WebSocketStream};
use log::info;

pub async fn connected(
  stream: Upgraded
) {

  let ws = WebSocketStream::from_raw_socket(stream, Role::Server, None).await;
  let (mut client_tx, mut client_rx) = ws.split();

  client_tx.send(Message::Text("OK".into())).await.unwrap();
  client_tx.send(Message::Close(None)).await.unwrap();
  let recv = client_rx.next().await;
  info!("asdf {:?}", recv);
  client_tx.close().await.unwrap();
}

