# 011 Native effect runtime contract and conformance

## Outcome

Define the bounded native runtime contract used by the graph compiler and every launch processor.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Specify semantic effect IDs; stable numeric parameter metadata with units, domains, mapping/default, automation rate and smoothing; prepared main/sidechain ports; dual-mono/link rules; factory, prepare, process and reset lifecycle; immutable prepared bypass/latency/tail/quality/resource declarations; bounded lane-isolated state-payload hooks; optional homogeneous bank binding; native registry/session preparation; and adversarial no-allocation conformance needed by the graph compiler and launch effects.

## Required public interfaces/contracts

`NativeEffectFactory` validates a static `EffectDescriptorV1` and prepares an owned `PreparedNativeEffect`; prepared metadata fixes rate, quantum, quality, bypass, link mode, ports, latency, tail and resource use. `PreparedNativeEffect::process` consumes bounded in-place planar dual-mono audio and canonical absolute-sample automation without allocation or blocking. State hooks expose exact common/left/right payload sections for the current nonzero layout version; this issue does not define a persisted envelope or migration.

## Deliverables

Render-reachable Rust traits and semantic descriptors in `miso-engine-effect-contract`; a control-plane native registry/session-preparation adapter in `miso-engine-effect-compiler`; descriptor, lifecycle, automation, bypass/latency, dual-mono state-isolation and realtime conformance tests; invalid mocks; fixtures; and integration documentation for issues 006 and 012–021.

## Explicit non-goals

Descriptor wire bytes or a C descriptor header; package/archive encoding; artifact hashes; CIDv1; download, resolution, cache, signature, trust, licensing or repository policy; persisted state envelopes or cross-version migration; third-party execution; production DSP algorithms; graph/PDC scheduling; arbitrary allocation in process; stringly runtime parameters; or a stable engine C ABI. Those interchange, persistence and identity concerns belong to **Canonical effect interchange, state migration, and CID package identity**.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Versioned TOML schema and transactional session compiler

## Hazards/decisions

Prepared latency and tail are immutable and bypass preserves latency. Linked detection may share only its declared detector value; audio/history/filter/delay/smoother state remains lane-local. Every processor must document equation/coefficient/stability/latency/tail/units/smoothing/denormal-NaN/citations/fixtures/tests/benchmark/listening evidence. No hash or byte serialization is part of the runtime program-cohort key.

## Acceptance gates with objective measurements

All descriptors reject non-finite/invalid domains, duplicate or zero parameter IDs, invalid ports, unsupported rate/quality/link combinations and inconsistent resource declarations. Native session preparation is transactional and yields immutable metadata consumed by issue 006. Across every required rate, declared quality and link mode, conformance detects allocation/free/lock/I/O/log/syscall activity, panic, unbounded span work, lane-state aliasing, malformed state payloads, undeclared/changing latency or tail, latency-changing bypass, invalid automation, and NaN/subnormal propagation. Enabled and bypass impulses land at the exact declared integer sample; one million 128-frame calls report zero forbidden operations.

## Target matrix

Native scalar, AArch64/portable four-lane and AVX2 eight-lane consumers; browser scalar/Wasm-SIMD compilation. Third-party package and ABI consumers depend on issue 029 instead of blocking this launch contract.

## Required evidence

Descriptor/session fixtures, prepared-metadata tables, state-payload isolation and same-layout round-trip results, enabled/bypass impulse reports, automation endpoint/smoothing results, one-million-call realtime audit, invalid-mock detector proof, cross-target build report and bounded benchmark JSONL.

## Terra attempt 1 evidence record (2026-08-20)

**ATTEMPT 1: FAIL.**

Implemented an initial compiling contract/package slice: stable descriptor and process-interface
types, lane-directory validation, CIDv1 raw/SHA2-256 binary/text encoding, package artifact hashes,
basic state envelope, FFI header records, control-plane package preparation boundary, and focused
unit/fuzz entry points. `sha2 = 0.11.0` is confined to the control-plane package crate.

This is not acceptance evidence. The full canonical 128-byte descriptor wire, complete session
preparation, migration dispatch, adversarial conformance mocks, checked golden vectors, ABI target
layout report, 10,000-run fuzz evidence, one-million-block allocation/syscall audit, cross-target
builds, and benchmark JSONL are incomplete. Those gates are FAIL unless later command evidence
replaces them; benchmark invocations: 0.

