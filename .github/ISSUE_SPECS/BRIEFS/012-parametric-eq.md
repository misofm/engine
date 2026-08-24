# Sol implementation brief — issue 012 Parametric EQ

## Post-stop verdict (2026-08-21)

**STOPPED/RESCOPED; no overall PASS.** Do not continue this brief, change its tolerances/probes or
invoke its benchmark. Checkpoints `46b4a37`, `7b9c01b` and `cf739ef` preserve bounded scalar,
automation and architecture-kernel work only as technical input for Issue 042.

The independent oracle reproduced the frozen 44,100 Hz, 10 Hz, -24 dB, S=0.1 low-shelf case as
-23.9999999963 dB at DC, while the exact independently rounded production `f32` coefficients
evaluate to -23.4572457785 dB. The 0.5427542178 dB miss is caused by cancellation in the retained
near-`1,-2,1` direct-form coefficients, not by the f64 oracle or transfer evaluator. Removing DC is
insufficient: the same row misses by 0.00776, 0.02292, 0.10107 and 0.08333 dB at the 10 Hz probe
for 44.1, 48, 88.2 and 96 kHz respectively. Low-frequency bell/pass/notch rows and exact notch
nulls expose the same representation limit.

The original frozen brief below remains historical failure evidence. **Numerically conditioned
launch parametric EQ realization** must compare conditioned candidates across the full unchanged
grid before Sol freezes any replacement coefficient/state representation or recurrence.

## Decision and attempt budget

**HISTORICAL PRE-STOP DECISION; NOT READY FOR FURTHER WORK.** Implement one four-section dual-mono EQ vertical. The normal
workflow permits Terra attempt 1 and at most two Sol corrections/reviews, then stops for rescope.
Never inspect V1/legacy.

Issues 002 and 011 are accepted inputs. Issue 008 stopped without overall PASS, but checkpoint
87783c5 is explicitly accepted as technical input for safe target dispatch, generic AoSoA
storage, effect-bank traits and exact-key graph retention. Do not claim Issue 008 PASS, reopen its
builtin work or depend on its benchmark history.

Issue 012 owns direct effect-boundary automation, audition PCM and listening preregistration. It
does not own graph/control automation delivery or completed human trials; record those only as the
follow-ups named in the issue body. Benchmark invocation count starts at zero.

## Accepted contracts and production boundary

- Issue 011 owns EffectDescriptorV1, factory/prepared traits, state sections, canonical spans,
  immutable metadata/program keys, reset, sanitization and latency-preserving bypass.
- BankWidth is only Four/Eight; scalar uses PreparedNativeEffect. Target/FMA selection is off
  render. Bank audio is separate L/R [sample][track] AoSoA.
- Existing graph topology, PDC, schedule, reductions and observer order do not change. The static
  integration fixture may use existing bank retention; production graph automation is untouched.
- Launch rates are only 44,100/48,000/88,200/96,000 Hz. There are no extended-rate rows.
- No octave unit exists. Launch controls Q and shelf slope, never mislabeled octave bandwidth.

Add crates/miso-engine-parametric-eq / miso_engine_parametric_eq, depending in production only on
core and effect-contract. Add a narrow prepared DF-I token under core's existing src/arch unsafe
boundary and re-export it safely. Intrinsics, pointers and target-feature functions remain private.

Extend the test-only DSP reference with independent f64 EQ families and analytic response. Complete
dsp-research/filters.md using existing bibliography keys. Add one fixture family, fixture/report
tool, audit tool, benchmark tool/runner and minimal workspace/policy/target wiring. Production
never depends on oracle/conformance crates.

Product tests may use effect/session/graph crates as dev dependencies. Do not change the generic
Issue-011 conformance runner to conceal its hardcoded one-parameter/sidechain assumptions. Do not
edit session/protocol wire, package/CID/persisted state, builtins, scheduler, sources, hosts or C
ABI.

## Effect, bands and parameters

