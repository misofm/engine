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

## Sol attempt 2 — registry and graph checkpoint (partial PASS)

- Added `miso.delay` to the caller-owned launch registry and exact effect-compiler dependency
  allowlist. Mutation coverage rejects an arbitrary extra dependency and missing or substituted
  delay dependency.
- One accepted 48-kHz/q128 ten-track fixture places delay only in the scalar dynamic rack, with no
  sidechains or prepared effect bank. All ten effect nodes retain stable `eq0..eq9` order, zero
  latency/PDC and `Infinite` tail. Exact declared effect storage is
  `10 * (768168 state + 36 fixed) = 7682040` bytes; every bank count/buffer/metadata row is zero.
- Two consecutive graph renders match ten independently prepared scalar processors bit-for-bit
  after the graph's accepted balanced reduction. The second block exercises carried delay history;
  direct state snapshots close at cursor/valid-history 256. Enabled and bypassed plans preserve
  schedule, routing, inserted-delay and canonical bytes. A graph plan cap exactly one byte below the
  accepted estimate rejects and returns all ten prepared effects and the complete session.
- PASS: focused format check; `cargo test --locked -p miso-engine-effect-compiler --all-targets`
  (4 passed); `cargo test --locked -p miso-engine-graph-compiler --lib` (22 passed);
  warning-denied all-target Clippy for both packages; shell syntax; effect-runtime baseline and
  mutation policies; and graph policy.
- Integration checkpoint verdict: partial PASS. The final clean nonbenchmark product seal remains
  separate; Issue-055 qualification remains untouched. `timed_benchmark_invocations=0`.

## Sol attempt 2 — final nonbenchmark product seal

- Final candidate `6781c88` preserves the accepted scalar correction and registry/compiler/graph
  checkpoints above. No production, fixture, policy or qualification file changed during this seal.
