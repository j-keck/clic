use super::{Step, Steps};
use crate::{phrases, Args};
use structopt::StructOpt;

/// parse the given spec file
///
pub fn parse(str: &str) -> (Steps, Option<Args>) {
    let mut lines = str.lines().peekable();

    let args = if lines
        .peek()
        .map(|s| s.starts_with("# clic"))
        .unwrap_or(false)
    {
        Some(parse_header(lines.next().unwrap()))
    } else {
        None
    };

    let steps = lines
        .map(|s| {
            if s.starts_with("# ") {
                let s = s.trim_start_matches("# ").to_string();
                Step::Comment(s.to_string())
            } else if s.starts_with("> ") {
                let s = s.trim_start_matches("> ").to_string();
                Step::Cmd(s.to_string())
            } else if s.starts_with("! ") {
                let s = s.trim_start_matches("! ").to_string();
                Step::Stderr(s)
            } else {
                Step::Stdout(s.to_string())
            }
        })
        .collect();

    (steps, args)
}

fn parse_header(header: &str) -> Args {
    let phrases = phrases::Phrases::parse(header.trim_start_matches("# "));
    Args::from_iter(phrases.vec())
}
