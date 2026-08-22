# 080 Effect state migration registry and bounded chains

## Outcome and status

Add explicit deterministic old-layout migration to the accepted current-layout state envelope
without changing effect runtime traits or permitting migration on render. **SOL XHIGH BRIEF PASS /
READY FOR SOL HIGH ATTEMPT 1.** One coherent Sol High implementation attempt and one bounded Sol
High correction are allowed; Sol XHigh adversarially reviews each. A second failed implementation
pass stops and requires rescope/rebrief. Workload, benchmark, timed, audit, browser and fuzz
execution counts remain exactly zero.

Remote Issue 80 was read-only verified open with the exact title on 2026-08-22 and has no comments.
Its short original body states the correct outcome but is not implementation-complete. Root must
synchronize this corrected local decision record only at the eventual CI-conscious batch delivery;
this checkpoint does not claim remote synchronization.

## Readiness correction and smallest closable product

Issue 082 descriptor identity covers the complete canonical descriptor wire, including
`state_layout_version`. Therefore an old-layout envelope cannot be verified with the current
factory's Issue-079 bound descriptor token: its historical descriptor identity is necessarily a
different registry key. A registry containing only `(current_descriptor_identity,
from_layout_version)` plus an unbound step is not implementable against the accepted APIs.

The bounded correction is:

- effect-package adds a structural/digest-only selector for a canonical Issue-079 envelope. It
  returns only descriptor identity and nonzero layout version, performs zero descriptor passes and
  preserves Issue 079's limits and diagnostic order;
- every registry edge owns exact, unforgeable historical source and target
  `BoundEffectDescriptorWireV1` tokens plus one control-plane migration step;
- an edge transforms only the three opaque payload sections. Compiler preserves the complete saved
  prepare replay and uses the target token plus the accepted Issue-079 encoder to produce the next
  complete canonical envelope;
- every source and generated target envelope is fully verified against its edge token before the
  next edge, and the final envelope is verified/restored only through the exact current
  `WireBoundNativeEffectFactoryV1` or by-value `UnpublishedEffectBankStateV1`; and
- resolution is a unique forward map, never graph search: one source key has at most one edge and
  every edge is exactly `N -> N+1`.

This is the smallest useful launch vertical: zero-step current restore, one-step migration and a
two-step bounded chain for scalar and bank-member destinations. Registry persistence, arbitrary
descriptor evolution and qualification breadth remain successors.

## Frozen wire, identity and compatibility

There is no migration wire and no serialized registry. Input, every intermediate and final output
are the unchanged canonical Issue-079 V1 envelope. Its 224-byte header, 16-byte initial records,
digest, offsets, limits and diagnostics remain byte-for-byte frozen. No old envelope is rewritten
in place. `EffectDescriptorIdentityV1` is the exact Issue-082 domain-separated identity; it is not
recomputed from effect-ID text and is never treated as factory provenance.

The registry key is exactly the source identity/layout selector carried by an opaque
`BoundEffectStateMigrationEdgeV1`. The package constructs that edge only from source and target
bound descriptor tokens. Both tokens must already have passed the sole Issue-082 static/wire
binding operation; migration performs no descriptor revalidation or identity recomputation.
Source and target descriptors must have:

- nonzero layouts with `target_layout == source_layout + 1` under checked `u32` arithmetic;
- different descriptor identities;
- bit-identical effect ID, display name, contract major/minor, supported link modes, ordered
  parameter descriptors and ordered port descriptors; and
- the same ordered quality rows, with every quality-row field equal except `maximum_state`, which
  may change.

All `f32` compatibility comparisons use `to_bits()`. Thus only the layout version and declared
common/left/right state sizes may change. Parameter schema/defaults, ports, quality/rate,
latency/tail and scratch rules cannot hitchhike on this issue. Zero-layout descriptors are rejected
by the accepted Issue-011/082 binding before registry construction; the registry never accepts an
unbound raw version.

Compiler preserves across every edge: effect ID, contract pair, sample rate, quantum, quality,
bypass, link, sidechain kind/ID/required bit, exact ordered initial rows including value bits and all
three saved prepare-request caps. Target latency, tail, scratch, automation and state sizes are
derived from the target descriptor and invariant replay, never copied from source metadata or
supplied by the step.

## Narrow package seams

Add only these allocation-free effect-package operations/types in `state.rs`; names/lifetimes may
vary mechanically:

- `inspect_effect_state_selector_v1(bytes, state_limits) -> EffectStateSelectorV1`, returning exact
  descriptor identity and nonzero state-layout version after the complete Issue-079
  limits/header/reserved/length/enum/text/order/initial-finite/digest phases, with no bound
  descriptor and zero Issue-082 passes; and
