use serde::{Deserialize, Serialize};

use crate::options::InputOptions;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClientConfig {
  lcnc_host: String,
}

impl Default for ClientConfig {
  fn default() -> Self {
    ClientConfig {
      lcnc_host: format!("ws://{}:{}/ws", "127.0.0.1", 80),
    }
  }
}

impl ClientConfig {
  pub fn from_options(opts: &InputOptions) -> Self {
    ClientConfig {
      lcnc_host: format!("ws://{}:{}/ws", opts.my_host, opts.listen_port),
    }
  }
}
