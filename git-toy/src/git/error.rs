use std::{error, fmt, result };
use std::ffi::NulError;

#[derive(Debug)]
pub struct Error {
  code: i32,
  message: String,
  class: i32
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
    self.message.fmt(f)
  }
}

impl error::Error for Error {}
impl From<String> for Error {
  fn from(message: String) -> Error {
    Error {code: -1, message, class: 0 }
  }
}

impl From<NulError> for Error {
  fn from(e: NulError) -> Error {
    Error { code: -1, message: e.to_string(), class: 0 }
  }
}

impl Error {
  pub fn throw(code: i32, message: String, class: i32) -> Error {
    Error {
      code,
      message,
      class
    }
  }
}

pub type Result<T> = result::Result<T, Error>;