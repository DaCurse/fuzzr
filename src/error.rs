use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum FuzzrError {
  ParserError,
}

impl Error for FuzzrError {}

impl Display for FuzzrError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      FuzzrError::ParserError => write!(f, "Parser Error"),
    }
  }
}

impl From<ParseIntError> for FuzzrError {
  fn from(_: ParseIntError) -> Self {
    FuzzrError::ParserError
  }
}
