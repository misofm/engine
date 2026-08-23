# Sol implementation brief — issue 013 launch feed-forward peak compressor

## Decision, authority, and attempt budget

**READY FOR TERRA ATTEMPT 1.** Deliver the bounded launch vertical below. There are exactly two
total attempts: one Terra implementation/review attempt and, if needed, one bounded Sol
correction/review. A second failed verdict stops; preserve the evidence and create a stateless
rebrief instead of changing a domain, tolerance, algorithm, or target gate. Never inspect V1.

This brief and `.github/ISSUE_SPECS/013-compressor.md` are authoritative. Issues 011, 006, 037 and
the current effect/rack/graph compiler APIs are accepted dependencies. Issue 008 is not an overall
PASS; checkpoint `87783c5` is technical architecture input only. Do not redesign session syntax,
effect metadata, program keys, graph topology/PDC, AoSoA layout, target detection, or state-envelope
interchange.

## Smallest launch product

Freeze exactly this V1:

```text
effect ID / contract       miso.compressor / 1.0
state layout / quality     1 / Normal only
rates                      44100, 48000, 88200, 96000 Hz
topology / detector        feed-forward / instantaneous absolute peak
links                      DualMono, Maximum, Average
ports                      main-in, main-out, optional sidechain-in; dual-mono planar
latency                    Fs/50 samples (exact 20 ms)
effective lookahead        independently 0..20 ms per lane, preparation/state only
tail                       Infinite
automation                 block Point; exact linear 64-update ramps
banking                    unconnected sidechain only; W4/W8 plus scalar tails
```

Feedback, RMS, hold, detector filters, program-dependent release, multiple qualities and changing
latency are independent products and are not implemented behind hidden defaults. A connected
sidechain uses the existing graph's scalar fallback; do not change that graph policy in Issue 013.

Create `miso-engine-compressor` / `miso_engine_compressor`, register `CompressorFactory` beside the
accepted launch effects, and use the existing `NativeEffectFactory`, `PreparedNativeEffect`,
`PreparedNativeEffectBank`, registry, effect-compiler, rack and graph ownership seams. The only
architecture addition is the narrow safe prepared gain/mix bank kernel specified below under
`miso-engine-core::arch`; it follows the existing private intrinsic/function-pointer pattern and
does not expose registers, raw pointers, target-feature calls, or unsafe callers.

## Descriptor and preparation contract

The descriptor is contract 1.0, state layout 1, `LinkModeSet::ALL`, and has ordered required
`main-in`/`main-out` plus optional `sidechain-in`, all `DualMonoPlanar`. Because the descriptor has
an optional sidechain, every prepare request selects either
`Unconnected { id: "sidechain-in", required: false }` or
`Connected { id: "sidechain-in", required: false }`; `None`, a wrong ID, or `required: true`
rejects through the accepted diagnostics.

All parameters are readable and `PerLane`. IDs and descriptor positions are identical and must
never be confused at render time:

| index | ID | name | unit | inclusive domain | default | mapping | rate / smoothing |
|---:|---:|---|---|---:|---:|---|---|
| 0 | 1 | threshold | dB | -80..0 | -18 | Linear | Block / Linear 64 |
| 1 | 2 | ratio | ratio | 1..20 | 4 | Logarithmic | Block / Linear 64 |
| 2 | 3 | knee | dB | 0..24 | 6 | Linear | Block / Linear 64 |
| 3 | 4 | attack | ms | 0.1..200 | 10 | Logarithmic | Block / Linear 64 |
| 4 | 5 | release | ms | 5..5000 | 100 | Logarithmic | Block / Linear 64 |
| 5 | 6 | makeup | dB | -24..24 | 0 | Linear | Block / Linear 64 |
| 6 | 7 | mix | linear | 0..1 | 1 | Linear | Block / Linear 64 |
| 7 | 8 | lookahead | ms | 0..20 | 5 | Linear | None / None |

Preparation consumes the accepted complete ordered 16-value L/R initial table. Negative zero,
nonfinite or out-of-domain values reject. The first seven values create current=target,
remaining=0 ramps. Lookahead is not delivered as automation; a changed value requires reprepare or
a compatible state restore. Normal-quality descriptor rows declare:

| Fs | latency `N=Fs/50` | ring length `B=N+1` | bytes/lane | total state bytes |
|---:|---:|---:|---:|---:|
| 44100 | 882 | 883 | 7160 | 14320 |
| 48000 | 960 | 961 | 7784 | 15568 |
| 88200 | 1764 | 1765 | 14216 | 28432 |
| 96000 | 1920 | 1921 | 15464 | 30928 |

