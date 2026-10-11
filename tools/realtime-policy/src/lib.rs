//! Checks the marked realtime regions (their boundaries, forbidden spellings and `try_pop` drains),
//! the C ABI's whole-plan rule and the unsafe-ownership boundary of the workspace from Rust's own
//! lexer and syntax tree.
//!
//! [`check`] walks `crates`, `hosts` and `tools` under a workspace root and returns a [`Report`].
//! [`run`] is the binary's whole behaviour with its environment passed in. [`Policy`] holds the
//! tool's constants; [`Policy::workspace`] has the real ones.
//!
//! The walk fails closed: a path the tool cannot read, list or classify is a finding, never a
//! skip.

mod boundary;
mod cli;
mod drain;
mod forbidden;
mod leaf;
mod markers;
mod policy;
mod report;
mod source;
mod syntax;
mod unsafe_scan;
mod walk;
mod whole_plan;

use std::path::Path;

pub use cli::run;
pub use policy::Policy;
pub use report::{Class, Finding, Report};

use source::Source;

/// The directory that must exist under the workspace root.
const REALTIME_MODULE: &str = "crates/engine/src/realtime";

/// Check the workspace under `workspace_root` against `policy`.
#[must_use]
pub fn check(workspace_root: &Path, policy: &Policy) -> Report {
    let mut findings = Vec::new();
    if !workspace_root.join(REALTIME_MODULE).is_dir() {
        findings.push(Finding::bare(Class::MissingRealtimeModule));
    }
    let mut regions = 0;
    let mut files = 0;
    // Whether any file under the roots defines or imports a macro named `debug_assert`: then no
    // file's `debug_assert!` is taken at its word (B2a-D2). It is known only after every file is
    // read, so the files that are parsed wait for it.
    let mut debug_assert_shadowed = false;
    let mut parsed = Vec::new();
    for file in walk::rust_files(workspace_root, &mut findings) {
        let source = match Source::load(&file.absolute, &file.path) {
            Ok(source) => source,
            Err(finding) => {
                findings.push(finding);
                continue;
            }
        };
        unsafe_scan::scan(&source, policy, &mut findings);
        debug_assert_shadowed |= drain::debug_assert_shadowed(&source.tokens);
        let markers = markers::scan(&source, &mut findings);
        if !markers.regions.is_empty() {
            regions += markers.regions.len();
            files += 1;
            forbidden::scan(&source, &markers.regions, &mut findings);
        }
        if (markers.mentions_marker || source.path == whole_plan::CAPI_FFI)
            && let Some(file) = syntax::parse(&source, &mut findings)
        {
            parsed.push((source, file, markers.regions));
        }
    }
    let mut capi_ffi = None;
    for (source, file, file_regions) in &parsed {
        if !file_regions.is_empty() {
            let boundaries = boundary::Boundaries::new(source, file, file_regions);
            boundary::check(source, &boundaries, file_regions, &mut findings);
            drain::check(
                source,
                file,
                &boundaries,
                file_regions,
                !debug_assert_shadowed,
                &mut findings,
            );
        }
        if source.path == whole_plan::CAPI_FFI {
            capi_ffi = Some((source, file));
        }
    }
    if !workspace_root.join(whole_plan::CAPI_FFI).is_file() {
        findings.push(Finding::bare(Class::MissingCapiFfi));
    } else if let Some((source, file)) = capi_ffi {
        whole_plan::scan(source, file, &mut findings);
    }
    if files < policy.marked_files_floor {
        findings.push(Finding::bare(Class::MarkedFileFloor(
            policy.marked_files_floor,
        )));
    }
    if regions < policy.marked_regions_floor {
        findings.push(Finding::bare(Class::MarkedRegionFloor(
            policy.marked_regions_floor,
        )));
    }
    findings.sort();
    Report {
        regions,
        files,
        findings,
    }
}
