use super::FilterParser;
use clap::Parser;
use fuzzr::filter::FilterType;
use regex::Regex;

const DEFAULT_USER_AGENT: &str =
  concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Parser, Debug)]
pub struct HttpArgs {
  #[clap(flatten, help_heading = "GENERAL")]
  pub general: HttpGeneralArgs,

  #[clap(flatten, help_heading = "INPUT")]
  pub input: HttpInputArgs,

  #[clap(flatten, help_heading = "FILTER")]
  pub filter: HttpFilterArgs,
}

#[derive(Parser, Debug)]
pub struct HttpGeneralArgs {
  /// Follow redirects
  #[clap(short = 'f', long, value_parser)]
  pub follow_redirect: bool,

  /// Don't validate TLS certificates
  #[clap(short = 'k', long, value_parser)]
  pub insecure: bool,
}

#[derive(Parser, Debug)]
pub struct HttpInputArgs {
  /// The target URL
  #[clap(short, long, value_parser)]
  pub url: String,

  /// User-Agent header value
  #[clap(short = 'a', long, value_parser, default_value = DEFAULT_USER_AGENT)]
  pub user_agent: String,

  /// Add a cookie to the request
  #[clap(short, long, value_parser)]
  pub cookie: Vec<String>,

  /// Add an aditional HTTP header
  #[clap(short = 'H', long, value_parser)]
  pub header: Vec<String>,

  /// HTTP Method to use (default "GET")
  #[clap(short, long, value_parser, default_value = "GET")]
  pub method: String,

  /// Username for Basic Auth
  #[clap(short = 'U', long, value_parser)]
  pub username: Option<String>,

  /// Password for Basic Auth
  #[clap(short = 'P', long, value_parser)]
  pub password: Option<String>,
}

#[derive(Parser, Debug)]
pub struct HttpFilterArgs {
  /// Status code or range to show (Overwritten by --status-hide)
  #[clap(short, long, parse(try_from_str=FilterType::parse_status))]
  pub status: Option<Vec<FilterType>>,
  /// Comma seperated list of statuses and ranges to hide (Overwritten by --status)
  #[clap(short = 'S', long, value_parser)]
  pub status_hide: Option<String>,

  /// Comma seperated list of Content-Lengths and ranges to show (Overwritten by --content-length-hide)
  #[clap(short = 'l', long, value_parser)]
  pub content_length: Option<String>,
  /// Comma seperated list of Content-Lengths and ranges to hide (Overwritten by --content-length)
  #[clap(short = 'L', long, value_parser)]
  pub content_length_hide: Option<String>,

  /// Filter based on response bodies with a Regular Expression
  #[clap(short, long, value_parser)]
  pub regex: Option<Regex>,

  /// Filter based on response headers with a Regular Expression
  #[clap(long, value_parser)]
  pub header_regex: Option<Regex>,
}
