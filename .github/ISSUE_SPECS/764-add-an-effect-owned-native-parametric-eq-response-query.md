# Add an effect-owned native parametric EQ response query

Parent: #763, Engine-owned analysis. Baseline: `77cbde0a3dc90e5ffb385e0bf39ac87a8f2e218c`.

## Product outcome and smallest closable slice

A native Rust caller can ask the parametric-EQ effect for the stationary total and optional individual-section magnitude responses of an explicitly supplied valid configuration, independently for L/R, into caller-owned buffers. The effect computes the response from its actual rounded production coefficient words. The caller no longer implements the EQ transfer function.

This child is an independently useful native library capability. It does not close #763. Input HPF/LPF, engine composition, generated discovery, browser/headless SDK APIs, applied-target snapshots, subscriptions, spectrum, transports, and joining are required subsequent children of #763. The first complete browser/headless response preview remains a required milestone; do not describe this native child as that milestone.

Scope approval: Astra (`gpt-6-astra`, xhigh), source inspection on 2026-09-12. User-directed workflow supersedes the default role names: Luna (`gpt-5.6-luna`, max) implements each coherent attempt; Astra (`gpt-6-astra`, medium) supplies its one adversarial verdict. Maximum five attempts; no hidden retries. Root checkpoints and delivers.

## Current facts and owned paths

- `crates/parametric-eq/src/lib.rs`: `design_svf` designs the actual six `f32` words; `BandTarget::words` implements disabled-section identity; `band_targets` reads the descriptor-ordered validated parameter list. `expected_prepared_metadata` validates the same `PrepareEffectRequest` used by the effect factory.
- `crates/lane/src/kernels.rs::svf_step` and its output mix define the realized recurrence. The response evaluator must not alter them.
- `crates/parametric-eq/tests/analytic.rs` already compares the rounded realization against independent `dsp-reference` state-space/cookbook oracles, with 0.005 dB tolerance above -120 dB. Reuse this corpus and test-only oracle, without promoting oracle implementation into production.
- `crates/parametric-eq/tests/support/mod.rs` already supplies valid requests, asymmetric parameter edits, snapshots, impulses, and DFT helpers.

Allowed implementation paths: new `crates/parametric-eq/src/response.rs`, its module/export declarations in `crates/parametric-eq/src/lib.rs`, new `crates/parametric-eq/tests/response.rs`, proportionate additions to existing test support if necessary, and this issue's spec/evidence. A short Rust API example may be placed in `crates/parametric-eq/examples/response.rs` if useful. Do not change audible DSP, generic effect descriptors, render traits, state layouts, frozen ABI/export sets, or dependencies. No new crate is needed. Additional boundaries require root scope review before edits.

## Frozen semantics and proposed public API

Keep internal Rust names unversioned. Use the crate's existing `PrepareEffectRequest` instead of inventing another parameter schema or accepted-domain table. The new function accepts an ordinary valid preparation request but does not allocate/instantiate/process a prepared effect. Its preparation resource fields retain the normal request validation semantics and are documented as such; a later host adapter can construct them from its admitted resources.

Suggested API (equivalent spellings are allowed if the decisions below remain exact):

```rust
pub struct EqResponseRequest<'a> {
    pub configuration_id: u64,
    pub configuration: PrepareEffectRequest<'a>,
    pub frequencies_hz: &'a [f32],
    pub maximum_points: usize,
}

pub struct EqResponseOutput<'a> {
    pub total_left_db: &'a mut [f32],
    pub total_right_db: &'a mut [f32],
    pub sections_left_db: Option<&'a mut [f32]>,
    pub sections_right_db: Option<&'a mut [f32]>,
}

pub enum EqResponseMode { RequestedConfiguration }

pub struct EqResponseSummary {
    pub configuration_id: u64,
    pub mode: EqResponseMode,
    pub sample_rate_hz: u32,
    pub points: usize,
    pub floor_db: f32,
    pub bypass: bool,
    pub enabled_left: [bool; EQ_SECTION_COUNT],
    pub enabled_right: [bool; EQ_SECTION_COUNT],
}

pub enum EqResponseError {
    Configuration(EffectPrepareError),
    InvalidFrequencyGrid,
    Capacity,
    OutputShape,
    Numerical,
}

pub fn query_response_into(
    request: EqResponseRequest<'_>,
    output: EqResponseOutput<'_>,
) -> Result<EqResponseSummary, EqResponseError>;
```