Freeze:

    effect ID             miso.parametric-eq
    contract              1.0
    state layout          1
    quality               Normal only
    ports                 required main-in/main-out; no sidechain
    link modes            DualMono only
    section count/order   exactly 4; 0 -> 1 -> 2 -> 3
    latency               0 samples
    tail                  Infinite
    state sizes           common=0, left=256, right=256 bytes
    scratch               fixed=0, per-frame=0

Export EQ_SECTION_COUNT_V1=4, EqBandKindV1 with Bell=1, LowShelf=2, HighShelf=3,
LowPass=4, HighPass=5, Notch=6, and EqBandDescriptorV1 fields index, cascade_order, enabled, kind,
frequency_hz, gain_db, q and shelf_slope.

For section i, base=1+16*i: enabled=base+0, kind=+1, frequency=+2, gain=+3, Q=+4, slope=+5.
Gaps are reserved and are not parameters. Export one static four-entry descriptor table. Effect
parameters remain strictly ID-ordered.

All 24 parameters are PerLane/readable:

| Field | Unit/display | Domain/default | Mapping | Automation |
| --- | --- | --- | --- | --- |
| enabled | Linear / on/off | Boolean; 0 | Stepped | None |
| kind | Linear / type | Enum 1..=6; Bell | Stepped | None |
| frequency | Hz / Hz | 10..=20,000; defaults 80/400/2000/10000 | Log | Block point, Linear 64 |
| gain | dB / dB | -24..=24; 0 | Linear | Block point, Linear 64 |
| Q | Ratio / Q | 0.1..=18; 0.70710677 | Log | Block point, Linear 64 |
| shelf slope | Ratio / S | 0.1..=1; 1 | Linear | Block point, Linear 64 |

Gain applies to bell/shelves, Q to bell/pass/notch and S to shelves. Inapplicable values remain
valid/readable but do not affect coefficients. Enabled/kind change only by replacement preparation.
A session Both value expands to independent L/R values and state.

All sections use fixed storage and a fixed loop bound. Disabled is exact identity. Whole bypass is
PrepareEffectRequest.bypass, not another parameter.

## RBJ design and stability

Cite [RBJ-COOKBOOK] for coefficient families/normalization and [SMITH-SASP]/[ORFANIDIS-ISP] for
realization/stability, without sound-quality claims. For legal Fs,f,G,Q,S, calculate in f64:

    w=2*pi*f/Fs; c=cos(w); s=sin(w); A=10^(G/40)
    alpha_q=s/(2*Q)
    alpha_s=s/2*sqrt((A+1/A)*(1/S-1)+2)
    beta=2*sqrt(A)*alpha_s

Raw coefficients (b0,b1,b2,a0,a1,a2):

    LP: b=((1-c)/2,1-c,(1-c)/2); a=(1+alpha_q,-2c,1-alpha_q)
    HP: b=((1+c)/2,-(1+c),(1+c)/2); a=(1+alpha_q,-2c,1-alpha_q)
    Notch: b=(1,-2c,1); a=(1+alpha_q,-2c,1-alpha_q)
    Bell:
      b=(1+alpha_q*A,-2c,1-alpha_q*A)
      a=(1+alpha_q/A,-2c,1-alpha_q/A)
    LowShelf:
      b0=A*((A+1)-(A-1)*c+beta)
      b1=2*A*((A-1)-(A+1)*c)
      b2=A*((A+1)-(A-1)*c-beta)
      a0=(A+1)+(A-1)*c+beta
      a1=-2*((A-1)+(A+1)*c)
      a2=(A+1)+(A-1)*c-beta
    HighShelf:
      b0=A*((A+1)+(A-1)*c+beta)
      b1=-2*A*((A-1)+(A+1)*c)
      b2=A*((A+1)+(A-1)*c-beta)
      a0=(A+1)-(A-1)*c+beta
      a1=2*((A-1)-(A+1)*c)
      a2=(A+1)-(A-1)*c-beta

Require finite intermediates and nonzero a0. Normalize in f64 to B0/B1/B2/A1/A2, cast once to
f32, require finite casts, then require:

    abs(A2)<1
    1+A1+A2>0
    1-A1+A2>0

Every legal value must design; never clamp. Initial failure returns effect.eq.coefficients. A
runtime redesign failure retains last valid parameters/coefficients, resets only that section,
emits +0 for that lane sample and reports recovery.

