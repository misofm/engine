# 021 Launch integer-time dual-mono and ping-pong delay

## Outcome

Ship one bounded, musically useful Normal-quality feedback delay: independent L/R delay times,
feedback, damping and mix, plus an explicit smoothed cross-feedback control from dual-mono through
ping-pong. The launch processor is a scalar dynamic-rack effect with exact nearest-sample taps and
click-bounded dual-tap time changes.

## Context

Engine V2 is greenfield; never inspect or inherit V1. The render plane exclusively owns a prepared
plan and performs no allocation/free, lock, I/O, logging, syscall, structural mutation or
data-dependent unbounded work. Launch rates are exactly 44.1, 48, 88.2 and 96 kHz. Audio is planar
`f32`; cross-channel behavior must be explicit. This issue consumes the accepted native-effect,
registry, dynamic-rack, graph/PDC and scalar-tail contracts plus Issue 008's preserved generic bank
seam as technical input, without inheriting its stopped benchmark claim.

The authoritative tracked brief is
`.github/ISSUE_SPECS/BRIEFS/021-dual-mono-stereo-delay.md`. There are exactly two total attempts:
Terra attempt 1 and one bounded Sol correction. A second failure stops and rescopes; no gate may
weaken. `timed_benchmark_invocations=0`; Issue 055 alone owns qualification and the future
descriptive benchmark.

## Scope

- Add contract-1/layout-1 `miso.delay`, Normal only, at the four launch rates.
- Use fixed prepared two-second L/R circular histories, exact nearest-sample delay mapping and a
  128-update linear crossfade between old/new integer taps. There is no fractional interpolator.
- Expose per-lane delay time, signed feedback, damping and mix plus shared cross-feedback. The
  local explicit matrix is dual-mono at zero and ping-pong at one.
- Implement strict Block-Point automation, exact reset/snapshot/atomic restore, sanitation and
  lane-local recovery. Latency is zero and tail is conservatively `Infinite`.
- Integrate the caller-owned registry/effect compiler and one ten-track dynamic-rack graph fixture
  proving scalar order, state continuation, zero PDC, canonical bypass and transactional caps.

## Frozen public contract

Stable parameter IDs are: delay time `1` (`1..2000 ms`, default `250`, PerLane, special Linear
128-tap crossfade); feedback `2` (`-0.95..0.95`, default `0.35`, PerLane, Linear 64); damping `3`
(`0..0.995`, default `0.25`, PerLane, Linear 64); mix `4` (`0..1`, default `0.35`, PerLane, Linear
64); cross feedback `5` (`0..1`, default `0`, Shared/Both, Linear 64). All are Block-rate Points,
readable and automatable with Linear mapping. Ports are required dual-mono `main-in`/`main-out`,
with no sidechain. Only `LinkMode::DualMono` is accepted because cross-linking is the explicit
parameterized matrix, not a detector-link alias.

Delay milliseconds map off the audio recurrence at each accepted Block Point as
`D=floor((delay_ms as f64)*(Fs as f64)/1000.0+0.5) as u32` samples. The ring length is
`R=2*Fs+3`; exact-delay reads require no interpolation. At a delay Point, retain the newest pending
integer tap. An idle transition starts on the next processed sample; samples 1..128 use linear
weights `j/128`, sample 128 selects the new tap exactly. Retargeting during a transition replaces
the single pending tap without interrupting the active crossfade; another transition begins only
after the current one completes.

For delayed taps `yL,yR`, damping states `zL,zR`, current damping `c`, feedback `f` and shared cross
feedback `p`, use separately rounded `f32` operations:

```text
v = y                              when c == 0
v = (1-c)*y + c*z; z = v           otherwise
gL=fL*vL; gR=fR*vR; q=1-p
fbL=q*gL + p*gR
fbR=p*gL + q*gR
wL=xL+fbL; wR=xR+fbR
out=x                              for bypass or mix==0
out=y                              for mix==1
out=x + mix*(y-x)                  otherwise
```