1. `RequestedConfiguration` means the stationary response of the supplied configuration after its words have settled. It is not a read of current render state and has no applied/captured sample. No audible timestamp, render clock identity, subscription, or plan identity is fabricated. `configuration_id` is an opaque caller-assigned correlation token, echoed exactly as `u64`, not an engine attestation or a content digest. Callers assign a new token when changing configuration; test a value beyond 2^53. The eventual session-bound target API is separate.
2. Validate the request through `expected_prepared_metadata(&PARAMETRIC_EQ_DESCRIPTOR, ...)` and reuse `band_targets`/`BandTarget::words` to preserve production coefficient and enable semantics. Rust child-module privacy permits this without exposing private targets or broad refactoring. Preserve launch rates exactly 44.1/48/88.2/96 kHz, all accepted parameter domains, strict initial-value ordering/channel validation, and zero normalization. Invalid disabled parameters are still subject to the ordinary descriptor/request validator; disabling a section is not permission to accept NaN.
3. This native low-level query accepts an explicit frequency grid. The high-level engine grid generator and returned axis are owned by the later SDK/discovery child; do not place any EQ math in that adapter. Here the result uses precisely the supplied frequencies, in the supplied order. Require at least one finite point, strictly increasing when multiple points are supplied, and `0 <= f <= Fs/2`; DC and Nyquist are supported endpoints. Reject above-Nyquist, NaN/infinity, negative, duplicate, and descending grids.
4. `maximum_points` is a caller-supplied resource bound, never a compiled track/point ceiling. Require `0 < len <= maximum_points`; check arithmetic before `4*len`. Total buffers must have exactly `len` words. Each optional section buffer, independently selected for L or R, must have exactly `4*len` words in section-major order (`section * len + point`). Reject malformed inputs/output shapes before publishing a valid result; no partial valid result may escape. Prefer preserving all caller buffers on any refusal; if numerical failures need a bounded preflight pass to guarantee this, document that simple choice rather than adding a cache/allocator.
5. Magnitude is `20 log10(|H|)` in dB relative to unit amplitude, with a fixed declared -120 dB floor. Use adequate `f64` intermediate precision and `math` crate transcendentals, with a final `f32` output cast. Every success word is finite. A true zero/null maps to the floor. Underflow below the floor maps to the floor; invalid/nonfinite intermediate arithmetic is a typed failure, not a believable flat curve. Do not clamp above-floor discrepancies to hide them.
6. Disabled bands contribute exact identity (0 dB). Optional individual curves report the enabled/bypassed-independent section configuration: effect-wide bypass changes the total to exact 0 dB because EQ latency is zero, while the individual-section curves remain available as the configured sections. The summary exposes bypass and each section enable, so consumers cannot confuse these meanings. This distinction is frozen by tests and docs.
7. Form total magnitude from the *unfloored* section responses (complex multiplication, power product, or additive unfloored log magnitude), then apply the total floor once. Summing individually floored public curves is incorrect when a deep null is followed by gain.
8. The function is control/worker-plane only. It receives configuration values and buffers, never a mutable/live prepared effect. Retained memory does not grow with session duration, no static cache is introduced, and there is no render-side call. Query itself should allocate/free zero heap bytes using fixed section/word scratch and caller outputs. Work is explicitly O(points * 2 * 4), with optional bounded validation/preflight overhead.

## Algorithm, numerical limits, and evidence authority

Derive the evaluator independently from the production recurrence. For prior states `(s1,s2)` and input `x`, with each word promoted from its exact `f32` value:

```
v3 = x - s2
v1 = (1-c1)*s1 + a2*v3
v2 = s2 + a2*s1 + a3*v3
s1' = (1-2*c1)*s1 - 2*a2*s2 + 2*a2*x
s2' = 2*a2*s1 + (1-2*a3)*s2 + 2*a3*x
y = m0*x + m1*v1 + m2*v2
```

Thus `A=[[1-2c1,-2a2],[2a2,1-2a3]]`, `B=[2a2,2a3]`, `C=[m1*(1-c1)+m2*a2, -m1*a2+m2*(1-a3)]`, and `D=m0+m1*a2+m2*a3`. Evaluate `H(z)=D+C(zI-A)^(-1)B` at `z=exp(j*2*pi*f/Fs)`, or an independently derived algebraically equivalent stable numerator/denominator. Handle identity words explicitly: their transfer is one even where the unreduced state-space inverse is singular at DC. Explain numerical conditioning at low frequencies and endpoints in code/docs.

