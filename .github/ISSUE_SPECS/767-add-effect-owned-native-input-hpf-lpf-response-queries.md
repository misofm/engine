# Add effect-owned native input HPF/LPF response queries

Parent: #763, Engine-owned analysis. Companion: #764, native parametric EQ response query.

Scope approved by Astra (`gpt-6-astra`, xhigh), 2026-09-12, from current-source inspection at `5db9bad6c66e4371056dddbcb5dd0d19bcffc4f2` in `/tmp/miso-engine-763`. DSP baseline is `77cbde0a3dc90e5ffb385e0bf39ac87a8f2e218c`; intervening committed changes are issue planning. No legacy source was inspected. This brief does not depend on #764 implementation internals.

## Smallest closable product slice

A native Rust caller supplies valid builtin configuration and a bounded frequency grid and receives the engine's stationary HPF, LPF, and combined input-filter magnitude responses for independent L/R channels into caller-owned buffers. The builtin DSP owner computes the transfer from its actual rounded coefficients; callers need no filter-response mathematics.

The result is explicitly the **HPF/LPF filter subtotal**. It excludes polarity, trim, input delay, racks, fader/mute, routing, and channel matrix. It is requested configuration, with an opaque configuration correlation identity and no invented application/capture sample. This child closes independently as native library functionality. #763 remains open for engine composition/discovery, SDK APIs, live state, spectrum, subscriptions, transport, and sample-time joining.

User-requested workflow: Luna max implements one coherent attempt; Astra medium provides its one adversarial verdict. Root owns checkpoint commit/push and GitHub synchronization. Maximum five attempts; stop and rescope after the fifth failure. Start implementation only after the preceding coherent tranche is checkpointed and this numbered local spec matches its GitHub issue.

## Source evidence and implementation boundary

- `crates/builtins/src/lib.rs:167`: `ChannelParameters`/`BuiltinParameters` are the existing configuration authority.
- `builtin_filter_cutoff_maximum_hz` and `validate_builtin_filter_cutoff` (around lines 287–330) own exact rate-specific inclusive cutoff bounds. Positive-zero bits disable; **negative zero is rejected**. Extended rates are retained for direct compatibility evidence; they are not launch support.
- `SvfSection::design` (around line 728) designs the owner-local Butterworth section: `g=tan(pi*fc/Fs)`, `k64=sqrt(2)`, `t1=g*(g+k64)`, `den=1+t1`, `c1=t1/den`, `a2=g/den`, `a3=(g*g)/den`, then one cast of `[c1,a2,a3,k64]` to `f32`; HPF output mix is `(1,-k,-1)`, LPF is `(0,0,1)`. The exact operation order differs from parametric EQ and must not be replaced with that crate's designer.
- `prepare_sections` (around line 2500) validates matrix, gain domains, both lane cutoffs, and `hpf<lpf` whenever both are enabled, then constructs the exact input coefficient words. `BuiltinChain::new` uses it. HPF/LPF remain `PreparedOnly`; this issue adds no live retarget.
- `crates/builtins/tests/response.rs` already owns frozen independent `ReferenceSvfStateSpace`/RBJ and impulse/sustained gates, including launch rates, 0.005 dB analytic and 0.05 dB PCM tolerances. Keep this existing file and its thresholds intact.
- Existing `builtins::test_support::{section_words,input_section_words,input_state_words,input_trim_ramp_words}` and bank equivalents expose all evidence needed for state/word tests. Do not add production access through `test_support`.

Allowed paths: new `crates/builtins/src/filter_response.rs`; its module/reexports and, only if necessary, a small owner-local preparation helper extraction in `crates/builtins/src/lib.rs`; new `crates/builtins/tests/filter_response.rs`; this issue's local spec/evidence. `crates/builtins/Cargo.toml` may add **only** existing `bench-support.workspace = true` under dev-dependencies to use the audited allocator. No production dependency change, no dependency on `parametric-eq`, no new crate, no broad response abstraction, no kernel/ABI/session/SDK/generic descriptor edits. In particular, leave the preexisting `tests/response.rs` file distinct from the new API tests.

## Native API and frozen behavior

Suggested concrete API, with equivalent spellings permitted if semantics remain exact:

