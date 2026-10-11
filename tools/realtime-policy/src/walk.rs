//! The fail-closed walk over the scan roots.
//!
//! Every condition the walk cannot classify is a finding, never a skip: a missing root, a
//! directory that cannot be listed, a link, a file that is not a regular file.

use std::fs;
use std::path::{Path, PathBuf};

use crate::report::{Class, Finding};

/// The directories the walk visits under the workspace root.
pub(crate) const SCAN_ROOTS: [&str; 3] = ["crates", "hosts", "tools"];

/// The only directories the walk skips, at any depth. Neither holds a compiled `.rs` file.
const SKIPPED_DIRECTORIES: [&str; 2] = ["target", "node_modules"];

/// A `.rs` file found by the walk.
pub(crate) struct RustFile {
    /// The path relative to the workspace root, with `/` separators.
    pub(crate) path: String,
    /// The path on disk.
    pub(crate) absolute: PathBuf,
}

/// Every regular `.rs` file under the scan roots, sorted by path. Findings go to `findings`.
pub(crate) fn rust_files(root: &Path, findings: &mut Vec<Finding>) -> Vec<RustFile> {
    let mut files = Vec::new();
    for scan_root in SCAN_ROOTS {
        let directory = root.join(scan_root);
        match fs::symlink_metadata(&directory) {
            Ok(metadata) if metadata.file_type().is_dir() => {
                walk_directory(&directory, scan_root, &mut files, findings);
            }
            _ => findings.push(Finding::at_path(Class::ScanRootUnusable, scan_root)),
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    files
}

/// Visit one directory; `relative` names it in findings.
fn walk_directory(
    directory: &Path,
    relative: &str,
    files: &mut Vec<RustFile>,
    findings: &mut Vec<Finding>,
) {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(_) => {
            findings.push(Finding::at_path(Class::DirectoryUnlistable, relative));
            return;
        }
    };
    for entry in entries {
        let Ok(entry) = entry else {
            findings.push(Finding::at_path(Class::DirectoryUnlistable, relative));
            continue;
        };
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        let path = format!("{relative}/{name}");
        let rust_name = file_name.as_encoded_bytes().ends_with(b".rs");
        let Ok(kind) = entry.file_type() else {
            findings.push(Finding::at_path(Class::UnreadableFile, &path));
            continue;
        };
        if kind.is_symlink() {
            // A link is followed by nothing. It is a finding when it could hide source: it has a
            // `.rs` name, or it points at a directory (or at something we cannot tell about).
            let hides_source = rust_name
                || match fs::metadata(entry.path()) {
                    Ok(target) => target.is_dir(),
                    Err(error) => error.kind() != std::io::ErrorKind::NotFound,
                };
            if hides_source {
                findings.push(Finding::at_path(Class::RefusedSymlink, &path));
            }
        } else if kind.is_dir() {
            if rust_name {
                findings.push(Finding::at_path(Class::NonRegularRustFile, &path));
            }
            if !SKIPPED_DIRECTORIES.contains(&&*name) {
                walk_directory(&entry.path(), &path, files, findings);
            }
        } else if kind.is_file() {
            if rust_name {
                files.push(RustFile {
                    path,
                    absolute: entry.path(),
                });
            }
        } else if rust_name {
            findings.push(Finding::at_path(Class::NonRegularRustFile, &path));
        }
    }
}