Commands recorded: focused package/contract/conformance tests PASS; `cargo fmt --all -- --check`,
locked workspace all-target/all-feature check and tests, Clippy with warnings denied, and rustdoc
with warnings denied PASS. Workspace, realtime, and conformance-boundary policy scripts PASS.
`scripts/check-effect-contract.sh` deliberately FAILS because full vectors and conformance gates
are not implemented. Per the Sol brief, the effect-contract benchmark was not invoked.

Release native tests/builds for both new crates and the tool PASS. Android AArch64 and iOS AArch64
checks PASS; wasm32 scalar and `+simd128` release builds PASS. A one-run Python stdlib CID check
for `hello` matches the Rust CID primitive vector. The fuzz manifest's locked check FAILS because
its independent `fuzz/Cargo.lock` has not been updated for the new package dependency; the required
two 10,000-execution fuzz runs were therefore not attempted. This is a failure, not an excuse to
invoke the benchmark.

## Sol adversarial review / correction attempt 2 (2026-08-20)

**ATTEMPT 2: BLOCKED BEFORE IMPLEMENTATION; ISSUE REMAINS FAIL. REBRIEF REQUIRED.**

The frozen brief is not implementable without inventing or changing normative wire/API semantics:

- `EffectDescriptorV1` requires a quality-independent `supported_link_modes` bitmask, descriptor
  validation must reject unsupported link modes, package/session preparation must consume it, and
  the canonical descriptor identity must preserve it. The frozen 128-byte descriptor header has
  `flags2` fixed to zero and bytes 92..128 reserved-zero; none of the parameter, enum, port, or
  quality records contains the bitmask. Consequently encode/decode cannot round-trip a required
  semantic field and independent implementations cannot derive the same complete descriptor from
  the wire. Reusing a reserved field would violate the frozen byte format and its mutation gates.
- The brief requires deterministic `descriptor_schema_hash` and
  `EffectProgramSignature([u8; 32])` values, but freezes neither the hash primitive nor the exact
  domain-separated canonical preimage for either value. The latter must include descriptor
  identity, contract major, quality, link mode, port topology, latency, and state layout version,
  yet there is no cross-implementation byte recipe or golden value.
- The package encoder's frozen public API writes to `&mut Vec<u8>`, while required fixtures include
  an `output-too-small/no-partial-write` gate. A growable `Vec` has no caller-supplied output
  capacity contract, so the named failure class cannot be exercised without adding another API or
  redefining the gate.
- State migration is required to dispatch every registered `N -> N+1` step with bounded scratch,
  but the frozen interfaces define no migration registry/step type, registration key, scratch
  sizing contract, or transaction API. The processor trait exposes a directly mutating restore
  method while the brief separately requires failed restore to leave the destination unchanged via
  a prepared temporary. This needs an explicit ownership/API decision before interoperable
  migration and conformance tests can be frozen.

The existing attempt also has independent implementation defects that a rebrief must not hide:
the descriptor encoder writes only a partial header followed by the effect ID; package verification
does not validate descriptor bytes; session preparation returns
`effect.prepare.unimplemented` for every effect-bearing session; state verification fabricates the
effect ID `unresolved`, accepts noncanonical bypass values as false, and does not validate the
stored effect-ID digest; descriptor validation does not require exact `main-in`/`main-out` IDs,
does not require all eight rates for every declared optional quality, and does not enforce header
maximum-state equality; span validation does not reject overlap; the conformance runner checks
only descriptor/configuration; checked-in vectors/docs/scripts are placeholders; all three new
scripts lack executable permission; and the benchmark script is deliberately unavailable.

Observed review commands:

- Focused locked tests for the contract, package, and conformance crates pass, but exercise only
  the compiling slice and are not acceptance evidence.
- `scripts/check-effect-contract.sh` cannot be executed directly (mode is `0644`); invoking it via
  Bash still exits nonzero by design.
- `cargo check --manifest-path fuzz/Cargo.toml --locked --all-targets` fails because the fuzz lock
  file requires an update. Neither required 10,000-execution fuzz run was started.
- A standalone C11 compilation confirms the four currently declared record sizes and sampled
  alignment/offset assertions on x86_64 Linux only. No required multi-target machine-readable ABI
  report exists.
