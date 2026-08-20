# 004 Versioned TOML schema and transactional session compiler

## Outcome

Define the complete strict V1 TOML session surface and compile it transactionally into an immutable, non-publishable `CompiledSession` control-plane IR with checked resource estimates. Downstream graph, DSP, source, and effect compilers extend this transaction before producing a publishable `PreparedRenderPlan`.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement strict versioned TOML parsing and canonical serialization for tracks, stable IDs, dual-mono builtin controls, named send taps, racks, effect ID/CID text and quality selections, sidechain declarations, submixes, source declarations with content identity, routes, automation, sample rate, quantum, render/output profiles, and queue/ring/memory limits.

Issue 004 validates version, required and unknown fields, TOML types, stable-ID syntax and uniqueness, schema-owned cross-references, finite and schema-local numeric domains, explicit units, ordered automation representation, and checked count/byte resource estimates. It returns an immutable candidate only after all issue-004 validation succeeds.

Graph cycles, port validity, PDC and latency propagation, builtin DSP/Nyquist domains, SIMD-bank compatibility, source asset resolution/rate verification, and effect/CID/package validity are validated by their owning downstream compiler stages before plan publication.

## Required public interfaces/contracts

`SessionTomlV1` is canonical strict input; `parse_session_toml(&str) -> Result<SessionTomlV1, DiagnosticSet>` parses it; `canonical_session_toml(&SessionTomlV1) -> Result<String, DiagnosticSet>` snapshots it; and `compile_session(&SessionTomlV1, CompileCaps) -> Result<CompiledSession, DiagnosticSet>` returns no partial artifact. `CompiledSession` is not a `PreparedRenderPlan`. Diagnostics have stable dotted codes and structured paths. All numeric values have explicit units. No issue-004 API accepts a `PlanPublisher`.

## Deliverables

Schema reference and ownership matrix, canonical serialization rules, parser, owned validator, checked preflight/resource estimator, immutable transactional compiler boundary, diagnostic registry, migration/version rejection tests, canonical fixtures, fuzz targets, and example session.

## Explicit non-goals

Preparing or publishing a render plan; graph scheduling, cycle analysis, port validation, PDC or latency propagation; builtin DSP validation or processing; SIMD cohort/bank validation; source I/O, asset resolution or SRC decisions; effect descriptor/CID/package verification or execution; runtime control protocol; V1/legacy compatibility; migration from unsupported versions; hidden defaults; or accepting unknown keys.

## Dependencies by exact issue title

- Real-time memory, buffers, queues, and plan lifetime

## Hazards/decisions

Configuration parsing/compilation is control-plane work. Exact rate and quantum are required; supported rates are 44100, 48000, 88200, 96000, 176400, 192000, 352800, 384000 Hz.

## Acceptance gates with objective measurements

Canonical round-trip is byte-stable. Parser/compiler fuzzing and at least 100 invalid issue-004-owned cases cover unsupported/missing versions, missing and unknown fields, wrong TOML types, malformed and duplicate stable IDs, non-finite/non-`f32`-representable values, schema-local range/unit errors, dangling schema-owned entity references, invalid source regions or channel indices, invalid automation ordering/ranges, zero/overflowing capacities, checked count/byte arithmetic overflow, and typed configured-resource rejection. Every rejection has an exact diagnostic path/code and returns no `CompiledSession`; issue 004 constructs or publishes no `PreparedRenderPlan`.

Generated sessions beyond 65,536 tracks complete validation/estimation and either return a `CompiledSession` within supplied `CompileCaps` or only a typed resource-limit diagnostic, never a product track-count limit. Downstream-owned invalidity is preserved in the typed IR and is not misreported as validation of a publishable render plan.

## Target matrix

Native, iOS, Android, and browser session compile; host capabilities may reject unavailable resources.

## Required evidence

Canonical TOML fixtures, compiler diagnostics, transaction trace, and allocation profile showing compilation stays off render.

## Sol-amended implementation boundary (2026-08-20)

Issue 004 owns strict TOML 1.0 syntax, `schema_version`, unknown fields, the complete declarative surface, stable ID syntax/uniqueness and declared-reference checks, finite/f32/local unit checks, ordered automation, checked estimates, canonical bytes, and immutable **non-publishable** `CompiledSession`. Its frozen API is `SESSION_SCHEMA_VERSION_V1`, `parse_session_toml(&str) -> Result<SessionTomlV1, DiagnosticSet>`, `canonical_session_toml(&SessionTomlV1) -> Result<String, DiagnosticSet>`, and `compile_session(&SessionTomlV1, CompileCaps) -> Result<CompiledSession, DiagnosticSet>`.

