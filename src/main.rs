use clap::Clap;
use hyper::server::Server;
use log::LevelFilter;
use log::{error, info};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use routerify::RouterService;

mod client_config;
mod endpoints;
mod options;
mod result;
mod router;

#[tokio::main]
async fn main() {
  init_logging();

  let input_options = options::InputOptions::parse();
  let router = router::router(input_options.clone());
  let service = RouterService::new(router).unwrap();
  let addr = ([0, 0, 0, 0], input_options.listen_port).into();
  let server = Server::bind(&addr).serve(service);

  if let Err(e) = server.await {
    error!("Server error occured {:?}", format!("{:?}", e));
  } else {
    info!("Gracefully stop server");
  }

}

pub fn init_logging() {
  let stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{d} {l} {m} {n}")))
    .target(Target::Stdout)
    .build();

  let config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .build(Root::builder().appender("stdout").build(LevelFilter::Info))
    .unwrap();

  let _handle = log4rs::init_config(config).unwrap();
}
