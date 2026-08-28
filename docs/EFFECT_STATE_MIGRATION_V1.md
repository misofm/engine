# Effect state migration V1

Effect state migration V1 is an off-render, bounded bridge between adjacent versions of the
canonical effect-state envelope. It migrates one scalar processor or one bank member and then
hands the final current-layout envelope to the transactional restore contract in
`EFFECT_STATE_V1.md`. It defines no new persistence wire, registry wire, effect identity, trust
decision, or render-time operation.

## Descriptor edges and replay

Each edge is constructed only from two accepted bound descriptor-wire tokens. The target layout
must be the checked successor of the nonzero source layout, the descriptor identities must differ,
and the descriptors must be bit-compatible. Effect ID, display name, contract pair, link modes,
ordered parameters, ordered ports, and ordered quality rows are identical. Parameter floats and
enum-choice floats compare by `f32::to_bits()`. Within a quality row, only the three
`maximum_state` section sizes may change; quality, rate, latency, tail, and scratch rules may not.

The invariant replay preserves effect ID and contract, rate, quantum, quality, bypass, link mode,
sidechain kind/ID/required bit, every ordered initial-value row including value bits, and all three
saved request ceilings. Target state sizes, scratch, automation, latency, and tail are derived from
the target bound descriptor. A structural selector reads only descriptor identity and nonzero
layout after all Effect State V1 structural and digest phases; it performs no descriptor
validation or identity construction.

The registry is a unique forward map keyed by exact source identity and layout. Edges are adjacent;
there is no downgrade, skipped version, graph search, fallback, or ID re-resolution. Construction
checks the inclusive entry cap first, then each caller entry in order: adjacency, unchanged
identity, effect/contract, descriptor compatibility, stored scratch host fit, and duplicate source.
The step scratch requirement is queried exactly once after its edge is accepted.

## Resolution and workspace

Resolution follows zero, one, or more unique adjacent edges up to the inclusive chain cap and
stops only at the exact current selector and static-descriptor provenance. It retains the exact
factory `Arc`, bound token, static pointer provenance, current effect ID, replay, state limits,
migration admission, and restore admission. Resolution invokes no migration, factory, snapshot, or
restore hook.

The caller receives exact requirements for alternating first and second envelope prefixes, the
maximum `target payload bytes + step scratch bytes`, and scalar initial-value scratch slots/bytes.
All arithmetic, conversions, caps, and host fit are checked. Oversized workspace suffixes are
untouched. For a zero-step chain, all three required migration prefixes have length zero. Empty
slices are canonical, while supplied nonempty slices are merely untouched oversized suffixes.

## Transactional execution

Execution preflights first envelope, second envelope, migration scratch, then scalar initial
scratch before a migration hook. Bank execution next checks index, replay/configuration and width,
program key, then exact provenance. Scalar and bank terminals require the retained `Arc` to be
pointer-equal and the live factory descriptor getter to remain pointer-equal to the capability's
stored static descriptor before a step.

A step sees only source/target layout numbers, exact common/left/right payload slices, and its exact
algorithm-scratch prefix. Target payload begins as `0xa5`. A rejected step that changed any target
payload byte is diagnosed before an unchanged rejection. Success must report exact common, left,
and right byte counts and zero reserved. Only then does compiler encode the canonical target
Effect State V1 envelope. Every intermediate is fully structure/digest/identity/current-layout and
replay verified under the target bound token before becoming the next source. Descriptor identity,
layout, replay, padding, metadata, and digest cannot be supplied by a step.

The final envelope is verified again with the exact current capability. Scalar execution consumes
the wire-bound factory and returns an unpublished restored processor only after the accepted
current restore succeeds. Bank execution consumes the unpublished bank by value; every failure
drops it, while success returns it for serial member restores. A live processor or published bank
is never an input and cannot be partially updated.

## Diagnostics

`EffectStateMigrationDiagnostic` is a 56-byte C-layout value: `code: u32` at byte 0, `detail:
u32` at 4, `item_index: u32` at 8, zero `reserved: u32` at 12, `required_bytes: u64` at 16, and the
exact 32-byte `EffectStateDiagnostic` at 24. Unavailable outer index is `u32::MAX`. Only State
and Restore carry a non-Ok nested diagnostic; all other nested values are canonical Ok with
unavailable nested index/offset.

Codes are `Ok=0`, `Limit=1`, `BufferTooSmall=2`, `Registry=3`, `Chain=4`, `Step=5`, `State=6`,
`Restore=7`, and `Overflow=8`.

- Limit details: registry entries `1`, chain steps `2`, intermediate envelope `3`, migration
  scratch `4`.
- Buffer details: first envelope `1`, second envelope `2`, migration scratch `3`, scalar initial
  values `4`.
- Registry details: nonadjacent `1`, unchanged identity `2`, effect/contract `3`, incompatible
  descriptor `4`, duplicate source `5`.
- Chain details: missing edge `1`, downgrade or same-layout wrong identity `2`, wrong terminal `3`.
- Step details: unchanged rejection `1`, changed output on rejection `2`, incomplete report `3`.
- State details: source `1`, intermediate target `2`, final current target `3`.
- Restore details: scalar `1`, bank member `2`.
- Overflow details: registry `1`, resolution/workspace `2`, scalar initial bytes `3`.

First-error order is registry cap then caller entries; resolution limits and structural selector,
downgrade, source binding, chain cap, missing edge, target requirements, envelope, scratch, exact
terminal, invariant replay, and current admission; execution buffers, bank destination checks,
source verification, step, canonical target verification, final current verification, and nested
Effect State V1 restore. Free-form hook or factory strings never enter canonical diagnostics.

## Realtime and portability boundary

Registry construction, resolution, allocation, migration, encoding, verification, preparation,
and restore are control-plane work requiring quiescent ownership. Migration is absent from core,
session, graph, rack, builtins, and render-callable paths. The format and algorithm are independent
of scalar versus bank execution, backend, SIMD width, and host target. There is no benchmark,
timing, workload, audit, browser, or fuzz execution requirement in this contract.
