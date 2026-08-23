# 048 Launch gate reset, restore, and recovery proof

## Outcome

Close the stopped launch gate/expander product with the four missing effect-local state proofs:
both resets, uninterrupted active snapshot/restore continuation, signed-zero identity, and
lane-local scalar/W8 recovery parity.

## Context

Issue **Launch hysteretic peak gate/expander** exhausted its two attempts without overall PASS.
Its descriptor, scalar, W4/W8 bank, registry/graph/PDC, all-rate latency/resource, sidechain,
automation and policy checkpoints are accepted only as technical input. Final Sol review found no
production defect, but static review could not substitute for the missing objective state proofs.

This stateless successor has exactly **two total attempts**: one Terra implementation/review and
one bounded Sol correction/review. A second failure stops. No benchmark or timing command is
authorized; `timed_benchmark_invocations=0` is invariant.

## Scope

- Add effect-local assertions for `FullToDefaults` and `DiscontinuityKeepParameters` on scalar and
  an executed available-host W8 bank.
- Compare an active uninterrupted scalar and W8 continuation against fresh instances restored
  from the exact layout-1 snapshot.
- Prove negative-zero main audio survives the fixed delay and gain-identity select bit-exactly.
- Inject one nonfinite computed lane state in test-only access and prove isolated scalar/W8 output,
  state and `ProcessReport` recovery parity.
- Make only a directly exposed bounded gate/expander state/reset/recovery repair if a proof fails,
  then seal the unchanged product with focused/workspace/policy gates.

## Frozen contracts

Reuse Issue 014 and its tracked brief without changing descriptor, equations, coefficient graph,
parameter domains/order, automation, latency/tail, payload layout, resource rows, sidechain policy,
bank operation graph, graph/PDC or public APIs.

At 48 kHz and quantum 128, `FullToDefaults` must clear both rings/cursor, restore the originally
prepared eight lane values and four fixed ramps, rederive detector delay/hold/coefficients, and set
`Open`, `hold_remaining=K`, `G=+0`. `DiscontinuityKeepParameters` must clear rings/cursor, retain
IDs 5–8, snap each ID 1–4 ramp current to target with zero remaining, and set the same Open/K/+0
runtime state. A later full reset must still restore the original prepared defaults. Bank reset
applies independently to every track and lane.

The active continuation begins only after the 480-sample delay and reaches nonzero attenuation.
Snapshot while both a nonzero gain state and a 64-update ramp are active; restore into a freshly
prepared peer, then feed identical asymmetric finite-normal audio and canonical automation/block
partitions. Uninterrupted and restored scalar PCM, payload and reports are byte-exact. The existing
W8 carried-state test must likewise compare its restored bank against the uninterrupted bank, not
only against peers restored from the same payload.

For signed-zero identity, keep `G=+0` through the delayed sample, send `-0` left and `+0` right,
and require exact output bits at sample 480 with zero sanitation/recovery. For recovery, warm the
delay with nonzero finite audio, test-only inject a nonfinite `G` into exactly one left lane, and
process one frame. Scalar must emit that lane's delayed dry sample, reset only that lane to
Open/K/+0 and increment only `nonfinite_left_blocks`. An executed W8 bank with the same injected
track/lane must match eight scalar peers exactly for output, per-track payload and reports; every
other track/right lane remains unchanged. A skipped/unavailable W8 path is not PASS evidence.

## Deliverables

- the smallest gate/expander unit-test correction and any directly exposed bounded production fix;
- exact scalar/W8 reset, continuation, signed-zero and recovery evidence; and
- final focused, workspace and relevant policy results with candid Terra/Sol verdicts.

## Explicit non-goals

DSP redesign; descriptor, domain, topology, state-layout, latency/tail, resource or public-API
change; new graph/session/core architecture; corpus or oracle expansion; randomized/million-sample
matrices; realtime audit; cross-target or object inspection; benchmark/preflight/timing;
performance work; audition or listening. All broad qualification remains Issue 047.

## Dependencies by exact issue title

- Launch hysteretic peak gate/expander
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels

The stopped Issue-014 dependency contributes only its explicitly preserved green checkpoint, not
PASS.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after local/remote Issue 048 synchronization.** The authoritative
tracked brief is
`.github/ISSUE_SPECS/BRIEFS/048-launch-gate-reset-restore-and-recovery-proof.md`. This checkpoint
changes no production code and authorizes no benchmark.

## Acceptance gates

1. Scalar and executed W8 snapshots prove both reset kinds word-for-word, including original
   defaults, retained preparation values, snapped ramps, cleared rings/cursors and Open/K/+0 state.
2. Active post-latency scalar and W8 snapshots restore transactionally and continue byte-exactly
   against uninterrupted instances for PCM, complete per-lane payload and reports.
