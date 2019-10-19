mod args;
pub use args::Args;

mod err;
pub use err::AppErr;

mod runner;

use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum Step {
    Comment(String),
    Cmd(String),
    Stdout(String),
    Stderr(String),
}

#[derive(Debug)]
pub struct TestSpec {
    cmd: String,
    args: Vec<String>,
    timeout: Duration,
    steps: Vec<Step>,
    exit_code: i32,
}

impl TestSpec {
    pub fn new(
        cmd: String,
        args: Vec<String>,
        timeout: Duration,
        steps: Vec<Step>,
        exit_code: i32,
    ) -> Self {
        Self {
            cmd,
            args,
            timeout,
            steps,
            exit_code,
        }
    }

    pub fn execute(&self) -> TestResult {
        match runner::run(self) {
            Ok(Some(exit_code)) if exit_code == self.exit_code => TestResult::Success, // OK
            Ok(Some(actual)) => TestResult::UnexpectedExitCode {
                expected: self.exit_code,
                actual,
            },
            Ok(None) => TestResult::Failure("kill per signal".to_string()), // FIXME
            Err(e) => TestResult::Failure(format!("{}", e)),
        }
    }
}

#[derive(Debug)]
pub enum TestResult {
    Success,
    UnexpectedExitCode { expected: i32, actual: i32 },
    Failure(String),
}
