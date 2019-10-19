use crate::err::AppErr;

/// command line to execute
///
/// Example:
/// ```
/// let cmdline = clic::Cmdline::from_str(r#"bash -c 'echo i\'m hungry"#).unwrap();
/// assert_eq!("bash", cmdline.cmd);
/// assert_eq!(vec!["-c", r#"echo i\'m hungry"#], cmdline.args);
/// ```
#[derive(Debug)]
pub struct Cmdline {
    pub cmd: String,
    pub args: Vec<String>,
}

impl Cmdline {
    pub fn from_str(cmdline: &str) -> Result<Self, AppErr> {
        if let Some((cmd, args)) = crate::phrases::Phrases::parse(cmdline).vec().split_first() {
            Ok(Cmdline {
                cmd: cmd.to_string(),
                args: args.to_vec(),
            })
        } else {
            Err(AppErr::InvalidArgs("'cmdline' was empty".to_string()))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let cmdline = Cmdline::from_str("ls -ltr").unwrap();
        assert_eq!("ls", cmdline.cmd);
        assert_eq!(vec!["-ltr"], cmdline.args);
    }

    #[test]
    fn single_cmd() {
        let cmdline = Cmdline::from_str("ls").unwrap();
        assert_eq!("ls", cmdline.cmd);
        assert!(cmdline.args.is_empty());
    }

    #[test]
    fn cmd_with_args() {
        let cmdline = Cmdline::from_str("ls -l -t -r").unwrap();
        assert_eq!("ls", cmdline.cmd);
        assert_eq!(vec!["-l", "-t", "-r"], cmdline.args);
    }

    #[test]
    fn empty_cmdline() {
        assert!(Cmdline::from_str("").is_err());
    }
}