- `validate_effect_state_replay_configuration_v1(verified_state, replay)`, comparing only the
  invariant saved replay fields listed above. It deliberately does not require the verified
  state's historical descriptor identity/layout/prepared resource metadata to equal the current
  descriptor;
- `bind_effect_state_migration_edge_v1(source_bound, target_bound)`, returning private-field
  `BoundEffectStateMigrationEdgeV1` only after the exact adjacent/identity/descriptor-compatibility
  checks above. Its getters expose source/target selector and bound tokens, never raw static
  descriptor construction; and
- `effect_state_descriptor_provenance_v1(bound)`, returning a private-field Copy/Eq token over the
  exact static descriptor pointer and descriptor identity. Compiler uses it only to bind a resolved
  chain to the terminal factory/bank capability.

The edge constructor returns a small closed package error enum for nonadjacent layout, unchanged
identity, effect/contract mismatch or other replay-descriptor incompatibility; compiler maps those
in order to Registry details `1..4`. It performs comparisons over already accepted tokens and is
not another Issue-082 validation/identity pass.

Refactor the existing parser only as needed to share the structural work. Existing
`verify_effect_state_v1`, current-layout validation, encode bytes, digest, diagnostics and exact
first-error behavior must remain unchanged. Differential mutation tests freeze that equivalence.
Effect-package must not depend on effect-compiler or contain a migration registry.

## Registry and migration-step APIs

Effect-compiler owns these control-plane types:

```text
EffectStateMigrationStepV1: Send + Sync
  scratch_bytes() -> u64
  migrate(source_layout, target_layout,
          StatePayloadInput, StatePayloadOutput, algorithm_scratch)
          -> Result<EffectStateMigrationStepReportV1,
                    EffectStateMigrationStepFailureV1>

EffectStateMigrationStepReportV1
  16-byte repr(C)
  { common_bytes: u32, left_bytes: u32, right_bytes: u32, reserved: u32 = 0 }

EffectStateMigrationStepFailureV1
  repr(u32) { Rejected = 1 }

EffectStateMigrationRegistrationV1::new(
  BoundEffectStateMigrationEdgeV1,
  Arc<dyn EffectStateMigrationStepV1>)

StateMigrationRegistryV1::new(
  maximum_entries: u32,
  registrations: Box<[EffectStateMigrationRegistrationV1]>)

resolve_effect_state_migration_v1(
  registry, &WireBoundNativeEffectFactoryV1, envelope,
  EffectStateLimitsV1, EffectStateMigrationAdmissionV1,
  EffectStateRestoreAdmissionV1)
  -> ResolvedEffectStateMigrationV1
```

`EffectStateMigrationAdmissionV1` contains inclusive
`maximum_chain_steps: u32`, `maximum_intermediate_envelope_bytes: u64` and
`maximum_migration_scratch_bytes: u64`. Zero steps is valid only when the source selector already
equals the current bound identity/layout. The envelope maximum must be nonzero for a nonempty
chain. Zero migration scratch is valid only when every target payload and step scratch is zero; all
byte maxima must fit `usize` and `isize::MAX` before use.

The step's `scratch_bytes()` is called exactly once during registration and the checked value is
stored; it is never a data-dependent runtime query. `migrate` receives exact-size source and target
payload sections plus only its exact algorithm-scratch prefix. It never sees an envelope output,
factory, processor or bank. On success it initializes every target payload byte deterministically.
Its success report contains exact common/left/right written-byte counts, which must equal the target
section sizes. Before a call, compiler fills the target payload with `0xa5`; on failure it diagnoses
any changed byte, while algorithm scratch is always disposable. A faulty partial-writing step is
nevertheless contained because no canonical envelope or destination ownership is published.

Resolution verifies the source structural selector, follows the single outgoing edge per exact key
and stops only at the exact current identity/layout. It allocates at most
`maximum_chain_steps` bounded edge references plus the bounded invariant owned
`EffectBankPreparationV1` reconstructed from the fully verified historical source, all off render.
It returns exact
`EffectStateMigrationWorkspaceRequirementsV1`:

- chain step count;
- maximum target-envelope bytes assigned to the first ping-pong buffer;
- maximum target-envelope bytes assigned to the second ping-pong buffer, zero for fewer than two
  steps;
- maximum over all edges of `target_payload_bytes + stored_step_scratch_bytes`; and
- final initial-value scratch slots/bytes for scalar restore.

Every addition, multiplication, `u64`/`u32` conversion and host slice conversion is checked.
Target envelope/payload sizes come from `effect_state_v1_requirements` and target expected metadata
using the invariant replay; a step cannot declare them. Resolution admits all exact workspace and
final target prepared resources against `EffectStateRestoreAdmissionV1` before any step or
factory/payload hook.

