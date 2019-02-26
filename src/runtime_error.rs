use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

#[derive(Debug)]
pub enum ArgError {
    InvalidArgs { details: String },
}

impl std::string::ToString for ArgError {
    fn to_string(&self) -> String {
        match self {
            ArgError::InvalidArgs { details } => details.to_string(),
        }
    }
}
