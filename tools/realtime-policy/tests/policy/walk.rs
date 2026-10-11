//! The fail-closed walk (D3), and the files it skips.

use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};

use realtime_policy::Class;

use super::common::{BEGIN, END, Edit, Tree, UNSAFE, assert_classes, expect_failure, expect_valid};

const BUFFER: &str = "crates/engine/src/realtime/buffer.rs";

#[test]
fn a_missing_scan_root_is_a_finding() {
    let tree = Tree::base("walk-missing-tools");
    fs::remove_dir_all(tree.path("tools")).expect("remove the tools root");
    let report = assert_classes(&tree, &[Class::ScanRootUnusable]);
    assert_eq!(report.findings[0].path.as_deref(), Some("tools"));
}

#[test]
fn a_scan_root_that_is_a_file_is_a_finding() {
    let tree = Tree::base("walk-root-is-file");
    fs::remove_dir_all(tree.path("hosts")).expect("remove the hosts root");
    tree.write("hosts", b"not a directory");
    // `hosts/host-web/src/lib.rs` held one region and its file: the floors fire too.
    assert_classes(
        &tree,
        &[
            Class::ScanRootUnusable,
            super::common::FILE_FLOOR,
            super::common::REGION_FLOOR,
        ],
    );
}

#[test]
fn a_missing_realtime_module_is_a_finding() {
    let tree = Tree::base("walk-no-realtime-module");
    for stem in [
        "buffer",
        "disjoint",
        "observe",
        "plan",
        "plan_exchange",
        "spsc",
    ] {
        fs::remove_file(tree.path(&format!("crates/engine/src/realtime/{stem}.rs")))
            .expect("remove a realtime file");
    }
    fs::remove_dir(tree.path("crates/engine/src/realtime")).expect("remove the module");
    // The six files held marked regions: the floors fire with the missing module.
    assert_classes(
        &tree,
        &[
            Class::MissingRealtimeModule,
            super::common::FILE_FLOOR,
            super::common::REGION_FLOOR,
        ],
    );
}

/// The test fails, rather than passes untested, when the directory can still be listed.
#[test]
fn a_directory_that_cannot_be_listed_is_a_finding() {
    let tree = Tree::base("walk-unlistable");
    let locked = tree.path("crates/lane/locked");
    fs::create_dir_all(&locked).expect("create the directory");
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).expect("lock the directory");
    let restore = RestoreMode(locked.clone());
    assert!(
        fs::read_dir(&locked).is_err(),
        "the directory is still listable (are the tests running as root?), so this case would pass untested"
    );
    let report = assert_classes(&tree, &[Class::DirectoryUnlistable]);
    assert_eq!(
        report.findings[0].path.as_deref(),
        Some("crates/lane/locked")
    );
    drop(restore);
}

/// Makes a directory listable again so that the tree can be removed, even when a test fails.
/// Declared after the tree, so it drops first.
struct RestoreMode(std::path::PathBuf);

impl Drop for RestoreMode {
    fn drop(&mut self) {
        let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o755));
    }
}

#[test]
fn a_symbolic_link_to_a_directory_is_a_finding() {
    let tree = Tree::base("walk-dir-link");
    symlink(tree.path("crates/lane"), tree.path("crates/linked")).expect("create the link");
    let report = assert_classes(&tree, &[Class::RefusedSymlink]);
    assert_eq!(report.findings[0].path.as_deref(), Some("crates/linked"));
}

#[test]
fn a_symbolic_link_named_rs_is_a_finding() {
    let tree = Tree::base("walk-rs-link");
    symlink(
        tree.path("crates/lane/src/fpenv.rs"),
        tree.path("crates/lane/src/alias.rs"),
    )
    .expect("create the link");
    // A dangling one hides nothing readable, and is still refused by its name.
    symlink(
        "/nonexistent/target.rs",
        tree.path("crates/lane/src/dangling.rs"),
    )
    .expect("create the dangling link");
    let report = assert_classes(&tree, &[Class::RefusedSymlink]);
    assert_eq!(report.findings.len(), 2, "{:#?}", report.findings);
}

/// Links that cannot hide source are not followed and not refused.
#[test]
fn a_link_to_a_file_with_another_name_is_ignored() {
    let tree = Tree::base("walk-other-links");
    symlink(
        tree.path("crates/lane/src/fpenv.rs"),
        tree.path("crates/lane/src/data.link"),
    )
    .expect("create the link");
    symlink(
        "/nonexistent/target",
        tree.path("crates/lane/src/dangling.link"),
    )
    .expect("create the dangling link");
    assert_classes(&tree, &[]);
}

#[test]
fn a_directory_named_like_a_source_file_is_a_finding() {
    let tree = Tree::base("walk-dir-named-rs");
    fs::create_dir_all(tree.path("crates/lane/src/x.rs")).expect("create the directory");
    let report = assert_classes(&tree, &[Class::NonRegularRustFile]);
    assert_eq!(
        report.findings[0].path.as_deref(),
        Some("crates/lane/src/x.rs")
    );
}

#[test]
fn a_source_file_that_is_not_utf8_is_a_finding() {
    let tree = Tree::base("walk-not-utf8");
    tree.write(
        "crates/lane/src/latin1.rs",
        &[b'/', b'/', b' ', 0xE9, 0xFF, b'\n'],
    );
    let report = assert_classes(&tree, &[Class::NotUtf8]);
    assert_eq!(
        report.findings[0].path.as_deref(),
        Some("crates/lane/src/latin1.rs")
    );
}