```rust
pub struct InputFilterResponseRequest<'a> {
    pub configuration_id: u64,
    pub sample_rate_hz: u32,
    pub configuration: BuiltinParameters,
    pub frequencies_hz: &'a [f32],
    pub maximum_points: usize,
}

pub struct InputFilterResponseOutput<'a> {
    pub total_left_db: &'a mut [f32],
    pub total_right_db: &'a mut [f32],
    pub sections_left_db: Option<&'a mut [f32]>,
    pub sections_right_db: Option<&'a mut [f32]>,
}

pub enum InputFilterResponseMode { RequestedConfiguration }

pub struct InputFilterResponseSummary {
    pub configuration_id: u64,
    pub mode: InputFilterResponseMode,
    pub sample_rate_hz: u32,
    pub points: usize,
    pub floor_db: f32,
    pub enabled_left: [bool; 2],
    pub enabled_right: [bool; 2],
}

pub enum InputFilterResponseError {
    UnsupportedSampleRate,
    Configuration(BuiltinParameterError),
    InvalidFrequencyGrid,
    Capacity,
    OutputShape,
    Numerical,
}

pub fn query_input_filter_response_into(
    request: InputFilterResponseRequest<'_>,
    output: InputFilterResponseOutput<'_>,
) -> Result<InputFilterResponseSummary, InputFilterResponseError>;
```

1. `RequestedConfiguration` means the stationary linear response of the supplied HPF/LPF configuration. It makes no claim about a live plan, queued command, currently applied state, audible content, or coefficient ramp. The `u64` identity is a caller-assigned correlation token, copied exactly, not a content hash or engine attestation. No sample timestamp is returned. An owner may assign a fresh token per edited configuration. Test identity at `2^53+1` and `u64::MAX`.
2. Admit only `engine::is_launch_sample_rate`: 44.1, 48, 88.2, and 96 kHz, even though the underlying direct builtin preparation retains extended compatibility rates. Return `UnsupportedSampleRate` for extended research rates and arbitrary/zero rates. Do not modify the existing preparation/descriptor compatibility behavior to implement this new query guard.
3. Reuse `prepare_sections` for the full supplied `BuiltinParameters` validation and coefficient preparation. It is acceptable to construct bounded temporary scalar builtin values on this control-plane path and read their private prepared coefficients; do not process audio or access a live instance. Alternatively extract a small private validated-input-record helper while preserving every existing operation and validation order. Do not duplicate cutoff bounds or invent an unrelated four-cutoff schema. Even excluded trim/fader/matrix fields must be valid because the request explicitly carries `BuiltinParameters`; document this and recommend defaults for filter-only callers.
4. Preserve exact disabled cutoff semantics: `+0.0` yields identity; `-0.0`, NaN/infinity, negative, sub-minimum nonzero, above exact maximum, and invalid HPF/LPF ordering fail. Validate before normalizing zeros or checking enabled flags. The maximum and its immediate `f32` successor must discriminate at every launch rate. No whole-effect bypass exists in the builtin configuration: do not invent one. Each disabled section is 0 dB; two disabled sections make exact 0 dB total on that channel.
5. Grid semantics match #764's native contract: nonempty, strictly increasing finite `f32` values, each in `[0,Fs/2]`. Support DC/Nyquist endpoints. `maximum_points` is caller-supplied admission, with `0 < len <= maximum_points`, and checked arithmetic for `2*len`; no compiled point/track cap. Total buffers are exactly `len` elements. Optional section buffers are independently selectable for L/R and exactly `2*len`, section-major: HPF at `0..len`, LPF at `len..2*len`. `enabled_*` uses the same HPF, LPF order.
6. Return stationary amplitude magnitude in dB relative to unity, using `f64` evaluation and `math` crate transcendentals before final `f32` storage. The fixed declared output floor is -120 dB. Every successful output must be finite. Preserve the actual rounded transfer at endpoints rather than substituting ideal cookbook values; only disabled identity is special-cased. Zero magnitude and below-floor underflow map to the floor. Invalid nonfinite intermediates are typed numerical failures.
7. Compose the unrounded/unfloored section responses and floor total only once. Do not add public already-floored curves. This matters even though the two Butterworth sections generally attenuate: the required composition rule will later be reused with gained EQ sections. Leave cross-effect composition to its later issue.
8. Every failure leaves all output buffers bit-unchanged and returns no valid summary. Prevalidate configurations/shapes/grids before writes. A fixed-work preliminary numerical pass is acceptable if needed to guarantee transactional writes without heap scratch. Do not return a plausible flat curve after failure.
9. Query is worker/control-plane code only, O(points*4), with fixed-size section scratch and caller-owned outputs, zero allocations/frees, no cache/static mutable storage, no session-duration retention, and no render callsite. No DSP latency, tail, smoothing, reset, input sanitization, or denormal recovery changes are authorized. The query performs no dynamic joining or spectral FFT.

