# MISO Control BTLV v1 registry

This freezes the semantic registry referenced by [the wire specification](CONTROL_BTLV_V1.md). `R` is one mandatory field, `O` is zero or one optional field, and `R*`/`O*` are contiguous repeats with that flag. Absent optional fields are omitted, never encoded as placeholder zeroes.

## Registries

| Commands and echoed response IDs | Events |
| --- | --- |
| `0001` capabilities get; `0002` session snapshot get; `0003` session transaction apply; `0004` parameter metadata get; `0005` parameter state get; `0006` automation enqueue; `0007` transport get; `0008` transport set; `0009` telemetry configure; `000a` counters get; `000b` diagnostics get | `8001` session committed; `8002` automation canceled; `8010` transport state; `8020` meter batch; `8021` counter snapshot; `8030` diagnostic |

Status IDs are: `0` OK, `1` malformed frame, `2` unsupported version, `3` unsupported message, `4` unknown required field, `5` invalid field, `6` limit exceeded, `7` revision conflict, `8` revision exhausted, `9` request ID reuse, `10` replay expired, `11` backpressure, `12` validation failed, `13` not found, `14` unavailable, `15` time in past, `16` automation order, `17` PCM forbidden, `18` internal.

`QueueKind` is control command `1`, automation `2`, reliable response `3`, reliable event `4`, telemetry `5`, replay cache `6`. Diagnostic severity is info `1`, warning `2`, error `3`. Transport state is stopped `1` or playing `2`. Parameter enums are: value `f32=1`; domain continuous/boolean/enumeration `1/2/3`; mapping linear/logarithmic/exponential/stepped `1..4`; automation sample/block/none `1..3`; rack SIMD1/dynamic/SIMD2/builtins `1..4` (`builtins` is the strip's own fixed section, appended by issue #178 so no existing code moved); channel left/right/both `1..3`; unit dB/Hz/ms/samples/linear/ratio `1..6`. Meter component is left/right/aggregate `1..3`; meter flags are valid/clipped/held bits `0..2`. All unallocated bits reject.

## Common forms

`Diagnostic` is `1:code UTF8 R`, `2:severity U8 R`, `3:path-segment MESSAGE R*`, `4:detail UTF8 O`, `5:operation-index U32 O`, `6:sample U64 O`, `7:provider-sequence U64 O`. A path segment has tag `1:U8 R` and exactly one variant: field name `2:UTF8 O`, index `3:U64 O`, or stable ID `4:UTF8 O`. Provider diagnostic pages/events require sequence.

`Backpressure` is queue kind `1:U8 R`, capacity `2:U64 R`, pre-attempt occupancy `3:U64 R`, requested slots `4:U16 R`, queue generation `5:U64 O`, retry sample `6:U64 O`, requested bytes `7:U64 O`, available bytes `8:U64 O`. Every non-OK response instead has diagnostics `1:MESSAGE R*`, omitted-diagnostic count `2:U32 R`, and backpressure `3:MESSAGE O` exactly when the status is backpressure.

## Commands and success responses

| ID | Command fields | Success response fields |
| ---: | --- | --- |
| `0001` | empty, revision-any | v1 min/max, effective frame/TLV/string/depth limits, batch/page/transaction limits, all queue/replay caps, density/quantum, packed command/event IDs, capability flags |
| `0002` | `1:offset U64 R`, `2:max-bytes U32 R` | `1:total U64 R`, `2:offset U64 R`, `3:canonical-TOML BYTES R`, `4:eof BOOL R` |
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
| `0004`–`0006` | set render profile, output profile, limits | one complete respective value |
| `0100` | upsert source | source |
| `0101`–`0104` | remove source; set source rate, content, mapping | source ID; then source ID plus replacement value |
| `0200` | upsert track | track |
| `0201` | remove track | track ID |
| `0202` | set source assignment | track ID, source ID, left/right source channel |
| `0203`–`0204` | set builtins; set rack | track ID plus builtins; track ID, rack name, rack |
| `0205` | put effect | track ID, rack name, final position, effect |
| `0206`–`020c` | remove/order effect; set identity/quality/bypass/link/sidechain | track ID, rack name, effect ID plus the respective replacement; order repeats effect ID |
| `020d`–`0210` | upsert/remove effect parameter; set fader/matrix | track ID, rack name, effect ID plus parameter or parameter ID/channel; track ID plus fader/matrix |
| `0300`–`0301` | upsert/remove submix | submix; submix ID |
| `0400`–`0401` | upsert/remove output | output; output ID |
| `0500`–`0505` | upsert/remove route; set source/destination/matrix/gain | route or route ID plus the respective replacement |
| `0600`–`0603` | upsert/remove automation; set target/segments | automation or automation ID plus target; ordered repeated segment |

There are exactly 42 allocated opcodes. A successful atomic transaction replaces the typed `SessionToml`, immutable control-plane `CompiledSession`, and revision together; its canonical snapshot is the committed `SessionToml`, never a compiled/render-plan serialization.

The nested model registry is: render/output profile `1:id,2:mode/channels,3:sample-format`; limits `1:PCM-ring-frames,2:control-queue-messages,3:memory-bytes`; source `1:id,2:rate,3:content,4:mapping`; track `1:id,2:source,3/4:channels,5:builtins,6/7/8:racks,9:fader,10:matrix/pan`; effect `1:id,2:identity,3:quality,4:bypass,5:link-mode,6*:parameter,7:sidechain`; route `1:id,2:source,3:destination,4:matrix,5:gain`; automation `1:id,2:target,3*:segment`. Tagged nested values use field `1:kind`; unknown enum/tag values reject and allocated codes never renumber within v1.
