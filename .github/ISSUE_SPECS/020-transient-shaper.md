# 020 Launch dual-envelope transient shaper

## Outcome

Deliver one causal, dual-mono transient shaper whose fast/slow peak-envelope contrast controls
attack and sustain gain, with scalar processing, homogeneous W4/W8 banks, scalar tails and one
launch-registry-to-graph vertical.

## Context

Engine V2 is greenfield and must never inspect or inherit V1/legacy. The realtime plane owns a
preallocated immutable-shape `PreparedRenderPlan`; render performs no allocation/free, locks,
feature detection, I/O, logging, syscalls, panic/unwind, structural mutation or data-dependent
unbounded work. Audio/state/parameters are dual-mono except for an explicit detector-link mode.
Launch rates are exactly 44,100, 48,000, 88,200 and 96,000 Hz.

This issue consumes the accepted native-effect runtime, Issue-013 dynamics conventions, prepared
compressor gain/mix bank kernel, launch registry and graph/PDC seams. It has exactly **two total
attempts**: Terra attempt 1 and one bounded Sol correction/review. A second failure stops and
requires a stateless rebrief; no gate may weaken. `timed_benchmark_invocations=0` and no benchmark
is authorized here.

## Scope

- Add `miso.transient-shaper`, contract 1.0, state layout 1 and Normal quality at every launch
  rate, with required dual-mono `main-in`/`main-out` and no sidechain.
- Use fixed causal fast and slow instantaneous-peak followers. Their signed dB contrast selects
  attack versus sustain shaping; output gain is bounded to +/-18 dB.
- Reuse `LinkMode::{DualMono, Maximum, Average}`. Linking shares only the current detector
  magnitude; envelope, parameter, recovery and state payload remain lane-local.
- Automate only attack amount, sustain amount and wet mix with exact 64-update Block Point ramps.
  V1 has fixed detector timing/range, no lookahead, zero latency and `TailSamples::Finite(0)`.
- Implement scalar and homogeneous W4/W8 banks using the accepted prepared compressor gain/mix
  kernel, scalar tails, registry/effect-compiler integration and one ten-track graph fixture.

## Required public interfaces/contracts

`TransientShaperFactory` implements `NativeEffectFactory`; scalar and bank products implement the
accepted `PreparedNativeEffect` and `PreparedNativeEffectBank` traits. Descriptor positions and
stable IDs are identical; all controls are readable `PerLane` values:

| ID | control | unit | inclusive domain | default | mapping | automation/smoothing |
|---:|---|---|---:|---:|---|---|
| 1 | attack amount | linear (`%` display) | -1..1 | 0 | linear | Block Point / Linear 64 |
| 2 | sustain amount | linear (`%` display) | -1..1 | 0 | linear | Block Point / Linear 64 |
| 3 | mix | linear | 0..1 | 1 | linear | Block Point / Linear 64 |

For linked detector magnitude `u`, fixed fast/slow envelopes `f,s`, and smoothed values `A,S,M`:

```text
e = a*e_previous + (1-a)*u       // attack coefficient when u>e_previous, release otherwise
c = clamp(20*log10(max(f,1e-8)) - 20*log10(max(s,1e-8)), -24, 24)
shape_db = clamp(A*max(c,0) + S*max(-c,0), -18, 18)
gain = 10^(0.05*shape_db)
wet = x*gain
out = x + M*(wet-x)
```

The tracked brief freezes coefficient bits, operation order, identity selection, state/resources,
automation, reset/restore/recovery and scalar/W4/W8 parity. Each lane state is exactly 11 words / 44
bytes; complete scalar state is 88 bytes and fixed reset defaults are 24 bytes. Exact retained
effect envelopes are 112 bytes per scalar track, 448 bytes/W4 and 896 bytes/W8.

The accepted runtime rejects negative-zero initial parameter values before factory publication;
automation and restored numeric-zero parameter words normalize to positive zero. Sanitation reports
one increment in the track's aggregate main-input counter for each sanitized lane sample.

## Deliverables

- `miso-engine-transient-shaper` descriptor/factory, scalar and homogeneous bank products;
- the smallest effect-compiler/registry integration using existing seams;
- independent representative envelope/contrast/identity/state/recovery tests; and
- one width-correct ten-track bank-plus-tail graph/resource fixture.