## Realized response derivation and independence

For each promoted rounded word set `(c1,a2,a3,m0,m1,m2)`, derive the same stationary state-space relation as the actual `lane::kernels::svf_step` recurrence:

```
A = [[1-2*c1, -2*a2], [2*a2, 1-2*a3]]
B = [2*a2, 2*a3]
C = [m1*(1-c1)+m2*a2, -m1*a2+m2*(1-a3)]
D = m0+m1*a2+m2*a3
H(z) = D + C*(z*I-A)^(-1)*B, z=exp(j*2*pi*f/Fs)
```

An independently derived numerically stable rational form is acceptable. Derive it from these exact owner-local words, not ideal RBJ coefficients or a production import from `dsp-reference`. Handle disabled identity explicitly because its unreduced inverse at DC is singular even though its transfer is one. Explain conditioning at the legal near-Nyquist cutoff maximum. A linear response does not model each `f32` arithmetic rounding or the very-low-level state flush; the PCM gate uses meaningful above-floor signals and the established measurement tolerance.

Primary method background: Andrew Simper, *Linear Trapezoidal Integrated State Variable Filter with Low Noise Optimisation*, https://cytomic.com/files/dsp/SvfLinearTrapOptimised2.pdf. The actual owner-local production recurrence and accepted-domain checks remain authoritative. The independent oracle stays in unchanged `dsp-reference` dev code. Do not copy its implementation into production. This observation-only addition needs no new listening exercise when audio/state noninterference is proved. No descriptive timing is required or claimed here; parent #763's integrated resource/transport child owns that one bounded benchmark invocation.

## Discriminating acceptance gates

Use a small explicit new corpus, not a new framework or expanded research matrix:

- At all four launch rates, exercise HPF-only, LPF-only, both enabled, both disabled, and asymmetric L/R. Probe cutoff values 10 Hz, 100 Hz, 1 kHz, a high ordinary cutoff, and exact per-rate maximum with valid pair ordering; probe frequencies including 0, each cutoff, representative pass/stopband points, and Nyquist. Construct independent `ReferenceSvfStateSpace` using the seven production words exposed by existing test support, preserving all actual mix words. Above -120 dB the new public query agrees within 0.005 dB. Near/below floor, compare finite floored results with the independent value, respecting oracle conditioning rather than forcing ideal endpoints. Assert identity outputs are exactly positive zero.
- Compare total against the independently composed raw section transfer, and distinguish left/right and HPF/LPF positions. Total-only and each independently requested section buffer are equivalent. Vary valid trim/polarity/fader/mute/matrix/smoothing fields while holding cutoffs fixed: response remains unchanged, demonstrating this is a labeled filter subtotal. Invalid excluded fields still reject through normal preparation validation.
- At each launch rate, render a representative asymmetric HPF/LPF cascade using `BuiltinChain::into_input_builtins` at unity trim and measure an impulse DFT or settled sine. Compare public-query magnitude at representative above-floor frequencies within existing 0.05 dB PCM tolerance. Use simple existing measurement patterns; no 1,488-row EQ corpus, new FFT, or large fixture artifact set is necessary.
- Cover every refused grid category and output shape, zero/at-limit/exceeded point budgets and checked length arithmetic. Assert all sentinel outputs unchanged on failures. Test both exact maximum and immediate successor, +0 versus -0 cutoff, both-enabled equality/inversion, invalid gain/matrix fields, unsupported/extended rates, and u64 identities beyond 2^53. Avoid impossible synthetic huge slice construction; test checked arithmetic through a small private helper if that branch cannot otherwise be reached safely.
- Prepare two identical nontrivial `InputBuiltins`, seed filter state by rendering, start a live trim/polarity ramp, and run response queries only between calls for one instance. Existing test-support snapshots of coefficients, integrators, ramp words and elision plan must remain unchanged by the query. Continue matching blocks through both instances; output bits and final state/ramp words must agree. The response intentionally excludes that active trim ramp.
- Use existing `bench-support::alloc::{assert_installed,current_thread_counters,current_thread_delta_since}` to measure zero allocations, frees and reallocations for valid and refused queries after buffers are prepared. Do not write a new unsafe allocator or weaken the policy allowlist. Production dependency graph remains unchanged.

