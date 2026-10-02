# MISO Control BTLV v1 registry

This freezes the semantic registry referenced by [the wire specification](CONTROL_BTLV_V1.md). `R` is one mandatory field, `O` is zero or one optional field, and `R*`/`O*` are contiguous repeats with that flag. Absent optional fields are omitted, never encoded as placeholder zeroes.

## Registries

| Commands and echoed response IDs | Events |
| --- | --- |
| `0001` capabilities get; `0002` session snapshot get; `0003` session transaction apply; `0004` parameter metadata get; `0005` parameter state get; `0006` automation enqueue; `0007` transport get; `0008` transport set; `0009` telemetry configure; `000a` counters get; `000b` diagnostics get | `8001` session committed; `8002` automation canceled; `8010` transport state; `8020` meter batch; `8021` counter snapshot; `8030` diagnostic |

Status IDs are: `0` OK, `1` malformed frame, `2` unsupported version, `3` unsupported message, `4` unknown required field, `5` invalid field, `6` limit exceeded, `7` revision conflict, `8` revision exhausted, `9` request ID reuse, `10` replay expired, `11` backpressure, `12` validation failed, `13` not found, `14` unavailable, `15` time in past, `16` automation order, `17` PCM forbidden, `18` internal.

`QueueKind` is control command `1`, automation `2`, reliable response `3`, reliable event `4`, telemetry `5`, replay cache `6`. Diagnostic severity is info `1`, warning `2`, error `3`. Transport state is stopped `1` or playing `2`. Parameter enums are: value `f32=1`; domain continuous/boolean/enumeration `1/2/3`; mapping linear/logarithmic/exponential/stepped `1..4`; automation sample/block/none `1..3`; rack inserts/builtins/console `2`/`4`/`5` (decision 12, #1093: `inserts` kept the retired `dynamic` code and `console` is appended; `1` (SIMD1) and `3` (SIMD2) are retired and refused, never reallocated; `builtins` is the strip's own fixed section, appended by issue #178; the same table is the session edits' rack name and an automation target's rack); channel left/right/both `1..3`; unit dB/Hz/ms/samples/linear/ratio `1..6`. Meter component is left/right/aggregate `1..3`; meter flags are valid/clipped/held bits `0..2`. All unallocated bits reject.

## Common forms

`Diagnostic` is `1:code UTF8 R`, `2:severity U8 R`, `3:path-segment MESSAGE R*`, `4:detail UTF8 O`, `5:operation-index U32 O`, `6:sample U64 O`, `7:provider-sequence U64 O`. A path segment has tag `1:U8 R` and exactly one variant: field name `2:UTF8 O`, index `3:U64 O`, or stable ID `4:UTF8 O`. Provider diagnostic pages/events require sequence.

`Backpressure` is queue kind `1:U8 R`, capacity `2:U64 R`, pre-attempt occupancy `3:U64 R`, requested slots `4:U16 R`, queue generation `5:U64 O`, retry sample `6:U64 O`, requested bytes `7:U64 O`, available bytes `8:U64 O`. Every non-OK response instead has diagnostics `1:MESSAGE R*`, omitted-diagnostic count `2:U32 R`, and backpressure `3:MESSAGE O` exactly when the status is backpressure.

## Commands and success responses

| ID | Command fields | Success response fields |
| ---: | --- | --- |
| `0001` | empty, revision-any | v1 min/max, effective frame/TLV/string/depth limits, batch/page/transaction limits, all queue/replay caps, density/quantum, packed command/event IDs, capability flags |
| `0002` | `1:offset U64 R`, `2:max-bytes U32 R` | `1:total U64 R`, `2:offset U64 R`, `3:canonical-JSON BYTES R`, `4:eof BOOL R` |
| `0003` | `1:SessionEdit MESSAGE R*`; exact revision, nonempty | `1:applied-operations U32 R` |
| `0004` | `1:after-handle U32 R`, `2:limit U16 R` | `1:last-handle U32 R`, `2:eof BOOL R`, `3:ParameterDescriptor MESSAGE R*` |
| `0005` | `1:handles PACKED_U32 R` (1–256 sorted unique nonzero) | observed sample `1:U64 R`, count `2:U16 R`, stride `3:U16=16 R`, records `4:BYTES R` |
| `0006` | count `1:U16 R`, stride `2:U16=32 R`, records `3:BYTES R`; exact revision | accepted count `1:U16 R`, occupancy `2:U64 R`, capacity `3:U64 R`, generation `4:U64 R` |
| `0007` | empty | transport snapshot below |
| `0008` | state `1:U8 R`, absolute position `2:U64 O`; exact revision | transport snapshot below |
| `0009` | meter handles `1:PACKED_U32 R`, meter period `2:U32 R`, counter IDs `3:PACKED_U32 R`, counter period `4:U32 R`, diagnostics enabled `5:BOOL R`, minimum severity `6:U8 R`; exact revision | canonical echo of those six fields |
| `000a` | all `1:BOOL R`, IDs `2:PACKED_U32 O` only when not all | observed sample `1:U64 R`, `CounterValue MESSAGE R*` |
| `000b` | after-sequence `1:U64 R`, limit `2:U16 R`, minimum severity `3:U8 R` | last sequence `1:U64 R`, eof `2:BOOL R`, diagnostic `3:MESSAGE R*` |

Snapshot continuations, metadata pages, and diagnostic pages use an exact revision after an any-revision first page. Counter values are `1:id U32 R`, `2:value U64 R`, ascending and non-resetting. A parameter-state record is `{handle:u32, flags:u32, value:f32, reserved:u32}`; a meter record is `{handle:u32, component:u16, flags:u16, value:f32, reserved:u32}`. Both fixed arrays are validated schema fields, not opaque bytes.

`ParameterDescriptor` fields are handle, track stable ID, rack, effect stable ID, stable parameter ID, channel, value kind, unit, domain, optional continuous min/max, default, mapping, automation rate, smoothing samples, flags, optional display fields, and optional enum choices. Handles are nonzero, revision-scoped, and strictly increasing. Continuous bounds are finite and include the default; boolean defaults are zero/one; enumerations have unique finite choices and a matching default. Descriptor flags are readable/automatable/per-channel bits `0..2`; state flags are valid/automation-active bits `0..1`.

## Events and automation

Reliable `8001` contains sequence, origin request ID, previous revision, and operation count. Reliable `8002` contains sequence, origin request ID, canceled count, cancellation reason, automation queue generation, and optional effective sample. Reliable `8010` contains sequence, state, position, effective sample, and optional origin request ID. Lossy `8020` contains observed sample plus the fixed 16-byte meter array. Lossy `8021` contains observed sample plus ascending counter values. Reliable `8030` contains one diagnostic with provider sequence. Events have header request ID zero; reliable events have endpoint-monotonic payload sequence.

An automation record is exactly 32 bytes: `kind:u8, flags:u8=0, reserved:u16=0, handle:u32, start:u64, end:u64, start:f32, end:f32`. Kinds are point/step/linear/exponential `1..4`. Point has equal time/value; segments have `end > start`; exponential endpoints are nonzero and same-sign. Values are finite and pass the descriptor domain. Records are ordered by `(start, handle)` with no overlapping/duplicate records per handle. A segment applies on `[start,end)` and holds its end value until replaced.

## Session edit registry

`SESSION_TRANSACTION_APPLY` carries each edit as `1:opcode U16 R`, `2:payload MESSAGE R`. Payload fields begin at one in the listed order. The mapping is one-to-one with [Session schema v1](SESSION_SCHEMA_V1.md), never TOML, JSON Patch, a string path, or a private-field mutation.

| Opcode | Variant | Payload fields |
| ---: | --- | --- |
| `0001` | set session ID | session ID |
| `0002` | set sample rate | sample rate Hz |
| `0003` | set quantum | quantum frames |
| `0004`–`0005` | set render profile, output profile | one complete respective value |
| `0007` | set console | one complete console declaration |
| `0100` | upsert source | source |
| `0101` | remove source | source ID |
| `0103` | set source content and complete shape | source ID, content, channels, bit depth, frames |
| `0200` | upsert track | track |
| `0201` | remove track | track ID |
| `0202` | set source assignment | track ID, source ID, left/right source channel |
| `0203`–`0204` | set builtins; set rack | track ID plus builtins; track ID, rack name, rack |
| `0205` | put effect | track ID, rack name, final position, effect |
| `0206`–`020c` | remove/order effect; set identity/quality/bypass/link/sidechain | track ID, rack name, effect ID plus the respective replacement; order repeats effect ID |
| `020d`–`0210` | upsert/remove effect parameter; set fader/matrix | track ID, rack name, effect ID plus parameter or parameter ID/channel; track ID plus fader/matrix |
| `0211` | set track console | track ID, then the complete ordered console entries (repeated) |
| `0300`–`0301` | upsert/remove submix | submix; submix ID |
| `0400`–`0401` | upsert/remove output | output; output ID |
| `0500`–`0505` | upsert/remove route; set source/destination/matrix/gain | route or route ID plus the respective replacement |
| `0600`–`0603` | upsert/remove automation; set target/segments | automation or automation ID plus target; ordered repeated segment |

There are exactly 41 allocated opcodes. Issue #241 retired three: `0006` (set limits), `0102` (set source sample rate) and `0104` (set source mapping), whose subjects left the schema with the `limits` table and the nested source mapping/region. Retired codes are **never reallocated** -- a v1 peer that spells one must be refused, not reinterpreted, which is why `SessionEditOpcode::from_raw` returns `None` for them and a conformance row pins that. A successful atomic transaction replaces the typed `SessionModel`, immutable control-plane `CompiledSession`, and revision together; its canonical snapshot is the committed canonical JSON, never a compiled/render-plan serialization.

The nested model registry is: render/output profile `1:id,2:mode/channels,3:sample-format`; source `1:id,2:content,3:channels,4:bit-depth,5:frames`; console `1*:pre-insert slot,2*:post-insert slot`; console slot `1:slot,2:identity,3:quality,4:link-mode` (no sidechain field); track `1:id,2:source,3/4:channels,5:builtins,7:inserts,9:fader,10:matrix/pan,11*:console entry`; submix `1:id,2:builtins,3*:console entry,4:inserts,5:fader,6:matrix/pan` (field 6 is tagged exactly as the track's field 10, #1199; field 3 repeats the track's field-11 console entry, #1202); console entry `1:slot,2:bypass,3*:parameter`; effect `1:id,2:identity,3:quality,4:bypass,5:link-mode,6*:parameter,7:sidechain`; route `1:id,2:source,3:destination,4:matrix,5:gain`; automation `1:id,2:target,3*:segment`. A track route source's tap codes are input `1`, post_input `2`, insert_send `3`, insert_return `4`, pre_fader `5`, post_fader `6`, post_pan `7`. Tagged nested values use field `1:kind`; unknown enum/tag values reject and allocated codes never renumber within v1. The render-profile mode enum has one live code, `1` (`single_thread`); issue #1063 retired `2` (`dependency_waves`), which now refuses like any unallocated code and is never reallocated.

### Decision 12: the session console (#1093, #1094)

Owner decision 12 (`docs/rulings/engine-footprint-2026-09-29.md`, "Wire identity") is an in-place v1 amendment on the #1063 precedent: nothing is renumbered, every retired code is refused and never reallocated, appended codes take the next unallocated ID, and there is no protocol major or minor increment and no `ABI_VERSION` bump. The prelaunch identity stays v1.

- **Renamed in place.** The track taps keep codes `1`–`7` under the spellings above (they were `input`, `post_input_builtins`, `post_simd1`, `post_dynamic`, `post_simd2_pre_fader`, `post_fader`, `post_matrix`). Rack code `2` is `inserts` (was `dynamic`), with the same semantics, and track field `7` carries it.
- **Retired.** Rack codes `1` (`simd1`) and `3` (`simd2`) refuse at decode in every rack-addressed edit and automation target (`MALFORMED_FRAME`). Track fields `6` (`simd1`) and `8` (`simd2`) refuse in either flag form: a mandatory one is `UNKNOWN_REQUIRED_FIELD`, and an optional one is `MALFORMED_FRAME`, never skipped as an unknown optional field would be.
- **Appended.** Rack code `5` (`console`); track field `11` (console entries) and the console, console-slot and console-entry messages; opcodes `0007` (set console) and `0211` (set track console). The session model's own root field for the console is `15`.

Console addressing. A console slot's `slot` ID is session-level and unique across both sections; every strip, track or submix (#1202), carries one entry per slot, in slot order.

- `0007` replaces the whole declaration: both sections and each slot's identity, quality and link mode. It is the only edit that changes the slot set.
- `0211` replaces one track's whole entry array. It edits that track's knobs and cannot change the slot set: an entry array that adds, drops or reorders a slot refuses at final validation.
- `020a` (bypass), `020d` (upsert parameter) and `020e` (remove parameter) with rack `console` edit one track's entry, with the slot ID as the effect ID; a slot the track does not carry is `session.edit.not_found`.
- `0204`–`0209`, `020b` and `020c` (set rack, put, remove, order, identity, quality, link mode, sidechain) with rack `console` are refused as `VALIDATION_FAILED` with diagnostic `session.edit.console_slot_fixed`, whatever the model holds. A track cannot add, remove or reorder a slot, a slot's declaration is the session's, and a slot has no sidechain. With rack `builtins`, every rack-addressed edit is `session.edit.not_found` (#178).

A transaction that changes the slot set (adds, removes, renames or reorders a slot) must rewrite every strip's entries in the same transaction: every track's with `0211` or `0200`, and every submix's with `0300` (#1202). The store validates only the final candidate, so the rewrite may follow the declaration in any order; a transaction that skips a strip fails final validation with that strip's `console.entry_missing`, `console.entry_order` or `reference.missing_entity` (at `$.tracks[<i>].console` or `$.submixes[<i>].console`), and is refused whole. Nothing is committed and nothing is acknowledged: the refusal is decided before the response is written, as for every transaction. A declaration change that keeps the slot sequence (a quality, link mode or identity change, or moving a slot across the section boundary without reordering it) leaves every entry valid and needs no rewrite; effect preparation judges the parameters against a changed identity, as it does for `0208` on an insert.
