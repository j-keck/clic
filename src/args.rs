use humantime::parse_duration;
use std::time::Duration;

#[derive(structopt::StructOpt, Debug)]
pub struct Args {
    #[structopt(short, long)]
    /// Command line to execute
    pub cmdline: String,

    #[structopt(short, long)]
    /// Test spec file path
    pub spec: std::path::PathBuf,

    #[structopt(
        short,
        long,
        default_value = "1s",
        parse(try_from_str = parse_duration)
    )]
    pub timeout: Duration,

    #[structopt(
        long = "expected-exit-code",
        default_value = "0",
//        parse(try_from_str = parse_exit_status),
    )]
    pub exit_code: i32,

    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(short = "V", long = "version")]
    pub show_version: bool,
}

// fn parse_exit_status(s: &str) -> Result<ExitStatus, Box<dyn Error>> {
//     Ok(ExitStatus::from_raw(s.parse()?))
// }