At `p=0` the rings are independent; at `p=1` feedback swaps lanes. The induced matrix 1-norm is at
most `max(abs(fL),abs(fR)) <= 0.95`. Feedback-zero write, damping-zero tap, dry and wet identities
preserve selected input/tap bits exactly. No FMA is permitted in the frozen scalar graph.

## State, resources and realtime behavior

Common state is four 32-bit words: cursor plus the shared cross-feedback `(current,target,
remaining)` ramp. Each lane is `R+16` words: damping state; latest delay target; active,
transition and pending integer taps; transition remaining; valid-history count; three ordinary
ramp triples in feedback/damping/mix order; then `R` physical ring words. Snapshot writes every byte; restore
parses common and both lanes into unpublished temporaries, validates the complete domain/canonical
transition shape and commits atomically. Scalar reset defaults retain nine `f32` values, exactly 36
fixed bytes.

The brief freezes the exact per-rate state totals and largest allocations. All allocation and cap
arithmetic is checked before publication. The accepted prepare API has no independent
largest-allocation carrier; the exact total-state cap bounds both fixed rings. Exact caps pass;
one byte below total state, fixed scratch, graph plan or session-plus-plan rejects transactionally
and returns ownership. Render performs bounded direct indexing only.

Sanitize each nonfinite/subnormal input lane to positive zero with one aggregate counter increment;
preserve finite signed zero on exact identities. Reset/recovery logically invalidates history with
a bounded valid-count update; it never clears an O(R) ring during render. A corrupt/nonfinite
delayed, damping, feedback, write or mix intermediate invalidates only that lane and clears its damping state, emits dry for that lane,
increments its recovery count once for the sample, and cannot damage the other lane. Parameters,
tap transition and the common cursor continue. Full reset clears histories/state and restores all
prepared defaults. Discontinuity reset clears histories/state, retains targets, snaps ordinary
ramps, and selects the latest delay target with no transition.

## Scalar-only launch eligibility

The launch effect is deliberately dynamic-rack scalar. Per-track variable large-ring gathers have
no accepted core W4/W8 kernel, and inventing a general gather-delay SIMD framework would violate
this half-day slice. `bind_homogeneous_bank` validates exact shape/member preparation before legal
`Ok(None)` fallback; every count executes as stable scalar members with no padding or track ceiling.
A future stateless product/optimization issue may derive a safe gathered W4/W8 delay bank. That
work is separate from Issue 055 and is not a hidden launch or qualification requirement.

## Deliverables and representative gates

1. Descriptor, exact rate/resource table, preparation/cap and strict automation/state fixtures.
2. An independent `f64` integer-ring/matrix oracle. Exact integer impulses and repeats match their
   samples; millisecond mapping differs from ideal by at most 0.5 sample; transition weights and
   queued retarget completion are exact.
3. Default active delay is nontrivial; mix-zero/bypass signed-bit identity still warms histories.
   Prove `p=0` L/R isolation, `p=1` ping-pong arrival, intermediate matrix coefficients, damping,
   both feedback signs, feedback bound, resets, active restore, sanitation and injected recovery.
4. Prove zero latency, `Infinite` tail, exact fixed memory independent of render duration, no bank,
   stable ten-track scalar order, consecutive graph/direct-scalar PCM/state, enabled/bypass zero
   PDC/canonical stability and one-byte-below ownership return.
5. Focused then clean locked workspace format/check/test, warning-denied Clippy/rustdoc and
   workspace/realtime/effect-runtime/rack/graph policies pass. No benchmark runs.

## Explicit non-goals

Fractional/allpass interpolation, tempo sync, modulation, multitap patterns, ducking, sidechain,
output matrix/pan, feedback above `0.95`, finite-tail marketing, a general delay framework, W4/W8
gather kernels, corpus/long stress, realtime audit, target/instruction proof, benchmark and
listening. Issue 055 owns the broad qualification surfaces, not future bank implementation.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

