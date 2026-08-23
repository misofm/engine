# Sol implementation brief — issue 021 launch integer-time dual-mono and ping-pong delay

## Decision, authority and attempt budget

**READY FOR TERRA ATTEMPT 1.** Deliver exactly this bounded product. There are two total attempts:
one Terra implementation/review and one bounded Sol correction/review. A second failure stops and
rescopes. Issue 055 owns broad qualification. No benchmark is authorized here and V1 is forbidden.

This brief and `.github/ISSUE_SPECS/021-dual-mono-stereo-delay.md` are authoritative. Reuse the
accepted effect contract, dynamic rack, caller-owned registry/effect compiler, scalar-tail graph
and checked plan accounting. Do not change the builtin matrix API or add a core SIMD framework.

## Frozen launch product

```text
effect / contract           miso.delay / 1.0
state layout / quality      1 / Normal only
rates                       44100, 48000, 88200, 96000 Hz
ports / link mode           main-in + main-out / DualMono only
delay                       nearest integer sample, 1..2000 ms
time change                 two integer taps, linear output crossfade, 128 updates
feedback routing            smoothed explicit 2x2, dual-mono through ping-pong
latency / tail              0 / Infinite
execution                   scalar dynamic rack; homogeneous bank unavailable
```

This is not a passthrough: the default is a 250-ms, 35%-feedback, damped wet/dry echo. It omits
fractional interpolation, modulation, tempo sync and multitaps because each would add a second
algorithm or framework. Exact integer taps preserve sample amplitude and have at most half-sample
timing quantization after millisecond mapping.

## Descriptor and preparation

Create `miso-engine-delay` / `miso_engine_delay`, `DelayFactory`, and descriptor/layout version 1.
Use required ordered dual-mono-planar `main-in`/`main-out`, no sidechain, `LinkModeSet::DUAL_MONO`,
Normal-only quality rows and these descriptor-order parameters:

| index / ID | name | unit | policy | domain / default | rate / smoothing |
|---:|---|---|---|---|---|
| 0 / 1 | delay time | Milliseconds | PerLane | 1..2000 / 250 | Block Point / Linear 128 special tap crossfade |
| 1 / 2 | feedback | Linear | PerLane | -0.95..0.95 / 0.35 | Block Point / Linear 64 |
| 2 / 3 | damping | Linear | PerLane | 0..0.995 / 0.25 | Block Point / Linear 64 |
| 3 / 4 | mix | Linear | PerLane | 0..1 / 0.35 | Block Point / Linear 64 |
| 4 / 5 | cross feedback | Linear | Shared | 0..1 / 0 | Block Point / Linear 64 |

All use continuous domains, Linear mapping, are readable/automatable and use the displayed unit
token matching the unit. Preparation consumes exactly L/R rows for positions 0..3 followed by one
Both row for position 4. Reject missing/extra/disordered, negative-zero, nonfinite or out-of-domain
initial values. Retain positive-zero reset defaults. Reject non-DualMono, sidechain, non-Normal,
unsupported rate, zero quantum or insufficient total-state/fixed-scratch caps before publication.

The fixed maximum is two seconds; the current API has no separate per-instance maximum-delay
carrier. For rate `Fs`:

```text
max_D = 2*Fs
R = max_D + 3
D(ms) = floor((ms as f64) * (Fs as f64) / 1000.0 + 0.5) as u32
```

The bounded `f64` conversion runs only when preparing or accepting a Block Point, not in the
per-sample audio graph, and makes the nearest-sample error relative to the accepted `f32` value
exactly at most half a sample. The three extra cells make checked cursor arithmetic simple; only exact integer taps are read.
Assert every accepted endpoint maps within `1..=max_D`. Allocate two zeroed `Box<[f32]>` rings of
exactly `R` words off render. Every size conversion/add/multiply and cap check precedes allocation.

## Delay transition state machine

Each lane retains `delay_target_ms`, `active_D`, `transition_D`, `pending_D`, and
`transition_remaining`. Initially all three taps equal the quantized prepared default and remaining
is zero. A valid delay Point immediately replaces `delay_target_ms` and `pending_D`.

At the start of a sample, after accepting block Points:

1. If remaining is zero and pending differs from active, copy pending to transition and set
   remaining to 128.
2. Read exact taps at active and transition. With remaining `r>0`, set
   `j=129-r`, `alpha=j*(1.0f32/128.0f32)`, `delta=new-old`, `scaled=alpha*delta`, and
   `y=old+scaled`, each as a separate `f32` operation. With no transition, return active tap bits.
3. If `r==1`, return the new tap bits exactly, set active to transition and remaining to zero;
   otherwise decrement remaining after producing the tap.
4. A Point arriving during an active transition changes only pending. The current crossfade is
   never restarted; if pending still differs after completion, the next sample starts one more
   bounded transition.

