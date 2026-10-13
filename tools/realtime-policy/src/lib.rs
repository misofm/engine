//! Checks the marked realtime regions (their boundaries, forbidden spellings and `try_pop` drains),
//! the C ABI's whole-plan rule, the browser host's render-locked wrap rule and the unsafe-ownership
//! boundary of the workspace from Rust's own lexer and syntax tree.
//!
//! [`check`] walks `crates`, `hosts` and `tools` under a workspace root and returns a [`Report`].
//! [`run`] is the binary's whole behaviour with its environment passed in. [`Policy`] holds the
//! tool's constants; [`Policy::workspace`] has the real ones.
//!
//! The walk fails closed: a path the tool cannot read, list or classify is a finding, never a
//! skip.

mod boundary;
mod calls;
mod cli;
mod drain;
mod forbidden;
mod leaf;
mod markers;
mod policy;
mod render_wrap;
mod report;
mod source;
mod source_reads;
mod syntax;
mod table;
mod unmarked;
mod unsafe_scan;
mod walk;
mod web_host;
mod whole_plan;

use std::collections::BTreeSet;
use std::path::Path;

pub use cli::run;
pub use policy::{ControlSideEntry, Policy};
pub use report::{Class, Finding, Report};
pub use table::{Caller, RenderThreadExport};

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
    // The trusted-name scan (B2a-D2, D3): whether any file under the roots gives `debug_assert`
    // another meaning, which withdraws the trust from every file's `debug_assert!`, and the other
    // names that no file may redefine. It is known only after every file is read, so the files
    // that are parsed wait for it.
    let mut names = drain::NameScan::default();
    let mut parsed = Vec::new();
    let mut all_files = BTreeSet::new();
    let mut aliases = BTreeSet::new();
    for file in walk::rust_files(workspace_root, &mut findings) {
        all_files.insert(file.path.clone());
        let source = match Source::load(&file.absolute, &file.path) {
            Ok(source) => source,
            Err(finding) => {
                findings.push(finding);
                continue;
            }
        };
        unsafe_scan::scan(&source, policy, &mut findings);
        source_reads::scan(&source, &file.absolute, &mut findings);
        names.read(&source);
        calls::read_aliases(&source, &mut aliases);
        let markers = markers::scan(&source, &mut findings);
        if !markers.regions.is_empty() {
            regions += markers.regions.len();
            files += 1;
            forbidden::scan(&source, &markers.regions, &mut findings);
        }
        // A file that a rule reads is parsed, and a parse failure is a finding. A file that holds
        // a `try_pop` token is parsed too (#1443 D2). A file that only declares modules is parsed
        // for the module graph (D1 rule 3), and a failure there is silent: the file's
        // declarations are then missing from the graph. A module that only this file declares
        // is then not test code (fail closed). A module that a second declaration reaches, where
        // only the unparsable file's declaration is production and the other is `#[cfg(test)]`,
        // is judged test code, and that case is not caught.
        let file = if markers.mentions_marker
            || source.path == whole_plan::CAPI_FFI
            || source.path.starts_with(web_host::WEB_SRC)
        {
            syntax::parse(&source, &mut findings)
        } else if unmarked::holds_pop(&source) {
            syntax::parse_tree(&source, &mut findings)
        } else if unmarked::declares_module(&source) {
            syn::parse_file(&source.text).ok()
        } else {
            None
        };
        if let Some(file) = file {
            parsed.push((source, file, markers.regions));
        }
    }
    let units: Vec<unmarked::Unit<'_>> = parsed
        .iter()
        .map(|(source, file, regions)| unmarked::Unit {
            source,
            file,
            regions,
        })
        .collect();
    let popping = unmarked::check(workspace_root, &all_files, &units, policy, &mut findings);
    calls::check(&units, &popping, &aliases, &mut findings);
    let (debug_assert_trusted, name_findings) = names.finish();
    findings.extend(name_findings);
    let mut capi_ffi = None;
    let mut web_ffi = None;
    let mut web_files = Vec::new();
    for (source, file, file_regions) in &parsed {
        if !file_regions.is_empty() {
            let boundaries = boundary::Boundaries::new(source, file, file_regions);
            boundary::check(source, &boundaries, file_regions, &mut findings);
            drain::check(
                source,
                file,
                &boundaries,
                file_regions,
                debug_assert_trusted,
                &mut findings,
            );
        }
        if source.path == whole_plan::CAPI_FFI {
            capi_ffi = Some((source, file));
        }
        if source.path == render_wrap::WEB_FFI {
            web_ffi = Some((source, file));
        }
        if source.path.starts_with(web_host::WEB_SRC) {
            web_files.push((source, file));
        }
    }
    if !workspace_root.join(whole_plan::CAPI_FFI).is_file() {
        findings.push(Finding::bare(Class::MissingCapiFfi));
    } else if let Some((source, file)) = capi_ffi {
        whole_plan::scan(source, file, &mut findings);
    }
    if !workspace_root.join(render_wrap::WEB_FFI).is_file() {
        findings.push(Finding::bare(Class::MissingWebFfi));
    } else if let Some((source, file)) = web_ffi {
        render_wrap::scan(source, file, policy, &mut findings);
    }
    for path in [web_host::WEB_LIB, web_host::WEB_RENDER_LOCK] {
        if !workspace_root.join(path).is_file() {
            findings.push(Finding::at_path(Class::MissingWebSource, path));
        }
    }
    web_host::scan_modules(&web_files, &mut findings);
    web_host::scan_export_names(&web_files, &mut findings);
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
