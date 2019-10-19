use clic::*;
use std::error::Error;
use std::fs::read_to_string;
use xstd::prelude::*;

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error>> {
    if args.show_version {
        println!("{}: v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    } else {
        // command to execute
        let (cmd, cmd_args) = {
            if let Some((cmd, cmd_args)) = split_cmdline(&args.cmdline).split_first() {
                (cmd.to_string(), cmd_args.to_vec())
            } else {
                panic!("given command was an empty string");
            }
        };

        // steps to execute / verify
        let steps = read_to_string(args.spec)?
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

        let spec = TestSpec::new(cmd, cmd_args, args.timeout, steps, args.exit_code);
        println!("res: {:?}", spec.execute());
    }

    Ok(())
}

fn split_cmdline(cmdline: &str) -> Vec<String> {
    let mut word = String::new();
    let mut words = Vec::<String>::new();
    let mut quotation_mark: Option<char> = None;

    // some helper
    fn is_quotation_mark(c: char) -> (bool, char) {
        (c == '\'' || c == '"', c)
    }
    fn is_space(c: char) -> bool {
        c.is_ascii_whitespace()
    }
    fn is_masked_quotation(iter: &MementoIter<std::str::Chars>) -> bool {
        iter.prev() == Some(&'\\') && iter.cur().map(|c| is_quotation_mark(*c).0).unwrap_or(false)
    }

    // action
    let mut cursor = cmdline.chars().memento();
    loop {
        if let Some(c) = cursor.next() {
            if is_masked_quotation(&cursor) {
                word.push(c);
            } else if let (true, mark) = is_quotation_mark(c) {
                if quotation_mark == Some(mark) {
                    quotation_mark = None;
                } else if quotation_mark.is_none() {
                    quotation_mark = Some(mark);
                } else {
                    word.push(c);
                }
            } else if is_space(c) && quotation_mark.is_none() {
                words.push(word);
                word = String::new();
            } else {
                word.push(c);
            }
        } else {
            words.push(word);
            break;
        }
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(vec!["ls", "-ltr"], split_cmdline("ls -ltr"));
    }

    #[test]
    fn quotation() {
        assert_eq!(
            vec!["bash", "-c", "echo hey"],
            split_cmdline("bash -c 'echo hey'")
        );
    }

    #[test]
    fn masked_quotation() {
        assert_eq!(
            vec!["bash", "-c", r#"echo \'ls -ltr\'"#],
            split_cmdline(r#"bash -c 'echo \'ls -ltr\''"#)
        );
    }

    #[test]
    fn mixed_quotation() {
        assert_eq!(
            vec!["bash", "-c", r#"echo "i\'m hungry"#],
            split_cmdline(r#"bash -c 'echo "i\'m hungry"#)
        );
    }
}