Historical state sizes are parse/verification inputs, not current destination allocation authority.
Before a step hook, do not apply the existing current-layout admission helper to historical prepared
sizes. Instead require the source replay's rate/quantum and saved request caps to fit the current
admission, derive final state/scratch/automation from the exact current bound descriptor plus the
invariant replay, and admit those derived current resources. The existing Issue-079 restore repeats
full admission against the final current envelope after migration.

Execution accepts a consumed resolved chain, exact current capability and caller-owned first/second
envelope workspaces, migration scratch and scalar initial-value scratch where applicable. It passes
only exact prefixes, preserves every oversized suffix, alternates the envelope buffers, writes
migrated payload only into disposable scratch, and calls the Issue-079 encoder only after a step
succeeds. Each encoded target is then structurally/digest/identity/current-layout/replay verified
with that edge's target token before it can become the next input.

Freeze these two terminal operations:

- `restore_scalar_effect_state_with_migration_v1` consumes the exact
  `ResolvedEffectStateMigrationV1`, `WireBoundNativeEffectFactoryV1`, first/second envelope
  workspaces, migration scratch and scalar initial scratch, and returns the existing
  `RestoredScalarEffectStateV1` only after all edges and the existing current-layout restore
  succeed; and
- `restore_unpublished_effect_bank_track_state_with_migration_v1` consumes the existing by-value
  resolved chain and `UnpublishedEffectBankStateV1`, plus track index and the three migration
  workspaces, and returns the bank capability only after the selected member succeeds.

State and both admission policies are copied into the resolved object and cannot be substituted at
execution. Zero-step execution requires empty migration workspaces; scalar initial scratch remains
the exact accepted Issue-079 requirement.

The resolved object privately captures the current selector and opaque static-descriptor/identity
provenance plus a clone of the exact injected factory `Arc`. Selector/provenance and `Arc::ptr_eq`
must all match the terminal capability before execution. The final edge target must match that
current provenance, and the final envelope is verified once more with the capability's current
token. No ID re-resolution, unbound factory, program-key-as-provenance or caller-owned mutable
publishable bank is accepted.

## Diagnostics

Do not extend Issue 079's frozen codes `0..16`. Add a migration-only 56-byte `repr(C)` diagnostic:

```text
u32 code
u32 detail
u32 item_index
u32 reserved = 0
u64 required_bytes
EffectStateDiagnosticV1 nested_state
```

`item_index` is the registration input index during construction or chain step index during
resolution/execution; unavailable is `u32::MAX`. `nested_state` preserves the complete exact
Issue-079 diagnostic only for State/Restore codes. Otherwise it is canonical Ok with zero detail,
unavailable item/offset, zero reserved/required. Migration codes are:

`Ok=0, Limit=1, BufferTooSmall=2, Registry=3, Chain=4, Step=5, State=6, Restore=7, Overflow=8`.

Details are frozen:

- Limit: registry entries, chain steps, intermediate envelope bytes, migration scratch bytes =
  `1..4`;
- BufferTooSmall: first envelope, second envelope, migration scratch, scalar initial values =
  `1..4`;
- Registry: nonadjacent layout, unchanged identity, effect/contract mismatch, incompatible replay
  descriptor, duplicate source key = `1..5`;
- Chain: missing edge, downgrade/source newer than current, wrong terminal identity = `1..3`;
- Step: execution rejected, output changed on failure, returned/incomplete output contract = `1..3`;
- State: source, intermediate target, final current target = `1..3`; and
- Restore: scalar, bank member = `1..2`.

Overflow detail is the phase ordinal: registry, resolution/workspace, scalar initial bytes = `1..3`.
Free-form step/factory strings never become canonical diagnostics.

First-error order is fixed. Registry construction checks entry cap/host fit, then registrations in
caller order: adjacency, identity, effect/contract, replay compatibility, stored scratch host fit,
duplicate. Resolution checks migration/state limits, source structural/digest selector, downgrade
or same-layout wrong identity, and exact source registration/full historical validation, then walks
edges in order: chain cap, missing edge, target requirements, envelope cap, scratch cap, exact
terminal. It next checks the
invariant replay and current restore admission/derived resources. Execution preflights first buffer,
second buffer, migration scratch and scalar initial scratch before a migration hook; validates bank
index/config/program key/provenance before a bank migration hook; then processes each edge as source
verification, step, canonical encode, target verification and replay equality. Existing final
restore diagnostics are nested last. Dual faults must freeze these priorities.

Any package diagnostic from source verification is State detail 1; target requirements, encode or
intermediate verification is State detail 2; and the final verification with the current token is
State detail 3. If a step returns failure after changing its `0xa5` target payload, detail 2 wins
over rejected detail 1. A success report with any wrong section count is detail 3.

## Resource, atomicity and scalar/bank semantics

