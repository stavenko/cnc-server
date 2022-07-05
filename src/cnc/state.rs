use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use futures::channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::channel::oneshot::{Receiver, channel};
use futures::{SinkExt, StreamExt};
use log::info;
use nalgebra::{Matrix4, Vector2};
use serde::{Deserialize, Serialize};
use thrussh::client::Config;
use thrussh::{ChannelId, client};
use thrussh_keys::{key, load_secret_key};
use tokio::net::{TcpStream, lookup_host};
use tokio::time::sleep;

use crate::cnc::protocol::ssh::{ ClientChannel, ClientOneShot};
use crate::options::InputOptions;

use super::protocol::recv::CncReceiver;
use super::protocol::send::CncSender;
use super::{commands::Command, updates::ProtocolUpdates};

struct CoordinationSystem {
  name: String,
  matrix: Matrix4<f64>,
}

pub struct Axis {
  is_homed: bool,
  current_position: f64,
  limit: Vector2<f64>,
  name: String,
}

#[derive(Serialize, Deserialize)]
pub enum SpindleState {
  Off,
  Cw,
  Ccw,
}

struct Spindle {
  state: SpindleState,
  speed: f64,
  limit: Vector2<f64>,
}

impl Default for Spindle {
  fn default() -> Self {
    Spindle {
      state: SpindleState::Off,
      speed: 0.0,
      limit: Vector2::default(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Program {
  name: String,
  file: String,
  current_line: String,
}

enum CncMode {
  Preparation,
  Milling,
}

struct Machine {
  estop: bool,
  power: bool,
  connected: bool,
}

impl Default for Machine {
  fn default() -> Self {
    Machine {
      estop: false,
      power: false,
      connected: false,
    }
  }
}

pub struct CncState {
  config: InputOptions,
  mode: CncMode,
  axes: Vec<Axis>,
  spindle: Spindle,
  program_loaded: Option<Program>,
  machine: Machine,
  update_state_channel: UnboundedSender<ProtocolUpdates>,
  upstream: UnboundedSender<String>, // send commands to lcnc
  upstream_rx: Option<UnboundedReceiver<String>>, //also send commands
  downstream_rx: Option<UnboundedReceiver<String>>, // this is where we get response from lcnc
}

impl CncState {
  pub fn new(tx: UnboundedSender<ProtocolUpdates>, config: InputOptions) -> Self {
    let (upstream, upstream_rx) = unbounded();
    CncState {
      config,
      mode: CncMode::Preparation,
      axes: Vec::new(),
      spindle: Spindle::default(),
      program_loaded: None,
      machine: Machine::default(),
      update_state_channel: tx,
      upstream,
      upstream_rx: Some(upstream_rx),
      downstream_rx: None,
    }
  }

  pub async fn handle_command(self, command: Command) -> Self {
    match command {
      Command::Lcnc(is_on) => {
        if is_on {
          self.turn_on_lcnc().await
        } else {
          self.turn_off_lcnc().await
        }
      }
      _ => self,
    }
  }

  async fn ssh_session(&self) -> (Receiver<String>, client::Handle<ClientOneShot>) {
    let ip = lookup_host(format!("{}:22",self.config.lcnc_rsh_host)).await.unwrap().next().unwrap();
    let (tx, rx) = channel();
    let client = ClientOneShot::new(tx);
    let config = Arc::new(Config::default());
    
    let mut session = thrussh::client::connect(config, ip, client).await.unwrap();
    if let Ok(true) = session.authenticate_password("pi", "raspberry").await {
      println!("okey");
      (rx, session)
    } else {
      panic!("something terrible");
    }
  }

  async fn ssh_session_process(&self) -> (UnboundedReceiver<String>, client::Handle<ClientChannel>) {
    let ip = lookup_host(format!("{}:22",self.config.lcnc_rsh_host)).await.unwrap().next().unwrap();
    let (tx, rx) = unbounded();
    let client = ClientChannel::new(tx);
    let config = Arc::new(Config::default());
    
    let mut session = thrussh::client::connect(config, ip, client).await.unwrap();
    if let Ok(true) = session.authenticate_password("pi", "raspberry").await {
      println!("okey");
      (rx, session)
    } else {
      panic!("something terrible");
    }
  }

  async fn cmd_output_with_ssh(&self, cmd: &str) -> String {
    let (rx, mut session) = self.ssh_session().await;

    let mut channel = session.channel_open_session().await.unwrap();
    channel.exec(true, cmd).await.unwrap();
    rx.await.unwrap()
  }
  async fn run_process(&self, cmd: &str) -> UnboundedReceiver<String> {
    let (rx, mut session) = self.ssh_session_process().await;

    let mut channel = session.channel_open_session().await.unwrap();
    channel.exec(true, cmd).await.unwrap();
    rx
  }

  async fn run_lcnc(&self) {
    let lcnc_command = self.config.cmd.clone();
    let mut rx = self.run_process(&lcnc_command).await;
    tokio::spawn(async move {
      loop {
        let data = rx.next().await;
        log::info!("DATA: {:#?}", data);
      }
    });
  }


  async fn get_lcnc_pids(&self) -> Vec<String> {
    self.cmd_output_with_ssh("pgrep linuxcnc")
      .await
      .trim()
      .split('\n')
      .map(String::from)
      .collect()
  }

  pub async fn turn_off_lcnc<'a>(self) -> Self {
    /*
    let pids = self.get_lcnc_pids().await;
    if !pids.is_empty() {
      if let Ok(output) = self.cmd_with_ssh(&format!("kill {}", pids)).output().await{
        println!("{}", String::from_utf8(output.stdout).unwrap());
      } else {
        println!("something is bad");
      }
    }
    */
    self
  }

  fn run_downstream_recv_loop(&mut self) {
    let downstream_rx = self.downstream_rx.take();
    if let Some(mut drx) = downstream_rx {
      tokio::spawn(async move {
        while let Some(item) =  drx.next().await {
          log::info!("recv {}", item);
        }
      });
    }
  }

  pub async fn turn_on_lcnc<'a>(mut self) -> Self {
    let pids = self.get_lcnc_pids().await;
    log::info!("linux cnc pids: {:#?}", pids);
    if pids.is_empty() {
      info!("no pids, run lcnc");
      self.run_lcnc().await;
    };

    let addr = SocketAddr::new(
      self.config.lcnc_rsh_host.parse().unwrap(),
      self.config.lcnc_rsh_port,
    );
    let (read, write) = loop {
      match TcpStream::connect(addr).await {
        Err(error) => {
          log::info!("connection failed: {}", error);
          sleep(Duration::from_secs(1)).await;
        }
        Ok(stream) => {
          log::info!("connection established");
          break stream.into_split();
        }
      }
    };
    let (tx, rx) = unbounded();
    tokio::spawn(CncReceiver::new(read).map(Ok).forward(tx));
    self.downstream_rx.replace(rx);
    self.run_downstream_recv_loop();
    let upstream_rx = self.upstream_rx.take().unwrap();
    tokio::spawn(upstream_rx.map(Ok).forward(CncSender::new(write)));
    self.upstream.send("hello EMC azl 1\n".into()).await.unwrap();
    self.upstream.send("set enable EMCTOO\n".into()).await.unwrap();
    self.upstream.send("get joint_limit\n".into()).await.unwrap();
    self
  }
}
