use clap::Parser;
use clap::Subcommand;
use fuzzr::error::FuzzrError;
use fuzzr::filter::FilterType;
use regex::Regex;

use self::http::HttpArgs;

mod http;

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

trait FilterParser: Sized {
  fn parse_status(arg: &str) -> Result<Self, FuzzrError>;
  fn parse_content_length(arg: &str) -> Result<Self, FuzzrError>;
}

fn parse_status(s: &str) -> Result<u16, FuzzrError> {
  Ok(s.trim().parse()?)
}

impl FilterParser for FilterType {
  fn parse_status(arg: &str) -> Result<Self, FuzzrError> {
    let status_range_re = Regex::new(r#"(\d{3})-(\d{3})"#).unwrap();

    let filter = if status_range_re.is_match(arg) {
      let captures = status_range_re.captures(arg).unwrap();
      let min = parse_status(captures[1].as_ref())?;
      let max = parse_status(captures[2].as_ref())?;
      FilterType::StatusRange(min..max)
    } else {
      FilterType::Status(parse_status(arg)?)
    };

    Ok(filter)
  }

  fn parse_content_length(_arg: &str) -> Result<Self, FuzzrError> {
    todo!()
  }
}