Disabled sections and bell/shelves at normalized +0 dB select exact identity. Identity returns the
input bits and warms histories as x1=y1=input and x2=y2=prior_x1, allowing later nonzero automation
without cold history.

## Frozen recurrence and architecture token

Each section/lane owns f32 x1,x2,y1,y2. Scalar, Wasm SIMD, NEON and AVX2-no-FMA preserve:

    p0=B0*x; p1=B1*x1; s0=p0+p1
    p2=B2*x2; s1=s0+p2
    p3=A1*y1; s2=s1-p3
    p4=A2*y2; y=s2-p4
    x2'=x1; x1'=x; y2'=y1; y1'=y

AVX2+FMA is separately dispatched and may fuse exactly:

    p0=B0*x
    s0=fmadd(B1,x1,p0)
    s1=fmadd(B2,x2,s0)
    s2=fnmadd(A1,y1,s1)
    y=fnmadd(A2,y2,s2)

No other reassociation/contraction. Base Wasm uses explicit f32x4 arithmetic/bit-select and no
relaxed SIMD.

Add safe PreparedBiquadBankKernelV1.try_new(KernelBackendV1). It detects x86 features only at
preparation and retains a safe function pointer. Processing takes exact-lane sample, five
coefficient, four mutable history and identity-mask slices; lengths and exact zero/all-one masks
validate before entry. Lanes are 1/4/8 for scalar/Wasm-or-NEON/AVX2.

Given identical coefficient/input/state bits, finite-normal data and no recovery, base same-target
paths are bit-identical. FMA/cross-target comparisons use:

    abs(candidate-scalar) <= 1e-6 + 2e-5*abs(scalar)

If a bounded preimplementation recurrence probe disproves this bound, amend/rebrief before
production; never tune it afterward.

## Smoothing, state and process behavior

Enabled/kind reject spans. Numeric fields accept only Point at the block's first sample. Reject and
count once per span other kinds/times, ordering/duplicate/overlap errors, wrong parameter/channel,
nonfinite or out-of-domain values. Rejection leaves the prior target unchanged.

Set all same-sample targets before update. Update 1 supplies the first sample; linear smoothing adds
(target-current)/remaining, reaches exact target on update 64, and restarts from current on a later
point. Redesign from current smoothed values at each active update; unchanged sections reuse cache.
Pure transcendental math must audit without forbidden imports, allocation, locks, I/O or syscalls.

The measurable continuity contract is the exact 64-value trajectory, exact endpoint and finite
strict-Jury coefficient path. Do not invent a signal-independent adjacent-output bound. Compare
each update's cast analytic response with the independent current-parameter response.

Common payload is empty. Each 256-byte lane contains four 64-byte band records. Each record is 16
little-endian words:

    x1,x2,y1,y2,
    frequency.current,frequency.target,frequency.remaining,
    gain.current,gain.target,gain.remaining,
    q.current,q.target,q.remaining,
    slope.current,slope.target,slope.remaining

Float words are exact f32 bits; remaining is u32<=64. Kind/enabled/prepared defaults/cache are
immutable/derived and absent. Restore accepts version 1 and the same prepared configuration,
validates all bytes/domains/counts/derived coefficients in bounded temporary state, then commits.
Snapshot is deterministic/all-or-none. Scalar continuation is bit-exact; bank track state matches.

Full reset zeros histories and restores prepared initial current/targets. Discontinuity reset zeros
histories, retains targets, snaps currents and clears remaining. Both redesign and preserve metadata.

At entry, nonfinite/subnormal main input becomes +0 and increments sanitization; finite signed zero
is valid. After each active section inspect output/new histories. If any is nonfinite/subnormal,
zero that section's histories/output and mark recovery. A lane recovery counter increments at most
once per sample. Valid stability cases may not use recovery.

Whole bypass saves sanitized dry input, advances hidden cascade/automation state, then writes dry
bits. Enabled/bypass impulses land at sample zero. Stable IIR state has no fixed finite drain bound,
so tail is Infinite; reset flushes it.

