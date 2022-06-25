use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod error;
pub mod filter;

pub fn read_wordlist(file: File) -> impl Iterator<Item = String> {
  BufReader::new(file)
    .lines()
    .map(Result::unwrap)
    .filter(|line| !line.is_empty() || !line.starts_with("#"))
}