Every row declares `TailSamples::Infinite`, `scratch_fixed_bytes=64` for the two retained
eight-`f32` prepared reset-default tables, and `scratch_bytes_per_frame=0`. State and fixed bytes
are independent of quantum, track count, and source duration. Check exact caps and one-byte-below
transactional rejection. Prepared metadata must equal `expected_prepared_metadata` and remain
immutable. The conservative infinite tail covers the asymptotic release even though numerical
flush-to-zero may terminate a particular run.

The latency is deliberately fixed at the maximum window because the accepted descriptor/program
key cannot vary latency by an initial value. At prepare and restore, derive per-lane effective
lookahead samples exactly as

```text
L = floor(f64(lookahead_ms) * f64(Fs) / 1000 + 0.5), clamped to 0..N
D = N - L
```

Do not report `L`; reported latency and bypass/PDC remain exactly `N`.

## Detector, gain computer, and sample order

The design follows the detector/gain-computer separation in `[REISS-COMP]`. For sanitized detector
sources `sL,sR`, compute linked magnitudes before either lane's gain computer:

```text
DualMono: dL=abs(sL),                 dR=abs(sR)
Maximum:  d=max(abs(sL),abs(sR)),     dL=dR=d
Average:  d=0.5*abs(sL)+0.5*abs(sR), dL=dR=d
```

The Average order is frozen as two half-products followed by one add to avoid overflow. Linking
shares only `d`; thresholds, timing, gain-reduction state, rings, ramps, output, payload and
recovery remain lane-local. With an unconnected sidechain, `s` is the current main input. With a
connected sidechain, `s` is the routed sidechain; main and sidechain sanitation/counters remain
separate. Graph PDC supplies time-aligned routed input and is not recomputed inside the effect.

For each lane, one ring cursor `w` addresses two `B`-sample rings. At sample `n`, after sanitation
and link computation:

```text
main_ring[w]     = x
detector_ring[w] = d
z = main_ring[(w + 1) mod B]
u = detector_ring[(w + B - D) mod B]   // current entry when D=0
w = (w + 1) mod B
```

Thus `z` is always the exact `N`-sample delayed dry sample and the detector precedes it by exactly
`L`. Rings start at positive zero. Never crossfade or move a tap at render: lookahead is immutable
under automation.

For lane parameters threshold `T`, ratio `rho`, knee width `W`, use production `f32` and the exact
ordered gain computer below. Detector flooring and clamping are part of the product, not test-only
guards:

```text
u0 = max(u, 1.0e-8)
X  = clamp(20*log10(u0), -160, 24)
h  = 0.5*W
lo = T-h
hi = T+h
q  = 1/rho

if W == 0 and X <= T: Y = X
if W == 0 and X >  T: Y = T + (X-T)*q
if W > 0 and X <  lo: Y = X
if W > 0 and X >  hi: Y = T + (X-T)*q
otherwise:
    v  = (X-T)+h
    p0 = v*v
    p1 = (q-1)*p0
    p2 = 2*W
    Y  = X+p1/p2
C = clamp(Y-X, -100, 0)
```

`C` is target gain reduction in dB. Attack/release are time constants: after one `tau`, the
one-pole has completed `1-exp(-1)` of a constant step. At every sample derive, in `f32`,

```text
aa = exp(-1 / (0.001*attack_ms*Fs))
ar = exp(-1 / (0.001*release_ms*Fs))
a  = aa if C < G_previous else ar
p0 = a*G_previous
p1 = (1-a)*C
G  = p0+p1
A  = 10^((G+makeup_db)*0.05)
wet = z*A
delta = wet-z
mixed = z+mix*delta
```

Every multiply/add/subtract is a separately rounded operation. The comparison selecting attack is
strict; equality takes release. `log10`, `exp` and `powf` execute once per active lane/sample with
bounded standard `f32` math. Production must not import the `f64` oracle or invent a SIMD
transcendental approximation in this issue.

At each frame: advance the seven ramps in ID order; derive timing and curve values; sanitize main
and, when connected, sidechain; compute the linked magnitudes; update rings/cursor; evaluate each
lane; update `G`; then choose output. A block Point at `first_sample` begins its first of 64 updates
at that same sample and reaches the exact target on update 64. A new Point restarts from current.