- The independent Python CID of the checked-in `golden/minimal-source.txt` does not equal the Rust
  tool's `--print-vector minimal-source` result because the checked-in file is placeholder text,
  not the canonical package bytes consumed by the Rust fixture.

No production correction was made, no nonbenchmark acceptance suite can pass, and the benchmark
was not invoked (benchmark invocation count remains 0). Amend and re-freeze the descriptor wire,
hash/signature preimages, output-buffer gate/API, and migration transaction interface, then restart
the Sol brief -> Terra attempt 1 workflow. A third Sol correction attempt against the current brief
is not warranted; the workflow requires rebrief rather than gate weakening or an incompatible wire
guess.

## Rescope decision and workflow reset (2026-08-20)

The failed attempts and observations above remain the complete evidence for the superseded combined
brief; none is converted into acceptance evidence. The issue is now narrowed to the launch-critical
native runtime contract described by the current Outcome through Required evidence sections.
Descriptor wire/C records, canonical package/artifact/CID identity, and full state envelopes and
migration moved intact to **Canonical effect interchange, state migration, and CID package
identity**. They are not gates for issue 011, the graph compiler, or launch native effects.

The prior attempt counter is closed with the superseded scope. The workflow restarts at a new Sol
brief followed by a new Terra attempt 1; no current implementation is presumed conforming, and no
old failed or missing gate is reported as a pass. The authoritative replacement brief is
`target/issue11-rescoped-sol-brief.md`.

## Restarted Terra attempt 1 evidence record (2026-08-20)

**ATTEMPT 1: FAIL — incomplete semantic implementation; benchmark not invoked.**

Implemented a compiling semantic-only replacement slice in
`miso-engine-effect-contract` and added `miso-engine-effect-compiler`. The contract no longer
depends on `miso-engine-effect-package` or `sha2`; the compiler depends only on core, session and
contract. The slice includes static semantic IDs, descriptor validation, typed preparation input,
immutable prepared metadata/program keys, scalar and bank trait shapes, bounded state sections,
registry validation, and an off-render session preparation adapter. `miso-engine-conformance` no
longer declares an effect-package dependency.

This is explicitly not acceptance evidence. It lacks the required correct processor and faulty
mocks, adversarial detector/mutation proof, semantic runtime fixtures and manifest generator,
one-million-call forbidden-operation/syscall audit, descriptor/session/span mutation run,
cross-target builds, state/isolation/latency reports, docs/policy mutation coverage, and the
two-round JSONL benchmark implementation. The workspace all-target build currently also FAILS:
the untouched issue-029 package crate still references superseded contract state/wire APIs
(`verify_lane_payload_v1`, `state_current`, `state_min_readable`). No issue-029 package/state/wire
file was changed to conceal that incompatibility.

Observed commands: `cargo test -p miso-engine-effect-contract -p
miso-engine-effect-compiler` PASS (there are currently no tests in either new slice);
`bash scripts/check-workspace-policy.sh`, `bash scripts/check-realtime-policy.sh`, and `bash
scripts/check-conformance-boundaries.sh` PASS. `cargo fmt --all -- --check` initially FAILed before
formatting; `cargo fmt --all` was run, but the required full post-format gate suite has not been
completed. `cargo check --workspace --all-targets --all-features` FAILS for the package boundary
above. `scripts/check-effect-contract.sh` remains the superseded package/vector checker and exits
nonzero by design. Benchmark invocation count: **0**.

## Restarted Sol adversarial review / correction attempt 2 (2026-08-20)

**ATTEMPT 2: PASS. ISSUE 011 MAY CLOSE.**

The 511-line rescoped brief was re-read in full and is adequate. It removes the superseded
wire/hash/CID/migration contradictions, freezes an implementable semantic runtime boundary, and
does not require an issue-029 design guess. Review found additional defects beyond the restarted
Terra record: orphaned superseded runtime modules, incomplete enum-label/negative-zero validation,
missing stepped mapping/smoother/segment behavior, no quantum or canonical automation
order/overlap/rate/capacity enforcement, incomplete compiler duplicate/domain/sidechain/resource
and metadata checks, and descriptor-only conformance. The provisional issue-029 package also
prevented the workspace from compiling against the superseded state APIs.