## Explicit non-goals

Lookahead, sidechain, RMS, hold, adaptive/program-dependent timing, detector filters, user detector
speed/sensitivity/range controls, another envelope topology, another quality or mode, auto gain,
clipping, broad corpus/matrices, 10,000 or million-sample rows, realtime audit, cross-target or
instruction qualification, benchmark/preflight/timing, optimization, audition or completed
listening. Qualification belongs only to Issue 054, **Launch transient-shaper qualification,
realtime audit, and benchmark**.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Production SIMD builtin bank graph retention and reachability qualification
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch feed-forward peak compressor

Stopped Issue 008 contributes only its preserved generic bank architecture, not an overall PASS or
benchmark claim.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after local/remote synchronization.** The authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/020-transient-shaper.md`. This docs checkpoint performs no
implementation, benchmark or GitHub mutation.

## Hazards/decisions

The product is truthfully described as a fixed dual-envelope contrast shaper, not a source
classifier. Detector/gain separation and one-pole time constants follow `[REISS-COMP]`; bounded
filter state follows `[SMITH-SASP]`. Zero lookahead avoids an invented latency framework. A zero
audio input produces zero output despite retained envelope state, so the exact audio tail is zero.

## Acceptance gates with objective measurements

1. Descriptor/program/quality/link/port/parameter/resource mutations reject transactionally at all
   launch rates; exact and one-byte-below state/scratch/bank caps pass.
2. Independent `f64` impulse, step and decaying-burst rows prove the four frozen envelope time
   constants, contrast sign and active +/-attack/sustain behavior within 0.01 dB and the greater of
   one sample or 2% timing.
3. Defaults, bypass, mix zero and computed zero shape return input bits exactly while warming state;
   signed-zero, silence and zero-latency/tail metadata pass.
4. All links distinguish as specified. Exact first/63rd/64th automation updates, retarget, both
   resets, active continuation restore, sanitation, injected lane recovery and lane/track isolation
   pass without hidden coupling.
5. Available same-target scalar/W4/W8 output, complete state and reports are bit-identical for
   finite-normal inputs. AVX2+FMA has zero contractions; unavailable legal backends fall back only
   after complete request validation. Scalar tails cover every count.
6. A ten-track graph retains host-width-correct full banks plus scalar tails, stable membership,
   zero latency/PDC, canonical enabled/bypass shape, scalar-delegate PCM/state and transactional
   corrected post-bank one-byte-below ownership return.
7. Focused and clean locked workspace format/check/test/Clippy/rustdoc plus applicable workspace,
   realtime, effect-runtime, rack and graph policies pass. No Issue-054 or excluded command runs;
   `timed_benchmark_invocations=0`.

## Target matrix

Product closure executes scalar and the available native bank backend. W4/W8 source/selection
contracts are mandatory; complete native/AArch64/Wasm target and instruction evidence is Issue 054.

## Required evidence

Candidate identity; descriptor/coefficient/state/resource tables; independent representative
maxima; identity/link/automation/reset/restore/recovery/bank/graph rows; exact commands and policies;
attempt count; strict Terra/final Sol verdict; successor link; and
`timed_benchmark_invocations=0`.

## Terra attempt 1 — scalar checkpoint (partial PASS)

- Added `miso-engine-transient-shaper`: Normal-only `miso.transient-shaper` scalar factory with
  frozen four-rate coefficient-bit rows, three dual-mono 64-update ramps, fixed 11-word/44-byte
  lane state, 88-byte scalar state and 24-byte reset-default envelope. The scalar path implements
  the frozen peak/link/follower/contrast graph, zero-latency/tail identity behavior, atomic restore,
  sanitation and lane-local recovery.
- Added an independent test-only `f64` transient-shaper oracle in `miso-engine-dsp-reference`.
  Focused tests cover descriptor/coefficient/resource caps, oracle comparison, signed-zero/default
  identity, exact detector-link magnitudes, ramp/state continuation, both resets, sanitation and
  injected lane recovery.
- PASS: `cargo fmt --check --package miso-engine-transient-shaper --package miso-engine-dsp-reference`;
  `cargo test --locked -p miso-engine-transient-shaper --lib` (3 passed);
  `cargo test --locked -p miso-engine-dsp-reference --lib` (6 passed, 1 documented ignored);
  `cargo clippy --locked -p miso-engine-transient-shaper --all-targets -- -D warnings`.
- This is a scalar-only partial checkpoint: W4/W8 banking, registry/effect-compiler/graph, Issue-054
  qualification, audits, targets and benchmark work remain unstarted. `timed_benchmark_invocations=0`.
  Terra checkpoint verdict: partial PASS; pause for root commit and Sol review.

## Sol attempt 2 — bounded scalar contract correction (partial PASS)

- Base candidate `d4f8c8c`. No production DSP equation, coefficient, state layout, descriptor,
  resource value or runtime path changed. The brief now preserves the accepted runtime's rejection
  of negative-zero initial values and names sanitation telemetry correctly as one aggregate
  main-input increment per sanitized lane sample.
- The independent reference now exposes its own validated f64 one-pole coefficient derivation.
  Tests derive and cast all sixteen launch-rate/time-constant values independently, match every
  frozen production bit pattern, and recover each 0.5/20/10/100-ms time within the greater of one
  sample or 2% gate.
- Compact impulse, step and warmed decaying rows execute positive and negative attack and sustain
  shaping. Every measured production/reference gain error is at most 0.01 dB, and the decay rows
  prove active negative contrast rather than merely comparing a rising envelope.
- Exact payload evidence covers all 11 words/44 bytes per lane, 88-byte state and 24-byte fixed
  defaults, exact-cap preparation, one-byte-below state and scratch rejection, and runtime
  negative-zero-initial rejection. Automation asserts update 1, 63 and 64 bits, retarget-from-
  current, right-lane isolation and active snapshot/restore continuation.
- Both resets now compare every payload word with independent expected bytes. Public processing
  proves default, bypass and mix-zero signed-bit identity while followers warm; one NaN plus one
  subnormal lane sample produces exactly two aggregate sanitation increments. An injected left
  follower fault exercises the public process/report path, returns dry, clears only that lane and
  leaves right PCM/state bit-identical to a healthy peer.
- PASS: `cargo fmt --all -- --check`; `cargo test --locked -p
  miso-engine-transient-shaper --lib` (8 passed); `cargo test --locked -p
  miso-engine-dsp-reference --lib` (6 passed, one pre-existing Issue-044 candidate ignored); and
  `cargo clippy --locked -p miso-engine-transient-shaper -p miso-engine-dsp-reference
  --all-targets -- -D warnings`.
- This is the frozen scalar correction checkpoint within the second and final authorized attempt,
  not an overall Issue-020 verdict. Bank, registry/effect-compiler, graph/PDC/cap and final
  workspace/policy closure remain unstarted. Issue 054 and all audit/target/instruction/benchmark/
  listening work remain untouched; `timed_benchmark_invocations=0`.

### Sol attempt 2 — homogeneous-bank checkpoint (partial PASS)

- Base candidate `bc38407`. Added W4/W8 homogeneous binding through the already accepted
  `PreparedCompressorGainMixKernelV1`. Each bank retains independent scalar-equivalent follower,
  ramp and reset-default rows, walks sample-major tracks deterministically, uses the frozen
  dry/wet identity masks, and exposes byte-compatible per-track snapshot/atomic restore/reset.
- Every member's metadata, initial values and exact 88-byte state/24-byte scratch caps validate
  before heterogeneous or unavailable-backend fallback. Executed evidence freezes retained
  envelopes at `4*(88+24)=448` and `8*(88+24)=896`; one-byte-below state and scratch members reject
  before legal unavailable W4 fallback, and mismatched backend/width/count rejects.
- Native W8 evidence is bit-exact to eight scalar peers for sample-major PCM, all state words and
  reports across DualMono/Maximum/Average links, distinct per-track defaults, block automation,
  signed zero, NaN/subnormal sanitation, both resets, track restore, injected lane-local recovery
  and healthy-track isolation. The core prepared token's existing tests continue to prove the W8
  operation graph and zero-contraction AVX2/FMA alias.
- PASS: `cargo fmt --all -- --check`; `cargo test --locked -p
  miso-engine-transient-shaper --lib` (11 passed); `cargo test --locked -p
  miso-engine-core --lib` (31 passed); `cargo clippy --locked -p miso-engine-core -p
  miso-engine-transient-shaper --all-targets -- -D warnings`.
- This remains a partial Issue-020 checkpoint. Registry/effect-compiler and graph/PDC/cap closure
  are deliberately unstarted, as are all Issue-054 qualification gates. No broad workspace,
  audit, target, benchmark or listening command ran; `timed_benchmark_invocations=0`.

### Sol attempt 2 — registry and graph checkpoint (partial PASS)

- Base candidate `632cb75`. Added `miso.transient-shaper` to the caller-owned launch registry and
  exact effect-compiler dependency allowlist. Baseline and mutation policy now require the
  transient-shaper dependency and reject its removal, substitution, or an arbitrary extra effect
  dependency.
- The frozen 48-kHz/q128 ten-track fixture uses one homogeneous dual-mono program with distinct
  per-track initial values and no sidechain. It proves host-width full-bank membership plus stable
  scalar-tail order, and compares two consecutive rendered blocks bit-for-bit with a scalar-only
  delegate graph while follower state crosses the block boundary.
- Enabled and bypass graphs both retain zero latency, zero PDC, identical schedule, route timing,
  inserted-delay set and canonical debug bytes. Exact post-bank bank-count, scratch, runtime-buffer
  and metadata rows reconcile with both incremental-plan and session-plus-plan estimates. A plan
  cap one byte below the complete post-bank estimate rejects transactionally and returns all ten
  prepared effect owners.
- PASS: `cargo fmt --all -- --check`; `cargo test --locked -p
  miso-engine-effect-compiler --test native_session` (4 passed); `cargo test --locked -p
  miso-engine-graph-compiler --lib
  launch_transient_shaper_fixture_closes_banks_tails_pdc_and_transactional_caps` (1 passed, 20
  filtered); `cargo clippy --locked -p miso-engine-effect-compiler -p
  miso-engine-graph-compiler --all-targets -- -D warnings`; `bash
  scripts/check-effect-runtime-policy.sh .`; `bash scripts/test-effect-runtime-policy.sh .`; and
  `bash -n scripts/check-effect-runtime-policy.sh scripts/test-effect-runtime-policy.sh`.
- This is the final focused product checkpoint before the separately authorized clean workspace
  seal, not an overall Issue-020 verdict. Issue 054, audit mains, target/instruction inspection,
  benchmarks and listening remain untouched; `timed_benchmark_invocations=0`.

## Sol attempt 2 — final nonbenchmark product seal

- Final candidate `4f2c1c4` preserves the accepted scalar, homogeneous-bank, registry and graph
  checkpoints above. No production, fixture, policy or qualification file changed during this
  seal.
- `cargo fmt --all -- --check`: PASS. `cargo check --locked --workspace --all-targets
  --all-features` and `cargo test --locked --workspace --all-targets --all-features`: PASS,
  including all 11 transient-shaper product tests and the ten-track graph closure fixture.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: PASS.
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features --no-deps`: PASS.
- Workspace, realtime, effect-runtime and rack baseline plus mutation suites PASS when invoked
  through `bash`; graph baseline policy PASS. The graph policy has no separate mutation script.