3. Gain identity preserves `-0`/`+0` bits through exact latency with zero sanitation/recovery.
4. One injected nonfinite computed left-lane state produces exact delayed dry recovery and
   scalar/W8 output/state/report parity while every other lane/track is unchanged.
5. Descriptor/resource/metadata bytes remain unchanged. Focused locked gate/core tests, format,
   warning-denied Clippy, locked workspace check/tests, warning-denied workspace Clippy/rustdoc,
   and applicable workspace/realtime/effect-runtime/rack/graph policies and mutations pass.
6. No Issue-047 gate, audit main, target/object command, benchmark, timing or listening work runs;
   `timed_benchmark_invocations=0`.

## Target matrix

Execute scalar and the available x86 W8 backend on the candidate host. Existing W4 source/core
contracts remain frozen but are not requalified here. Cross-target and instruction evidence stays
in Issue 047.

## Required evidence

Preserved Issue-014 candidate plus Issue-048 candidate hash; exact reset payload words; active
continuation boundary and equality results; signed-zero output bits; injected recovery track/lane,
output/state/report identities; W8 execution proof; focused/full/policy outputs; unchanged public
API/resource statement; attempt count; Terra/final Sol PASS/FAIL; and
`timed_benchmark_invocations=0`.

## Terra attempt 1 evidence — FAIL

- Candidate base: `cfdc76f`; this attempt made no production, public API, descriptor, resource,
  core, registry, graph, PDC, corpus, audit, target/object, benchmark, timing, or listening
  change.
- `cargo fmt --check --package miso-engine-gate-expander`: PASS after the mechanical Rustfmt
  rewrite requested during the attempt.
- `cargo test --locked -p miso-engine-gate-expander --lib`: FAIL. Eight tests passed and one
  failed: `tests::active_snapshot_restore_continues_against_uninterrupted_scalar_and_w8` at
  `crates/miso-engine-gate-expander/src/lib.rs:1576`, `W8 scalar left track 0, frame 0`, with
  output bits `918470466` versus `918595474`.
- Focused all-target Clippy was not run after the semantic test failure. No repair was applied.
- The reset and active-continuation probes executed an available x86 W8 path. Signed-zero and
  injected lane-local recovery probes were not completed before the mandatory stop at the active
  continuation failure; they are not claimed as evidence.
- `timed_benchmark_invocations=0`.

**Terra attempt 1 verdict: FAIL.** The exact W8/scalar continuation mismatch must be classified
before any bounded correction; this attempt does not claim the remaining acceptance gates.

## Sol attempt 2 evidence — PASS

- Candidate base: `ffc1c53`. The reported bits `918470466` (`0x36bebf42`) versus `918595474`
  (`0x36c0a792`) were a test-oracle indexing defect, not a production bank-state, automation or
  restore defect. A public-API diagnostic showed all eight bank/scalar outputs and payloads equal
  at the active snapshot boundary, transactional restore preserving every payload, and the first
  continuation frame matching every corresponding lane. The helper re-enumerated each one-track
  scalar slice as packed lane zero, so outer track one was incorrectly compared with W8 lane zero
  and mislabeled as track zero.
- The bounded correction is confined to the gate/expander test module. It indexes the packed W8
  frame by the actual outer track and completes only the frozen signed-zero and injected-recovery
  assertions. Production code, public APIs, descriptor/metadata bytes, state/resource layouts,
  core kernels, registry, graph and PDC are unchanged.
- `cargo test --locked -p miso-engine-gate-expander --lib`: PASS, 11/11. Both reset kinds are
  word-exact for scalar and an executed x86 W8 bank; active post-latency scalar and W8 restore
  continue exactly against uninterrupted instances; delayed `-0` left and `+0` right retain their
  bits at gain identity; and one injected nonfinite track-three left-lane gain recovers to delayed
  dry/Open/K/+0 with exact scalar/W8 PCM, payload and `ProcessReport` parity while the other
  tracks and right lanes remain unchanged.
- Focused dependency and integration gates PASS: locked checks for core, gate/expander,
  effect-compiler and graph-compiler; the two focused core gate-gain tests; all four native-session
  tests; the width-correct launch gate/expander graph fixture; package format; and warning-denied
  all-target/all-feature Clippy for those four crates.
- Workspace, realtime, effect-runtime, rack and graph policy checks PASS. The workspace,
  realtime and effect-runtime mutation suites and the rack mutation suite also PASS.
- Final candidate seal PASS: `cargo fmt --check --all`; locked workspace all-target/all-feature
  check and tests; warning-denied workspace all-target/all-feature Clippy; and warning-denied
  locked workspace rustdoc without dependencies. `git diff --check` also PASS.
- No Issue-047 corpus/fixture expansion, audit main, cross-target, object inspection, benchmark,
  timing or listening command ran. `timed_benchmark_invocations=0`.

**Final Sol verdict: PASS.** All four Issue-048 effect-local proofs and the unchanged-product seal
are satisfied within the second and final authorized attempt.