Makeup affects wet only. Whole prepared bypass returns `z` bits exactly while still advancing
automation, both rings, detector and gain state. `mix==0` returns `z` bits exactly; `mix==1`
returns `wet` bits exactly. When `G==0` and makeup is positive zero, return `z` exactly for any mix.
These identity selections still warm all state. No implicit auto-gain or channel coupling exists.

## Automation, sanitation, reset, and recovery

Only canonical Block `Point` spans at `first_sample`, with equal start/end sample and bit-equal
values, are legal for descriptor positions 0..6. The lane channel must be Left or Right; Both,
positions 7/out-of-range, Step/Linear/Exponential, duplicates, disorder, excess capacity and domain
errors are invalid. Scan into a fixed pending table, count each invalid span with saturating
`invalid_spans`, then apply every valid target in stable descriptor/lane order; one malformed span
must not discard another valid target. Normalize an accepted numeric zero target to positive zero.

Use the accepted `sanitize_sample` policy. Each nonfinite or subnormal main/connected-sidechain
sample becomes positive zero and increments the corresponding saturating sample counter once.
Signed finite zero is retained in audio rings and exact identity outputs. A finite computed
subnormal `G`, gain, wet or output is intentionally flushed to positive zero without a recovery
count. A nonfinite curve, timing coefficient, gain state, gain or output clears only that lane's
`G` to positive zero, emits its delayed `z`, and increments that lane's recovery counter once for
the sample. Rings and the other lane/track remain untouched. No valid bounded fixture may recover.

`FullToDefaults` clears both rings/cursors and `G`, and restores the complete prepared initial
table. `DiscontinuityKeepParameters` clears rings/cursors and `G`, retains lookahead, snaps each of
the seven ramps to its target, and discards active ramp progress. Both resets leave immutable
metadata unchanged.

## Exact state payload and transaction

Common payload is empty. Each lane is exactly `24 + 2B` little-endian 32-bit words:

```text
word 0       cursor u32
word 1       lookahead_ms f32
word 2       G gain-reduction dB f32
words 3..23  (current f32, target f32, remaining u32) for IDs 1..7 in order
next B       main ring f32, physical index order
final B      detector ring f32, physical index order
```

Prepared reset defaults and derived `L/D` are not serialized. Snapshot requires exact section
lengths and writes every byte deterministically. Restore accepts only layout 1 and exact lengths;
parses both complete lanes into unpublished temporaries; validates `cursor<B`, finite nonnegative
lookahead in domain, `G` normal-or-zero in `[-100,0]`, every current/target in its parameter domain,
`remaining<=64`, and every ring word finite normal-or-zero; rederives `L/D`; and only then commits
both lanes. Reject negative-zero parameter/lookahead words, invalid common bytes, trailing bytes,
one corrupt lane, arithmetic overflow, or incompatible rate/length without changing either lane.
Ring signed zero is legal. Scalar and bank track payloads are byte-compatible, and a failed bank
track restore changes no track.

## Scalar, W4, W8, and FMA operation graphs

Scalar executes the exact sample graph above independently for L and R. A homogeneous bank owns
exactly `W` independent per-track `Lane` states per channel, including each lane's two fixed-size
ring allocations, cursor, `G`, ramps and prepared defaults. This is an internal layout choice:
public state payloads remain byte-compatible per track, while graph audio remains sample-major
AoSoA and the gain/mix step remains one packed W4/W8 call per channel/sample. The bank walks tracks
in ascending lane order for sanitation, linking, ring access, scalar `log10/exp/powf`, gain-computer
and smoother work. Retained state and fixed defaults are exactly `W` times the descriptor's
per-effect envelope: for latency `N`, `lane_bytes = 4 * (24 + 2 * (N + 1))`, and the bank retains
exactly `W * (2 * lane_bytes + 64)` declared bytes. Allocator headers and object-vtable storage are
not state-payload bytes.

Add the safe `PreparedCompressorGainMixKernelV1` following the existing prepared core-kernel seam.
Its call accepts exact-width `samples`, `gains`, `mixes`, `dry_mask`, and `wet_mask`; masks are only
zero or `u32::MAX` and mutually exclusive. Per lane its frozen graph is:

```text
dry = samples
p0 = dry*gain
p1 = p0-dry
p2 = mix*p1
p3 = dry+p2
out = dry when dry_mask; p0 when wet_mask; otherwise p3
```

