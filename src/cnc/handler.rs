use crate::options::InputOptions;

use futures::{
  channel::mpsc::{UnboundedSender},
  Stream, StreamExt,
};

use super::{commands::Command, updates::ProtocolUpdates};
use tokio_tungstenite::{
  tungstenite::{protocol::Message, Result},
};

use super::state::CncState;

pub async fn cnc_handler<St>(config: InputOptions, mut rx: St, tx: UnboundedSender<ProtocolUpdates>)
where
  St: Stream<Item = Result<Message>> + Unpin,
{
  let mut cnc_state = CncState::new(tx, config);
  cnc_state = cnc_state.handle_command(Command::Lcnc(true)).await;
  while let Some(Ok(Message::Text(text))) = rx.next().await {
    let command: Command = serde_json::from_str(&text).unwrap();
    cnc_state = cnc_state.handle_command(command).await;
  }
}

/*
async fn command_processor<'a>(
  config: &'a InputOptions,
  command: Command,
  sender: UnboundedSender<ProtocolUpdates>,
) -> Result<CncState, CncError> {
  use Command::*;

  match command {
    Lcnc(isOn) => {
      let user = config.ssh_user.clone();
      let lcnc_rsh_host = config.lcnc_rsh_host.clone();
      let lcnc_command = config.cmd.clone();

      let process = tokio::spawn(async move {
        let mut cmd = process::Command::new("ssh");
        cmd
          .arg(format!("{}:{}", user, lcnc_rsh_host))
          .arg(format!(r#""{}""#, lcnc_command));
        cmd.spawn()
      });

      if let Err(error) = sender.send(ProtocolUpdates::CncState {
        app: true,
        estop: false,
        power: false,
      }) {
        Err(error)
      } else {

      }
    }

    _ => {
      unimplemented!("Define all arms here");
    }
  }
}
*/
