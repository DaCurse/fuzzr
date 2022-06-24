use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use regex::Regex;
use reqwest::blocking::{Client, Response};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn read_wordlist(file: File) -> impl Iterator<Item = String> {
  BufReader::new(file)
    .lines()
    .map(Result::unwrap)
    .filter(|line| !line.is_empty() || !line.starts_with("#"))
}

struct ResponseView {
  status: u16,
  content_length: u64,
  headers: Vec<String>,
  body: String,
}

impl ResponseView {
  fn from_response(response: Response) -> Self {
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

#[allow(unused)]
enum FilterType {
  Status(u16),
  StatusRange(Range<u16>),
  ContentLength(u64),
  ContentLengthRange(Range<u64>),
  Headers(Regex),
  Body(Regex),
}

impl FilterType {
  fn matches(&self, response: &ResponseView) -> bool {
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

enum Filter {
  Allow(FilterType),
  Deny(FilterType),
}

impl Filter {
  fn matches(&self, response: &ResponseView) -> bool {
    match self {
      Self::Allow(f) => f.matches(response),
      Self::Deny(f) => !f.matches(response),
    }
  }
}

fn main() {
  let file = File::open("./wordlist.txt").unwrap();
  let client = Client::builder().build().expect("Failed creating Client");
  let pool = ThreadPoolBuilder::new()
    .num_threads(10)
    .build()
    .expect("Failed creating thread pool");

  let filters = vec![Filter::Allow(FilterType::Body(
    Regex::new("NotFound").unwrap(),
  ))];

  pool.install(|| {
    read_wordlist(file).par_bridge().for_each(|line| {
      let response = client
        .get(format!("http://192.168.2.128/{}", line))
        .send()
        .unwrap();

      let response = ResponseView::from_response(response);

      if filters.iter().all(|f| f.matches(&response)) {
        println!("Matched: /{}", line);
      }
    })
  });
}
