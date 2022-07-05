use crate::cnc::handler::cnc_handler;
use crate::cnc::updates::ProtocolUpdates;
use crate::options::InputOptions;
use futures::channel::mpsc::unbounded;
use futures::StreamExt;
use hyper::upgrade::Upgraded;
use hyper::Body;
use hyper::Request;
use routerify::ext::RequestExt;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub async fn connected(req: Request<Body>, ws: WebSocketStream<Upgraded>) {
  let options = req.data::<InputOptions>().unwrap().to_owned();
  let (client_tx, client_rx) = ws.split();
  let (tx, rx) = unbounded();

  tokio::spawn(rx.map(ProtocolUpdates::json)
    .map(|t| Ok(Message::Text(t)))
    .forward(client_tx)
  );
  cnc_handler(options, client_rx, tx).await;
}
