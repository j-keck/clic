use clic::*;
use std::error::Error;
use std::{fs, ffi::OsStr, io::Write, process};
use log::*;
use env_logger::{Env, Target};

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error>> {
    let default_log_level = match (args.quiet, args.verbose) {
        (true, _) => "warn",
        (_, true) => "debug",
        _         => "info",
    };
    env_logger::from_env(Env::default().default_filter_or(default_log_level))
        .target(Target::Stdout)
        .format(|buf, record| {
            writeln!(buf, "{}", record.args())
        }).init();


    if args.show_version {
        println!("{}: v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    } else
        if let Some(dir) = args.dir.clone() {
            for entry in fs::read_dir(dir)? {
                let path = entry?.path();

                if path.as_path().extension() != Some(OsStr::new("spec")) {
                    debug!("ignore file with invalid extension: {}", path.display());
                    continue;
                }

                run_spec(&args, &path);
            }
        } else if let Some(spec) = args.spec.clone() {
            run_spec(&args, &spec);
        } else {
            error!("'--spec' or '--dir' missing!");
        }

    Ok(())
}


/// runs the given spec
fn run_spec(args: &Args, path: &std::path::Path) {
    let (steps, args_from_spec) =
        spec::parser::parse(&fs::read_to_string(path).unwrap());

    let args = args_from_spec.as_ref().unwrap_or(args);

    if args.cmdline.is_none() {
        error!("'--cmdline' missing!");
    } else {
        info!("validate spec: {}", path.display());
        match spec::TestSpec::new()
            .cmdline(args.cmdline.clone().expect("'cmdline' missing"))
            .steps(steps)
            .timeout(args.timeout)
            .expected_exit_code(args.expected_exit_code)
            .execute()
        {
            TestResult::Success => info!("Success"),
            TestResult::UnexpectedExitCode { expected, actual } => {
                warn!(
                "Failure - unexpected exit code - expected: {}, actual: {}",
                expected, actual
                );
                process::exit(1);
            }
            ,
            TestResult::Failure(msg) => {
                warn!("Failure: {}", msg);
                process::exit(1);
            },
        };
    }
}
