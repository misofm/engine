# Compare stored automation bits on AArch64 and across seek times

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A5, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch Q.

## Product outcome

The native host on AArch64 (NEON, FPCR) renders the stored-automation fixtures of draft 23a to the
same bits as the browser and x86-64, in the debug and the shipping release profile. And a session
seek that lands at two different render samples, or at two quanta, gives the same output once the
ramps it started have completed: stored automation after a seek depends only on the session and
the seek target. With draft 23a this completes the evidence for the owner's ruling that stored
automation renders identically on every platform. It adds no new digest owner. Batch Q.

## Context

- **The fixtures and pins.** Draft 23a adds `automation-session.json` and
  `automation-matrix-session.json` to `hosts/host-web/tests/browser-v1/`, their transcript (96
  blocks of exact integer-formula PCM, one session seek at block 40), two native pin tests in
  `hosts/host-web/src/tests.rs` and their pins in `expected.json`. The three older native pin tests
  are `hosts/host-web/src/tests.rs:1632`, `:1751` and `:6690`.
- **AArch64 legs.** `scripts/run-aarch64-tests.sh` runs the product crates' tests in debug (capi's
  workspace closure, `:132-149`) and `lane`, `math`, `console-workload` and the audits in release
  (`:153-195`). `host-web` is in neither list, so no session digest is compared on AArch64 today
  (README A5). The `aarch64-debug` and `aarch64-release` jobs run it on an arm64 runner
  (`.github/workflows/qualification.yml:924-981`).
- **Known defects.** Expected failures are rows of `scripts/lib/aarch64-known-defects.py`, by name
  and reason; a row's test must fail as that test for that reason. LANE-3 (#1019) is open: on
  AArch64 release builds a `max`/`min` fold changes `exp2_lane` and `log2_lane` on NaN and
  signed-zero inputs (`docs/TARGET_MATRIX.md:163-167`).
- **Seeks.** A1.4: where a seek reaches a node, builtin lanes are set exactly and each effect cell
  stages a `Point`; A5: effect parameters settle after their smoothing (64, or 128 for the delay
  time), and effect state after the pre-seek audio's tail reaches exact rest (D15-4). The delay
  time's jump waits for a running crossfade (draft 07's "no restart" rule).

## Decisions frozen for this slice

- **D1. The AArch64 leg.** `scripts/run-aarch64-tests.sh` runs draft 23a's two automation pin
  tests and the three existing native pin tests in both modes:
  `cargo test --locked -p host-web --features test-support --lib -- --exact <names>` in debug,
  and the same with `--release` in release. They compare with the same pins in `expected.json`. A
  failure that is LANE-3's (#1019) is recorded as a row of `aarch64-known-defects.py` naming #1019
  and its reason, by the script's own rule; any other failure fails the leg.
- **D2. Seek-time invariance.** A new test renders `automation-session.json` on two C ABI engines
  with silent sources before the seek and draft 23a's transcript after it. Engine A seeks to `T` at
  render sample `r1`, engine B at `r2`, with `r2 - r1` not a multiple of 64; once at quantum 128
  and once at quantum 100. Aligned at the seek, the two outputs are equal bit for bit from 256
  samples after the seek reaches the output: the longest ramp a seek starts is the delay time's
  held jump plus its crossfade (2 x 128). Builtin lanes are set exactly at the seek, and silent
  inputs leave no pre-seek effect state to wait for.
- **D3. One-ulp evidence (PR evidence, not committed).** With draft 07's `value_at` changed by one
  ulp, both AArch64 legs turn red; revert.

## Deliverables

1. D1's script steps (and a #1019 row only if D1 needs one).
2. D2's test.
3. D3's record in the PR.

## Authorized paths

- `scripts/run-aarch64-tests.sh`, `scripts/lib/aarch64-known-defects.py` (a #1019 row only).
- `crates/capi/src/runtime/tests.rs` (D2).

## Non-goals

- Any engine change. A leg that disagrees is a defect for its own issue, recorded here with its
  first differing sample.
- New fixtures or pins (draft 23a owns them).
- Seek invariance with audible pre-seek audio: its tail is bounded by #1329's rest bound, not by
  the automation, and is not this slice's claim.

## Hazards

- **LANE-3 (#1019).** The fixture's compressor and limiter reach `exp2_lane`. If the AArch64
  release leg differs and #1019 is still open, the difference is that defect, not automation; D1
  records it by the script's rule, and #1019's fix removes the row.
- **No silent skip.** The script refuses a test that returns early on a backend width (`:81-92`).
  The pin tests must run at the width the leg has.
- **Runner time.** The release leg builds `host-web`'s tests in release once more; keep the run to
  the five named tests.

## Objective gates

1. **AArch64.** `bash scripts/run-aarch64-tests.sh debug` and `bash scripts/run-aarch64-tests.sh release`
   pass on the arm64 runner (the `aarch64-debug` and `aarch64-release` jobs), with the five pin
   tests run in each.
2. **Seek-time invariance** (D2): `cargo test --locked -p capi` passes with the new test at both
   quanta.
3. **One ulp turns both AArch64 legs red** (D3, PR evidence).
4. **Workspace.** `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `python3 -B scripts/check-script-reachability.py`.

## Test value

- The AArch64 runs turn red if a NEON build or FPCR changes a bit of a session render. No session
  digest is compared on AArch64 today.
- The seek test turns red if automation after a seek depends on when the seek landed or on the
  quantum, which no existing test checks.

## Dependencies

Batch Q. Direct dependencies:

- Draft 23a *Compare stored automation bits across the browser and native hosts*: fixtures, pins
  and the two pin tests.
- *Make the compressor's AArch64 release build bit-identical to the browser (LANE-3)* (#1019), for
  a clean release leg (D1).

Draft 05 *Seek the timeline and every source in one C ABI call* and draft 07 *Compile stored
automation into per-cell events in node time* arrive through draft 23a.