Thus the first changed sample uses `1/128`, update 64 uses `1/2`, update 128 uses the new tap
exactly, and arbitrary retarget density needs only one latest-wins pending cell. This smoothing is
an output crossfade, not fractional-delay interpolation or pitch modulation.

The cursor denotes the current write cell. Exact tap `D` reads
`ring[(cursor + R - D) % R]` before either lane writes, and then cursor advances once modulo `R`.
No division, search, allocation or data-dependent loop occurs in render.

## Exact sample graph and explicit matrix

At each frame: apply Points; advance feedback, damping, mix and shared cross-feedback ramps in
descriptor order; start/read both delay transitions; sanitize/recover both taps; update both
damping states; compute the cross-feedback matrix; write Left then Right; mix Left then Right; and
advance the shared cursor. All listed arithmetic is separately rounded `f32`; no FMA is permitted.

For tap `y`, previous filter state `zp`, damping `c`, feedback `f`, shared cross feedback `p`, and
sanitized dry input `x`:

```text
when c==0: v=y                         # exact identity
otherwise: a=1-c; d0=a*y; d1=c*zp; v=d0+d1
z=v

gL=fL*vL
gR=fR*vR
q=1-p
pL=p*gL
pR=p*gR
qL=q*gL
qR=q*gR
fbL=qL+pR
fbR=pL+qR

when fb==0: w=x                        # exact write identity
otherwise: w=x+fb
ring[cursor]=w

bypass or mix==0: out=x               # exact dry identity
mix==1: out=y                          # exact wet identity
otherwise: delta=y-x; scaled=mix*delta; out=x+scaled
```

Use explicit branches at `p==0` and `p==1` to return `(gL,gR)` and `(gR,gL)` bits exactly. At
intermediate `p`, the matrix is
`[[q*fL,p*fR],[p*fL,q*fR]]`; its induced 1-norm is bounded by
`max(abs(fL),abs(fR))<=0.95`. The matrix operates only in the feedback write path; it is not an
implicit stereo output transform. Damping filters repeats while the first wet echo remains the
exact tap. Bypass and mix-zero still run taps, filter, matrix and writes.

Latency is `LatencySamples(0)`: the dry path is immediate and a chosen echo time is not PDC.
Every quality row declares `TailSamples::Infinite`; although `abs(f)<1`, the public immutable tail
cannot depend on parameters or an arbitrary silence threshold.

## Automation

Accept only canonical Block `Point` spans at `first_sample`, equal start/end samples and bit-equal
values. PerLane positions require exact Left/Right; the shared position requires Both. Reject wrong
kind/time/channel, missing domain, duplicates, disorder and spans beyond prepared capacity while
retaining other valid entries and saturating `invalid_spans`. Scan into a fixed nine-cell pending
table, then apply valid positions/channels in descriptor order. Automation accepts numeric zero and
normalizes it positive; initial preparation continues to reject negative zero.

Feedback/damping/mix and cross-feedback Points retarget from current, begin update one on the first
sample and reach target on update 64. Delay Points follow the special queued 128-update tap state
machine above. Automation partitioning cannot change PCM, state or reports.

## Sanitation, reset and recovery

Use accepted `sanitize_sample`: each nonfinite or subnormal main-input lane becomes positive zero
and increments that track's aggregate main counter once; finite signed zero remains available to
exact identity branches. Flush finite subnormal computed state/output to positive zero.

Each lane retains `valid_history` in `0..=R`. A tap whose age exceeds valid history is logical zero;
after each successful write, increment valid history saturating at `R`. Full/discontinuity reset
and recovery set valid history to zero instead of clearing an O(R) ring on render. Snapshot emits
canonical positive-zero words for logically invalid cells, so stale physical storage is neither
observable nor trusted after restore.

If a delayed/crossfade/damping/matrix/write/mix intermediate is nonfinite, recover the affected
lane at most once for that host sample: logically invalidate its history, clear its damping state,
use zero as its feedback contribution, emit sanitized dry for its output, and saturating-increment
that lane recovery counter. Do not reset parameters, delay transition or the shared cursor. Compute
both sanitized feedback contributions before writes so one fault cannot contaminate the healthy
lane. No legal bounded fixture may recover.

`FullToDefaults` logically clears histories/damping, resets cursor zero, restores all nine prepared
defaults, selects each default delay with no transition and clears ramps. `DiscontinuityKeepParameters`
logically clears histories/damping, resets cursor zero, snaps ordinary/shared currents to targets,
selects each latest delay target and clears transitions. Both are fixed work independent of `R`.

## Exact state and resources

State layout is little-endian 32-bit words. Common is exactly 4 words / 16 bytes:

```text
0 cursor u32
1..3 cross-feedback current f32, target f32, remaining u32
```

Each lane is exactly `R+16` words:

```text
0 damping state f32
1 delay target ms f32
2 active D u32
3 transition D u32
4 pending D u32
5 transition remaining u32
6 valid history u32
7..15 feedback, damping, mix (current f32,target f32,remaining u32)
16..(16+R-1) physical ring f32 words; logically invalid words snapshot as +0
```

