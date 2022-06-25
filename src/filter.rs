use regex::Regex;
use reqwest::blocking::Response;
use std::ops::Range;

pub struct ResponseView {
  status: u16,
  content_length: u64,
  headers: Vec<String>,
  body: String,
}

impl ResponseView {
  pub fn from_response(response: Response) -> Self {
    return ResponseView {
      status: response.status().as_u16(),
      content_length: response.content_length().unwrap_or(0),
      headers: response
        .headers()
        .iter()
        .map(|(name, value)| {
          format!("{}: {}", name.as_str(), value.to_str().unwrap())
        })
        .collect(),
      body: response.text().unwrap(),
    };
  }
}

pub enum FilterType {
  Status(u16),
  StatusRange(Range<u16>),
  ContentLength(u64),
  ContentLengthRange(Range<u64>),
  Headers(Regex),
  Body(Regex),
}

impl FilterType {
  pub fn matches(&self, response: &ResponseView) -> bool {
    match self {
      Self::Status(s) => s.to_owned() == response.status,
      Self::StatusRange(r) => r.contains(&response.status),
      Self::ContentLength(cl) => cl.to_owned() == response.content_length,
      Self::ContentLengthRange(r) => r.contains(&response.content_length),
      Self::Headers(r) => response
        .headers
        .iter()
        .any(|header| r.is_match(header.as_str())),
      Self::Body(r) => r.is_match(response.body.as_str()),
    }
  }
}

pub enum Filter {
  Allow(FilterType),
  Deny(FilterType),
}

impl Filter {
  pub fn matches(&self, response: &ResponseView) -> bool {
    match self {
      Self::Allow(f) => f.matches(response),
      Self::Deny(f) => !f.matches(response),
    }
  }
}
