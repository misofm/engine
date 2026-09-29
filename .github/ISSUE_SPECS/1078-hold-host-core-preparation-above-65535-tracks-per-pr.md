# Hold host-core preparation above 65,535 tracks per PR

Successor A of #1045 (Sol verdict, attempts 3-5). Engine rule: arbitrary track counts, never a compiled `MAX_TRACKS`. #1045 made every graph-compile, bank-plan, bank-lowering, bind and render layer catch a track cap per PR; host-core preparation is a pre-existing gap (main had it too).

## Smallest closable slice

Prepare a compiled 65,537-track session through `prepare_host_runtime` with a configured later refusal (`maximum_builtin_retained_bytes = 1`), and assert exactly `builtin.resource.limit` at `$.builtin_compile_caps`. About 4.5 s in debug, in `crates/host-core/tests/` (a working probe is in #1045's verification notes, `verify-1045/a3/probe_host_1045.rs`).

## Gates

1. A planted track cap in host-core preparation goes red per PR (in `test-debug-a` and `aarch64-debug`), green when reverted.
2. Per-PR cost stated; clippy, fmt, policy scripts pass.