Restore requires exact lengths/layout 1, cursor `<R`, finite normal-or-zero damping/ring values,
all parameters in domain, ramp remaining `<=64`, taps in the prepared mapped domain, transition
remaining `<=128`, valid history `<=R`, and canonical transition relations: remaining zero requires
active equals transition; remaining nonzero requires they differ. Ring cells outside the valid
history window must be positive zero. Parse common and both lanes completely before committing.

The 36 fixed bytes are two four-parameter per-lane reset rows plus one shared default. Exact rows:

| Fs | R words | one ring bytes | one lane state | total state | scalar retained state+fixed |
|---:|---:|---:|---:|---:|---:|
| 44100 | 88203 | 352812 | 352876 | 705768 | 705804 |
| 48000 | 96003 | 384012 | 384076 | 768168 | 768204 |
| 88200 | 176403 | 705612 | 705676 | 1411368 | 1411404 |
| 96000 | 192003 | 768012 | 768076 | 1536168 | 1536204 |

Quality rows declare common 16, the listed left/right lane bytes, scratch fixed 36 and zero scratch
per frame. Largest effect allocation is informationally exactly one ring; the accepted prepare API
has no separate largest-allocation limit, so the exact total-state cap is the authoritative bound
for both allocations. Exact state/fixed caps pass and each one-byte-below case rejects before
allocation/publication. Memory is independent of quantum and render duration.

## Scalar-only eligibility and graph vertical

`bind_homogeneous_bank` checks backend/width/count and validates every member's metadata, ports,
initial values and caps before returning legal `Ok(None)`. There is no W4/W8 kernel: per-track tap
indices require variable gathers into large independent rings, and existing core tokens do not
implement this operation. Put the launch effect in the dynamic rack; every track count remains a
stable scalar sequence with no padding or ceiling.

Register the factory and exact effect-compiler dependency policy. One accepted 48-kHz/q128
ten-track fixture has distinct legal times/feedback/damping/mix values and cross feedback values
covering 0, 0.5 and 1. Prove no retained bank, stable scalar membership, no sidechain, consecutive
graph PCM/state against ten direct scalar delegates, latency/route arrival/PDC zero, `Infinite`
  tail propagation, enabled/bypass schedule and canonical stability, exact state/runtime/metadata
  accounting, and effect-state plus graph plan/session one-byte-below rejection returning all owners.

A future stateless product/optimization issue may derive and prove a gathered delay bank. Issue 055
must not absorb that architecture work; launch correctness does not claim SIMD delay execution.

## Representative product gates

1. Descriptor, mapping endpoints, exact resource rows and all cap boundaries pass at four rates.
2. An independent `f64` circular delay/matrix oracle shares no production table or ring helper.
   Integer impulse/feedback repeats are sample-exact; requested-ms error is `<=0.5` sample.
3. Crossfade updates 1/64/128, completion and one queued retarget are word/PCM exact. Ordinary
   ramps prove updates 1/63/64, retarget and partition invariance.
4. Prove both feedback signs, damping-zero identity and active damping, default nontrivial echoes,
   dual-mono isolation, exact ping-pong arrival, intermediate matrix reference agreement, bounded
   representative decay and finite state.
5. Prove mix-zero/bypass signed-bit dry identity with warming; active snapshot/restore continuation;
   both reset payloads; sanitation; injected lane-local recovery; other-lane and other-track
   isolation; canonical lazy invalidation.
6. Prove validation-before-bank-fallback and the complete registry/dynamic-rack graph/cap vertical.
7. Run focused and one clean locked workspace format/check/test, warning-denied Clippy/rustdoc and
   applicable workspace/realtime/effect-runtime/rack/graph policies. Benchmark count remains zero.

## Qualification split and stop rules

Issue 055 alone owns expanded rates/parameter/cross-feedback corpus, long feedback/retarget stress,
expanded graph cohorts/determinism, 100,000-render realtime audit, native/AArch64/Wasm and
instruction evidence, benchmark preflight plus the sole eventual one-warmup/two-measured-round
descriptive invocation, and listening handoff. It does not own a SIMD delay-bank implementation.

Fractional/allpass interpolation, modulation, tempo, multitap, extra quality/mode, a new core
kernel, changed domains/resources, or a second failed attempt stops Issue 021 for stateless
rebriefing. Do not tune or weaken gates. Record exact commands, candidate, evidence, attempt and
`timed_benchmark_invocations=0`.

## Research basis

`[SMITH-SASP]` is primary technical support for circular delay and explicit interpolation choices.
`[VST3-LATENCY]` supports immutable prepared latency reporting. `[LAWO-FLOW]` is workflow evidence
for explicit channel delay only. The measurable adoption reason is exact full-band integer-tap
amplitude, at-most-half-sample mapped-time error, bounded two-second memory and explicit transition
behavior; no unsupported fractional-response claim is made.

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