This describes the linear stationary realized recurrence of rounded words, not every per-operation `f32` rounding or the denormal-flush nonlinearity of a very quiet rendered sample. Actual PCM comparison therefore uses the existing finite-window tolerances and meaningful above-floor signals; it must not falsely claim bit-exact frequency-domain equivalence to all render inputs. A ramp is time-varying; this query does not call its target the exact transfer over the ramp.

Primary method background: Andrew Simper, *Linear Trapezoidal Integrated State Variable Filter with Low Noise Optimisation*, https://cytomic.com/files/dsp/SvfLinearTrapOptimised2.pdf. The source recurrence is the implementation authority. The independent `dsp-reference` oracle remains a dev dependency only and remains unchanged. This observation-only addition changes no audio latency, DSP tail, parameter smoothing, NaN recovery, or denormal behavior. Listening is unnecessary when unchanged audio/state is objectively shown. Descriptive benchmark work belongs to the integrated #763 transport/resource child; this child makes no speed claim or timed benchmark.

## Discriminating acceptance gates

- Compare the production response query against the independent realized-word oracle using all six families, both gain signs, low/high frequency and high-Q corners, all four launch rates, DC/Nyquist, and the existing 1,488-row corpus with representative probes. Above the -120 dB reference floor, total/section error must stay within 0.005 dB; floors/nulls must be finite and correct. Keep existing frozen full-grid tests and thresholds intact. The new test must call production `query_response_into`; merely rerunning the old oracle comparison is insufficient.
- Query a four-section asymmetric L/R configuration, disabled sections, total-only, each one-sided optional-section choice, and bypass. Compare cascade results to multiplication of independent section responses. Include a deep-cut-plus-gain case that distinguishes flooring once from summing floored sections.
- At each launch rate, compare a representative query to measured settled production output using existing impulse/DFT or settled sine helpers, at meaningful frequencies and the established 0.05 dB finite-window tolerance. Include distinct L/R and a nontrivial cascade. Avoid using the production response evaluator to create its own expected values.
- Validate malformed/short/oversized/overflowing shapes, capacity zero and at/above limit, every invalid-grid category, invalid preparation/channel/order/sample-rate/domain cases, and unchanged output sentinels on refusals. Check opaque identity survives above 2^53 and max u64 without narrowing.
- Serialize a real prepared effect state before and after queries during an in-flight coefficient ramp; query must leave every state/target/ramp word unchanged. Continue render alongside an otherwise identical instance that was not queried; output and final state must be bit-identical. Reuse existing state helpers; do not expose mutable internals for the test.
- Use the existing `bench-support::alloc` instrumentation to show query allocations/frees are zero after caller input/output setup. Scope is this query, not an invented generic harness.

Required local commands, once after the coherent attempt:

```
cargo test --locked -p parametric-eq --test response
cargo test --locked -p parametric-eq --lib --tests
cargo clippy --locked -p parametric-eq --all-targets -- -D warnings
cargo fmt --all -- --check
bash scripts/check-parametric-eq-render-contract.sh
bash scripts/check-realtime-policy.sh
CARGO_TARGET_DIR=target/issue-763-response-wasm-scalar RUSTFLAGS='-C target-feature=-simd128' cargo check --locked --release --target wasm32-unknown-unknown -p parametric-eq
CARGO_TARGET_DIR=target/issue-763-response-wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --release --target wasm32-unknown-unknown -p parametric-eq
```

Record which checks actually ran and their outcomes. Native render already uses the compile-pinned AVX2/FMA configuration. Wasm checks establish build portability only; do not call them numerical execution parity. Runtime browser/Wasm parity is a required parent integration gate when this API becomes reachable there. No extra cross-target framework or timed workload is authorized here.

## Checkpoints, review, and delivery

Luna makes one coherent implementation pass, runs focused tests, reports exact changed paths and candid results, then pauses for root's local commit/push audit before further edits. Astra medium adversarially reviews against this body, especially rounded-word correctness, oracle independence, floor composition, identity/bypass semantics, validation, no mutable-render access, and absence of parent-completion overclaim. Root records one PASS/FAIL verdict per attempt, updates the local/GitHub decision record, and closes this child only after evidence is upstream and GitHub state is verified. Parent #763 remains open.

