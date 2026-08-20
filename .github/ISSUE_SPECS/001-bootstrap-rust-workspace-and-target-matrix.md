# 001 Bootstrap Rust workspace and target matrix

## Outcome

Create the greenfield Cargo workspace and explicit feature/target policy for core DSP, native C ABI, iOS, Android, and browser Wasm. No V1 or legacy repository may be read.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Create packages `miso-engine-core`, `miso-engine-session`, `miso-engine-protocol`, `miso-engine-capi`, and `miso-engine-*` host/example shells; pin MSRV/toolchain; define scalar, Wasm `simd128`, AArch64 NEON, AVX2, and FMA build/runtime policy. Cargo package and directory names must use the `miso-engine-` prefix; Rust crate identifiers use the matching `miso_engine_` form. Document CI target checks, unsafe-code ownership, benchmark metadata, and forbidden realtime dependencies.

## Required public interfaces/contracts

`miso-engine-core` exposes `EngineVersion`, `SampleRateHz(u32)`, `QuantumFrames(u32)`, and target capability query; Cargo features must not change session semantics.

## Deliverables

Workspace manifests, target matrix document, CI compile/check jobs, minimal cross-target smoke crate, and reproducible toolchain lock.

## Explicit non-goals

Implementing DSP, session parsing, platform audio callbacks, plugin execution, or a default maximum track count.

## Dependencies by exact issue title

- None.

## Hazards/decisions

Do not compile AVX2/FMA globally: native code must dispatch after separate feature detection. `wasm32-unknown-unknown` has no OS filesystem/thread baseline: https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html.

## Acceptance gates with objective measurements

`cargo check` succeeds for host and `wasm32-unknown-unknown`; cross-checks cover AArch64 iOS/Android; scalar and SIMD artifacts are built separately where runtime selection requires it; AVX2 without FMA is a tested capability; no public/session capacity imposes a track ceiling; benchmark output records CPU, OS, power/governor mode, compiler, target features, runtime/browser, rate, quantum, fixture, warm-up, duration, and statistical method.

## Target matrix

Native x86_64 scalar/AVX2/FMA, ARM64 iOS/Android NEON, wasm32 browser scalar/SIMD128.

## Required evidence

CI logs, locked toolchain version, feature-resolution report, and lint output.

## Implementation decision and evidence record

### Decision record (Terra attempt 1)

- Rust is pinned to 1.97.1 in both the workspace MSRV declaration and `rust-toolchain.toml`.
  The lock requests only clippy, rustfmt, browser Wasm, Android ARM64, and iOS ARM64; simulator,
  SDK, NDK, and browser tooling are deliberately not implied.
- CPU ISA is not represented by a Cargo feature. The bootstrap exposes only control-plane
  capability discovery: scalar is always available; Wasm reports a compilation artifact setting;
  AArch64 reports NEON; and x86 AVX2/FMA are separately runtime detected. The internal assembler
  has an AVX2-without-FMA test independent of the host CPU.
- The scalar, AVX2/no-FMA, AVX2/FMA and baseline/SIMD128 Wasm probes use distinct target
  directories. Scoped `RUSTFLAGS` are evidence-only CI probe inputs, never workspace defaults.
- The bootstrap contains no unsafe code. A later exception is restricted by policy to core
  architecture internals or C-ABI FFI internals and needs an explicit local invariant, safety
  explanation, tests, and review.
- Issue 001 has no third-party dependencies. Future additions require an issue-scoped review of
  realtime reachability, allocation behavior, target support, license, size, and failure modes.
- The benchmark is a std-only, fixed-work capability-query harness. It is descriptive setup
  evidence, not a render-performance or sound-quality result.

### Local evidence (2026-08-20)

- `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`,
  `cargo check --locked --workspace --all-targets`,
  `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
  `cargo test --locked --workspace`, and `cargo doc --locked --workspace --no-deps` passed.
- The separate x86 probes passed with `-avx2,-fma`, `+avx2,-fma`, and `+avx2,+fma`; each matched
  the expected `rustc --print cfg` assertion. Separate release Wasm baseline and `+simd128`
  artifacts built successfully. Pure-Rust mobile host checks passed for
  `aarch64-linux-android` and `aarch64-apple-ios`.
- One two-round benchmark run on the local `AMD Ryzen 7 9700X 8-Core Processor` reported
  `1.089288` and `1.086924` ns/query. Its recorded governor was `powersave`, so the result is
  descriptive only and not a performance gate.
- Xcode/iOS linking and device execution, Android NDK linking and device execution, and browser
  runtime validation were unavailable in this environment and remain deferred to their explicit
  platform issues. No nightly-only benchmark facility is required.
- No V1, legacy, or old repository was inspected, copied, or benchmarked.

### Sol adversarial review and correction attempt 2 (2026-08-20)

Terra attempt 1 compiled and passed its recorded local checks, but did not satisfy the complete
brief: the two scalar carrier types lacked their required ordering/hash derives; several CI jobs
checked only transitive package subsets; the host workflow omitted exact all-feature/all-target and
rustdoc-warning gates; the bounded benchmark was absent from CI; package-to-crate-name policy did
not cover binary targets; and Rust `escape_default` could emit escapes that are not valid JSON.

Sol correction attempt 2 fixed those defects without changing the issue scope. Independent local
verification passed formatting, policy and policy-mutation tests, locked all-target/all-feature
check, Clippy with warnings denied, all-target tests, rustdoc with warnings denied, the feature
tree, metadata, and the native host smoke. The scalar, AVX2/no-FMA, and AVX2/FMA checks passed in
distinct target directories with the expected cfg assertions. Full portable package sets passed
release Wasm scalar/SIMD128 builds and pure-Rust Android/iOS checks.

`wasm-objdump` verified that the scalar artifact does not advertise `simd128`, the SIMD artifact
does, and neither advertises relaxed SIMD. Their SHA-256 hashes were respectively
`b2459ba6a4ac66dd5f0a0015674120137ecc47e1e4f570f4563acf68908edbea` and
`112218213e64a40a93ec57758387e4b2193f606d32ba1ecf7f3133a377d2cc4c`.

The single Sol benchmark invocation emitted valid one-line JSON with Rust 1.97.1/LLVM 22.1.6,
native x86_64, the AMD Ryzen 7 9700X, `powersave` governor, 48 kHz, 128 frames, one fixed warm-up,
two one-million-query rounds, and `1.273239`/`1.279531` ns/query. No timing threshold, retry, or
optimization was applied. GitHub-hosted workflow execution is not claimed because this workspace
is not a Git checkout; the corresponding commands and artifact production were run locally, and
the checked-in workflow defines the publication-time CI evidence gate.
