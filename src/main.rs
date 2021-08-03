use clap::{ Clap };
use hyper::service::{make_service_fn, service_fn};
use hyper::server::Server;
use log::{ error, info };
use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

mod router;
mod options;
mod endpoints;
mod result;



#[tokio::main]
async fn main() {
  init_logging();

  let input_options = options::InputOptions::parse();

  let addr = ([0, 0, 0, 0], input_options.listen_port).into();
  let make_service = make_service_fn(move |_| {
    let input_options = input_options.clone();
    info!("bbbb");

    async move { 
      info!("asdfas");
      Ok::<_, hyper::Error>(service_fn(move |req| router::router(req, input_options.clone()))) 
    }
  });

  let server = Server::bind(&addr).serve(make_service);

  /*
  let graceful = server.with_graceful_shutdown(async move {
    info!("system signal changed");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
  });
  */

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
