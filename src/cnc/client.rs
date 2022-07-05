use tokio::net;
use tokio::sync::mpsc::{UnboundedReceiver,unbounded_channel, UnboundedSender};

pub struct LcncClient {
  host: String,
  port: u16,
  sender:  UnboundedSender<String>,
  recv: UnboundedReceiver<String>
}

/*
impl LcncClient {
  pub fn new(host: String, port: u16) -> Self {
    let (sender, recv) = unbounded_channel();
    LcncClient{host, port, sender, recv}
  }
  pub async fn connect(&mut self) {
    let host =format!("{}:{}", self.host, self.port); 
    for addr in net::lookup_host(host).await.unwrap() {
        println!("socket address is {}", addr);
    }
  }
}
*/