W4 Wasm `simd128` and AArch64 NEON and W8 AVX2 use packed multiply/add/subtract and bit selection.
AVX2+FMA deliberately aliases the noncontracting AVX2 graph: V1 permits **zero FMA contractions**.
Wasm relaxed SIMD is forbidden. The token is prepared off render, validates backend/width/masks,
and retains a safe function pointer; render performs no feature detection or unsafe call.

`bind_homogeneous_bank` requires exact width request count, matching backend/width, identical
program keys, Normal quality and `Unconnected sidechain-in`. It returns `Ok(None)` for connected
sidechains or an unavailable architecture; malformed homogeneous requests reject before ownership
is consumed. Initial values may differ per track/lane. Absent rack slots remain the accepted rack
identity behavior. Never pad tracks or impose a count ceiling.

For finite-normal inputs without sanitation/recovery, scalar and base same-target W4/W8 output,
rings, `G`, ramps and reports are bit-identical. Cross-target results use
`abs(error) <= 1e-6 + 2e-5*abs(reference)`. The zero-FMA backend has the same bound and operation
count; no timing result may justify a different graph.

## Independent reference and compact fixtures

Build an independent test/tool-only `f64` reference from the equations in this brief. It owns its
own curve, one-pole and ring implementation, never calls production helpers, never reads production
coefficients/state, and is unreachable from production dependencies. A source-boundary mutation
must fail if production imports it or the oracle imports the production compressor.

Freeze one checked `fixtures/compressor/v1` corpus and sorted manifest with safe relative paths,
exact lengths and lowercase SHA-256. It contains expanded static-curve rows; step and sine-burst
envelope traces; latency/lookahead/bypass/mix impulses; main versus external sidechain; asymmetric
DualMono/Maximum/Average traces; automation/reset/restore continuation; sanitation/recovery; and
short audition PCM. Read-only checking rejects changed, missing, unlisted, unsafe-path and
coverage-invalid files. Plots are derived evidence, not a second corpus.

Objective comparisons:

- Sweep detector level -160..24 dB over thresholds `[-80,-18,0]`, ratios `[1,2,4,20]`, knees
  `[0,6,24]`: maximum static curve/gain-reduction error versus independent `f64` is 0.01 dB.
- At every launch rate, attack/release minima, defaults and maxima match the independent trace
  within `0.005 dB`; measured `1-exp(-1)` crossings are within the greater of one sample or 2%.
- At lookahead `[0,5,20]` ms, enabled, bypass and mix-zero dry impulses land at `N`; detector
  action precedes that output by exact rounded `L`. External sidechain and every link mode match.
- Block partitions `1/63/64/127/128`, consecutive blocks and restart Points prove the exact update
  trajectory and partition-invariant continuation. Snapshot/restore output/state is bit-identical.
- Run exactly 10,000 legal configurations from seed `0x000000000013c0de`, covering all rates,
  ports, links and parameter edges. Record a transcript hash and maxima.
- Run twelve frozen one-million-sample rows (four rates by three link modes); each row drives both
  unconnected and connected scalar instances and an available unconnected bank with bounded
  finite-normal asymmetric audio and extreme block Points. All output/state is finite and valid
  instances report zero recovery. Invalid/nonfinite/subnormal and corrupted-payload cases are
  separate expected sanitation/rejection/recovery evidence.

## Registry, graph, realtime, and target gates

The public launch registry contains both the accepted existing effects and `miso.compressor`.
Prepare a deterministic ten-track, 48-kHz/128-frame fixture: nine stable-ID tracks have the same
unconnected compressor program in SIMD rack 1 with asymmetric per-lane/per-track parameters; the
tenth has a connected sidechain sourced from one of those tracks and remains scalar. Host W8
retains one full bank, one unconnected scalar tail and the connected scalar; W4 retains two full
banks, one unconnected scalar tail and the connected scalar; scalar dispatch retains all scalar.
Compare with a scalar-only registry across consecutive blocks.

Assert exact member IDs/order, no padding, program keys, latency/tail, connected fallback, scalar
remainder, resource report, sidechain and main PDC, topological/reduction/observer order, canonical
graph bytes, deterministic PCM/state/report hash, bypass PDC, cap/overflow ownership return, and
unchanged graph structure. Include counts `1,2,3,4,5,7,8,9,17` for cohort/tail coverage; 65,537
tracks is a control-plane no-ceiling case, not an audio workload.

Using that real production graph, render exactly 100,000 128-frame blocks in a release functional
audit. Before arming assert backend/width/member/bank/scalar metadata and stable backing addresses.
While armed, allocation, deallocation, lock, feature detection, logging, file/network I/O, syscall,
panic/unwind and structural-mutation counters are zero. Retained address-free counters prove the
real compressor bank kernel and scalar fallback ran. Disarm before reading counters or destroying
any plan/effect. This is not a benchmark and reports no timing.

