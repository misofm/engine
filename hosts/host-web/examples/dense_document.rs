//! Emit the dense console shape of issue #2191 at `session::test_support::DENSE_GATE_TRACKS` (7,000
//! tracks since issue #2219, about 19.2 MB of JSON): every track with four full EQ inserts and a
//! post-pan route to the main output. Its exact retained rows are 16.9 % over the default 512 MiB
//! budget; `scripts/check-web-boot-budget.mjs` boots it in the wasm module and
//! requires the boot's retained projection to refuse it before the linear memory grows past the
//! budget plus one page (issue #2199).
//!
//! The shape is `session::test_support::dense_document` (issue #2219), which
//! `tests/boot_transient_budget.rs` builds for its native twin of that gate.

use std::io::{self, Write as _};

use session::test_support::{DENSE_GATE_TRACKS, dense_document};

fn main() -> io::Result<()> {
    io::stdout().write_all(&dense_document(DENSE_GATE_TRACKS))
}
