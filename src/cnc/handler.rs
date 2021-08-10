use futures::{StreamExt, stream::SplitStream};
use hyper::upgrade::Upgraded;
use super::{commands::Command, updates::ProtocolUpdates};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{
  tungstenite::protocol::Message,
WebSocketStream
};

use super::state::CncState;

pub async fn cnc_handler(mut rx: SplitStream<WebSocketStream<Upgraded>>, tx: UnboundedSender<ProtocolUpdates>)
{
  loop {
    match rx.next().await {
    Some(Ok(Message::Text(text))) => {
      let command: Command = serde_json::from_str(&text).unwrap();
    },
    _ => {break;}
  }
  }
}




async fn command_processor(command: Command, sender: UnboundedSender<ProtocolUpdates> ) {
  use Command::*;
  // let cnc_state = CncState::default();

  match command {

    Lcnc(isOn) => {

      if let Err(error) = sender.send(ProtocolUpdates::CncState{app: true, estop: false, power: false}) {
        log::info!("Error while sending: {}", error)
      }
    },

    _ => {
      unimplemented!("Define all arms here");
    }
      
  }
}