It explicitly defers issue 006 cycles/ports/PDC, issue 007 DSP/Nyquist, issue 008 bank compatibility, issue 010 asset/rate resolution, issue 011 native effect validation/preparation, and issue 029 CID/package validation. It never imports `PlanPublisher`, prepares/publishes a plan, or changes `miso-engine-core`; the one-way session-to-core value-carrier dependency is permitted.

The frozen V1 route surface uses tagged sources (`track` plus tap, or `submix_output`) and tagged destinations (`submix_input` or `output_input`). Thus a submix output is representable while an output source or track destination is not. Routed sidechains reuse the tagged source and require a stable `port_id`; existence and effect-port compatibility remain downstream validation. V1 output is exactly two planar channels so every declared channel map is an explicit 2x2 matrix.

The dependency requirement is `serde = 1.0.228` plus Cargo `toml = 0.9.9`, resolved as package version `0.9.9+spec-1.0.0`. `spec-1.0.0` is not a Cargo feature. TOML default features are disabled and only `parse` and `serde` are enabled; the custom canonical writer does not enable `display`.

Invalid-corpus evidence uses the frozen distribution: 16 schema/version/type cases, 20 ID cases, 24 finite/unit/local-range cases, 16 source/region cases, 20 schema-owned reference cases, 20 automation cases, 20 checked-arithmetic cases, and 16 configured-resource/capacity cases (152 total). Each case asserts its exact code and leaf path; multiplying duplicate inputs does not count.

The performance protocol is one invocation of `scripts/run-session-benchmark.sh`. That invocation performs exactly two internal rounds for representative parse-plus-canonical and compile workloads and emits descriptive JSONL only. It has no pass threshold, retry, or tuning loop and is invoked only after every nonbenchmark gate passes.

### Attempt 1 evidence (Terra, FAIL/incomplete)

- Added strict manual `toml::Value` walking under exact `toml = 0.9.9+spec-1.0.0` parse/serde features; `display` is intentionally excluded because canonical TOML is custom-written.
- Added typed model, parser, canonical fixture, diagnostics, validation, checked estimation, and a non-publishable compilation artifact. It has no core realtime/plan-exchange import.
- Local format, session Clippy with warnings denied, session unit tests, workspace naming policy, and conformance-boundary policy passed before the final diagnostic/index audit follow-up. Target, fuzz, scale, benchmark, and CI evidence remain incomplete at this record.

### Attempt 2 evidence (Sol, benchmark evidence REJECTED)

- Replaced the contradictory original outcome/scope/non-goals/acceptance language with the issue-004 ownership boundary above. No V1/legacy or Git/GitHub source was inspected. `miso-engine-core` was not edited; the dependency remains session-to-core only.
- Reworked diagnostics into a stable dotted `DiagnosticCode` registry and structured field/index/ID `DiagnosticPath`. The exact-path corpus includes the right builtins lane and every nested table family.
- Added tagged route source/destination roles, stable sidechain `port_id`, explicit `none` sidechains, and the V1 two-channel output constraint without taking downstream cycle, PDC, DSP, asset, or effect validity.
- Made checked resource estimation cover retained strings, model/nested vectors, normalized indexes, canonical allocation upper bound/actual bytes, queue and per-source ring bytes, largest allocation, `usize`/`isize`, and checked totals. Successful preflight has no per-item diagnostic formatting or collection allocation. Preflight and all cap comparisons execute before semantic-validation allocations, canonical writing, model cloning, sorting, or index construction.
- `cargo test --locked -p miso-engine-session --all-targets`: PASS. This includes the genuine 152-case distribution, strict nested unknowns, full tagged-surface canonical round-trip, two TOML 1.0 behavior fixtures, 4,096 deterministic mutations, transactional failure, preflight precedence, and adequate-cap compilation of 65,537 returned tracks plus typed cap rejection.
- `cargo test --locked --workspace --all-targets`: PASS.
- `cargo fmt --all -- --check`; workspace all-target/all-feature check and Clippy with `-D warnings`; rustdoc with `-D warnings`; workspace, realtime, session, conformance, and research policy gates: PASS.
- Coverage-instrumented ASan fuzzing under pinned `nightly-2026-08-20` and `cargo-fuzz 0.13.2`: parser 10,000 runs PASS; compiler 10,000 runs PASS, both seeded with `canonical.toml`. The first stable-toolchain fuzz invocation failed before execution because sanitizer `-Z` flags require nightly; this was an environment/toolchain exception, not a code failure, and CI now installs the pinned fuzz-only nightly.
- `miso-engine-session` target gates: native host tests PASS; `aarch64-linux-android` check PASS; `aarch64-apple-ios` check PASS; `wasm32-unknown-unknown` scalar release build PASS; `wasm32-unknown-unknown +simd128` release build PASS. These are compile/build claims only, not Android/iOS device or browser runtime claims.
- Feature evidence resolves `serde 1.0.228` and `toml 0.9.9+spec-1.0.0` with TOML `parse`/`serde`, no `display`. Descriptive session archive evidence is recorded in `docs/REALTIME_DEPENDENCY_POLICY.md`.
- The implementation and nonbenchmark gates above were reported passing, but root rejected the attempt-2 benchmark evidence. The rejected artifact is archived as `target/session-benchmark-attempt2-invalid.jsonl`, SHA-256 `ef93f15ac37b2d40a28db705dc5e1d1f9ed4173775356df2dee4fe5ee4b9bca5`, 2,320 bytes, timestamp `1787243385`, with four records. It measured the one-track seed (`track_count = 1`), not a fixed representative 256-track workload, and omitted CPU model, kernel, governor/power mode, Rust/LLVM, target triple/features, runtime, opt/LTO/codegen units, fixture SHA/size/full structural counts, 48 kHz/128-frame fields, p99.9, and explicit missing-metadata disclosure. Its numeric timings are invalid issue-004 acceptance evidence and are retained here only as a rejected provenance record.