Run, on one candidate before timing:

1. formatting; focused locked core/compressor/effect/rack/graph/reference/fixture tests;
2. descriptor/prepare/session/span/state/resource/ownership mutation tests;
3. complete fixtures, static/envelope/latency/link/sidechain and seeded/million-sample gates;
4. scalar/W4/W8 differential, operation graph, state/report and isolation tests;
5. public ten-track graph vertical and the release 100,000-render audit;
6. locked workspace check/tests and warning-denied all-target Clippy/rustdoc;
7. workspace/realtime/effect/rack/graph/research/compressor policies and applicable mutations;
8. native scalar baseline, x86 AVX2 and AVX2+FMA, Android/iOS AArch64, Wasm scalar and `+simd128`;
9. named object inspection proving clean scalar, packed W4/W8 gain/mix, zero compressor FMA, Wasm
   base SIMD/bitselect, and no relaxed-SIMD; and
10. workload-free benchmark argument/schema/persistence/shell-failure/overwrite preflight with
    `workload_launches=0`.

Cross-target evidence is compilation/instruction evidence unless hardware is actually available;
do not claim device/browser runtime execution. An unavailable backend may be reported only with
selection and object proof, never by forging capabilities.

## Descriptive benchmark — exactly once

Only after every nonbenchmark gate passes and root explicitly authorizes timing, invoke exactly:

```text
bash scripts/run-compressor-benchmark.sh
```

The command accepts no arguments and refuses overwrite. It performs one untimed warmup and exactly
two measured rounds, with no retry, tuning, optimization loop, or timing threshold. Each round has
1,000 observations for exactly three 48-kHz/128-frame workloads: one unconnected DualMono scalar
track; one full host-selected unconnected Maximum-link bank; and the ten-track production graph
bank plus connected-sidechain scalar fallback. Exactly six JSONL records report nearest-rank
min/p50/p95/p99/p99.9/max ns/frame/track, cycles when available, backend/width, fixture/build hashes,
allocations/frees, CPU/OS/governor/Rust/LLVM/features/optimization/LTO/codegen, and explicit missing
metadata.

Preserve the first raw output if promotion fails and do not rerun. Runner defects move to tooling;
performance surprises move to the weekly optimization issue. Neither permits a second invocation.

## Listening handoff, evidence, and stop rules

After objective sealing, generate checksum-stable, level-matched audition PCM for slow/fast
attack/release, hard/soft knee, parallel mix, asymmetric link modes and external sidechain. Record
the matching method and an answer-key-separated blinded preregistration using
`listening/TEMPLATE.md`. Completed human listening is a nonblocking follow-up; do not fabricate
listeners, preferences, confidence, or sound-quality claims.

Append to Issue 013: candidate/source hashes; descriptor/metadata/state/resource tables; equation
and citation decisions; fixture/reference/transcript hashes; all comparison maxima; exact graph,
audit and target findings; benchmark preflight launch count and, only after the sole invocation,
raw/accepted benchmark hashes and record count; audition/preregistration hashes; and Terra plus
final Sol PASS/FAIL verdicts.

FAIL immediately for feedback/RMS/filter/hold scope, a changed effect/runtime/session/graph
contract, variable reported latency, connected-sidechain banking, f64 production state, imported
oracle logic, noncanonical automation, shared lane/track state, runtime allocation/detection,
unaccounted delay/default storage, relaxed SIMD, an FMA contraction, a tolerance/domain change,
benchmark before authorization, any benchmark retry, or an attempt beyond the two-attempt budget.

## Amendment (2026-08-23) — issue #88 re-land on the wave-1 foundation

Appended, not rewritten. The clauses below are **superseded** by master plan #83; everything this
brief says that is not listed here still stands, and in particular the parameter table, the port
list, latency `N = Fs/50`, the `L`/`D` derivation and ring semantics, the link laws, the four
identities, the state payload layout and `state_layout_version = 1` are unchanged.

### Superseded implementation clauses

