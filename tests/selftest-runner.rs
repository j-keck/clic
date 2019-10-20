use env_logger::Env;
use log::{debug, info};
use std::process::Command;
use std::{ffi::OsStr, fs, path::PathBuf};

#[test]
fn selftest_runner() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let spec_files_path = "tests/selftest";
    let entries =
        fs::read_dir(spec_files_path).expect(&format!(r#"path "{}" not found"#, spec_files_path));

    for entry in entries {
        let path = entry.unwrap().path();

        // process only '.spec' files
        if path.as_path().extension() != Some(OsStr::new("spec")) {
            continue;
        }

        info!("run selftest spec: {}", path.display());

        // expected stdout / stderr
        let expected_stdout = expected_for("stdout", &path);
        let expected_stderr = expected_for("stderr", &path);

        // execute 'clic' and collect stdout / stderr
        let (actual_stdout, actual_stderr) = {
            let output = Command::new("cargo")
                .arg("run")
                .arg("--quiet")
                .arg("--")
                .arg("--spec")
                .arg(&path)
                .arg("--quiet")
                .output()
                .unwrap();

            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            debug!(r#"stdout: "{}""#, stdout);

            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            debug!(r#"stderr: "{}""#, stderr);
            (stdout, stderr)
        };

        // validate the output
        assert_eq!(expected_stdout, actual_stdout);
        assert_eq!(expected_stderr, actual_stderr);
    }
}

fn expected_for(id: &str, path: &PathBuf) -> String {
    let path = path.with_extension(id);

    debug!("read expected {} from: {}", id, path.display());
    let s = fs::read_to_string(path).unwrap_or("".to_string());

    debug!(r#"expected {}: "{}""#, id, s);
    s
}
