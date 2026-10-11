//! The base tree, the edits that turn it into a case, and the assertions.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use realtime_policy::{Class, Policy, Report, check};

/// The word that opens an unsafe item. Tests build source text from it so that this file holds no
/// text the repository's older source scan would read as unsafe code.
pub(crate) const UNSAFE: &str = "unsafe";
/// The two marker words, built so that this file holds neither in one piece.
pub(crate) const BEGIN: &str = concat!("REALTIME_POLICY", "_BEGIN");
pub(crate) const END: &str = concat!("REALTIME_POLICY", "_END");

/// The base tree's marked files and regions. The case policy's floors are these counts.
pub(crate) const BASE_FILES: usize = 26;
pub(crate) const BASE_REGIONS: usize = 93;

/// The floor findings that a tree one file and one region short reports.
pub(crate) const FILE_FLOOR: Class = Class::MarkedFileFloor(BASE_FILES);
pub(crate) const REGION_FLOOR: Class = Class::MarkedRegionFloor(BASE_REGIONS);

/// The case policy: the real policy with the floors set to the base tree's counts. The unsafe
/// allowlist stays the real one, because the base tree's unsafe files use its paths.
pub(crate) fn policy() -> Policy {
    Policy {
        marked_files_floor: BASE_FILES,
        marked_regions_floor: BASE_REGIONS,
        ..Policy::workspace()
    }
}

/// One edit of the base tree.
pub(crate) enum Edit<'a> {
    /// Replace the one line that is exactly `old` in `path` with `new` (which may span lines).
    ReplaceLine {
        path: &'a str,
        old: &'a str,
        new: &'a str,
    },
    /// Append `text` to `path`.
    Append { path: &'a str, text: &'a str },
    /// Create `path` with `text`.
    Create { path: &'a str, text: &'a str },
    /// Delete the file `path`.
    Delete { path: &'a str },
}

/// A tree under `CARGO_TARGET_TMPDIR`, removed when it is dropped, even when the test fails.
pub(crate) struct Tree {
    name: String,
    root: PathBuf,
}

impl Tree {
    /// An empty tree for the case `name`.
    pub(crate) fn empty(name: &str) -> Self {
        let root = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("realtime-policy-{name}-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove a leftover case tree");
        }
        fs::create_dir_all(&root).expect("create the case tree");
        Self {
            name: name.to_owned(),
            root,
        }
    }

    /// The base tree: every file under `tests/fixtures/base`, without its `.txt` suffix.
    pub(crate) fn base(name: &str) -> Self {
        let tree = Self::empty(name);
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/base");
        copy_fixtures(&fixtures, &tree.root);
        tree
    }

    /// The tree's root.
    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    /// The path of `relative` in the tree.
    pub(crate) fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    /// The text of `relative`.
    pub(crate) fn read(&self, relative: &str) -> String {
        fs::read_to_string(self.path(relative)).expect("read a tree file")
    }

    /// Write `contents` to `relative`, creating its directories.
    pub(crate) fn write(&self, relative: &str, contents: &[u8]) {
        let path = self.path(relative);
        fs::create_dir_all(path.parent().expect("a file has a parent"))
            .expect("create directories");
        fs::write(path, contents).expect("write a tree file");
    }

    /// Apply the edits in order. A missing anchor or an existing file fails the test.
    pub(crate) fn apply(&self, edits: &[Edit]) {
        for edit in edits {
            match edit {
                Edit::ReplaceLine { path, old, new } => {
                    let text = self.read(path);
                    let mut found = 0;
                    let replaced: Vec<&str> = text
                        .lines()
                        .map(|line| {
                            if line == *old {
                                found += 1;
                                *new
                            } else {
                                line
                            }
                        })
                        .collect();
                    assert_eq!(found, 1, "anchor line {old:?} must occur once in {path}");
                    self.write(path, format!("{}\n", replaced.join("\n")).as_bytes());
                }
                Edit::Append { path, text } => {
                    let mut contents = self.read(path);
                    contents.push_str(text);
                    self.write(path, contents.as_bytes());
                }
                Edit::Create { path, text } => {
                    assert!(!self.path(path).exists(), "{path} must not exist yet");
                    self.write(path, text.as_bytes());
                }
                Edit::Delete { path } => {
                    fs::remove_file(self.path(path)).expect("delete a tree file");
                }
            }
        }
    }

    /// Check the tree against the case policy.
    pub(crate) fn check(&self) -> Report {
        check(&self.root, &policy())
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        let removed = fs::remove_dir_all(&self.root);
        if let Err(error) = removed
            && !std::thread::panicking()
        {
            panic!("remove the case tree {}: {error}", self.root.display());
        }
    }
}

fn copy_fixtures(from: &Path, to: &Path) {
    for entry in fs::read_dir(from).expect("list the fixtures") {
        let entry = entry.expect("read a fixture entry");
        let name = entry
            .file_name()
            .into_string()
            .expect("fixture names are UTF-8");
        if entry.file_type().expect("fixture file type").is_dir() {
            fs::create_dir_all(to.join(&name)).expect("create a fixture directory");
            copy_fixtures(&entry.path(), &to.join(&name));
        } else {
            let target = name
                .strip_suffix(".txt")
                .expect("every fixture file ends in .txt");
            fs::copy(entry.path(), to.join(target)).expect("copy a fixture file");
        }
    }
}

/// Assert that the tree's findings have exactly the classes `expected`.
pub(crate) fn assert_classes(tree: &Tree, expected: &[Class]) -> Report {
    let report = tree.check();
    let expected: BTreeSet<Class> = expected.iter().copied().collect();
    assert_eq!(
        report.classes(),
        expected,
        "case {}: findings: {:#?}",
        tree.name,
        report.findings
    );
    report
}

/// Build the base tree for `name`, apply `edits`, and assert the classes `expected`.
pub(crate) fn expect_failure(name: &str, edits: &[Edit], expected: &[Class]) -> Report {
    assert!(!expected.is_empty(), "a failure case names its classes");
    let tree = Tree::base(name);
    tree.apply(edits);
    assert_classes(&tree, expected)
}

/// Build the base tree for `name`, apply `edits`, and assert that the tree passes.
pub(crate) fn expect_valid(name: &str, edits: &[Edit]) -> Report {
    let tree = Tree::base(name);
    tree.apply(edits);
    assert_classes(&tree, &[])
}