## Attempt 1 implementation evidence — Luna

Implementation paths changed in this attempt: `crates/parametric-eq/src/lib.rs`,
`crates/parametric-eq/src/response.rs`, and `crates/parametric-eq/tests/response.rs`.

The focused functional response suite passed with exit 0:

```
cargo test --locked -p parametric-eq --test response
```

Observed result: 4 passed, 0 failed. The suite covers production-query comparison against the
independent realized-word oracle across all six section families and four launch rates, asymmetric
cascade multiplication, disabled sections, total-only and optional section outputs, bypass identity,
grid/shape/configuration refusal with unchanged sentinels, an opaque ID above 2^53 and `u64::MAX`,
and prepared-effect state immutability. `cargo check --locked -p parametric-eq` also passed with exit
0 after implementation debugging. The terminal evidence summaries are preserved outside the
repository at `/tmp/miso-engine-763-response-functional.log` and
`/tmp/miso-engine-763-response-allocation.log`.

The required allocation gate failed with exit 101:

```
cargo test --locked -p parametric-eq --test response response_query_allocates_nothing_after_caller_setup -- --exact --nocapture
```

After a warm-up query, the measured call reported 15 allocations, 15 frees, and 1,752 requested
bytes. The allocations originate in the required `expected_prepared_metadata` path, whose shared
`validate_descriptor` implementation constructs `BTreeSet`/`BTreeMap` validation state on every
call. This is a contract conflict: #764 requires validation through that function and also
requires the query to allocate/free zero heap bytes, while forbidding a static cache. The focused
functional implementation is checkpoint-ready; this allocator conflict requires root scope review
before any further implementation edit. No broad gates, benchmarks, or Wasm checks ran in this
attempt.

## Adopted attempt-1 discovery amendment: immutable configuration preparation

Astra xhigh source review and root approve a two-phase API within the existing allowed paths. This supersedes the original single-call preparation API and its claim that preparation itself allocates nothing. The compiling failed-gate checkpoint `beab7326` remains evidence; this is an implementation discovery correction before attempt 1's adversarial verdict, not a new attempt or a weakened query gate.

Add `EqResponseConfiguration::prepare(PrepareEffectRequest) -> Result<Self, EqResponseError>`. It calls the existing complete `expected_prepared_metadata` validator and prepares owner-local coefficient words off render. It privately owns sample rate, bypass, enable flags, and eight rounded coefficient sets in fixed-size storage, borrowing no source parameter slice and exposing no mutation. Preparation may allocate transiently through existing descriptor validation; the observed 15 allocations are descriptive evidence, not a pinned count. No effect instance is allocated or processed.

Change `EqResponseRequest.configuration` to `&EqResponseConfiguration`. `query_response_into` validates grid, shapes and budgets before writes and evaluates those immutable words with zero allocations/frees, including its first invocation and refused query inputs. All configuration correlation, requested-versus-applied timing, floor/composition, bypass/enable, numerical and buffer guarantees remain. No effect-contract changes, duplicate validator, static/lazy cache or dependencies are authorized.

Move invalid preparation/domain/channel/order/rate tests to the constructor. Prove it retains full existing request validation and the exact four-launch-rate guard. Prove mutation of the original caller parameter slice after construction cannot change responses. Measure query allocation immediately after preparation with no warm-up query; include refused grid/shape/budget calls. Remaining original oracle/PCM/state/portability and review gates are unchanged.

### Amendment implementation result

The approved two-phase correction is implemented within the same three implementation paths. Public
`EqResponseConfiguration::prepare` runs the complete existing preparation validator and owns the
sample rate, bypass/enable flags, and fixed left/right rounded coefficient words. The query now
borrows only that immutable configuration; it does not retain or inspect the source parameter
slice, instantiate an effect, read render state, or call the allocating validator.

The focused command was captured with real stdout/stderr and exited 0:

```
cargo test --locked -p parametric-eq --test response -- --test-threads=1
```

Captured logs: `/tmp/miso-engine-763-response-attempt1-focused.stdout` and
`/tmp/miso-engine-763-response-attempt1-focused.stderr`.

