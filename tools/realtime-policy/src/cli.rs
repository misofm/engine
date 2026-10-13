//! The binary's whole behaviour, with its environment passed in.

use std::ffi::OsString;
use std::io::Write;
use std::path::Path;

use crate::check;
use crate::policy::Policy;

/// Run the tool: zero or one argument, the workspace root (default `.`, resolved against `cwd`).
///
/// Findings go to `err`, one line each, and the exit code is 1. With none, the success line goes
/// to `out` and the exit code is 0. A bad argument list gives 2. A failure to write the output is
/// itself a failure: the code is 1.
pub fn run(
    args: &[OsString],
    cwd: &Path,
    policy: &Policy,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> u8 {
    let root = match args {
        [] => cwd.to_path_buf(),
        [root] => cwd.join(root),
        _ => {
            let _ = writeln!(err, "usage: realtime_policy [workspace-root]");
            return 2;
        }
    };
    let report = check(&root, policy);
    let written = if report.is_ok() {
        writeln!(out, "{}", report.success_line()).and_then(|()| out.flush())
    } else {
        report
            .findings
            .iter()
            .try_for_each(|finding| writeln!(err, "{finding}"))
            .and_then(|()| err.flush())
    };
    match (report.is_ok(), written) {
        (true, Ok(())) => 0,
        _ => 1,
    }
}