All work is control-plane and requires quiescent ownership. Resolution/allocation/migration/restore
must be statically absent from render-callable crates and paths. Registry allocation and bounded
replay/chain ownership may allocate off render; no step, factory prepare or payload hook runs until
all caller workspace and destination admission checks pass.

The source envelope is immutable. Intermediate envelope buffers and migration scratch are caller
owned and disposable. No migrated envelope is returned as publishable product: only the existing
Issue-079 unpublished scalar or bank capability may be returned. Any selector, registry, limit,
step, encode, intermediate verification, replay, factory or payload failure returns no scalar
processor and consumes/drops the by-value bank capability. A sentinel live processor/bank supplied
only to the test harness remains bit-identical. Existing Issue-079 partial-restore-hook disposal is
not weakened.

The migration path is destination-independent. For identical current bound descriptor and replay,
one old envelope must migrate to byte-identical final canonical state before either scalar or bank
member restore. Portable unequal common/left/right fixtures prove old scalar snapshot -> migrated
bank member and old bank-member snapshot -> migrated scalar continuation. Sequential successful
bank member restores continue to consume/return one capability; any failed member migration or
restore returns none and cannot affect an already published live bank. Zero-step calls invoke no
migration step and are diagnostically/semantically equivalent to the accepted Issue-079 direct
restore.

## Product-closing gates and checkpoints

Checkpoint 1 freezes package selector/configuration seams and registry construction/resolution:

- every malformed Issue-079 row returns the same nested diagnostic through the selector path, and
  selector/current verification differ only at descriptor binding;
- zero descriptor passes for selector, exactly one pass per token construction and no pass during
  registry use;
- exact compatible one/two-step registrations; duplicate, nonadjacent, identity, contract and each
  forbidden descriptor-field mutation reject deterministically;
- zero-step, missing edge, same-layout wrong identity, downgrade, chain-cap, envelope/scratch cap,
  overflow/host-fit and exact workspace requirements; and
- package/compiler dependency direction and no render reachability.

Root must commit that focused-green exact-path tranche before checkpoint 2 begins.

Checkpoint 2 adds terminal scalar/bank execution and portable fixtures:

- zero-step, one-step and two-step success with nonempty unequal common/left/right payloads and
  bit-exact deterministic next snapshot/continuation;
- exact, one-short and oversized first/second envelope, migration scratch and scalar initial
  workspaces with pre-hook call counts and untouched suffixes;
- step reject/partial-write/incomplete-output, malformed/wrong-identity intermediate, replay change,
  final current validation, scalar factory prepare/metadata/payload failure and bank
  index/config/key/provenance/payload failure;
- zero factory prepare before complete migration; no returned scalar destination on failure;
  by-value bank disposal on every failure; serial member restore and unrelated-member isolation;
  and
- scalar-to-bank and bank-to-scalar parity using the same historical/current descriptor chain.

Root commits checkpoint 2 before qualification. The closing tranche adds exact docs and a narrow
policy/static checker, then runs focused package/compiler tests, existing Issue-079 regressions,
locked checks for the two packages, warning-denied Clippy, rustdoc, format, dependency/realtime/
workspace policies plus mutations, scalar-Wasm package compilation, script syntax and artifact/
render-reachability scans. Root then runs one clean locked nonbenchmark workspace seal. No
benchmark, timing, workload, audit-main, browser run or fuzz execution is authorized. Broad fuzz,
allocation accounting, process/target matrices and the sole later benchmark remain Issue 081.

## Allowed paths and non-goals

Allowed implementation paths are limited to:

- `crates/miso-engine-effect-package/src/{state,lib}.rs` and narrow state-vector regression tests;
- `crates/miso-engine-effect-compiler/src/{migration,prepare,lib}.rs` and one migration test module;
- `docs/EFFECT_STATE_MIGRATION_V1.md`;
- an optional small `fixtures/effect-state-migration/v1/` portable-mock corpus;
- one narrow migration check script and the minimum direct workspace/realtime policy mutation; and
- direct Cargo manifest/lock changes only if mechanically required without a new dependency.

No change to the Issue-079 envelope wire/digest/diagnostic, Issue-082 descriptor wire/identity/C
ABI, Issue-011 runtime traits, production DSP, session schema, graph/render plan, package/CID,
factory registry, current snapshot APIs, fuzz targets or benchmark tooling. No implicit migration,
downgrade, skipped edge, arbitrary graph search, in-place old-envelope rewrite, live processor/bank
mutation, registry serialization, trust/signature claim, broad qualification, benchmark or timing.

## Dependencies by exact issue title

- Prepared effect state envelope and transactional current-layout restore
- Close canonical effect descriptor wire, identity, and C inspection ABI
- Native effect runtime contract and conformance

Acceptance unblocks **Canonical effect interchange qualification, fuzzing, and benchmark** without
importing that issue's work here.