## Research basis

`[SMITH-SASP]` supports bounded circular delay and the distinction between exact integer and
fractional-delay interpolation. `[VST3-LATENCY]` supports immutable prepared latency reporting.
`[LAWO-FLOW]` is workflow evidence that explicit channel delay is useful, not DSP authority. The
chosen integer tap preserves full-band sample values and trades only at most half-sample timing
quantization for a bounded implementation; time changes are explicitly crossfaded rather than
silently interpolated.

## Required evidence

Candidate identity; exact equations/operation order; descriptor/state/resource tables; oracle,
impulse/repeat/transition/matrix/reset/restore/recovery rows; graph/cap results; commands and policy
results; attempt number; strict verdict; Issue-055 link; and `timed_benchmark_invocations=0`.

## Terra attempt 1 — scalar checkpoint (partial PASS)

- Added `miso-engine-delay`: Normal-only `miso.delay` scalar dynamic-rack effect with exact
  two-second prepared L/R rings, rounded integer taps, queued 128-update output crossfades,
  signed feedback, damping, wet/dry mix and an explicit smoothed dual-mono-to-ping-pong feedback
  matrix. Reset/recovery use bounded logical history invalidation; snapshots canonicalize stale
  ring cells and restore parses both lanes atomically.
- Added a test-only independent `f64` integer-ring/matrix oracle to `miso-engine-dsp-reference`.
  Focused tests cover all frozen rate/resource rows and cap rejection, sample-exact impulse and
  ping-pong repeat behavior against the oracle, zero latency/Infinite tail metadata, crossfade
  updates and queued retarget, automation/state continuation, lazy reset, sanitation and one
  lane-local recovery injection.
- PASS: `cargo fmt --check --package miso-engine-delay --package miso-engine-dsp-reference`;
  `cargo test --locked -p miso-engine-delay --lib` (3 passed);
  `cargo test --locked -p miso-engine-dsp-reference --lib` (6 passed, 1 documented ignored);
  `cargo clippy --locked -p miso-engine-delay --all-targets -- -D warnings`.
- This is a scalar-only partial checkpoint. Registry/effect compiler/graph, Issue-055
  qualification, audit, target and benchmark work remain unstarted; homogeneous bank execution is
  deliberately unavailable after validation. `timed_benchmark_invocations=0`. Terra checkpoint
  verdict: partial PASS; pause for root commit and Sol review.

## Sol attempt 2 — bounded scalar correction (PASS, integration ready)

- Adversarial review found that a nonfinite tap returned before consuming its active crossfade
  update. `read_transition` now advances or commits the frozen 128-update state machine before
  propagating a fallible tap result, so recovery still emits dry, invalidates only that lane and
  reports once while the host sample consumes its transition update.
- Executed scalar tests now prove a true latest-wins retarget delivered during an active transition;
  ordinary updates 1/63/64, retarget and block partition equivalence; default activity; exact
  dual-mono and ping-pong cases; intermediate cross-feedback, active damping and negative feedback
  against the independent `f64` oracle; nonfinite/subnormal sanitation; active-transition recovery;
  atomic invalid restore; word-exact full/discontinuity resets; signed-zero mix-zero/bypass identity
  with history warming; and validation of every member before legal homogeneous-bank fallback.
- PASS: `cargo fmt --check --package miso-engine-delay --package miso-engine-dsp-reference`;
  `cargo test --locked -p miso-engine-delay -p miso-engine-dsp-reference --lib` (delay 7 passed;
  reference 6 passed, 1 pre-existing documented ignore); and
  `cargo clippy --locked -p miso-engine-delay --all-targets -- -D warnings`.
- Scalar correction verdict: PASS and ready for the separately checkpointed registry/compiler/graph
  integration. This is not an overall Issue-021 verdict; the frozen integration and final product
  seal remain. Issue-055 qualification remains untouched. `timed_benchmark_invocations=0`.
