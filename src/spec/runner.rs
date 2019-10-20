use super::{Step, TestSpec};
use crate::*;
use log::debug;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};
use timeout_readwrite::TimeoutReader;

pub fn run(spec: &TestSpec) -> Result<Option<i32>, Box<dyn Error>> {
    // execute the test command
    let mut proc = Command::new(spec.cmdline.cmd.clone())
        .args(spec.cmdline.args.clone())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| AppErr::SpawnError(e.into()))?;

    // take program input / output
    let mut proc_stdin = proc.stdin.take().ok_or("unable to take stdin")?;
    let mut proc_stdout = BufReader::new(TimeoutReader::new(
        proc.stdout.take().ok_or("unable to take stdout")?,
        spec.timeout,
    ));
    let mut proc_stderr = BufReader::new(TimeoutReader::new(
        proc.stderr.take().ok_or("unable to take stderr")?,
        spec.timeout,
    ));

    // execute / validate each step
    for step in spec.steps.iter() {
        match step {
            Step::Comment(comment) => debug!("ignore comment: {}", comment),
            Step::Cmd(cmd) => {
                debug!("send to process stdin: {}", cmd);
                writeln!(proc_stdin, "{}", cmd)?;
            }
            Step::Stdout(expected) => {
                debug!("expect on stdout: {}", expected);
                validate_response(&mut proc_stdout, &expected)?;
            }
            Step::Stderr(expected) => {
                debug!("expect on stderr: {}", expected);
                validate_response(&mut proc_stderr, &expected)?;
            }
        }
    }

    // wait for program termination
    // FIXME: use 'try_wait()' in a loop to prevent dead lock
    proc.wait().map(|s| s.code()).map_err(|e| e.into())
}

fn validate_response<R: BufRead>(reader: &mut R, expected: &str) -> Result<(), Box<dyn Error>> {
    let mut actual = String::new();
    reader
        .read_line(&mut actual)
        .map_err::<Box<dyn Error>, _>(|e| match e.kind() {
            std::io::ErrorKind::TimedOut => AppErr::ResponseTimeout.into(),
            _ => e.into(),
        })?;
    let actual = actual.trim_end_matches('\n');

    if expected != actual {
        let expected = expected.to_string();
        let actual = actual.to_string();
        return Err(AppErr::UnexpectedResponse { expected, actual }.into());
    }
    Ok(())
}
