//! Emit the dense 6,000-track console shape of issue #2191 (about 16.5 MB of JSON): every track
//! with four full EQ inserts and a post-pan route to the main output. Its retained rows exceed the
//! default 512 MiB budget; `scripts/check-web-boot-budget.mjs` boots it in the wasm module and
//! requires the boot's retained projection to refuse it before the linear memory grows past the
//! budget plus one page (issue #2199).
//!
//! The shape is `session::test_support::dense_document` (issue #2219), which
//! `tests/boot_transient_budget.rs` builds for its native twin of that gate.

use std::io::{self, Write as _};

const TRACKS: usize = 6_000;

fn main() -> io::Result<()> {
    io::stdout().write_all(&session::test_support::dense_document(TRACKS))
}
