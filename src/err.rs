use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppErr {
    SpawnError(Box<dyn Error>),
    ResponseTimeout,
    UnexpectedResponse { expected: String, actual: String },
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppErr::SpawnError(e) => write!(f, "Spawn subprocess error: {}", e),
            AppErr::ResponseTimeout => write!(f, "Timeout waiting for response"),
            AppErr::UnexpectedResponse { expected, actual } => write!(
                f,
                "Unexpected response\n  expected: '{}'\n  acutal  : '{}'",
                expected, actual
            ),
        }
    }
}

impl std::error::Error for AppErr {}
