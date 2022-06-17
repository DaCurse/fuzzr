use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_wordlist(file: File) -> impl Iterator<Item = String> {
  BufReader::new(file)
    .lines()
    .map(Result::unwrap)
    .filter(|line| !line.starts_with("#"))
}

fn main() {
  let file = File::open("./wordlist.txt").unwrap();
  for line in read_wordlist(file) {
    println!("{}", line);
  }
}