Required proportional local commands, once per coherent attempt:

```
cargo test --locked -p builtins --test filter_response
cargo test --locked -p builtins --lib --tests
cargo clippy --locked -p builtins --all-targets -- -D warnings
cargo fmt --all -- --check
bash scripts/check-builtins-policy.sh
bash scripts/check-realtime-policy.sh
CARGO_TARGET_DIR=target/input-filter-response-wasm-scalar RUSTFLAGS='-C target-feature=-simd128' cargo check --locked --release --target wasm32-unknown-unknown -p builtins
CARGO_TARGET_DIR=target/input-filter-response-wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --release --target wasm32-unknown-unknown -p builtins
```

The existing builtin crate suite includes preserved historical extended-rate evidence and frozen scalar/bank corpus gates. Passing it does not expand the new query's launch support. Do not run ignored descriptive speed tests. Wasm checks prove build portability, not numerical execution; parent integration supplies actual portable-query execution parity when exported. Record exact commands/outcomes and any limitation without manufacturing success.

## Completion and review

Luna pauses at the first coherent compiling/focused-green tranche and reports exact changed paths; root commits and performs status/upstream audit before further implementation. Astra medium checks owner-local coefficient use and validation, negative-zero/rate boundaries, filter-only meaning, floor/endpoint handling, independent expectations, atomic buffers, and state noninterference. Preserve one verdict per attempt. Root synchronizes local/GitHub evidence and closes this child only after PASS evidence is upstream and remote state is verified. #763 remains open and its required successor scopes remain intact.

## Attempt 1 dependency clarification

The focused allocation gates use the repository's existing audited `bench-support` allocator from
dev tests. Root authorizes the corresponding minimal `Cargo.lock` edge for `builtins`: add only
the already-resolved `bench-support` workspace dependency under the builtins package entry, with
no package, version, source, or transitive resolution changes. This is the direct lock consequence
of the already approved `Cargo.toml` dev-dependency and is required for all final `--locked` gates.

## Attempt 1 focused checkpoint evidence

The initial `cargo check -p builtins` completed with exit `0`; its authentic output is in
`/tmp/miso-engine-767-attempt1-lock-check.stdout`, with stderr in
`/tmp/miso-engine-767-attempt1-lock-check.stderr` and the recorded status in
`/tmp/miso-engine-767-attempt1-lock-check.exit`. Cargo initially reordered two pre-existing
`native-pcm-runner` dependency lines while adding the authorized edge; that unrelated ordering was
restored, leaving the final `Cargo.lock` diff with only `builtins -> bench-support`.

The focused implementation tranche is green. `cargo test --locked -p builtins --test
filter_response -- --test-threads=1` exited `0` with 7 tests passing; exact stdout, stderr and exit
status are in `/tmp/miso-engine-767-attempt1-focused.stdout`,
`/tmp/miso-engine-767-attempt1-focused.stderr`, and
`/tmp/miso-engine-767-attempt1-focused.exit`. The focused `cargo clippy --locked -p builtins
--test filter_response -- -D warnings` exited `0`; its authentic logs are in
`/tmp/miso-engine-767-attempt1-focused-clippy.stdout`,
`/tmp/miso-engine-767-attempt1-focused-clippy.stderr`, and
`/tmp/miso-engine-767-attempt1-focused-clippy.exit`. `cargo fmt --all -- --check` exited `0`,
with its captured streams in `/tmp/miso-engine-767-attempt1-fmt.stdout`,
`/tmp/miso-engine-767-attempt1-fmt.stderr`, and `/tmp/miso-engine-767-attempt1-fmt.exit`.

This checkpoint contains only the owner-local response module/re-exports, its focused tests, the
approved `bench-support` dev dependency and its single lock edge, plus this evidence record. The
remaining proportional crate, policy and Wasm gates are intentionally deferred until root
checkpointing and review authorization.