- The Git-free static seal found no conflict markers or trailing whitespace in the Issue-020
  product paths, no `.orig`, `.rej`, `.tmp` or `.profraw` artifacts outside excluded build/VCS
  directories, and valid shell syntax for every invoked policy script.
- No Issue-054 qualification, functional audit main, cross-target build, instruction/object
  inspection, benchmark, timing or listening command ran. `timed_benchmark_invocations=0`.

**Final Sol verdict: PASS.** Issue 020 closes the fixed Normal dual-envelope transient-shaper
scalar, W4/W8 bank, launch registry/effect compiler and ten-track graph/PDC/resource product
vertical in the second and final authorized attempt. Issue 054 remains the sole owner of deferred
corpus expansion, large deterministic rows, realtime audit, target/instruction evidence,
descriptive benchmark and listening handoff.

## Audit #92 re-land (2026-08-23) — evidence record

The crate was re-landed on the wave-1 foundation (`miso-engine-lane`, `miso-engine-math`,
`miso-engine-effect-runtime`) per the #92 plan and master plan #83 §6/D6/D7/D10/D11. Branch
`audit-092-shaper`.

### Changes by bit-safety class (master plan §1.8)

| change | class | evidence |
|---|---|---|
| Followers moved to `effect-runtime::envelope::ar_one_pole_step` (two-product form, `1 - c` precomputed by `ArCoef`) | **A** — no bits move | `1 - c` is exact for c in [0.5, 1] (Sterbenz); the coefficient table is bit-identical to `retention_coefficient`, asserted for all 16 rows |
| Detector linking monomorphised over `const LINK` | **A** | same three rounding sites; `link_modes_drive_the_detector_as_specified` reads the detector off the envelope word |
| `f32::max`/`clamp` replaced by the `Lane` select forms (D8) | **A** on finite inputs | inputs are finite by the boundary check; ±0 and NaN behaviour is now the packed semantics, which is what a bank executes |
| `normalize_zero` on the render path deleted | **A** | `q == 0` is already true for `-0.0`; the identity select is unchanged |
| Contrast: `20log10(f) - 20log10(s)` → `DB_PER_OCTAVE * log2_lane(f/s)` | **B** | derived bound below |
| Gain: `10^(q*0.05)` → `exp2_lane(q * OCTAVES_PER_DB)` | **B** | derived bound below |
| `wet_identity` (`mix == 1` returns `x*g` exactly) dropped | **B**, <= 1 ulp | no test pinned it; the master plan §4.2 mix law has no such select |
| Ramps: per-sample division → D11 precomputed step (`effect-runtime::ramp::LinearRamp`) | **B**, exact on every pinned row | `automation_updates_...` pins 1/64, 63/64, 1.0, 0.96875 and the mid-ramp restore, all unchanged |
| D7: per-value `sanitize`/`flush`/`recover` deleted; one flush per envelope word, one boundary check per block | behavioural | see "What changed for a caller" |