- `cargo fmt --all -- --check`: PASS. `cargo check --locked --workspace --all-targets
  --all-features` and `cargo test --locked --workspace --all-targets --all-features`: PASS,
  including all seven delay product tests and the ten-track scalar graph closure fixture.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: PASS.
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features --no-deps`: PASS.
- Workspace, realtime, effect-runtime and rack baseline plus mutation suites PASS when invoked
  through `bash`; graph baseline policy PASS. The graph policy has no separate mutation script.
- The Git-free static seal found no conflict markers or trailing whitespace in the Issue-021
  product paths, no `.orig`, `.rej`, `.tmp` or `.profraw` artifacts outside excluded build/VCS
  directories, and valid shell syntax for every invoked policy script.
- No Issue-055 qualification, functional audit main, cross-target build, instruction/object
  inspection, benchmark, timing or listening command ran. `timed_benchmark_invocations=0`.

**Final Sol verdict: PASS.** Issue 021 closes the fixed two-second integer-time dual-mono/ping-pong
delay scalar effect, launch registry/effect compiler and ten-track dynamic-rack graph/PDC/resource
vertical in the second and final authorized attempt. Issue 055 remains the sole owner of deferred
corpus and long-stress expansion, realtime audit, target/instruction evidence, descriptive benchmark
and listening handoff.

## #93 amendment (master plan #83, D3/D6/D7/D10/D11)

Issue #93 re-lands this effect on the `miso-engine-lane` / `miso-engine-math` /
`miso-engine-effect-runtime` foundation. The frozen tables above — descriptor, resource totals,
latency, tail, state layout and its version, integer tap mapping, transition timing, automation
validation and the `Ok(None)` bank fallback — are unchanged. What follows amends the numerics and
the recovery granularity.

1. **Damping is a topology-preserving one-pole with a rate-invariant mapping (class B).** The
   control keeps its `[0, 0.995]` linear domain, its identifier 3, its default `0.25` and its
   64-sample smoothing. It is no longer the raw coefficient of `v = (1-c)*y + c*z`. At prepare, at
   every automation point and at restore it is mapped, at control rate, through
   `miso_engine_math::{log, tan}`:

   ```text
   fc(c) = min(19_845 Hz, -ln(c) * 48_000 / (2*pi))     c > 0
   G     = tan(pi * fc / Fs)
   g     = G / (1 + G)                                   g(0) = 0 exactly
   ```

   and the recurrence, in the frozen operation order, is

   ```text
   d = y - z;  h = g*d;  v = fma(g, d, z);  z = flush(v + h)   g != 0
   v = y;      z = flush(y)                                     g == 0 (per-sample select)
   ```

   The reason is that `c` alone fixes a pole per *sample*, so the tone of the feedback tail moved
   with the sample rate (issue #93 finding F5). Evaluating the old pole's cutoff once at the 48 kHz
   reference rate and re-designing `g` for the running rate holds that cutoff in hertz at every
   rate, and leaves the 48 kHz sound exactly where it was. Reference values: `c = 0.25` is
   10_590.6 Hz, `c = 0.995` is 38.3 Hz, every `c <= 0.0745` clamps at 19_845 Hz (`0.45 * 44_100`,
   which keeps `tan` finite at the lowest launch rate). The mapping is strictly decreasing in `c`.
   The damping ramp triple in the state layout holds **`g`**, not `c`; its restore domain is
   therefore `[0, g_max(Fs)]`, with `g_max` about `0.863` at 44.1 kHz, `0.781` at 48 kHz, `0.461`
   at 88.2 kHz and `0.432` at 96 kHz. The word positions and the layout version do not change.
   *Open, for the owner:* exposing damping in hertz would be the honest control, but that is a
   descriptor change and therefore a contract change; it is not taken here.

2. **FMA is now permitted, and only through `Lane::fma` (D3).** The sentence "No FMA is permitted
   in the frozen scalar graph" is superseded. There are six fused sites per stereo frame: the
   crossfade blend `fma(alpha, new - old, old)`, the damping output `fma(g, d, z)`, the two matrix
   products `fma(q, gL, p*gR)` / `fma(p, gL, q*gR)`, and the wet mix `fma(mix, y - x, x)`. Every
   one replaces a separately rounded pair, so each carries one rounding instead of two. Nothing
   else fuses: Rust never contracts `a*b + c`, and `mul_add` may not appear in the crate.

3. **Denormals and non-finite values (D7).** `flush(x) = andnot(|x| < 1e-20, x)` is applied to
   exactly two recursive words per lane per sample — the damping state and the ring write — and
   nowhere else. Every per-value `is_finite`/`is_subnormal` classification is deleted. `-0.0` is
   never stored in a ring or a damping state; the dry, wet and bypass identities still deliver the
   selected input's or tap's bits, sign of zero included, because those are selects and not
   arithmetic. Finiteness is checked **once per block per lane**, over the lane's output, the ring
   cells the block wrote and the lane's damping state. A failing lane has its output zeroed, its
   damping state cleared and its history logically invalidated, and increments its recovery
   counter once for the *block*; parameters, tap transition and the shared cursor continue. With
   `p > 0` a non-finite value in one lane reaches the other inside the same block and both lanes
   recover; at `p = 0` the lanes stay independent, because the matrix identities are bitwise
   selects. Input sanitisation is no longer performed here — the input stage sanitises once per
   track per block — so `sanitized_main_samples` is always zero from this effect and a non-finite
   input is counted as a recovery instead. The `recovered_*_samples` counters therefore count
   blocks; issue #95 renames them.

4. **Ramps (D11).** All seven ramps are `effect_runtime::ramp::LinearRamp`: one division when the
   target changes, iterated additions per sample, and an exact assignment of the target on the
   final (64th) update. The per-sample division is gone. Restore re-derives the step from the
   stored `(current, target, remaining)` triple and requires `remaining == 0` to come with
   `current == target`, which every snapshot this effect writes satisfies.

5. **Chunked evaluation, and why the bits do not move.** A block is rendered in chunks of at most
   128 frames whose length is the minimum of: the frames left in the block; `R - cursor`; each
   lane's `active_delay`, and its `transition_delay` while a crossfade runs; `transition_remaining`
   while a crossfade runs; `D - valid_history` for each tap `D` that is not yet valid; and
   `remaining - 1` for each running ramp (`1` when `remaining == 1`, so the D11 snap is its own
   frame). Every per-sample decision is therefore constant inside a chunk, and the tap windows can
   be copied out with two contiguous slice copies before the chunk writes anything: sample `k`
   reads cell `(cursor + k - D) mod R` and sample `j < k` wrote `cursor + j`, so an overlap would
   need `j = k - D < 0`. The rendered bits and the resulting state are consequently identical for
   any partition of a stream into blocks — proven over `{1, 7, 64, 128, 512}` and against a
   one-frame chunk cap — with the single, deliberate exception of the block-granular recovery in
   point 3, which is what "once per block" means.

6. **Still open.** The crossfade law (linear, 128 updates, sample 128 selecting the new tap) is
   unchanged; whether a raised-cosine law or a length that scales with the tap distance would be
   better is issue #93 finding F6 and stays open.
