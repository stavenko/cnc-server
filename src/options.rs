use clap::{AppSettings, Clap};
#[derive(Clap, Debug, Clone)]
#[clap(version = "0.0.1", author = "Stavenko V. G. <stavenko@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct InputOptions {
  #[clap(long)]
  pub listen_port: u16,
  #[clap(long)]
  pub path_to_compiled_client: String,
  #[clap(long)]
  pub my_host: String,
  #[clap(long)]
  pub lcnc_rsh_host: String,
  #[clap(long)]
  pub lcnc_rsh_port: u16,
  #[clap(long)]
  pub temp_ngc_files: String,
  
  #[clap(long)]
  pub ssh_user: String,
  #[clap(long)]
  pub cmd: String,
}