### Derived tolerance for the class-B transcendental swap

With gate M1's `<= 2 ulp` on `log2_lane` and `exp2_lane`: the ratio rounds at `2^-24`; only
`|log2 r| <= 24 / 6.0206 = 3.99` survives the ±24 dB clamp, where `delta(log2) <= 5.6e-7`; scaling
by `DB_PER_OCTAVE` gives `5.8e-6` dB; the shape law's three roundings give `8.7e-6` dB; both clamps
are 1-Lipschitz; `exp2_lane` contributes `1.45e-6` relative (`1.26e-5` dB); the mix adds three
roundings. Total: `|y_new - y_exact| <= 1.7e-6 * |x| * max(1, g)`, about **1.5e-5 dB**, about 15
ulp of the output.

**Measured old-to-new deviation: 4.7e-6 dB, 8 ulp** (worst over four launch rates x three link
modes x twelve parameter points, plus an impulse, a step and a 5,312-sample decay; the pre-audit
scalar chain was transcribed and proven bit-exact against `ae02d2a` before it was used as the
reference). The plan's stop condition was 1e-4 dB.

The old bits were target-specific libm results (audit finding F1: `f32::log10` and `f32::powf` are
whatever the target links). The new bits are IEEE-only and identical across `Scalar`/`Simd4`/`Simd8`
and across `x86_64`/`aarch64`/`wasm32`.