### Attempt 3 evidence (Sol final correction, PASS)

- This final allowed correction changed only the benchmark contract, validator, and evidence. The benchmark now deterministically expands the canonical V1 seed to 256 tracks, 256 routes, and 256 automation programs. Its canonical fixture is 318,842 bytes with SHA-256 `2697ee9b0d857b21cb5a568326718cb99754c076c503f3c395d2c9be02af021d`; full counts are 1 source, 256 tracks, 0 submixes, 1 output, 256 routes, 256 automation programs, 256 effects, 256 effect parameters, and 256 automation segments.
- Benchmark schema version 2 requires exactly four unique workload/round records, exactly two internal rounds per workload, the frozen fixture counts/hash/size, 48,000 Hz, 128 frames, complete machine/toolchain/build/runtime fields, explicit missing-metadata disclosure, and nearest-rank `min <= p50 <= p95 <= p99 <= p99.9 <= max`. It rejects the attempt-2 artifact.
- Before timing, `cargo test --locked -p miso-engine-session-bench --all-targets` passed 2 tests, including fixture byte stability/counts and a SHA-256 known-answer test. Warning-denied Clippy, repository format checking, shell syntax, and the negative attempt-2 schema check passed. No broader nonbenchmark gates were rerun for this evidence-only correction.
- After those gates passed, `bash scripts/run-session-benchmark.sh > target/session-benchmark.jsonl` was invoked exactly once. It ran exactly two internal rounds, made no retries or tuning changes, and emitted exactly four validator-accepted JSONL records. The resulting 6,182-byte artifact has SHA-256 `a5f04e6941f71320180fcc988b314a872243d4ec13b4e76a0558f4fb78f8dcd4`.
- Parse-plus-canonical rounds reported `(min, p50, p95, p99, p99.9, max)` of `(6,950,634; 7,031,337; 7,060,771; 7,113,260; 7,379,057; 7,379,057)` and `(7,026,084; 7,077,401; 7,101,424; 7,111,478; 7,182,010; 7,182,010)` ns/operation. Compile rounds reported `(1,045,995; 1,048,759; 1,052,471; 1,053,702; 1,056,009; 1,056,009)` and `(1,043,878; 1,049,156; 1,053,306; 1,055,113; 1,056,535; 1,056,535)` ns/operation. Results are descriptive only and have no decision threshold.
- Recorded environment: AMD Ryzen 7 9700X 8-Core Processor, x86_64 Linux kernel 6.8.0-138-generic, `powersave` governor, Rust 1.97.1 with LLVM 22.1.6, `x86_64-unknown-linux-gnu`, target-default CPU/features, native CLI runtime, release opt-level 3, LTO off, and 16 codegen units. Power source was unavailable and is explicitly recorded as `unknown` in `missing_metadata = ["power_source"]` with `metadata_incomplete = true`; background load was explicitly not measured.
