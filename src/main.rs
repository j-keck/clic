use clic::*;
use std::error::Error;
use std::fs::read_to_string;

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error>> {
    if args.show_version {
        println!("{}: v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    } else {
        // steps to execute / verify
        let steps = read_to_string(args.spec.unwrap())?
            .lines()
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

        let spec = TestSpec::new(args.cmdline, args.timeout, steps, args.expected_exit_code);
        println!("res: {:?}", spec.execute());
    }

    Ok(())
}
