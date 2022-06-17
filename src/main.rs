use std::fs::File;
use std::io::{BufRead, BufReader};

struct WordlistIterator {
  reader: BufReader<File>,
}

impl WordlistIterator {
  fn new(file: File) -> Self {
    let reader = BufReader::new(file);
    WordlistIterator { reader }
  }
}

impl Iterator for WordlistIterator {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    let mut line = String::new();
    match self.reader.read_line(&mut line) {
      Ok(0) => None,
      Err(_) => None,
      _ => {
        if line.starts_with("#") {
          self.next()
        } else {
          Some(line.trim_end().to_owned())
        }
      }
    }
  }
}

fn main() {
  let file = File::open("./wordlist.txt").unwrap();
  let wordlist = WordlistIterator::new(file);
  for line in wordlist {
    println!("{}", line);
  }
}
