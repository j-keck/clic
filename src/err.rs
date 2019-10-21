use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppErr {
    InvalidArgs(String),
    SpawnError(Box<dyn Error>),
    ResponseTimeout {
        last_cmd: Option<String>,
    },
    UnexpectedResponse {
        last_cmd: Option<String>,
        expected: String,
        actual: String,
    },
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AppErr::*;
        match self {
            InvalidArgs(msg) => write!(f, "Invalid command line: {}", msg),
            SpawnError(e) => write!(f, "Spawn subprocess error: {}", e),
            ResponseTimeout { last_cmd } => write!(
                f,
                "Timeout waiting for response{}",
                last_cmd
                    .as_ref()
                    .map(|cmd| format!(" for input: '{}'", cmd))
                    .unwrap_or("".to_string()),
            ),
            UnexpectedResponse {
                last_cmd,
                expected,
                actual,
            } => write!(
                f,
                "Unexpected response{}\n  expected: '{}'\n  acutal  : '{}'",
                last_cmd
                    .as_ref()
                    .map(|cmd| format!(" for input: '{}'", cmd))
                    .unwrap_or("".to_string()),
                expected,
                actual
            ),
        }
    }
}

impl std::error::Error for AppErr {}
