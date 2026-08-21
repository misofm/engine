# 032 Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

## Outcome

Make 44,100, 48,000, 88,200, and 96,000 Hz the exact launch-supported session and realtime
sample-rate set. Preserve 176,400, 192,000, 352,800, and 384,000 Hz only as a clearly labeled
extended-rate compatibility corpus until a later stateless issue supplies complete DSP, host,
realtime, performance, and release qualification.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy,
benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated
`PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only
through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O,
logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are
retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono
L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix
declares otherwise. Source/engine mismatches have no implicit SRC. Output is PCM.

The repository currently duplicates an eight-rate policy across core realtime, strict session
parsing and validation, native-effect descriptors, conformance block/fixture validation, fixture
generation, docs, and issue contexts. Issue 007 demonstrated and accepted a launch-quality builtin
filter recurrence at the first four rates; its observations at the four higher rates are useful
diagnostics but are not complete launch qualification. Leaving the old duplicated policy in place
would make a session or protocol mutation claim support that hosts, launch effects, and release
qualification have not proved.

This issue is independently implementable only after its exact dependencies are complete. Its
change follows the Sol-approved brief -> Terra attempt 1 with evidence -> Sol adversarial review
workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed
rather than weakening gates.

## Frozen rate tiers

- Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz.
- Extended compatibility rates are exactly 176,400, 192,000, 352,800, and 384,000 Hz.
- An extended compatibility rate is not accepted as an engine session rate, cannot produce or be
  published as a launch `PreparedRenderPlan`, and is not a host or release support claim.
- A nonzero source-declared rate remains a carrier for asset metadata. It may differ from the
  engine rate, but issue 010 must reject the mismatch before publication because this sprint has no
  implicit SRC.
- Existing extended-rate `.mepcm` files, effect-conformance observations, and issue-007 numerical
  observations remain useful compatibility evidence. Their historical labels such as “required”
  or “all eight” describe the superseded policy at the time and do not expand the launch tier.

## Scope

Centralize the two exact rate sets and their predicates in `miso-engine-core`; remove independent
engine-support lists from realtime, session, effect-contract, and conformance code. Make strict TOML
parse/compile and typed protocol transactions reject every non-launch engine rate with the same
stable diagnostic. Require every declared native-effect quality to contain all four launch rows,
while permitting unique ordered rows for any subset of the four extended compatibility rates.

Keep the conformance file format and checked-in eight-rate impulse corpus readable and byte-stable.
Separate launch gates from extended informational probes in conformance reports and tests. Update
the architecture guide, normative docs, future issue contexts/dependencies, host/runner language,
and release qualification so none advertises extended launch support. Add explicit annotations to
historical evidence where old “required/all-eight” wording would otherwise be mistaken for the new
normative contract.

## Required public interfaces/contracts

`miso-engine-core` is the single authority and exports these exact typed values and predicates:

```rust
pub const LAUNCH_SAMPLE_RATES: [SampleRateHz; 4] = [
    SampleRateHz(44_100),
    SampleRateHz(48_000),
    SampleRateHz(88_200),
    SampleRateHz(96_000),
];

pub const EXTENDED_COMPATIBILITY_SAMPLE_RATES: [SampleRateHz; 4] = [
    SampleRateHz(176_400),
    SampleRateHz(192_000),
    SampleRateHz(352_800),
    SampleRateHz(384_000),
];