## Attempt 1 broad gate evidence

After root checkpointed the focused tranche and merged the synchronized main ancestry without
content changes, all remaining required gates passed on the frozen source. The full builtins
command `cargo test --locked -p builtins --lib --tests` exited `0`; its authentic stdout, stderr
and exit status are in `/tmp/miso-engine-767-attempt1-full.stdout`,
`/tmp/miso-engine-767-attempt1-full.stderr`, and `/tmp/miso-engine-767-attempt1-full.exit`.
The all-target command `cargo clippy --locked -p builtins --all-targets -- -D warnings` exited
`0`, with logs in `/tmp/miso-engine-767-attempt1-all-clippy.stdout`,
`/tmp/miso-engine-767-attempt1-all-clippy.stderr`, and
`/tmp/miso-engine-767-attempt1-all-clippy.exit`. `cargo fmt --all -- --check` exited `0`, with
logs in `/tmp/miso-engine-767-attempt1-fmt-broad.stdout`,
`/tmp/miso-engine-767-attempt1-fmt-broad.stderr`, and
`/tmp/miso-engine-767-attempt1-fmt-broad.exit`.

`bash scripts/check-builtins-policy.sh` exited `0` (`builtins policy: ok`) and
`bash scripts/check-realtime-policy.sh` exited `0` (`44 marked regions in 12 files`). Their
authentic streams and statuses are in `/tmp/miso-engine-767-attempt1-builtins-policy.{stdout,stderr,exit}`
and `/tmp/miso-engine-767-attempt1-realtime-policy.{stdout,stderr,exit}`. The release Wasm
checks with `-C target-feature=-simd128` and `+simd128` each exited `0`; their streams and statuses
are in `/tmp/miso-engine-767-attempt1-wasm-scalar.{stdout,stderr,exit}` and
`/tmp/miso-engine-767-attempt1-wasm-simd.{stdout,stderr,exit}`. Both used the existing
`wasm32-unknown-unknown` target and `--locked` resolution.

## Attempt 1 adversarial review: FAIL

Verdict: **FAIL — explicit acceptance cases are missing**.

Reviewed frozen source: `2aa7c26090439b517d5b909a151c78e58f82598a`, `/tmp/miso-engine-767`. Feature checkpoint `ebd3e365` and its ancestry merge have identical file contents; comparison against delivered main shows only the allowed #767 implementation/test/spec and dev-dependency changes. Reviewer: Astra, medium. Read AGENTS.md, the full #767 brief/amendment/evidence, complete new implementation and tests, owner preparation/design/validation and state helpers. No implementation edits, agents, GitHub actions, commits, expensive gate reruns, or legacy-source inspection were performed.

## Required bounded correction

1. **Grid validation coverage does not meet the frozen refusal contract.** `crates/builtins/tests/filter_response.rs:386` tests only a duplicated grid. There is no public-query test for empty, NaN, infinity, negative, above-Nyquist or descending grids. Add those cases to the existing small refusal table with correctly shaped buffers, typed `InvalidFrequencyGrid`, allocation counters and bit-unchanged sentinels. This discriminates the individual checks in `src/filter_response.rs:265`, rather than merely demonstrating the duplicate branch. Existing short/long total and optional-section shape cases and budget cases are useful and should remain.

2. **The cutoff corpus and boundary checks omit named acceptance points.** The configurations at `tests/filter_response.rs:143` never use a 10 Hz cutoff; placing 10 Hz in the response probe grid does not exercise a filter designed at 10 Hz. HPF is never independently oracle-compared at a high ordinary cutoff or the exact maximum; its sole maximum query at line 508 only checks successful return at 48 kHz. LPF maximum is oracle-compared at all rates, but the immediate successor rejection at line 504 tests only 48 kHz. The ordering test covers inversion, not equality. Extend the existing small corpus to cover 10 Hz and high/max cutoff designs for HPF and LPF at all four rates with valid pair ordering, retaining independent total and both-channel section comparisons. Loop the exact-maximum/successor checks over the four rates and include equality refusal. No large corpus or new framework is needed.

