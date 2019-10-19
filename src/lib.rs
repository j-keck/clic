mod args;
pub use args::Args;

mod err;
pub use err::AppErr;

mod runner;

mod cmdline;
pub use cmdline::Cmdline;
mod phrases;
pub use phrases::Phrases;

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
    cmdline: Cmdline,
    timeout: Duration,
    steps: Vec<Step>,
    expected_exit_code: i32,
}

impl TestSpec {
    pub fn new(
        cmdline: Cmdline,
        timeout: Duration,
        steps: Vec<Step>,
        expected_exit_code: i32,
    ) -> Self {
        Self {
            cmdline,
            timeout,
            steps,
            expected_exit_code,
        }
    }

    pub fn execute(&self) -> TestResult {
        match runner::run(self) {
            Ok(Some(exit_code)) if exit_code == self.expected_exit_code => TestResult::Success, // OK
            Ok(Some(actual)) => TestResult::UnexpectedExitCode {
                expected: self.expected_exit_code,
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