## Homogeneous bank

bind_homogeneous_bank validates every request, matching backend/width, exact request count and equal
program keys; per-track values may differ. Return Ok(None) for a legal unavailable backend.

Transpose:

    coefficients [section][L/R][B0/B1/B2/A1/A2][track]
    histories    [section][L/R][x1/x2/y1/y2][track]
    parameters   [track][L/R][section]

Consume each track's bounded span subslice, update targets before the first sample, and execute
four sections independently for L/R. Reports remain per track; entries beyond width are zero.
Snapshot/restore uses scalar encoding. Do not add another cohort compiler or pad scalar tails.

The production fixture uses nine compatible tracks: one 8-wide bank plus one tail on available
AVX2, two 4-wide banks plus one tail on four-lane targets, all scalar under scalar dispatch.

## Oracle, response and stability matrices

The f64 oracle must not call production design/processing. Freeze the full applicable Cartesian
grid at every launch rate:

    f0:   10,20,100,1000,10000,20000 Hz
    Q:    0.1,0.7071067811865476,1,18
    gain: -24,-6,0,6,24 dB
    S:    0.1,0.5,1

Bell uses f0*Q*gain; pass/notch f0*Q; shelves f0*gain*S. Probe a 2,048-point log grid
10..20,000 Hz plus exact f0 and DC/Nyquist limits. Require:

- finite/Jury-valid casts for every row;
- cast analytic error <=0.005 dB where reference >=-120 dB;
- one-second impulse/DFT error <=0.05 dB above -120 dB;
- theoretical null cast-analytic magnitude <=-100 dB;
- sustained amplitude-0.5 probes settle Fs/2 and measure Fs/4: above -90 dB reference,
  fundamental error <=0.05 dB and fitted residual <=-100 dB relative input RMS; below it,
  production total gain <=-88 dB.

Frequency gates:

- LP/HP at Q=1/sqrt(2): -3.0102999566 dB crossing within 0.1% of f0;
- nonzero bell: peak/cut center within 0.1%, gain within 0.005 dB;
- nonzero shelf: half-gain midpoint within 0.1%;
- notch: minimum within 0.1% and absolute null gate.

All-disabled and zero-dB bell/shelf output is bit-identical, including signed zero. A four-section
cascade matches the independent product response.

Run exactly 10,000 legal designs from seed 0x000000000012e911, stratified by rate/kind/endpoints.
Report worst Jury margin/coefficient/response; no legal rejection.

Run exactly 48 million-sample sequences: every rate/kind at
f=10,Q=0.1,gain=-24,S=0.1 and f=20000,Q=18,gain=24,S=1, ignoring inapplicable values. Input is
deterministic finite-normal bipolar noise <=0.99 with initial impulse. Output/state stays finite
normal-or-zero with zero recovery. Separate NaN/infinity/subnormal/signed-zero/alternating f32::MAX
probes prove sanitization and bounded recovery.

Automation covers every numeric field/lane/edge, 64 updates, simultaneous targets, restart,
malformed spans and partitions 1/63/64/127/128. Scalar/bank tests prove resets, bypass, state
continuation and metadata immutability. A one-lane/track perturbation leaves every other output,
report and payload bit-identical.

## Compact fixture and graph vertical

Check in only:

    fixtures/effects/parametric-eq-v1/MANIFEST.tsv
    fixtures/effects/parametric-eq-v1/cases.toml
    fixtures/effects/parametric-eq-v1/input.f32le
    fixtures/effects/parametric-eq-v1/scalar-expected.f32le
    fixtures/effects/parametric-eq-v1/reference-response.csv

The 48-kHz/128-frame nine-track asymmetric fixture covers all kinds, enabled/disabled, zero/nonzero
gain, distinct lane/track parameters, whole bypass, full bank(s) and scalar tail. It has no
automation because graph delivery is follow-up scope. Scalar alone produces expected PCM.

The sorted manifest freezes safe relative paths, exact length and lowercase SHA-256. Read-only
--check rejects changed/missing/unlisted/unsafe/coverage-invalid files. Response CSV fully expands
the grid; plots are derived evidence, not another corpus.