3. **Configuration/rate refusals and max correlation identity lack their required assertions.** Most malformed-configuration and unsupported-rate calls at `tests/filter_response.rs:551` onward supply anonymous temporary buffers and never inspect them after refusal. Use retained total/optional-section sentinels and verify no writes (and measured zero allocations for representative preparation/rate refusals) through those early-return paths. `u64::MAX` is supplied by the optional-output calls at lines 253/269 but their successful summaries are discarded; capture a summary and assert that the returned ID is exactly `u64::MAX`, keeping the existing `2^53+1` assertion. Finally, document the safe-slice length bound behind the unreachable `2*len` overflow branch, or isolate/test the checked arithmetic with a small private helper; the current checked multiplication is correct, but the brief's arithmetic evidence/rationale is absent. Do not construct unsafe synthetic oversized slices.

These are gaps against the frozen brief, not demonstrated defects in the current production implementation. Complete them in one coherent test/evidence correction with unchanged DSP and tolerances.

## Source findings supporting the implementation

- `prepared_sections` enforces the four launch rates and calls the existing complete `prepare_sections`, then reads the scalar owner's actual private HPF/LPF words. It does not substitute the parametric-EQ designer or duplicate cutoff/schema validation. The owner preserves validation order: matrix/gain/cutoff/order checks occur before zero normalization and coefficient preparation, so negative-zero cutoff remains rejected. The query does not change extended-rate behavior of ordinary builtin preparation.
- The state-space coefficients and adjugate/determinant evaluation agree algebraically with the owner SVF recurrence. Rounded production words and mixes are promoted to f64; `math` supplies transcendentals. Disabled sections bypass the removable singularity. Sections compose as unfloored complex responses before the total floor. Successful output is finite; numerical preflight precedes deterministic publication into caller buffers.
- The API and docs label the result as the HPF/LPF subtotal, excluding trim/polarity/fader/mute/matrix. Tests vary valid excluded fields and obtain unchanged responses; complete supplied configuration still undergoes normal validation. No live object is passed to the query, and all scratch/preparation is bounded, owner-local and allocation-free by inspection and existing measured successful/refusal cases.
- Existing test-support words feed the independent realized-word oracle. Both left/right totals and both HPF/LPF section positions are asserted separately in the tested corpus, avoiding the missing asymmetric-right assertion from the earlier EQ work. Optional output modes are compared with those independently checked results.
- The actual PCM test drives unity-trim `InputBuiltins` with an asymmetric cascade at all four rates and compares impulse DFT values at the prescribed 0.05 dB tolerance. It derives measured output from production processing, not from the query.
- The state test seeds nontrivial filter state, starts trim/polarity changes, renders 16 samples of the 64-sample ramp, and confirms nonzero countdown words 6/7 (verified against the helper's documented layout). Coefficients, trim, integrators, ramp words and elision plan are snapshotted around the query; continued outputs and final snapshots agree with the unqueried twin.
- Existing `lib.rs` changes are module/reexports only. The preexisting response tests, kernels, state, runtime API and production dependency graph are unchanged. Cargo changes are exactly the authorized existing `bench-support` dev edge and its one lockfile dependency line.

## Executed-gate evidence

Directly read the captured focused result: 7 passed, none failed/ignored. Root forwarded all remaining gates passing, and I inspected the saved exit files plus representative terminal tails: full builtin lib/tests, all-target Clippy, formatting, builtin policy, realtime policy, release Wasm scalar and SIMD checks all exited 0. Logs are `/tmp/miso-engine-767-attempt1-{full,all-clippy,fmt-broad,builtins-policy,realtime-policy,wasm-scalar,wasm-simd}.{stdout,stderr,exit}`. Clippy emits existing configuration warnings about two unreachable math paths but exits 0; no new gate failure is claimed. These are supplied execution logs, not reviewer reruns. Wasm checks establish build portability only.

This is the single attempt-1 verdict. Passing commands do not supply omitted test assertions. No architecture rescope, tolerance change, generic harness, timed run, or revisit of delivered #764/#766 is needed. Parent #763 remains open; do not claim this child delivered until it earns PASS and completes required remote delivery/synchronization.

Root authorizes attempt 2 for these bounded test assertions and checked-length rationale in existing allowed paths. Production arithmetic and all thresholds remain frozen. Run focused tests, all-target Clippy, formatting and diff checks before checkpoint; already-passing unchanged-production gates retain applicability for test-only corrections.