pub const fn is_launch_sample_rate(rate: SampleRateHz) -> bool;
pub const fn is_extended_compatibility_sample_rate(rate: SampleRateHz) -> bool;
```

`SampleRateHz` remains a lossless `u32` carrier; construction alone never promises engine support.
There is no combined public `SUPPORTED_SAMPLE_RATES` alias and no second literal match/list that
owns launch policy. Conformance code may accept the union only by calling the two core predicates.

Strict TOML parsing and compilation use `DiagnosticCode::SampleRateUnsupportedAtLaunch`, whose
stable registry string is `sample_rate.unsupported_at_launch`, path is `$.sample_rate_hz`, and
message names exactly `44100, 48000, 88200, or 96000 Hz`. Parsing or compiling 176,400/192,000/
352,800/384,000 Hz therefore returns no model/artifact. Canonical serialization cannot be used to
legitimize an invalid typed model.

The BTLV codec continues to represent the `u32` field without policy. Applying a final candidate
whose engine rate is non-launch returns `validation_failed` and the same code/path through the
existing protocol diagnostic mapping. Its `operation_index` remains the existing final-validation
sentinel `edits.len()`. The prior model, revision, canonical snapshot, and reliable event queues
remain unchanged; no plan is prepared or published. A transaction that temporarily writes an
extended rate but ends on a launch rate is judged by its final candidate, consistently with
existing atomic session semantics.

For each declared `EffectQuality`, descriptor validation requires exactly one row for each launch
rate. It may additionally accept at most one ordered row for each extended compatibility rate;
those rows are optional and do not create session, host, or release support. All other rates,
duplicate rows, missing launch rows, or out-of-order `(quality, sample_rate)` rows reject under the
existing descriptor-quality diagnostic contract. `REQUIRED_SAMPLE_RATES` is removed in favor of
the core authority.

The native PCM runner rejects a non-launch session rate through this shared session/core contract.
Mobile and browser adapters reject an actual host rate outside the launch set with their typed
unsupported/reprepare-required state; they neither silently choose 96 kHz nor imply SRC. Release
qualification exercises representative dry, console, and mastering paths at all four launch rates
and removes the former mandatory 384 kHz mastering path.

## Deliverables

- Core typed rate constants/predicates and removal of the realtime-local support constant.
- Strict parse, direct typed compile, canonicalization, protocol-store, and controller diagnostic
  coverage for all four launch, all four extended, zero, and at least one unrelated nonzero rate.
- Effect descriptor validation and compiler/conformance fixtures with four required launch rows and
  optional extended rows.
- A byte-identity check for the existing eleven-file issue-002 fixture corpus and manifest; no
  fixture rename, removal, version bump, CRC change, or regeneration is expected.
- Explicit launch-versus-informational grouping in conformance APIs/reports/tests without making
  production crates depend on the harness.
- Updated `AGENTS.md`, `docs/IMPLEMENTATION_PLAN.md`, `docs/SESSION_SCHEMA_V1.md`,
  `docs/EFFECT_CONTRACT_V1.md`, `dsp-research/README.md`, applicable host/runner/release docs, and
  affected stateless issue contexts/dependencies.
- An audited inventory of every old eight-rate literal/claim. Each occurrence is removed,
  delegated to the core authority, or retained only with an adjacent historical/compatibility
  annotation. At minimum, the issue-002 fixture record, issue-006 render evidence, issue-007
  numerical evidence, and issue-011 effect-conformance record are annotated without changing their
  measured values, hashes, dates, or attempt outcomes.

## Explicit non-goals

Adding SRC; changing source-rate metadata to the launch set; deleting or rewriting compatibility
fixtures; claiming extended-rate realtime/effect/host support; adding a general rate-capability
negotiation protocol; changing BTLV field encoding or status IDs; changing render DSP, coefficients,
latency, tail, smoothing, routing, PDC, SIMD dispatch, or ABI layout; implementing downstream issue
008/010/012–026 feature deliverables; running or changing performance benchmarks; or inspecting
V1/legacy source.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix
- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Versioned TOML schema and transactional session compiler
- Transport-neutral binary control protocol
- Dual-mono builtins and metering
- Native effect runtime contract and conformance

## Hazards/decisions

Do not equate “recognized by a compatibility parser” with “supported for launch rendering.” Do not
make `SampleRateHz::new` validate engine support: source metadata and fixture headers need a lossless
carrier. Do not silently drop extended descriptor or fixture evidence. Do not rewrite historical
attempt measurements as though they were collected under the new policy; annotate their status.
Do not add a fallback from an extended session rate to 96 kHz or change sample time, latency, or
duration units.

This is a policy/contract correction, not a performance change. It has no benchmark gate and must
not rerun an issue benchmark. Compilation, tests, fixture-byte checks, policy scripts, and exact
diagnostic assertions are the evidence.

## Acceptance gates with objective measurements

1. `LAUNCH_SAMPLE_RATES` and `EXTENDED_COMPATIBILITY_SAMPLE_RATES` contain the exact sorted disjoint
   four-element sets above. Predicate truth tables cover all eight values plus 0, 32,000, and
   192,001 Hz. `PreparedRenderPlan::prepare` accepts each launch rate and rejects each extended or
   unrelated rate as unsupported before publication. No production/control crate owns a second
   literal engine-rate policy.
2. Each launch rate parses, compiles, canonical-round-trips, initializes a `SessionStore`, and can
   be the final value of an atomic protocol transaction. Each extended rate and the unrelated
   invalid rates reject parse and direct typed compile with exact code/path/message and no partial
   artifact.
3. For each rejected protocol rate, both typed and BTLV transaction paths return
   `validation_failed`, `sample_rate.unsupported_at_launch`, `$.sample_rate_hz`, and final-validation
   `operation_index == edits.len()`. Revision, canonical snapshot, authoritative compiled model,
   and reliable events are byte/structurally unchanged, and no plan is prepared or published.
4. Every effect quality with exactly the four launch rows passes. Missing any launch row fails.
   Adding any unique ordered subset or all of the four extended rows passes descriptor validation;
   duplicate, unordered, or any ninth-rate row fails. Preparation at launch rates remains covered;
   extended preparation results, if exercised, are labeled informational.
5. All existing conformance fixture bytes and `MANIFEST.tsv` bytes remain identical. Parsers and
   `PlanarBlock` accept the eight corpus rates via core predicates, but documentation and reports
   label only the first four as launch gates and the latter four as extended compatibility.
6. Workspace tests and warning-denied checks pass on native, browser Wasm scalar/SIMD, Android
   AArch64, and iOS AArch64 compile targets already established by issue 001. Existing realtime and
   dependency-boundary policy scripts pass. No new production dependency on conformance exists.
7. A repository-wide rate-policy audit excluding `target/`, `.git/`, binary fixtures, numeric values
   unrelated to sample rate, and preserved historical quotations finds no unannotated claim that
   176,400/192,000/352,800/384,000 Hz is launch-required or launch-supported. Host and release gates
   cover the four launch rates; issue 026 no longer requires a 384 kHz mastering path.
8. `git diff --check` passes. The evidence records benchmark invocation count **0** and identifies
   any pre-existing unrelated worktree changes without modifying or claiming them.

## Target matrix

The authority, session diagnostics, descriptor validation, and compatibility parsing compile for
native, iOS AArch64, Android AArch64, and browser Wasm scalar/SIMD. Runtime/device/browser execution
remains owned by the applicable host issues. This issue makes no extended-rate target claim.

## Required evidence

Exact API diff; literal/claim inventory; rate-tier truth table; session parse/compile/canonical
results; typed and BTLV rollback traces; descriptor matrix; fixture and manifest before/after
hashes; native/workspace/cross-target check logs; policy-script results; documentation/issue audit;
and an explicit benchmark invocation count of zero.

## Sol-approved implementation boundary (2026-08-21)

The four-rate launch decision is approved and ready for Terra attempt 1. The implementation is a
bounded contract correction: centralize authority, enforce session/control rejection, relax
effect-descriptor completeness from eight required rows to four required plus optional extended
rows, preserve the extended fixture corpus as informational compatibility evidence, and correct
normative host/release/issue language. It does not alter audio algorithms or run benchmarks.

## Terra attempt-1 evidence (2026-08-21)

Implemented the core `LAUNCH_SAMPLE_RATES` and `EXTENDED_COMPATIBILITY_SAMPLE_RATES` authority
with predicate truth-table coverage; render-plan preparation accepts only launch rates. Strict
parse and direct typed compilation use `sample_rate.unsupported_at_launch` at
`$.sample_rate_hz` with the frozen message. Typed transactional final-candidate validation keeps
the existing `edits.len()` sentinel and preserves revision/model/canonical snapshot; a temporary
extended edit followed by a launch final rate commits. Descriptor validation now requires all four
launch rows per declared quality and permits unique ordered extended rows. Compatibility fixture
and `PlanarBlock` parsing use the core predicates, while the byte-frozen corpus remains unchanged.

Focused core/session/protocol/effect/conformance/compiler/graph tests passed. Wasm scalar and
simd128 release builds, Android AArch64 checks, iOS AArch64 checks, `git diff --check`, and all
workspace/session/protocol/realtime/effect/graph/conformance/DSP policy checks and mutation tests
passed. SHA-256 comparison before/after found identical bytes for all eleven `.mepcm` files and
`MANIFEST.tsv`.

The complete workspace test run is currently **FAIL** on a pre-existing builtin numerical gate:
`miso_engine_builtins::tests::coherent_sustained_sines_cover_launch_and_extended_compatibility_rates`
fails at 88,200 Hz, cutoff 10 Hz, frequency 4 Hz (`residual=-94.24403629784449`, gate `<= -100 dB`).
Warning-denied workspace Clippy is also **FAIL** on the pre-existing
`clippy::needless_range_loop` in that builtin test. This attempt did not alter DSP algorithms,
coefficients, tolerances, or that loop. Benchmark invocation count: **0**.

## Sol correction/review attempt-2 checkpoint (2026-08-21)

Adversarial review found that the core predicates repeated both frozen sets as literal `match`
policies, despite the single-authority contract, and that native-effect conformance still merged
extended-rate outcomes into launch pass/fail. The bounded correction makes both predicates inspect
only the core constant arrays, makes fixture generation and the checked conformance descriptor
consume those arrays, and splits effect results into explicit `launch_gates` and
`extended_compatibility_probes`. A rate-selective fault proves that an extended preparation failure
is retained in the informational report without failing launch gates.

Coverage now checks all 16 ordered subsets of optional extended descriptor rows, multiple declared
qualities, missing launch rows, duplicates, unordered rows, and a ninth rate. Invalid typed models
also reject direct canonicalization with the exact one diagnostic. All launch rates initialize a
`SessionStore` and commit as final atomic candidates. For every extended/zero/32,000/192,001 rate,
typed-controller and complete BTLV transactions produce the same exact status, code, path, detail,
and final-validation operation index while preserving revision, canonical snapshot, authoritative
model, and reliable-event occupancy.

Focused core/session/protocol/effect-contract/effect-compiler/conformance tests pass, including the
one-million protocol mutation test. The complete nonbenchmark workspace, documentation, policy,
audit, fixture-byte, and cross-target matrix remains to be rerun after this correction checkpoint.
Benchmark invocation count: **0**.
