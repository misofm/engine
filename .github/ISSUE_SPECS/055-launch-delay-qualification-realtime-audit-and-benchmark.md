# 055 Launch delay qualification, realtime audit, and benchmark

## Outcome

Qualify the exact accepted Issue-021 integer-time feedback-matrix delay candidate across expanded
correctness, long-state, realtime, target, descriptive-performance and listening surfaces without
changing its DSP or adding a SIMD delay bank.

## Context

This stateless successor begins only after **Launch integer-time dual-mono and ping-pong delay**
has a closed product candidate. It gates only Issue 026 release qualification and does not block
other effects, control, hosts or deployment work. Never inspect V1. Render remains allocation-,
lock-, I/O-, logging- and syscall-free. Launch rates are 44.1, 48, 88.2 and 96 kHz.

There are exactly two total attempts: Terra attempt 1 and one bounded Sol correction. A second
failure stops and rescopes. `timed_benchmark_invocations=0`; briefing and implementation work do
not authorize a timed run until every zero-launch preflight gate is recorded green.

## Scope

- Expand the independent checked corpus across launch rates, delay endpoints/midpoints, both
  feedback signs, damping/mix boundaries, cross-feedback 0/intermediate/1, time transitions and
  reset/restore partitions.
- Add frozen seeded and long feedback/queued-retarget sequences, expanded ten-track scalar cohort
  and determinism evidence, and duration-independent allocation/accounting checks.
- Run a 100,000-render realtime audit with allocation/free, lock, I/O, logging, syscall and
  unbounded-work probes, including sanitation/recovery and reset paths.
- Prove native scalar, AArch64 and Wasm builds/behavior and relevant instruction/static contracts.
  These are scalar effect claims; no W4/W8 delay-bank execution is implied.
- Freeze a descriptive workload/validator, preflight without launching it, then permit exactly one
  future benchmark invocation with one warmup and two measured rounds. Preserve raw output.
- Produce the preregistered audition/listening handoff and candid known limits.

## Required public interfaces/contracts

Do not change Issue-021 effect ID, descriptor, equations, nearest-sample mapping, 128-update tap
transition, feedback matrix, domains, two-second rings, latency, tail, state layout, resources or
scalar-only eligibility. Qualification consumes the exact pushed candidate and emits versioned,
checksummed evidence. Any correction requiring a DSP/resource/interface change stops and returns
to a separate stateless product issue.

## Deliverables

1. Checked corpus manifest and independent-oracle results with exact worst rows.
2. Frozen seeded/long sequence serialization, graph/cohort determinism and duration-independent
   allocation evidence.
3. 100,000-render realtime audit report with functional probes proven non-vacuous.
4. Native/AArch64/Wasm build and static/instruction evidence for the scalar implementation.
5. Benchmark workload/schema/validator and zero-launch preflight evidence; later, exactly one
   one-warmup/two-round descriptive result if authorized.
6. Listening protocol/handoff, results when completed, and known-limits record.

## Explicit non-goals

Fractional interpolation, modulation, tempo sync, multitap, new delay modes/qualities, DSP tuning,
domain/tolerance changes, finite-tail claims, a general oversampling/delay framework, graph
architecture changes, or a W4/W8 gathered delay bank. Any future bank promotion is a separate
product/optimization issue, not qualification scope.

## Dependencies by exact issue title

- Launch integer-time dual-mono and ping-pong delay

## Acceptance gates with objective measurements

All expanded oracle/corpus rows satisfy the frozen Issue-021 sample/tolerance contract; long
feedback and repeated queued transitions remain finite, deterministic and bounded; resets and
restores partition exactly; every duration-independent resource row is exact. The 100,000-render
audit has zero forbidden realtime events and proves its hooks trigger under injection. Target and
static evidence supports only tested scalar paths. Benchmark preflight proves arguments, fixture,
schema, validator, persistence, overwrite refusal and shell exit behavior without timing; after
explicit authorization, one invocation uses exactly one warmup and two measured rounds with no
threshold or retry. Listening evidence complements but never replaces objective gates.

## Target matrix

Native x86-64 scalar, AArch64 scalar, `wasm32-unknown-unknown` scalar, and representative mobile/
browser compilation under fixed memory caps. AVX2/NEON/Wasm SIMD delay-bank claims are forbidden.

## Required evidence

Candidate and toolchain identity; corpus/seed hashes; exact failure maxima; audit probe transcript;
target/static rows; allocation/resource proof; benchmark preflight and, only when authorized, raw
one-warmup/two-round output; listening artifacts; strict attempt verdict; successor/follow-up links;
and `timed_benchmark_invocations` (currently `0`).
