#[macro_use]
extern crate include_repo;

use std::io::Write;

// A pathspec may be used to include only a subset of the git repo's files
const REPO_CARGO_TOML_TAR: &[u8] = include_repo!("Cargo.toml");

fn main() {
    let mut cmd = std::process::Command::new("tar");
    cmd.arg("t")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(&REPO_CARGO_TOML_TAR[..]).unwrap();
    }
    let output = child.wait_with_output().unwrap();
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Cargo.toml");
}