Prepare via public registry -> effect compiler -> graph compiler. Assert bank membership/tail,
unchanged graph/PDC/schedule/observer/canonical metadata and differential output. No track ceiling.

## Nonbenchmark gates

Before timing, one candidate passes:

1. Formatting and focused locked core/EQ/effect/rack/graph/oracle/fixture tests.
2. Descriptor/registry/session invalid parameter/unit/domain/channel/quality/link/port/resource and
   metadata-exactness mutations.
3. Fixture check and changed/missing/unlisted/path/coverage mutations.
4. Full response grid, 10,000 designs, 48 stability runs, automation/recovery/reset/state/identity.
5. Scalar/four/eight/FMA differential, bank offsets and lane/track isolation.
6. Nine-track public registry-to-render bank/tail vertical.
7. Exactly 100,000 prepared 128-frame graph renders: while armed, zero allocation/free, lock, log,
   file/network I/O, syscall, feature detection, panic/unwind or structural mutation; destroy only
   after disarming.
8. Locked workspace check/tests, warning-denied Clippy/rustdoc, relevant policy scripts/mutations.
9. Native baseline; x86 AVX2/no-FMA and AVX2+FMA probes; Android/iOS AArch64; Wasm scalar and
   +simd128 builds.
10. Named inspection proves baseline no AVX/FMA, AVX2 packed nonfused arithmetic, exactly four FMA
    sites, NEON four-lane, Wasm explicit SIMD/bit-select and no relaxed SIMD.
11. scripts/preflight-parametric-eq-benchmark.sh validates arguments, schema/record mutations,
    persistence, shell failures and overwrite refusal with workload_launches=0.

Cross-target results are compile/instruction claims. Runtime-unavailable FMA can be reported only
with selection/object proof, never as executed. Legal-domain rejection, shared state, relaxed
tolerance, runtime allocation/detection, generic-conformance false claim, graph-automation claim
or early benchmark is immediate FAIL.

## Descriptive benchmark — exactly once

After every gate and explicit root authorization, invoke exactly:

    bash scripts/run-parametric-eq-benchmark.sh

No arguments/overwrite. One untimed warmup, exactly two measured rounds, no tuning/retry. Three
48-kHz/128-frame workloads: one four-section scalar track; one full host-selected bank; nine-track
graph bank+tail. Each round has 1,000 observations and reports nearest-rank
min/p50/p95/p99/p99.9/max ns/frame/track, cycles if available, backend/width, fixture/build hash,
allocations, CPU/OS/governor/Rust/LLVM/features/optimization/LTO/codegen and missing metadata.
Exactly six valid JSONL records, zero errors.

There is no timing threshold. Preserve first raw output if promotion fails and do not rerun.
Runner defects move to tooling; performance surprises move to weekly optimization.

## Evidence and listening handoff

After objective sealing, generate level-matched audition PCM for bell boost/cut, shelves, pass
filters, notch, cascade, asymmetric lanes and automation ramp. Freeze hashes/matching method and a
blinded preregistration using the listening template. Do not fabricate listeners or quality claims.

Append to Issue 012: candidate commit; API/parameter/state tables; research decision; fixture/
response hashes; response/frequency maxima; randomized seed/count; stability, automation, state,
isolation and differential results; instruction/audit/target findings; benchmark preflight count;
and, only after authorization, benchmark hashes/count. Record audition/preregistration hashes and
point completed human evidence to its follow-up.

## 2026-08-24 amendment (#84 phase A)

Superseded by #83 D4/D10 via #84 phase A: the per-sample kernel tokens
(`Prepared*KernelV1`), `KernelBackendV1`, `TargetCapabilities`,
`miso_engine_core::target_capabilities()` and `miso_engine_rack::KernelDispatch` were
deleted along with `crates/miso-engine-core/src/arch`. Kernels live in
`crates/miso-engine-lane`; the backend is the compile-time constant
`miso_engine_lane::Backend::current()`, and
`miso_engine_effect_contract::BankWidth::for_backend` is the one backend-to-width law.
The historical text above is kept as the decision record of its time and is not rewritten.
