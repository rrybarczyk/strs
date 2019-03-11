use std::io;
use std::path::PathBuf;
use std::fmt::{Formatter, Display};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MissingFile(PathBuf),
    UnknownFormat(PathBuf),
    InvalidArgs { details: String },
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
      match self {
          Error::Io(err) => write!(f, "{}", err),
          Error::MissingFile(err) => write!(f, "Cannot find file {:?}", err),
          Error::UnknownFormat(path) => write!(f, "Cannot read this format {:?}", path),
          Error::InvalidArgs { details } => write!(f, "{}", details.to_string()),
      }
  }
}


