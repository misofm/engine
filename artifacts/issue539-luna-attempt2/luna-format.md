Attempt 2 final mechanical-format tranche complete; paused for root checkpoint.

- `cargo fmt --all`: passed.
- `cargo fmt --all -- --check`: passed.
- Private old-behavior/path-selector debug: 1 passed, 29 filtered.
- Mono debug/release: 2 passed each.
- Diff: only the three authorized existing `lib.rs` rustfmt sites.
- Post-format `lib.rs` SHA-256: `677a5596a305039ddbed39d634cde80e90bf99d39439896b1d58a4a539d73b55`
- Git blob: `efbb0251cf2ab006cc901dafeed2e64a5688e7eb`
- `allocation.rs` and `mono_collapse.rs` unchanged.

Fresh retained captures:

`attempt2-format-all`, `attempt2-format-check`, `attempt2-format-private-oldbehavior-pathselector-debug`, `attempt2-format-mono-debug`, `attempt2-format-mono-release` — all status 0 with command JSON/stdout/stderr/status files.

No broader tests, builds, corpus, mutation, Git/GitHub writes, or report file writes. Prior lowering remains attributed to `615787e9`; no old candidate compilation claim is made for the formatted hash.