# Sol implementation brief — issue 079 current-layout effect state

## Decision

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE.** Deliver only the canonical current-layout envelope
and transactional unpublished restore. Per the user override, Sol High implemented and Sol XHigh
briefed and adversarially verified. All workload/benchmark/timed/audit/browser/general-target counts
stay zero. Remote closure remains root work after the one eventual bulk `main` merge/push/CI and
GitHub evidence synchronization; neither is claimed by this local record.

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

## Completion evidence

The accepted local checkpoints are `c91fdb0` (package wire/state), `77e9bc1` (scalar restore),
`d4377cd` (bank restore) and `7bb0e0e` (reference/product qualification). The clean seal candidate
was `7bb0e0ea8d57674f6e5cbb5cdb6a74470fe9ecfa`, tree
`15074afdf345b8c3b302bda459de1b5083852d95`.

The independent standard-library fixture is a 309-byte envelope with a 224-byte header, three
16-byte initial records and payload at byte 296. Its 653-byte descriptor wire SHA-256 is
`cff6a313be6b04a8932343928a0ab69c296fadcdca88922032b217c20a81aea9`, descriptor identity is
`752552864ed6796526d1859f83795bb3facea6cc91bc1fa5e3e796ee67284ff1`, envelope digest is
`858e6db10df1b69626736bf2d5f29634866269599d0428710c95e2103dfc837f`, state-file SHA-256 is
`b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48`, and manifest SHA-256 is
`3cee39d0fd213989d81f92675085b2d90d11bf6ff1f4b4a4323b158fce8b7220`. The manifest covers all
eight listed members (nine fixture files including the manifest), and Rust binding/re-encoding
matches all 309 independently authored bytes.

The one locked broad nonbenchmark seal registered 541 workspace tests: 535 passed, six existing
ignored/manual tests stayed ignored and zero failed; doctests passed 8/8. All remaining ordered
format/check/strict-Clippy/rustdoc/reference+manifest/fuzz-compile-only/scalar-Wasm/static/policy
gates passed, including 11 policy commands and syntax checks for all 76 shell scripts. Pre/post
candidate observations and these four hashes were unchanged:

- `Cargo.lock`: `8db695d722dc2055faaac82ffebb8741bf948117fc733834de9e157ff4e31e6c`;
- `fuzz/Cargo.lock`: `af4547d5bae367e4249c6fcf482b249ff8af0ae29b9a933957d34b36ec36e5d5`;
- `fixtures/effect-state/v1/MANIFEST.sha256`:
  `3cee39d0fd213989d81f92675085b2d90d11bf6ff1f4b4a4323b158fce8b7220`; and
- `scripts/effect-state-v1-reference.py`:
  `9dc95018daa5c993c16fc10ca5185ec17193bd69d6b71a199737b110ba7f0c0e`.

Invocation counts were benchmark 0, timed 0, workload 0, audit-main 0 and fuzz execution 0; the
fuzz target was compiled/linted only. Issue 080 migration and Issue 081 broad
fuzz/allocation/multitarget/benchmark scope were not imported.

**Sol XHigh verdict: PASS. Issue 079 is complete and ready to close after root's eventual single
batch delivery, remote evidence synchronization and GitHub closure.**

## Downstream handoff

Accepted Issue 079 unblocks **Effect state migration registry and bounded chains**. Issue 081 keeps
broad fuzzing, allocation accounting, multitarget qualification and the sole later benchmark.
