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
Open/K/+0 and increment only `recovered_left_samples`. An executed W8 bank with the same injected
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
