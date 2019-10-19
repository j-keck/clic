use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppErr {
    InvalidArgs(String),
    SpawnError(Box<dyn Error>),
    ResponseTimeout,
    UnexpectedResponse { expected: String, actual: String },
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AppErr::*;
        match self {
            InvalidArgs(msg) => write!(f, "Invalid command line: {}", msg),
            SpawnError(e) => write!(f, "Spawn subprocess error: {}", e),
            ResponseTimeout => write!(f, "Timeout waiting for response"),
            UnexpectedResponse { expected, actual } => write!(
                f,
                "Unexpected response\n  expected: '{}'\n  acutal  : '{}'",
                expected, actual
            ),
        }
    }
}

impl std::error::Error for AppErr {}
