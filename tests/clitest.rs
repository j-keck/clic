use std::process::Command;

#[test]
fn simple_bc_check() {
    assert_eq!(
        clic(&[
            "--cmdline",
            "bc -q",
            "--spec",
            "tests/clitest/bc.spec"
        ]),
        "res: Success\n");
}
#[test]
fn check_exit_code() {
    assert_eq!(
        clic(&[
            "--cmdline",
            "bash -c 'exit 1'",
            "--expected-exit-code",
            "1",
            "--spec",
            "/dev/null"
        ]),
        "res: Success\n"
    );
}

fn clic(args: &[&str]) -> String {
    String::from_utf8_lossy(
        &Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .output()
            .unwrap()
            .stdout,
    )
    .to_string()
}
