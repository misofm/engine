# Effect state V1

Effect state V1 is the canonical, bounded envelope for one prepared native-effect member at its
current state-layout version. It is control-plane data. Snapshot and restore require quiescent
ownership and are not render-callable.

The envelope is per effect member and intentionally has no scalar, bank, backend, SIMD-width,
cohort, or session-instance discriminator. A payload may move between a scalar processor and a bank
member only when the bound descriptor, complete preparation replay, current layout, and program key
match. Backend selection and sibling replays belong to the unpublished destination capability, not
to the envelope.

## Canonical bytes

All integers are little-endian. The byte sequence is:

`224-byte header || effect_id || sidechain_port_id || minimum zero pad to 8 || initial records || common || left || right`

Each initial record is 16 bytes: `u32 parameter_index`, `u32 ParameterChannel`, raw
`f32::to_bits()` in a `u32`, and a zero `u32` reserved field. Records are complete and use
descriptor parameter order; shared parameters have `Both`, while per-lane parameters have `Left`
then `Right`. Non-finite values and negative zero reject.

Frozen enum values are:

- quality: `Draft=1`, `Normal=2`, `High=3`;
- bypass: `false=0`, `true=1`;
- link: `DualMono=1`, `Maximum=2`, `Average=3`;
- initial channel: `Left=1`, `Right=2`, `Both=3`;
- sidechain kind: `None=0`, `Unconnected=1`, `Connected=2`;
- tail kind: `Finite=1`, `Infinite=2`.

The effect ID is 1..=127 ASCII bytes, begins with `a`..`z`, and continues with lowercase ASCII
letters, digits, `.`, `_`, or `-`. A sidechain ID is zero bytes for `None` and 1..=127 bytes
otherwise, with the same grammar. `None` requires `required=0`; `Unconnected` requires a nonempty
ID and `required=0`; `Connected` requires a nonempty ID and preserves the descriptor's required
bit. Infinite tail requires zero tail samples. Strings are followed by only the minimum zero
padding needed to align the initial table to eight bytes. There is no trailing pad after the right
payload.

| Offset | Bytes | Field |
|---:|---:|---|
| 0 | 8 | ASCII `MISOEFST` |
| 8 | 2 | version `1` |
| 10 | 2 | header bytes `224` |
| 12 | 4 | zero flags |
| 16 | 8 | exact total bytes |
| 24 | 32 | bound descriptor V1 identity |
| 56 | 32 | state-envelope digest |
| 88 | 2 | contract major |
| 90 | 2 | contract minor |
| 92 | 4 | nonzero current state-layout version |
| 96 | 4 | sample rate |
| 100 | 4 | render quantum |
| 104 | 4 | quality |
| 108 | 4 | canonical bypass boolean |
| 112 | 4 | link mode |
| 116 | 4 | sidechain kind: none, unconnected, or connected |
| 120 | 4 | canonical sidechain-required boolean |
| 124 | 4 | effect-ID bytes |
| 128 | 4 | sidechain-port-ID bytes |
| 132 | 4 | initial-record count |
| 136 | 8 | latency samples |
| 144 | 4 | finite or infinite tail kind |
| 148 | 4 | zero reserved |
| 152 | 8 | finite tail samples; zero for infinite |
| 160 | 4 | common payload bytes |
| 164 | 4 | left payload bytes |
| 168 | 4 | right payload bytes |
| 172 | 4 | zero reserved |
| 176 | 8 | exact prepared scratch bytes |
| 184 | 4 | exact automation capacity |
| 188 | 4 | exact `initial_count * 16` bytes |
| 192 | 8 | replayed request state ceiling |
| 200 | 8 | replayed request scratch ceiling |
| 208 | 4 | replayed request automation ceiling |
| 212 | 4 | zero reserved |
| 216 | 8 | exact common + left + right payload bytes |

Common, left, and right are three independent opaque sections. V1 imposes no nonempty or
common-versus-lane equality rule. The accepted descriptor contract currently requires equal left
and right maximum sizes, but the envelope parser and layout arithmetic retain all three counts
independently.

The digest is:

`SHA-256("miso.engine.effect-state.current-layout.v1\0" || u64_le(total) || bytes[0..56] || 32 zero bytes || bytes[88..total])`

It detects corruption and binds every envelope byte. It is not authentication, authorization, a
signature, or a trust decision.

## Binding, snapshot, and restore

