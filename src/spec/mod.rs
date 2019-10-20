use crate::Cmdline;
use crate::TestResult;
use std::time::Duration;
pub mod parser;
mod runner;

#[derive(Debug, PartialEq)]
pub enum Step {
    Comment(String),
    Cmd(String),
    Stdout(String),
    Stderr(String),
}

type Steps = Vec<Step>;

#[derive(Debug, Default)]
pub struct TestSpec {
    cmdline: Cmdline,
    steps: Steps,
    timeout: Duration,
    expected_exit_code: i32,
}
impl TestSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cmdline(&mut self, cmdline: Cmdline) -> &mut Self {
        self.cmdline = cmdline;
        self
    }

    pub fn steps(&mut self, steps: Steps) -> &mut Self {
        self.steps = steps;
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn expected_exit_code(&mut self, expected_exit_code: i32) -> &mut Self {
        self.expected_exit_code = expected_exit_code;
        self
    }

    pub fn execute(&self) -> TestResult {
        match runner::run(self) {
            Ok(Some(exit_code)) if exit_code == self.expected_exit_code => TestResult::Success, // OK
            Ok(Some(exit_code)) => TestResult::UnexpectedExitCode {
                expected: self.expected_exit_code,
                actual: exit_code,
            },
            Ok(None) => TestResult::Failure("process killed per signal".to_string()),
            Err(e) => TestResult::Failure(format!("{}", e)),
        }
    }
}