#[test]
fn a_source_file_that_cannot_be_read_is_a_finding() {
    let tree = Tree::base("walk-unreadable");
    let path = tree.path("crates/lane/src/secret.rs");
    tree.write("crates/lane/src/secret.rs", b"fn secret() {}\n");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).expect("lock the file");
    assert!(
        fs::read(&path).is_err(),
        "the file is still readable (are the tests running as root?), so this case would pass untested"
    );
    assert_classes(&tree, &[Class::UnreadableFile]);
}

#[test]
fn a_source_file_that_does_not_lex_is_a_finding() {
    // An unterminated string on line 2.
    let report = expect_failure(
        "walk-lex-error",
        &[Edit::Create {
            path: "crates/lane/src/bad.rs",
            text: "fn a() {\n    let s = \"open;\n}\n",
        }],
        &[Class::LexError],
    );
    assert_eq!(report.findings[0].line, Some(2));
}

#[test]
fn a_marked_file_that_does_not_parse_is_a_finding() {
    // The file lexes, and its markers pair: only the parse refuses it.
    let report = expect_failure(
        "walk-parse-error",
        &[Edit::Append {
            path: BUFFER,
            text: "fn broken() { let = ; }\n",
        }],
        &[Class::ParseError],
    );
    assert_eq!(report.findings[0].line, Some(4));
}

#[test]
fn the_c_abi_entry_file_that_does_not_parse_is_a_finding() {
    expect_failure(
        "walk-ffi-parse-error",
        &[Edit::Append {
            path: "crates/capi/src/ffi.rs",
            text: "fn broken() { let = ; }\n",
        }],
        &[Class::ParseError],
    );
}

/// Each row is a node that `syn` keeps as raw tokens, in a marked file.
#[test]
fn syntax_the_parser_keeps_verbatim_is_a_finding() {
    let rows = [
        ("item", "impl const Foo for Bar {}\n"),
        ("trait-item", "trait T { default fn f(); }\n"),
        ("impl-item", "impl X { fn f(); }\n"),
        ("foreign-item", "extern \"C\" { safe fn f(); }\n"),
        ("expression-in-a-function", "fn v() { become v(); }\n"),
        ("pattern", "fn v() { match x { box y => {} } }\n"),
        ("type", "type X = dyn* Foo;\n"),
        ("bound", "fn f<T: [const] Foo>() {}\n"),
    ];
    for (name, text) in rows {
        expect_failure(
            &format!("walk-verbatim-{name}"),
            &[Edit::Append { path: BUFFER, text }],
            &[Class::Verbatim],
        );
    }
}

/// `target` and `node_modules` are skipped at any depth, even with source the tool would refuse.
#[test]
fn target_and_node_modules_are_skipped() {
    let bad = format!("{UNSAFE} fn bad() {{}}\n// {BEGIN}\nfn open() {{\n");
    expect_valid(
        "walk-skipped-directories",
        &[
            Edit::Create {
                path: "crates/lane/target/debug/bad.rs",
                text: &bad,
            },
            Edit::Create {
                path: "tools/audit/node_modules/pkg/bad.rs",
                text: &bad,
            },
            Edit::Create {
                path: "target/bad.rs",
                text: &bad,
            },
        ],
    );
}

/// Only those two names are skipped: a directory with a lookalike name is scanned.
#[test]
fn a_directory_with_a_lookalike_name_is_scanned() {
    let bad = format!("{UNSAFE} fn bad() {{}}\n");
    expect_failure(
        "walk-lookalike-directory",
        &[Edit::Create {
            path: "crates/lane/targets/bad.rs",
            text: &bad,
        }],
        &[Class::UnsafeOutsideOwnership],
    );
}

/// The marker pass reads a file the walk found in a nested directory under every root.
#[test]
fn nested_files_under_every_root_are_found() {
    let text = format!("// {BEGIN}\nfn nested() {{}}\n// {END}\n");
    for path in [
        "crates/a/b/c/nested.rs",
        "hosts/a/b/nested.rs",
        "tools/a/b/c/d/nested.rs",
    ] {
        let tree = Tree::base(&format!("walk-nested-{}", path.replace('/', "-")));
        tree.apply(&[Edit::Create { path, text: &text }]);
        let report = assert_classes(&tree, &[]);
        assert_eq!(report.regions, super::common::BASE_REGIONS + 1, "{path}");
    }
}

/// A FIFO named like a source file is neither a regular file nor a directory nor a link: the walk
/// cannot read it, so it is a finding. (A FIFO is the one such file that `mkfifo` makes without
/// a socket path-length limit.)
#[test]
fn a_fifo_named_like_a_source_file_is_a_finding() {
    let tree = Tree::base("walk-fifo");
    let fifo = tree.path("crates/lane/pipe.rs");
    let status = std::process::Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .expect("run mkfifo");
    assert!(status.success(), "mkfifo failed");
    let report = assert_classes(&tree, &[Class::NonRegularRustFile]);
    assert_eq!(
        report.findings[0].path.as_deref(),
        Some("crates/lane/pipe.rs")
    );
}

/// A FIFO with another name is not source and is ignored.
#[test]
fn a_fifo_with_another_name_is_ignored() {
    let tree = Tree::base("walk-fifo-other-name");
    let status = std::process::Command::new("mkfifo")
        .arg(tree.path("crates/lane/pipe.txt"))
        .status()
        .expect("run mkfifo");
    assert!(status.success(), "mkfifo failed");
    assert_classes(&tree, &[]);
}
