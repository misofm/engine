# Hold host-core preparation above 65,535 tracks per PR

Successor A of #1045 (Sol verdict, attempts 3-5). Engine rule: arbitrary track counts, never a compiled `MAX_TRACKS`. #1045 made every graph-compile, bank-plan, bank-lowering, bind and render layer catch a track cap per PR; host-core preparation is a pre-existing gap (main had it too).

## Smallest closable slice

Prepare a compiled 65,537-track session through `prepare_host_runtime` with a configured later refusal (`maximum_builtin_retained_bytes = 1`), and assert exactly `builtin.resource.limit` at `$.builtin_compile_caps`. About 4.5 s in debug, in `crates/host-core/tests/` (a working probe is in #1045's verification notes, `verify-1045/a3/probe_host_1045.rs`).

## Gates

1. A planted track cap in host-core preparation goes red per PR (in `test-debug-a` and `aarch64-debug`), green when reverted.
2. Per-PR cost stated; clippy, fmt, policy scripts pass.

## Evidence and decision record (attempt 1, 2026-10-25)

- Test: `crates/host-core/tests/scale.rs` `prepares_65_537_tracks_until_the_configured_builtin_cap`, beside the release-only nightly kitchen-sink row (#1079/#1947) in the same file. Per PR: it is not ignored, so `test-debug-a` and `aarch64-debug` run it with the workspace tests.
- Cost: 2.34 s debug on x86 (one run, load average 6.8/11.6/11.1, descriptive), under the 10 s per-PR budget of the scope note; the nightly fallback was not needed.
- Session built inline from the nine-track fixture, not with `tests/support/edit_sessions.rs::plain_session`: that module needs the `control-provider` feature (it imports `protocol`), and gating `scale.rs` on it would take the nightly row's featureless command down with it.
- The refusal's kind is `PrepareRejection::Builtin` (the builtin compiler's own resource limit), not `Resource` as the scope note guessed; the bytes are `builtin.resource.limit\t$.builtin_compile_caps\n`.
- Red first: `check_counts`'s count helper narrowed to `u16` (`u16::try_from(len).map(u64::from)`) refuses with `host.count\t$` and the test goes red; reverted. A narrowing that truncates silently instead of refusing is caught only where it changes the result before the builtin cap.
