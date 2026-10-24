# Move PDC-staged inputs without the intermediate copy

## Product outcome

An op with plugin-delay compensation copies its producer into a staging buffer and then swaps the staging buffer through the delay ring, one extra full-block pass per delayed edge. Read the producer directly with a three-way move. Class A.

## Root evidence

- `crates/graph/src/runtime.rs:2183` `execute_op`, PDC staging near `:2207-2218`; `pdc_delay_block` (`crates/lane/src/kernels.rs:697`) swaps `io` with the ring.

## Smallest closable slice

Authorized paths: `crates/lane/src/kernels.rs` (a `pdc_delay_move_block(ring, cursor, source, out)`), `crates/lane/tests/`, `crates/graph/src/runtime.rs`, their tests, and this spec.

## Non-goals

No change to PDC sample counts or ring sizing.

## Objective gates

1. New lane test: for random rings, cursors and inputs the new kernel's output and ring state equal copy-then-`pdc_delay_block`.
2. Existing PDC sample-count fixtures and `scripts/check-graph-determinism.sh` pass; `scripts/check-lane-policy.sh`, `tools/realtime-policy`.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`tools/realtime-policy` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Decision record (attempt 1, 2026-10-24)

- Scope note wins over this body for paths and decisions. Root accepted it: R1 keep the staging
  buffer and move into it; R2 one stereo arena borrow so one cursor covers both planes; R3 `source`
  is read-only, the kernel documents and `debug_assert`s non-aliasing.
- `lane::kernels::pdc_delay_move_block(ring, cursor, source, out)`: per segment `out <- ring`, then
  `ring <- source`; same two segments and cursor advance as `pdc_delay_block`, which stays for
  `TrackDelayLine`.
- `DisjointArena::write_read_stereo` already existed (#1201 sidechain reads), so
  `crates/engine/src/realtime/disjoint.rs` is not changed.
- `CompensationDelay::process_from(source, out)` replaces the in-place `process`; a zero-length
  line copies `source` through (the old path left the staged copy in place). The `execute_op`
  staging loop is one borrow and one call.
- Gate 1 (new lane differential) is dropped by the scope note: the restated
  `compensation_delay_is_partition_invariant_and_matches_per_sample_reference` drives
  `process_from` against its independent per-sample oracle and owns MUTATIONS rows M3/M12. The
  #1283 successor-line test's two call sites move to `process_from` mechanically.

## Evidence (attempt 1)

- Mutants, each RED in the restated test and restored: kernel writes `source` into the ring before
  reading its pending word; kernel's first segment has the wrong length (segments misordered);
  `process_from` keeps the entry cursor (M3).
- No bit moved (one-time PR evidence, not a committed test): the 50-session random DAG corpus
  (`fifty_random_dag_sessions_render_deterministic_nonsilent_pcm`, 411 compensation lines over 49
  sessions) renders the same bits on base `ff862018c` and on this change (sha256 of the printed
  bits `99be5099...dabb` on both). `scripts/check-graph-determinism.sh`: PASS 100/100, digest
  identical to base (it covers the compiled graph, not rendered PCM).
- `cargo test -p graph` all pass; rt9 alloc test (release) pass; lane policy ok; realtime policy
  ok (192 regions); fmt, clippy `-p lane -p graph --all-targets --all-features`, aarch64
  `--tests` check and `-D warnings` doc pass. The kernel is not in the wasm `KERNEL_ROSTER`.
- Descriptive benchmark: one `scripts/operator/run-console-benchmark.sh --step b11-899` invocation
  under the exclusive lock refused before the workload (`precondition_loadavg_above_ceiling`,
  loadavg 2.95/6.74/8.72 from other streams; 0 launches, 0 rounds). Not retried and not forced
  uncontrolled; no number is quoted.