Observed result: 8 passed, 0 failed. The suite now also proves first-call query allocation/frees are
zero immediately after preparation, refused grid/shape/budget queries preserve outputs, constructor
rejection of short/reordered/wrong-channel/NaN/zero-capacity/extended-rate requests, source-slice
mutation independence, one-sided and total-only outputs, a deep-null-plus-gain case that
distinguishes unfloored cascade composition from summing public floors, and measured one-second
production impulse DFT agreement at all four launch rates within 0.05 dB. No broad checks,
benchmarks, or Wasm checks ran after the amendment; root runs those after checkpoint authorization.

## Attempt 1 adversarial review: FAIL

Verdict: **FAIL**.

Reviewed source head: `281289d894117939cc2f721b634c3788e48a4a97`.
Baseline: `77cbde0a3dc90e5ffb385e0bf39ac87a8f2e218c`.
Reviewer: Astra, medium. Reviewed the full issue including its adopted immutable-configuration preparation amendment, repository instructions, production response implementation, new tests, existing oracle/fixture helpers, factory validation and render recurrence. This is one coherent attempt-1 verdict. No source edits, commits, GitHub actions, agents, benchmarks or additional test runs were performed by this reviewer.

## Findings requiring correction

1. **The required production-query numerical corpus is absent.** `crates/parametric-eq/tests/response.rs:90` runs six families at four rates with one 1 kHz nominal configuration per family and four probes. It does not call `support::frozen_grid`, and does not exercise the required 1,488 configurations, low/high design-frequency corners, both gain signs across the corpus, or high-Q corners through the production query. Existing `analytic.rs` tests exercise their own evaluator and cannot cover regressions in the new evaluator. This is particularly relevant to cancellation near endpoints/deep nulls. Reuse the existing frozen rows with a small representative sorted/deduplicated probe grid including DC/Nyquist and meaningful near-design probes; assert total and section results for both channels against the independent realized-word oracle at the frozen 0.005 dB tolerance. Do not map arbitrary nonfinite oracle results to the floor as lines 137–142 currently do: only a true zero/-infinity is a valid null; reject NaN/+infinity in the expectation. Add the requested short explanation of low-frequency/endpoint conditioning alongside the evaluator (the existing comment only explains identity's removable DC singularity). No new fixture framework is needed.

2. **The state test does not test an in-flight ramp or unchanged subsequent audio.** `crates/parametric-eq/tests/response.rs:666` starts automation at sample 0 and processes 128 samples before the first snapshot. Production `RAMP_SAMPLES` is 64 (`src/lib.rs:98`), so the query occurs after settling. The test has no independent twin and never renders after the query. Use a shorter prefix or a later-in-block event, prove remaining ramp samples are nonzero using existing payload helpers, snapshot before/after the query, then continue identical nonzero input through a queried and unqueried twin and compare output bits and final payloads. This closes the exact unchanged-state/audio gate without changing production DSP.

3. **Several explicitly required buffer/refusal and allocation gates are untested.** `tests/response.rs:306` checks invalid grid categories, one over-budget refusal, and a short left total. It does not exercise a zero point budget, oversized totals, right total shape errors, malformed optional section buffers on either side, or preserve section sentinels on a refusal. The allocation measurement at line 704 measures one successful first query only; the amendment explicitly requires refused grid/shape/budget calls under the counters too. Add these bounded table-driven cases. Keep the checked multiplication; if overflow cannot be reached with a safely constructible slice, document that fact and/or test the arithmetic helper without constructing invalid slices. Do not fabricate huge unsafe slices. Also assert actual right section values in the asymmetric/right-only modes (the current tests never inspect those arrays) and compare bypass section curves with their non-bypassed configuration, rather than merely checking that one value is nonzero. These are evidence gaps; inspection did not identify a present buffer-write or allocation bug.

4. **The committed test contains a required-Clippy failure.** `tests/response.rs:135` uses `assert_eq!(summary.enabled_left[0], true)` under a `-D warnings` gate. Root reports the exact-head Clippy failure. During review I observed a sole uncommitted change replacing it with `assert!(summary.enabled_left[0])`; that mutation is outside the reviewed source head and does not alter this verdict. Include the one-line lint correction in the next coherent attempt and run the required gates on its checkpoint.

## Source findings that support the design

- The two-phase API implements the adopted amendment: preparation calls the complete existing validator, checks the exact launch-rate predicate, and reuses `band_targets`/`BandTarget::words`. Private fixed arrays own the rounded words and flags, with no retained parameter borrow, mutator, cache, effect instance, or dependency addition.
- Algebraic inspection against `lane::kernels::svf_step` confirms the `A`, `B`, `C`, `D` derivation and the adjugate/determinant solution. Exact rounded `f32` words are promoted to `f64`; transcendentals use `math`. The new implementation does not call the dev oracle. Its arithmetic association differs from the pre-existing oracle, which independently inverts matrix entries before applying B.
- Identity words bypass their removable DC singularity. Disabled sections therefore contribute exact identity; total bypass returns positive exact zero while configured section responses remain available. Unfloored complex products compose the cascade before applying its public floor. The deep-cut-plus-gain fixture discriminates that choice.
- Grid and shape validation happen before writes. Full numerical preflight is followed by deterministic recomputation using immutable inputs, so inspection supports transactional output preservation on typed refusals. Fixed scratch and caller output slices support zero query allocation and bounded O(points * 2 * 4) work. I found no reachable numerical-refusal counterexample and do not claim one.
- The one-second impulse/DFT tests use actual production effect output, distinct L/R, a nontrivial cascade, four launch rates, and the specified 0.05 dB tolerance. They do not derive expected PCM from the response evaluator.
- Only module/export declarations change in existing production `lib.rs`; audible DSP, recurrence, state codec/layout, automation, dependencies, and ABI are unchanged. No parent/browser completion claim is made.

## Gate evidence and limits

The issue records eight focused tests passing after the amendment. Root forwarded Luna's final gate report: full parametric-EQ lib/tests passed at the committed head; initial Clippy failed with exit 101 for the boolean assertion comparison. After the sole uncommitted lint correction, Clippy, formatting, and the EQ render-contract check passed. Those corrected-tree checks are not exact-head passes. Realtime policy and both Wasm builds were not run after root stopped work. These run outcomes are Luna-supplied/root-forwarded evidence, not reviewer-executed commands; the one-line worktree correction was directly inspected. No runtime Wasm parity is claimed. Root authorized finalization because the acceptance gaps above already determine FAIL.

The next attempt can remain within the current allowed response source/test/support/spec paths: reuse existing fixtures and payload helpers, strengthen the discriminating assertions, add the numerical-conditioning explanation and lint fix, then obtain one new adversarial verdict. No architectural rescope, broader matrices, benchmark runner, or changed tolerance is justified by this review.

Root authorizes attempt 2 within the unchanged allowed paths to address these four findings and complete the existing gates. The one-line Clippy correction is preserved in this review checkpoint; substantive test corrections follow only after this checkpoint is pushed. No tolerance or parent requirement changes.

## Attempt 2 focused checkpoint evidence — Luna

Attempt 2 changes remain within the approved paths `crates/parametric-eq/src/response.rs` and
`crates/parametric-eq/tests/response.rs`. The evaluator now documents its `f64` endpoint and
low-frequency conditioning, including the identity singularity exception and typed numerical
failure behavior. The response tests now drive the production query through all 1,488 frozen rows,
both channels and section outputs, with sorted/deduplicated DC, near-design, and Nyquist probes;
they also assert right-side section values and bypass section curves against the non-bypassed
configuration. The state gate uses a 16-frame prefix to leave the coefficient ramp unfinished,
checks its serialized remaining count, queries between snapshots, then compares bit-identical
continued nonzero audio and final state with an unqueried twin. A table-driven refusal gate covers
invalid grids, zero/short budgets, short and oversized left/right totals, and short and oversized
left/right section buffers; it checks every output sentinel and allocation counters for each
refusal. The checked production multiplication remains the arithmetic overflow guard; no unsafe
oversized slice is constructed.

The focused command ran after formatting and exited 0:

```
cargo test --locked -p parametric-eq --test response -- --test-threads=1
```

Observed result: 10 passed, 0 failed, 0 ignored. Actual stdout, stderr, and exit status are
preserved at `/tmp/miso-engine-763-response-attempt2-focused.stdout`,
`/tmp/miso-engine-763-response-attempt2-focused.stderr`, and
`/tmp/miso-engine-763-response-attempt2-focused.exit` (exit status `0`). No broad gates,
benchmarks, Wasm checks, commits, or pushes ran after these changes; root must checkpoint and
review this exact tranche before any further edits.