### Re-pin table

| fixture | oracle | result |
|---|---|---|
| `descriptor_coefficients_..._are_frozen` (id, parameters, qualities, 16 coefficient bits, 88/24-byte caps) | contract | **not re-pinned** |
| `independent_coefficients_...` (each coefficient bit) | `dsp-reference` f64 `exp`, **and** `effect-runtime::envelope::retention_coefficient` | **not re-pinned**; both agree on all 16 bits |
| `automation_updates_...` (1/64, 63/64, 1.0, 0.96875, remaining 63/1/0/63, restored continuation) | contract | **not re-pinned** |
| `both_resets_...` (word-exact payloads) | contract | **not re-pinned** |
| identity rows (defaults / bypass / mix 0, signed zero) | contract | **not re-pinned** |
| `scalar_matches_the_independent_f64_oracle` (tolerance `2.0e-5`) | `ReferenceTransientShaper` (f64) | tolerance **unchanged**; worst 1.67e-5 |
| `impulse_step_and_decay_...` (gate 2, `0.01` dB) | `ReferenceTransientShaper` (f64) | tolerance **unchanged** |
| `corpus::CROSS_TARGET_DIGESTS` (3 cases) | **new**; pinned from the scalar `Lane` instantiation (§8) | new fixture, no old value |