| brief clause | superseded by | what it says now |
|---|---|---|
| The `X = clamp(20*log10(u0), -160, 24)` and `A = 10^((G+makeup)*0.05)` conversions (the sample-graph block around line 136 and line 165) | **D6** | `X = log2_lane(u0) * 20*log10(2)` and `A = exp2_lane((G+makeup) * log2(10)/20)`, through `miso-engine-math`. The platform libm may not be called from a render path: `f32::log10`, `f32::powf` and `f32::exp` are whatever the target links, which is audit finding F1 and the reason the cross-target claim needed a tolerance. |
| `aa = exp(...)`, `ar = exp(...)` evaluated **per sample** (lines 159-160) | **D6**, and audit finding F1 | The two ballistic coefficients are designed on the control plane, in `f64`, through `miso_engine_math::exp`, and are recomputed only while a `Linear 64` ramp on `attack` or `release` is in flight — at most 64 samples after an event. `1 - exp(-1/(tau*fs))` evaluated in `f64` and rounded **once** also fixes a real accuracy defect: at the release maximum, 5,000 ms at 96 kHz, computing it in `f32` leaves about seven significant bits of the coefficient. |
| `p0 = a*G_previous; p1 = (1-a)*C; G = p0+p1` — two roundings (lines 161-163) | **D3** | `G = fma(c, C - G_previous, G_previous)` — one rounding, with `c = 1 - a` the *rate* coefficient. `envelope::rms_follow` composed with one `Lane::select` for the strict `C < G` branch. The direction convention is unchanged: equality takes release. |
| "The bank walks tracks in ascending lane order for sanitation, linking, ring access, scalar `log10/exp/powf`, gain-computer and smoother work" (lines 233-240) | **D10** | There is no per-lane scalar walk. One `Lane`-generic block body is instantiated at `f32`, `Simd4` and `Simd8`; the scalar instance *is* the `L = f32` instantiation. The only remaining per-lane loop inside a frame is the detector gather, which is per lane because `lookahead` is a per-lane parameter and deliberately not part of the program key. The rings are one row-major allocation per channel rather than two per lane. |
| "sanitation/recovery" and the per-value `is_finite`/subnormal checks | **D7** | There is no per-value sanitiser, no `recover` and no recovery frame. `miso_engine_lane::flush` is applied to `G`, the one recursive word, and the finished block is inspected once per channel by `effect_runtime::bank::finish_channel`. `ProcessReport::sanitized_main_samples` and `sanitized_sidechain_samples` are therefore always `0`, and `nonfinite_left_blocks`/`nonfinite_right_blocks` now count **rejected blocks**, not samples. |
| "Cross-target results use `abs(error) <= 1e-6 + 2e-5*abs(reference)`" (lines 269-270) | **D5** | Deleted, not loosened. Cross-target and cross-backend results are `to_bits()` identity. `miso_engine_compressor::corpus` is the frozen four-case corpus and `tools/miso-engine-wasm-gates` replays it under wasmtime, with and without `simd128`, against the same pins the native test uses. |
| The ramp law "`current += (target - current) / remaining`" | **D11** | One division at the event, iterated additions, an exact assignment of the target on the final sample: `effect_runtime::ramp::LinearRamp`. |
| "the gain/mix step remains one packed W4/W8 call per channel/sample" | **D10** | `PreparedCompressorGainMixKernelV1` is gone; the step is `miso_engine_lane::kernels::gain_mix_step` inside the one kernel body, at the width the build was compiled for. |

### Behaviour a caller can observe

- Output samples are no longer flushed. Only `G` is (D7). A tiny wet sample now passes through
  instead of becoming `+0.0`.
- A NaN or a magnitude at or above `1e30` in the output is caught at the **end of the block** it
  appears in, not at the sample. The block is zeroed, that channel's rings, cursor and `G` are
  cleared, and one counter is incremented. The two channels are independent: a divergent right
  channel leaves the left channel bit-identical to a clean render.
- A NaN reaching only the *detector* — a connected sidechain carrying one — is clamped to the level
  floor and produces no gain reduction, rather than poisoning `G`. Two clamps do this and either
  alone would: the D8 `max` against `1e-8` and `log2_lane`'s own argument clamp.
- Bypass, `mix == 0` and `mix == 1` keep the detector and `G` running, as before. This is
  intentional and is what makes un-bypassing click-free; it answers open question 5 of the audit.
- The bank's per-block guard now rejects `frames == 0`, a slice whose length does not match
  `frames * lanes`, an automation-offset array of the wrong length and offsets that are descending
  or run past the span slice — **before** it indexes them.
- A bank binds only at the width the build was compiled for. D4 revision 4 removed runtime SIMD
  dispatch, so a request for any other width is an unavailable backend and takes the `Ok(None)`
  scalar fallback, exactly as an unavailable backend did before.
