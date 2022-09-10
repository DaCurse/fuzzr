use crate::parser::http::HttpArgs;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[clap(author, version, about, help_heading = "GLOBAL OPTIONS")]
pub struct FuzzrArgs {
  /// Delay between each request
  #[clap(long, value_parser)]
  pub delay: Option<u32>,

  /// Proxy to use for requests
  #[clap(long, value_parser)]
  pub proxy: Option<String>,

  /// Request timeout
  #[clap(long, value_parser, default_value_t = 5)]
  pub timeout: u32,

  /// Number of threads to spawn
  #[clap(short, long, value_parser, default_value_t = 10)]
  pub threads: u16,

  /// Placeholder to use
  #[clap(short, long, value_parser, default_value = "FUZZ")]
  pub placeholder: String,

  /// Output file
  #[clap(short, long, value_parser)]
  pub output: Option<String>,

  #[clap(subcommand)]
  pub mode: FuzzrMode,
}

#[derive(Subcommand, Debug)]
pub enum FuzzrMode {
  /// HTTP fuzzing mode
  Http(HttpArgs),
}