State layout is unchanged: 11 words / 44 bytes per lane, `state_layout_version = 1`, common section
0 bytes. The D11 `step` is derived on restore rather than persisted, which is what keeps it at 11.

### What changed for a caller

* `ProcessReport::sanitized_main_samples` and `recovered_left/right_samples` are now always `0`.
  D7 deletes per-value sanitisation and per-value recovery; a non-finite or out-of-range block is
  instead zeroed as a unit, the envelopes are reset, and the runtime's `nonfinite_blocks` counter
  is incremented. A subnormal input sample is no longer replaced by zero — it renders, and cannot
  reach the recurrence because `miso_engine_lane::flush` clears anything below `1e-20`.
* A bank binding is `Ok(None)` for any width this build does not render (D4: no runtime dispatch).
  Previously the same answer came from `PreparedCompressorGainMixKernelV1::try_new`.

### Deferred

* `scratch_fixed_bytes = 24` re-accounting (finding F9) → **#95**: it moves the canonical descriptor
  bytes and therefore the Issue-082 effect identity.
* The runtime `state_payload`'s two-word versioned header (83c decision 2) is **not** adopted: it
  would move `maximum_state.common_bytes` from 0 to 8, which is a contract fixture and a descriptor
  identity change → **#95**, with the layout version bump.
* `-0.0` initial values are still rejected at prepare. That rejection is the *contract*'s
  (`validate_initial_values`), not this crate's, so 83c decision 3's lenient rule is #95's to apply.
* `miso_engine_core::KernelBackendV1` is still named by the crate's **tests** (the contract's
  `PrepareEffectBankRequest::backend` is that type and the contract does not re-export it).
  Production no longer depends on `miso-engine-core`.

### Commands run

`cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
`cargo test --locked -p miso-engine-transient-shaper` debug and release; `cargo test --locked
--workspace`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; every
`scripts/check-*.sh` that was green on `origin/main` at the branch point; `cargo check` for
`wasm32-unknown-unknown` +/-`simd128`, `aarch64-linux-android` and `aarch64-apple-ios`;
`bash scripts/run-wasm-gates.sh`.

### Descriptive before/after (not a gate)

Release profile, 48 kHz, 128-frame blocks, 20,000 blocks, one warm-up plus two timed rounds,
through the public factory; ns per lane-frame counting both channels.

| path | before (`ae02d2a`) | after | ratio |
|---|---|---|---|
| `W = 1` `process` | 41.97 ns | 20.47 ns | 2.05x |
| `W = 8` `process_bank` | 58.86 ns | 4.96 ns | 11.9x |

The W8 bank was *slower per lane-frame than the scalar product* before this change, which is what
finding F2 predicted: the pre-audit "bank" did over 95% of its work in a scalar per-track loop and
called into the SIMD kernel per sample for four operations.

## 2026-08-24 amendment (#84 phase A)

Superseded by #83 D4/D10 via #84 phase A: the per-sample kernel tokens
(`Prepared*KernelV1`), `KernelBackendV1`, `TargetCapabilities`,
`miso_engine_core::target_capabilities()` and `miso_engine_rack::KernelDispatch` were
deleted along with `crates/miso-engine-core/src/arch`. Kernels live in
`crates/miso-engine-lane`; the backend is the compile-time constant
`miso_engine_lane::Backend::current()`, and
`miso_engine_effect_contract::BankWidth::for_backend` is the one backend-to-width law.
The historical text above is kept as the decision record of its time and is not rewritten.