A descriptor wire is admitted once with the exact static descriptor and yields an opaque bound
token. Compiler capabilities retain that token, the exact static descriptor pointer, and the exact
injected `Arc<dyn NativeEffectFactory>`. State operations reuse the token and do not repeat
descriptor validation or identity construction.

Requirements compute exact output bytes, payload scratch bytes, and initial-value scratch slots
with checked arithmetic. Encode and snapshot accept oversized slices, use only the exact prefix,
and preserve every suffix byte. Encode preflights before writing. Snapshot preflights output and
payload scratch, lets the hook write only disposable scratch, and publishes the envelope only after
the hook succeeds. One-short output or scratch fails before the hook and leaves the complete output
unchanged.

Scalar restore consumes a wire-bound factory capability. It verifies structure, digest, descriptor
identity, and current caller admission before copying initial values or calling `prepare`. It then
reconstructs the complete saved request, validates all returned metadata including exact descriptor
pointer provenance, restores payload into an unpublished processor, and returns ownership only on
success. The live processor is never an input.

Bank preparation owns the unpublished bank, backend, width, exact factory capability, and all
sibling replays. Every sibling is admitted and validated before the one bank bind. Legal
`Ok(None)` remains a fallback result and never creates a state capability. Bank-member restore
consumes the capability by value; index, destination config/replay, program key, provenance, and
metadata are checked before the payload hook. Any failure drops the possibly partially mutated
unpublished bank. Success returns the capability, enabling serial member restores. Snapshot uses
the exact selected owned replay.

Current restore admission fixes one of the four launch rates, a nonzero exact quantum, and current
state, scratch, and automation ceilings. Saved request ceilings are historical replay inputs, not
allocation authority. Saved ceilings and derived payload/scratch/automation must fit the caller's
current policy before scalar preparation or bank restoration.

## Limits and diagnostics

Default caller limits are 4,194,304 descriptor bytes, 268,435,456 envelope bytes, 134,217,728
payload bytes, and 4,096 initial values. Caps are inclusive. All sums, multiplication, integer
conversions, host fit, and slicing are checked.

`EffectStateDiagnosticV1` is a 32-byte C-layout value: `code: u32` at byte 0, `detail: u32` at byte
4, `item_index: u32` at byte 8, zero `reserved: u32` at byte 12, `byte_offset: u64` at byte 16,
and `required_bytes: u64` at byte 24. Diagnostics follow frozen phase order: caller limits and
arithmetic; fixed header; reserved fields; structural lengths and padding; enums, text, and initial
records; digest; descriptor binding/identity; admission and replay/metadata; payload hook.
Free-form factory or hook strings never enter canonical diagnostics.

Diagnostic codes are `Ok=0`, `Limit=1`, `BufferTooSmall=2`, `Header=3`, `Length=4`, `Reserved=5`,
`Enum=6`, `Order=7`, `Text=8`, `Descriptor=9`, `Digest=10`, `Metadata=11`, `InitialValues=12`,
`Payload=13`, `Factory=14`, `Restore=15`, and `Overflow=16`. Unavailable item index is
`u32::MAX`; unavailable byte offset is `u64::MAX`.

Frozen detail values are:

- `BufferTooSmall`: envelope output `1`, payload scratch `2`, initial-value scratch `3`;
- `Descriptor`: `(kind << 16) | descriptor_wire_code`, where malformed external wire is kind `1`,
  static-versus-wire mismatch is `2`, and state-envelope identity mismatch is `3`; kinds 1 and 2
  retain the external descriptor-wire offset, while kind 3 has no offset;
- `Metadata`: effect ID `1`, contract `2`, layout `3`, rate `4`, quantum `5`, quality `6`, bypass
  `7`, link `8`, ports `9`, latency `10`, tail `11`, state sizes `12`, scratch `13`, automation
  `14`, request limits `15`;
- `Payload`: scalar snapshot `1`, bank snapshot `2`, scalar restore `3`, bank restore `4`;
- `Factory`: bound factory unavailable `1`, request invalid `2`, prepare failure or `Ok(None)` `3`,
  returned metadata mismatch `4`;
- `Restore`: bank track index `1`, replay/config mismatch `2`, program-key mismatch `3`, provenance
  mismatch `4`.

The independent standard-library oracle is `scripts/effect-state-v1-reference.py`. It authors and
verifies the sealed files in `fixtures/effect-state/v1/`, including descriptor wire/identity, binary
and hexadecimal state bytes, digest, exact malformed diagnostics, and the SHA-256 manifest. Rust
binds that independently authored descriptor wire, verifies the envelope, validates the replay,
and re-encodes every byte identically.
