# Skip the held-peak state machine when hold is zero and decay is off

## Product outcome

The meter inner loop runs a per-sample held-peak state machine with two to three data-dependent branches even though both hosts configure `peak_hold_frames: 0` with decay disabled, where the held value is exactly the running maximum. Add a specialised path for that configuration. Class A: identical published words.

## Root evidence

- `crates/builtins/src/lib.rs:4810` `observe_segment` (and `observe_selected_segment` at `:4858`): the `held` update is an `if / else if / else` on hold and decay state per sample; the doc comment at `:4805-4809` calls it branch-free, which the emitted code contradicts.
- `crates/host-core/src/prepare.rs:1117` sets `peak_hold_frames: 0` and no decay for both hosts.

## Smallest closable slice

Authorized paths: `crates/builtins/src/lib.rs` (meter accumulator only), `crates/builtins/src/tests.rs` or the existing meter test module, and this spec.

At accumulator construction, record `hold_and_decay_disabled = peak_hold_frames == 0 && !decay_enabled`. When true, dispatch to a variant of the segment loop in which `held` is the running `max` (same select form the module already mandates, never `f32::max`) and no hold counter or decay multiply is evaluated. Everything else in the loop is unchanged.

## Non-goals

No change to the energy/RMS accumulation, to sanitised/clipped counting, to snapshot cadence, or to the non-zero-hold path.

## Objective gates

1. New test: for 1 000 random blocks (including `NaN`, `-0.0`, subnormals, `+inf`) the specialised path and the general path with `hold=0, decay off` produce bit-identical snapshots for every metric.
2. New test: the general path still runs when `peak_hold_frames > 0` or decay is enabled, and its output is unchanged against the current fixtures.
3. `scripts/check-builtins-fixtures.sh` and `scripts/check-builtins-policy.sh` pass; `cargo test -p builtins` green.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
