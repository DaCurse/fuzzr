use fuzzr::error::FuzzrError;
use fuzzr::http::FilterType;
use regex::Regex;

pub trait FilterParser: Sized {
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
