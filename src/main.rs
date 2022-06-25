use crate::parser::FuzzrArgs;
use clap::Parser;
use fuzzr::filter::{Filter, FilterType, ResponseView};
use fuzzr::{self};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use regex::Regex;
use reqwest::blocking::Client;
use std::fs::File;

mod parser;

fn main() {
  let args = FuzzrArgs::parse();
  dbg!(args);

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
    fuzzr::read_wordlist(file).par_bridge().for_each(|line| {
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
