use crate::Cmdline;
use humantime::parse_duration;
use std::{path::PathBuf, time::Duration};

#[derive(structopt::StructOpt, Debug)]
pub struct Args {
    #[structopt(short, long, parse(try_from_str = Cmdline::from_str))]
    /// Command line to execute
    pub cmdline: crate::Cmdline,

    #[structopt(short, long)]
    /// Directory with spec files to execute
    pub dir: Option<PathBuf>,

    #[structopt(short, long)]
    /// Test spec file path
    pub spec: Option<PathBuf>,

    #[structopt(
        short,
        long,
        default_value = "1s",
        parse(try_from_str = parse_duration)
    )]
    pub timeout: Duration,

    #[structopt(long, default_value = "0")]
    /// Expected exit code from the executed program
    pub expected_exit_code: i32,

    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(short = "V", long = "version")]
    pub show_version: bool,
}
