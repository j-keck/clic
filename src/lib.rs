mod args;
pub use args::Args;

mod err;
pub use err::AppErr;

mod cmdline;
pub use cmdline::Cmdline;

mod phrases;

pub mod spec;

#[derive(Debug)]
pub enum TestResult {
    Success,
    UnexpectedExitCode { expected: i32, actual: i32 },
    Failure(String),
}
