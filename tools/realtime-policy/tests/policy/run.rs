//! `run`: the binary's whole behaviour, with the case policy.

use std::ffi::OsString;
use std::path::Path;

use realtime_policy::run;

use super::common::{BASE_FILES, BASE_REGIONS, Edit, Tree, UNSAFE, policy};

/// Run with `args` in `cwd`; returns the exit code, stdout and stderr.
fn run_in(args: &[&OsString], cwd: &Path) -> (u8, String, String) {
    let args: Vec<OsString> = args.iter().map(|arg| (*arg).clone()).collect();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run(&args, cwd, &policy(), &mut out, &mut err);
    (
        code,
        String::from_utf8(out).expect("stdout is UTF-8"),
        String::from_utf8(err).expect("stderr is UTF-8"),
    )
}

fn success_line() -> String {
    format!("realtime policy: ok ({BASE_REGIONS} marked regions in {BASE_FILES} files)\n")
}

#[test]
fn a_valid_tree_exits_zero_and_prints_the_success_line() {
    let tree = Tree::base("run-valid");
    let (code, out, err) = run_in(&[&tree.root().into()], Path::new("/"));
    assert_eq!((code, out, err), (0, success_line(), String::new()));
}

#[test]
fn findings_exit_one_and_print_one_line_each_on_stderr() {
    let tree = Tree::base("run-failing");
    let bad = format!("{UNSAFE} fn bad() {{}}\n");
    tree.apply(&[Edit::Append {
        path: "crates/engine/src/realtime/buffer.rs",
        text: &bad,
    }]);
    let (code, out, err) = run_in(&[&tree.root().into()], Path::new("/"));
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(
        err,
        format!(
            "realtime policy failure: unsafe code exists outside the issue-approved \
             ownership/audit files: crates/engine/src/realtime/buffer.rs:4: {UNSAFE} fn bad() {{}}\n"
        )
    );
}

#[test]
fn findings_without_a_place_print_without_one() {
    let tree = Tree::base("run-floors");
    tree.apply(&[
        Edit::Delete {
            path: "crates/builtins/src/lib.rs",
        },
        Edit::Create {
            path: "crates/builtins/src/lib.rs",
            text: "fn builtins() {}\n",
        },
    ]);
    let (code, _, err) = run_in(&[&tree.root().into()], Path::new("/"));
    assert_eq!(code, 1);
    assert_eq!(
        err,
        "realtime policy failure: expected at least 26 marked realtime files\n\
         realtime policy failure: expected at least 93 marked realtime regions\n"
    );
}

#[test]
fn a_relative_root_is_resolved_against_the_working_directory() {
    let tree = Tree::base("run-relative");
    let parent = tree.root().parent().expect("the tree has a parent");
    let name = tree
        .root()
        .file_name()
        .expect("the tree has a name")
        .to_owned();
    // The process's own working directory is the package directory, so only `cwd` can find it.
    let (code, out, _) = run_in(&[&name], parent);
    assert_eq!((code, out), (0, success_line()));
}

#[test]
fn no_argument_means_the_working_directory() {
    let tree = Tree::base("run-no-argument");
    let (code, out, _) = run_in(&[], tree.root());
    assert_eq!((code, out), (0, success_line()));
}

#[test]
fn two_arguments_exit_two() {
    let tree = Tree::base("run-two-arguments");
    let root: OsString = tree.root().into();
    let (code, out, err) = run_in(&[&root, &root], tree.root());
    assert_eq!((code, out.as_str()), (2, ""));
    assert!(err.starts_with("usage: "), "usage text expected: {err:?}");
}
