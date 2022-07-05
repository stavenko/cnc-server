use futures::{SinkExt, channel::{mpsc::UnboundedSender, oneshot::{self, Sender}}, future::{ready, Ready}};
use log::info;
use thrussh::{client, ChannelId};
use thrussh_keys::key;

pub struct ClientOneShot {
  tx: Option<Sender<String>>,
}

pub struct ClientChannel {
  tx: UnboundedSender<String>
}

impl ClientOneShot {
  pub fn new(tx: Sender<String>) -> Self {
    ClientOneShot{tx: Some(tx)}
  }
}

impl ClientChannel {
  pub fn new(tx: UnboundedSender<String>) -> Self {
    ClientChannel{tx}
  }
}

impl client::Handler for ClientChannel {
  type Error = anyhow::Error;
  type FutureUnit = Ready<Result<(Self, client::Session), Self::Error>>;
  type FutureBool = Ready<Result<(Self, bool), Self::Error>>;

  fn finished_bool(self, b: bool) -> Self::FutureBool {
    ready(Ok((self, b)))
  }
  fn finished(self, session: client::Session) -> Self::FutureUnit {
    ready(Ok((self, session)))
  }
  fn check_server_key(self, server_public_key: &key::PublicKey) -> Self::FutureBool {
    println!("check_server_key: {:?}", server_public_key);
    self.finished_bool(true)
  }
  /*
  fn channel_open_confirmation(
    self,
    channel: ChannelId,
    max_packet_size: u32,
    window_size: u32,
    session: client::Session,
  ) -> Self::FutureUnit {
    println!("channel_open_confirmation: {:?}", channel);
    self.finished(session)
  }
  */

  fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
    println!(
      "data on channel {:?}: {:?}",
      channel,
      std::str::from_utf8(data)
    );
    info!("data");

    {
      let mut tx = self.tx.clone();
      let data = (*data).to_vec();
      tokio::spawn(async move {
        info!("s--------------->");
        tx.send(String::from_utf8(data).unwrap()).await.unwrap();
        info!("<-----------------done");
      });
    }
    self.finished(session)
  }

  fn extended_data(self, channel: ChannelId, ext: u32, data: &[u8], session: client::Session) -> Self::FutureUnit {
      
    info!("ext data");
    self.finished(session)
  }
}

impl client::Handler for ClientOneShot {
  type Error = anyhow::Error;
  type FutureUnit = Ready<Result<(Self, client::Session), Self::Error>>;
  type FutureBool = Ready<Result<(Self, bool), Self::Error>>;

  fn finished_bool(self, b: bool) -> Self::FutureBool {
    ready(Ok((self, b)))
  }
  fn finished(self, session: client::Session) -> Self::FutureUnit {
    ready(Ok((self, session)))
  }
  fn check_server_key(self, server_public_key: &key::PublicKey) -> Self::FutureBool {
    println!("check_server_key: {:?}", server_public_key);
    self.finished_bool(true)
  }
  /*
  fn channel_open_confirmation(
    self,
    channel: ChannelId,
    max_packet_size: u32,
    window_size: u32,
    session: client::Session,
  ) -> Self::FutureUnit {
    println!("channel_open_confirmation: {:?}", channel);
    self.finished(session)
  }
  */

  fn data(mut self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
    println!(
      "data on channel {:?}: {:?}",
      channel,
      std::str::from_utf8(data)
    );
    info!("data");

    if let Some(tx) = self.tx.take() {
      tx.send(String::from_utf8((*data).to_vec()).unwrap()).unwrap();
    }
    self.finished(session)
  }

  fn extended_data(self, channel: ChannelId, ext: u32, data: &[u8], session: client::Session) -> Self::FutureUnit {
      
    info!("ext data");
    self.finished(session)
  }
}
