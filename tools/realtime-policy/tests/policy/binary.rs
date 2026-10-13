//! The built binary, on paths that do not depend on the policy it carries.

use std::fs;
use std::process::Command;

use super::common::Tree;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_realtime_policy"))
}

/// A root with no `tools` directory fails with a failure line, whatever the policy says. The
/// exit code is the whole point: CI reads nothing else.
#[test]
fn a_root_with_a_missing_scan_root_exits_one() {
    let tree = Tree::base("binary-missing-tools");
    fs::remove_dir_all(tree.path("tools")).expect("remove the tools root");
    let output = binary().arg(tree.root()).output().expect("run the binary");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty(), "no success line on failure");
    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert!(
        stderr.lines().any(|line| line
            == "realtime policy failure: scan root is missing or is not a directory: tools"),
        "failure line expected: {stderr}"
    );
}

#[test]
fn two_arguments_exit_two() {
    let output = binary()
        .args(["one", "two"])
        .output()
        .expect("run the binary");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}