Sol correction attempt 2 replaced the reachable semantic slice with the frozen static ID,
descriptor, mapping, smoothing, automation, process-block, state-section, factory, registry,
metadata and program-key contract; completed transactional native session preparation; removed
the unreachable superseded modules; and isolated issue 029 only enough to compile by moving its
provisional lane-envelope helper back into its own package and treating the sole current runtime
layout as both provisional wire bounds. No issue-029 package, wire, state, hash or CID result is
used as issue-011 evidence.

The conformance crate now supplies a bounded dual-accumulator/three-sample-delay processor with
independent lane state and optional sidechain, plus faulty modes for allocation, deallocation,
lock, file I/O, network I/O, logging, syscall, panic, shared lane state, changing latency/tail,
bypass latency, resource metadata, malformed spans, nonfinite propagation, nondeterministic or
partial snapshot and bad restore. Across all eight rates and enabled/bypass it exercises frame
counts 1/127/128, 100 impulse reprepare repetitions per configuration, main/sidechain
sanitization, both resets, deterministic snapshot, current-layout restore/continuation and lane
payload isolation. The structured release report records 16 configurations, 1,840 process calls
and zero failed gates. Separate tests prove exact mapping, smoother and segment endpoints.

Deterministic mutation evidence passes 10,000 descriptor mutations, 10,000 span mutations and
10,000 compiled-session parameter mutations without panic or partial success. The sorted runtime
fixture manifest contains four exact-length lowercase-SHA-256 entries; missing, unlisted, changed
and policy-boundary mutations fail as intended. The runtime dependency policy proves contract ->
core only, compiler -> core/session/contract only, no reverse core/session dependency, and no
issue-029 package/hash/persistence API in the issue-011 path.

Observed passing nonbenchmark commands:

- formatting; workspace/realtime/conformance/research/effect-runtime policy checks and policy/
  fixture mutation tests; the complete `scripts/check-effect-contract.sh` semantic suite;
- locked workspace all-target/all-feature check and tests, Clippy with warnings denied, and
  rustdoc with warnings denied;
- release tests/builds for contract, compiler, conformance and the issue-011 tool;
- AArch64 Android and iOS release checks, wasm32 baseline release build, and separate wasm32
  `+simd128` release build;
- release conformance report with zero failures; and
- `scripts/trace-effect-contract-audit.sh` around 1,000,000 128-frame calls containing finite
  alternating extreme samples: zero allocation/free/lock/log/file/network/syscall counters and no
  intervening native syscall between trace markers. The audit JSON is
  `target/issue11/strace/audit.json`.

The benchmark driver and validator cover scalar no-op, real four/eight-lane bank trait calls,
descriptor validation, factory preparation and state snapshot/restore. They compute nearest-rank
percentiles and report actual allocation/deallocation counts plus fixture and build metadata.
After all nonbenchmark gates passed, the root Sol agent invoked the authorized command exactly
once: `bash scripts/run-effect-contract-benchmark.sh`. It exited zero without tuning or retry.
Benchmark invocation count is **1**.

`target/issue11/effect-contract-benchmark.jsonl` is 7,221 bytes and has SHA-256
`912b250a29dfd6bbc69cfb6b32e23e253eba77a53cc448a99a036eac9c556785`. It contains exactly 12
records: rounds 1 and 2 for each of the six workloads, with 1,000 observations per record. Every
record has ordered `min <= p50 <= p95 <= p99 <= p99.9 <= max` nanosecond metrics. Scalar,
four-lane-bank and eight-lane-bank process records report zero allocations and zero deallocations
in both rounds with widths 1/4/8. State snapshot/restore also reports zero allocations/frees;
descriptor validation reports 4,000/4,000 and factory preparation 6,000/6,000 control-plane
allocations/frees per round. Every record identifies four fixtures and manifest SHA-256
`26e35dacebe4922d7fd7bf63d6cdc6c7084128bf64390a35a17907e249cb1e0b`, matching the checked-in
runtime manifest. OS, optimization, LTO and codegen-unit metadata are present; unavailable CPU,
governor, Rust/LLVM, target and feature environment values are explicitly disclosed as unknown.

All frozen acceptance gates are therefore satisfied and issue 011 may close. Remaining caveats
are nonblocking and explicit: timings are descriptive with no hardware-independent threshold;
the bank workloads measure the homogeneous no-op trait/interface path rather than production DSP;
Android/iOS/Wasm results are compilation claims, not device-run claims; and all external
descriptor/package/CID bytes and persisted migration semantics remain issue-029 scope and were not
used as issue-011 evidence.
