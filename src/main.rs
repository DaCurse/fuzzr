use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_wordlist(file: File) -> impl Iterator<Item = String> {
  BufReader::new(file)
    .lines()
    .map(Result::unwrap)
    .filter(|line| !line.is_empty() || !line.starts_with("#"))
}

fn main() {
  let file = File::open("./wordlist.txt").unwrap();
  let client = Client::builder().build().expect("Failed creating Client");
  let pool = ThreadPoolBuilder::new()
    .num_threads(10)
    .build()
    .expect("Failed creating thread pool");

  pool.install(|| {
    read_wordlist(file).par_bridge().for_each(|line| {
      client
        .get(format!("http://localhost/{}", line))
        .send()
        .unwrap();
    })
  });
}
