# Sol implementation brief — issue 080 bounded effect-state migration

## Decision

**SOL XHIGH BRIEF PASS / READY FOR SOL HIGH ATTEMPT 1.** Implement only deterministic adjacent
payload migration between exact historical/target descriptor bindings, followed by the accepted
Issue-079 unpublished scalar or bank-member restore. One Sol High attempt and one bounded Sol High
correction are allowed; Sol XHigh reviews each. A second failed pass stops for rescope. Benchmark,
timing, workload, audit, browser and fuzz execution counts remain zero.

## Core correction

Descriptor identity includes `state_layout_version`, so an old envelope cannot be opened with the
current factory's bound token. Add a package structural/digest selector returning `(identity,
layout)` without a descriptor pass. Each compiler registry edge stores exact historical source and
target `BoundEffectDescriptorWireV1` tokens inside a package-constructed opaque
`BoundEffectStateMigrationEdgeV1`. Verify every source/target with its own token; the final
selector and opaque static-descriptor/identity provenance must equal the exact current factory/bank
token.

There is no migration wire. A step transforms only exact common/left/right payload slices. Compiler
preserves the saved replay and canonically encodes the next unchanged Issue-079 envelope under the
target token. Source/target descriptors are identical in every field except adjacent nonzero layout
version and per-quality `maximum_state`; `f32` fields compare by bits. This excludes parameter,
port, quality/rate, latency/tail, scratch, contract and effect-ID migration.

## Implementation order

1. Refactor package parsing to expose `inspect_effect_state_selector_v1` after the complete existing
   structural/digest phases and add replay-configuration-only comparison. Preserve every existing
   byte, digest and diagnostic. Add only an opaque compatible-edge constructor and opaque
   static-descriptor/identity provenance derived from accepted bound tokens. Selector and these
   comparisons perform zero descriptor validation/identity passes.
2. Add compiler-owned `EffectStateMigrationStepV1`, exact source/target registration and bounded
   `StateMigrationRegistryV1`. The step has one stored exact `scratch_bytes()` value and receives
   only source/target layout, exact `StatePayloadInput`, exact `StatePayloadOutput` and exact
   algorithm scratch. Success reports exact common/left/right written-byte counts.
3. Resolve a unique linear chain by exact `(identity, layout)`. Reject duplicate, nonadjacent,
   unchanged-identity, incompatible-descriptor, missing, downgrade, wrong-terminal and excess-step
   rows. Compute exact alternating-envelope and `target_payload + step_scratch` workspace from the
   invariant replay before any hook. Resolve from `&WireBoundNativeEffectFactoryV1` and retain a
   clone of its exact factory Arc plus opaque descriptor provenance without borrowing the capability.
4. Freeze the separate 56-byte migration diagnostic. Preserve complete Issue-079 diagnostics as a
   nested field rather than extending codes `0..16` or translating offsets/details.
5. Execute into two caller ping-pong envelope buffers plus one disposable migration scratch. Step
   writes payload scratch; only success reaches canonical target encode. Fully verify target and
   replay before the next edge.
6. Scalar terminal restore consumes the exact `WireBoundNativeEffectFactoryV1`; bank terminal
   restore consumes the by-value `UnpublishedEffectBankStateV1`. Preflight every caller buffer,
   invariant replay, current admission/derived resources and bank index/config/key/provenance before
   a step/factory/payload hook. Return existing unpublished capability types only on total success.
7. Add portable unequal-section historical/current mocks for zero/one/two steps, exact continuation,
   scalar↔bank parity, serial bank members, one-short/canary/call-count rows and every failure stage.
8. Finish docs and narrow policies, run focused gates, checkpoint, then one clean nonbenchmark
   workspace seal. Do not run Issue-081 breadth or any benchmark/timed/workload/audit/browser/fuzz
   execution.

## Exact API and resource invariants

- `StateMigrationRegistryV1::new(maximum_entries, Box<[registration]>)` validates in caller order
  and allocates only within that cap off render.
- Registration accepts only the package's private-construction compatible-edge token and derives
  key/version/identities from it; no raw identity/version/static descriptor is trusted. A zero
  version cannot reach it through accepted binding.
- `EffectStateMigrationAdmissionV1` has inclusive maximum chain steps, intermediate-envelope bytes
  and migration-scratch bytes. A nonempty chain requires a nonzero envelope maximum; zero migration
  scratch is valid only when every exact target-payload and step-scratch requirement is zero. All
  byte maxima are host-fit.
- Resolved requirements contain chain count, exact maximum bytes for alternating first and second
  envelope buffers, exact maximum `target payload + stored algorithm scratch`, and final scalar
  initial scratch slots/bytes. All arithmetic/conversions are checked.
- Resolution calls no step/factory/payload hook. `scratch_bytes()` is captured once at registration;
  `migrate` is called exactly once per executed edge.
- Complete saved replay is invariant: type ID, contract, rate, quantum, quality, bypass, link,
  sidechain, ordered initial value bits and request caps. Target prepared resources are derived.
- Source input is immutable. Only exact workspace prefixes are passed; oversized suffixes remain
  exact. Target payload starts as `0xa5`; a failing step that changes it gets Step detail 2, while a
  success report whose three byte counts differ from the exact target sizes gets Step detail 3. It
  can never publish an envelope or destination.
- Current restore admission and derived target state/scratch/automation caps reject before a
  migration hook. Do not admit historical prepared sizes as current resources: check source
  rate/quantum/request caps, then derive and admit resources from the current target descriptor and
  invariant replay. Saved request caps remain history/replay, not allocation authority.
- The resolved current selector and opaque static-descriptor/identity provenance must match those
  derived from the consumed final capability token, and the retained factory clone must satisfy
  `Arc::ptr_eq`. State/restore/migration admission policies are stored in the resolved object and
  cannot be substituted at execution. Program key is never descriptor/factory provenance.

## Diagnostic freeze

`EffectStateMigrationDiagnosticV1` is 56-byte `repr(C)`:
`u32 code`, `u32 detail`, `u32 item_index`, zero `u32 reserved`, `u64 required_bytes`, then the exact
32-byte nested `EffectStateDiagnosticV1`. Codes are
`Ok=0, Limit=1, BufferTooSmall=2, Registry=3, Chain=4, Step=5, State=6, Restore=7, Overflow=8`.

- Limit details: registry entries, chain steps, intermediate envelope, migration scratch = 1..4.
- Buffer details: first envelope, second envelope, migration scratch, scalar initial values = 1..4.
- Registry details: nonadjacent, unchanged identity, effect/contract, compatibility, duplicate =
  1..5.
- Chain details: missing, downgrade, wrong terminal = 1..3.
- Step details: rejected, output changed on failure, incomplete output contract = 1..3.
- State details: source, intermediate, final = 1..3.
- Restore details: scalar, bank = 1..2.
- Overflow details: registry, resolution/workspace, scalar initial bytes = 1..3.

Unavailable outer index is `u32::MAX`; non-State/Restore nested diagnostics are canonical Ok with
unavailable nested index/offset. Free-form errors never become diagnostics. Use the spec's exact
registry, resolution and execution first-error phases; add dual-fault tests.

## Checkpoints and adversarial gates

Checkpoint 1 is package selector/config comparison plus registry/resolution/workspace. Focused tests
must freeze malformed diagnostic equivalence, descriptor pass counts, every compatibility field,
zero/one/two-step resolution, missing/duplicate/nonadjacent/downgrade/terminal/cap/overflow rows and
dependency/render boundaries. Pause for root's exact-path commit.

Checkpoint 2 is scalar/bank execution. Freeze zero/one/two-step continuation, unequal three-section
payloads, exact/one-short/oversized workspaces, pre-hook counts, partial step output containment,
malformed intermediate, replay change, terminal provenance, zero scalar prepare until chain success,
no failed scalar destination, by-value bank disposal, serial member isolation and both scalar↔bank
directions. Pause for root's exact-path commit.

Closing qualification is exact docs, narrow policy/static mutations, focused package/compiler and
Issue-079 regression tests, locked checks, warning-denied Clippy, rustdoc/fmt, scalar-Wasm package
compile, dependency/realtime/workspace/script/artifact scans, then one clean locked nonbenchmark
workspace seal. Invocation counters for benchmark/timing/workload/audit/browser/fuzz stay zero.

## Stop conditions

STOP rather than changing runtime traits; Issue-079 wire/digest/diagnostics; Issue-082 identity/C ABI;
parameter/port/quality/latency/tail/scratch semantics; accepting raw identities/versions; skipping
layouts; graph search; trusting program keys as provenance; exposing a migrated envelope as
publishable state; mutating live ownership; adding render reachability; importing package/CID,
session/graph/DSP, fuzz/multitarget/allocation or benchmark scope; or needing more than the two
implementation passes.

## Allowed paths

Only the package state/lib selector seam and state-vector tests; compiler
`migration.rs`/prepare/lib plus one migration test module; one exact migration doc; optional small
portable fixture corpus; one narrow checker and minimum direct policy mutation; mechanical direct
manifest/lock changes without new dependencies. No fuzz target, production effect, effect-contract,
session, graph, render, descriptor wire, C ABI or benchmark-tool change.
