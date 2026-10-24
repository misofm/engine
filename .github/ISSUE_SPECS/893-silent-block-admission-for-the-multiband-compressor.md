# Silent block admission for the multiband compressor

## Product outcome

The compressor, EQ and true-peak limiter skip a settled all-`+0.0` block; the multiband compressor has no such path and runs its crossovers and two band detectors on silence. Add the same admission. Class A once the rest state is proven.

## Root evidence

- Since #1745/#1747 the rest contract is `PreparedNativeEffectBank::lanes_at_rest` and `advance_clocks` (`crates/effect-contract/src/lib.rs`, defaults `0` and no-op). The rack ANDs every slot's mask before its whole-chain skip, so a chain holding a multiband slot never skipped a silent block: the rack's skip is the first consumer, the multiband's own in-bank fast path the second.
- Templates: compressor `Instance::render` fast path and `advance_clocks` (`crates/compressor/src/lib.rs`), EQ `advance_clocks` (`crates/parametric-eq/src/lib.rs`); `block_is_positive_zero` (`crates/effect-runtime/src/bank.rs`) refuses `-0.0` and subnormals.
- `crates/multiband-compressor/src/lib.rs` `process_bank` had no silent check.

## Smallest closable slice

Authorized paths (scope note, 2026-10-24): `crates/multiband-compressor/src/lib.rs`, a new `crates/multiband-compressor/src/rest_tests.rs`, and this spec.

Define the multiband's rest state (every crossover integrator, detector envelope and gain smoother at the exact value it reaches on infinite `+0.0` input; ramps settled). When the input block is all `+0.0`, no ramp is in flight, and the state is at rest, return without touching the planes and leave the state unchanged. Prove the rest state is a fixed point of the kernel on `+0.0` input.

## Non-goals

No change to active-block arithmetic; no `-0.0` admission; no change to bypass.

## Objective gates

1. New test: feed 64 blocks of `+0.0` after a loud passage; the outputs and the final state are bit-identical with the admission on and off (this proves the fixed point).
2. New test: a `-0.0` block and a block with one subnormal are not admitted.
3. Existing multiband fixtures pass; `scripts/check-effect-runtime-policy.sh`, `tools/realtime-policy`; `cargo test -p multiband-compressor` green.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`tools/realtime-policy` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Decision record

- 2026-10-24 (scope note, root): implement `lanes_at_rest`, `advance_clocks` and the in-bank fast path. Rest per lane, both channels: the four crossover words and both `gain_db` words exact `+0.0` bits, every ramp of the track settled (`remaining == 0`, step `+0.0`). The silence counter is the clock: `advance_clocks` takes it to `min(run + frames, 2^24)` (`silence_advance`) and is a no-op under prepared bypass. The claim flag lives on the bank and is withdrawn by any automation block, reset and restore.
- Keep the plain `flush`; the compressor's B1 rest snap would move bits (class B, out of scope). Scalar `process` and a mono body are out of scope.
- Gates 1 and 2 fold into one `conformance::rest_probe` driver test (its fork renders the whole kernel; its schedule carries `-0.0` blocks and `1e-40` signal); one more test checks that `advance_clocks` and the fast path leave the words a rendered `+0.0` block leaves.

## Evidence (attempt 1)

- Class A: a one-time digest over bank output and every track payload (W4 and W8, three link modes, bypass on and off, default and 5 ms releases; schedules with signal, long silence, a point, `-0.0` and `1e-40` blocks) is identical before and after; the fast path was taken more than 80,000 times in that run. Not committed.
- Class A after the #1338 merge: the same digest, with crossover points during signal and then rest added to the schedule, is identical on origin/main 40e3b61e9 and branch head 80c300822 (24 configurations). Not committed.
- Driver reach: W4 claims 16,789, earned 6,666; W8 claims 45,588, earned 22,845. Red with the default `lanes_at_rest` (`earned == 0`).

