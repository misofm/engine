# Sol implementation brief — issue 079 current-layout effect state

## Decision

**READY FOR TERRA ATTEMPT 1.** Deliver only the canonical current-layout envelope and transactional
unpublished restore. One Terra implementation and one bounded Sol correction are allowed; the
second failure stops. All workload/benchmark/timed/audit/browser/general-target counts stay zero.

## Implementation order

1. Replace provisional `state.rs` with the exact 224-byte header, 16-byte initial records, derived
   strings/table/payload sections, four caps and 32-byte diagnostic. Remove false left/right-size
   equality. Implement checked requirements, atomic caller-buffer encode, private borrowed verify
   and the domain-separated full-envelope digest.
2. Add one narrow Issue-082 helper that constructs an opaque `BoundEffectDescriptorWireV1` from a
   static descriptor plus canonical caller wire in one coherent validation/comparison/identity
   pass. Reuse the private borrowed semantic view, compare every semantic field/table in frozen
   order and keep token fields private. Do not encode scratch, invoke the public verifier twice,
   trust wire, leak it or extend its lifetime.
3. Define a package-owned borrowed replay/authoring view over effect-contract types. Compiler adapts
   retained `EffectBankPreparationV1`; package never imports compiler. Serialize the descriptor's
   native effect type ID, not the session effect-instance ID.
4. Add the narrow effect-compiler dependency on effect-package and wrap the descriptor token with
   the exact injected factory whose `factory.descriptor()` was bound. Snapshot consumes the exact
   retained `EffectBankPreparationV1`; never infer initial values, caps or provenance from metadata
   or program key. Preflight output and payload scratch before the hook and publish only after hook
   success.
5. Scalar restore uses the wire-bound factory capability, decodes initial rows into exact caller
   scratch, prepares a new processor, compares all metadata, restores payload and returns owned
   unpublished processor + replay. Before prepare, require exact current rate/quantum and admit
   saved request caps plus derived state/scratch/automation against caller current ceilings. Never
   accept a live processor, trust saved limits as policy or re-resolve an unbound factory.
6. Keep envelopes per bank member, but prepare an opaque capability that owns the unpublished bank,
   wire-bound factory provenance and all sibling replays. Restore consumes that capability by value
   and returns it only on success; any partial hook failure drops it. Snapshot may borrow it. Do not
   persist/reconstruct backend, width or siblings from one envelope. Portable mock gates prove the
   intentional scalar↔bank-member interchange for identical descriptor/replay/key.
7. Freeze one independent vector, exact diagnostics/canaries, scalar delay continuation and
   portable unequal-section/bank proofs. Include zero-prepare admission rejection, oversized suffix,
   partial-hook atomicity and exact diagnostic-order proofs. Update the obsolete state fuzz target
   to compile only.
8. Run focused package/compiler/effect/reference/scalar-Wasm/policy gates, then one proportional
   locked nonbenchmark workspace seal and record strict PASS/FAIL.

## Review invariants

- Descriptor identity is exactly Issue 082. The envelope digest covers every envelope byte with its
  digest field zeroed; it is not trust/authentication.
- Replay rate, quantum, quality, bypass, link, ports, complete ordered initial values and all three
  caps. Bind latency, tail, state sizes, scratch and automation from complete prepared metadata.
  Saved caps are replay, not admission authority; caller current ceilings reject before prepare.
- Common/left/right counts are independent opaque current-layout bytes.
- Exactly one Issue-082 validation/comparison/identity pass occurs when the descriptor-binding token
  is constructed. State operations reuse it without a second pass. Its temporary and accepted
  off-render factory/bank preparation are the only allocation boundaries; wire work retains none.
  An early state-envelope limits/header/length/digest rejection adds zero descriptor passes.
- Caller payload scratch makes snapshot output atomic. Restore only prepares/restores unpublished
  ownership and returns it after complete success.
- Program-key equality is never factory/descriptor provenance. A bank capability is unforgeably
  bound to the factory descriptor and owns the bank plus complete sibling replay; failed restore
  returns no capability that could be published.
- Free-form hook/factory strings map to frozen stage/detail values, never canonical text.
- Diagnostic priority uses explicit phase/subphase/traversal ordinals, not unavailable emitted
  indices. External descriptor offsets remain external; Restore owns bank index/config/key/provenance.
- No runtime trait, package/CID, migration, session schema, whole-bank or render-plan change.

## Stop conditions

STOP rather than changing accepted traits/wire/schema/DSP; inferring missing initial config;
trusting raw descriptor wire or program-key provenance; accepting caller-owned mutable bank restore;
using serialized request caps as current allocation policy; introducing a package/compiler cycle;
persisting backend/siblings; retaining a hidden envelope copy; accepting an old layout; touching
live state; weakening digest/diagnostic/canary gates; or importing Issue-080/081 breadth.

## Downstream handoff

Accepted Issue 079 unblocks **Effect state migration registry and bounded chains**. Issue 081 keeps
broad fuzzing, allocation accounting, multitarget qualification and the sole later benchmark.
