//! Test files (#1443 D1, rules 1 and 3): a file is test code when it sits under
//! `<package>/tests/`, or when every `mod` declaration that reaches it is test code.
//!
//! A declaration is a `mod x;` item, resolved by Rust's default lookup or by its `#[path]`. A file
//! that no declaration reaches is not test code, and no module is exempt by its name.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;

use syn::Lit;

use super::cfg::is_test;
use crate::drain::bare;

/// A `mod x;` item that reaches a file.
struct Declaration {
    /// The file that holds the item.
    declaring: String,
    /// Whether the item, or a node around it in its file, is test code (rule 2).
    test: bool,
    /// Whether the item resolved through a `#[path]` attribute.
    via_path: bool,
}

/// The module graph of the parsed files.
pub(super) struct Modules<'a> {
    root: &'a Path,
    /// The declarations that reach each file.
    incoming: HashMap<String, Vec<Declaration>>,
    /// The files whose inner attributes make the whole file test code (rule 2).
    inner_test: HashSet<String>,
    /// The files that a `#[path]` in a file outside every `<package>/tests/` resolves to.
    path_targeted: HashSet<String>,
}

impl<'a> Modules<'a> {
    /// Build the graph from the parsed files; `files` holds every `.rs` path of the walk.
    pub(super) fn new<'f>(
        root: &'a Path,
        files: &BTreeSet<String>,
        parsed: impl IntoIterator<Item = (&'f str, &'f syn::File)>,
    ) -> Self {
        let mut modules = Self {
            root,
            incoming: HashMap::new(),
            inner_test: HashSet::new(),
            path_targeted: HashSet::new(),
        };
        for (path, file) in parsed {
            if is_test(&file.attrs) {
                modules.inner_test.insert(path.to_owned());
            }
            let mut walk = Declarations {
                declaring: path,
                files,
                inline: Vec::new(),
                found: Vec::new(),
            };
            walk.items(&file.items, false);
            for (target, declaration) in walk.found {
                if declaration.via_path && !modules.in_package_tests(path) {
                    modules.path_targeted.insert(target.clone());
                }
                modules
                    .incoming
                    .entry(target)
                    .or_default()
                    .push(declaration);
            }
        }
        modules
    }

    /// Whether `path` is test code by rule 1 or rule 3 (or by its own inner attributes).
    pub(super) fn is_test(&self, path: &str) -> bool {
        self.judge(path, &mut Vec::new())
    }

    fn judge(&self, path: &str, visiting: &mut Vec<String>) -> bool {
        if self.inner_test.contains(path) {
            return true;
        }
        if self.in_package_tests(path) && !self.path_targeted.contains(path) {
            return true;
        }
        let Some(declarations) = self.incoming.get(path) else {
            return false;
        };
        if visiting.iter().any(|seen| seen == path) {
            return false;
        }
        visiting.push(path.to_owned());
        let test = declarations
            .iter()
            .all(|declaration| declaration.test || self.judge(&declaration.declaring, visiting));
        visiting.pop();
        test
    }

    /// Rule 1: whether `path` is under `<package>/tests/`, where `<package>` holds a `Cargo.toml`.
    fn in_package_tests(&self, path: &str) -> bool {
        let parts: Vec<&str> = path.split('/').collect();
        (1..parts.len().saturating_sub(1)).any(|at| {
            parts[at] == "tests"
                && self
                    .root
                    .join(parts[..at].join("/"))
                    .join("Cargo.toml")
                    .is_file()
        })
    }
}

/// The `mod x;` items of one file.
struct Declarations<'a> {
    declaring: &'a str,
    files: &'a BTreeSet<String>,
    /// The inline modules around the item, with each one's `#[path]` value.
    inline: Vec<(String, Option<String>)>,
    found: Vec<(String, Declaration)>,
}

impl Declarations<'_> {
    fn items(&mut self, items: &[syn::Item], test: bool) {
        for item in items {
            let syn::Item::Mod(module) = item else {
                continue;
            };
            let test = test || is_test(&module.attrs);
            let path_value = path_attribute(&module.attrs);
            if let Some((_, content)) = &module.content {
                self.inline.push((bare(&module.ident), path_value));
                self.items(content, test);
                self.inline.pop();
                continue;
            }
            for target in self.targets(&bare(&module.ident), path_value.as_deref()) {
                if self.files.contains(&target) {
                    self.found.push((
                        target,
                        Declaration {
                            declaring: self.declaring.to_owned(),
                            test,
                            via_path: path_value.is_some(),
                        },
                    ));
                }
            }
        }
    }

    /// The paths that `mod name;` can resolve to. A file the lookup cannot place with certainty
    /// is given every plausible base: an extra declaration only makes a file less likely to be
    /// test code, never more.
    fn targets(&self, name: &str, path_value: Option<&str>) -> Vec<String> {
        let (directory, file_name) = self
            .declaring
            .rsplit_once('/')
            .unwrap_or(("", self.declaring));
        let stem = file_name.strip_suffix(".rs").unwrap_or(file_name);
        let directory_name = directory.rsplit('/').next().unwrap_or("");
        let mut bases = Vec::new();
        if matches!(file_name, "lib.rs" | "main.rs" | "mod.rs") {
            bases.push(directory.to_owned());
        } else {
            bases.push(join(directory, stem));
            // A crate root other than `lib.rs` and `main.rs` is read like one.
            if file_name == "build.rs"
                || matches!(directory_name, "bin" | "examples" | "benches" | "tests")
            {
                bases.push(directory.to_owned());
            }
        }
        let mut out = Vec::new();
        for base in bases {
            let mut inside = base;
            for (module, path) in &self.inline {
                inside = join(&inside, path.as_deref().unwrap_or(module));
            }
            match path_value {
                // Relative to the declaring file's directory, or to the inline module's.
                Some(value) if self.inline.is_empty() => out.push(join(directory, value)),
                Some(value) => out.push(join(&inside, value)),
                None => {
                    out.push(join(&inside, &format!("{name}.rs")));
                    out.push(join(&join(&inside, name), "mod.rs"));
                }
            }
        }
        out.sort();
        out.dedup();
        out.into_iter().map(|path| normalize(&path)).collect()
    }
}

/// The value of the `#[path = ".."]` attribute, when it is a string literal.
fn path_attribute(attributes: &[syn::Attribute]) -> Option<String> {
    attributes.iter().find_map(|attribute| {
        if !attribute.path().is_ident("path") {
            return None;
        }
        let syn::Meta::NameValue(pair) = &attribute.meta else {
            return None;
        };
        match &pair.value {
            syn::Expr::Lit(literal) => match &literal.lit {
                Lit::Str(value) => Some(value.value()),
                _ => None,
            },
            _ => None,
        }
    })
}

fn join(left: &str, right: &str) -> String {
    if left.is_empty() {
        right.to_owned()
    } else {
        format!("{left}/{right}")
    }
}

/// `path` without `.` components and with each `..` applied to the component before it.
fn normalize(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    parts.join("/")
}
