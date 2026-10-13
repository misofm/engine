# MISO Control BTLV v1 registry

This freezes the semantic registry referenced by [the wire specification](CONTROL_BTLV_V1.md). `R` is one mandatory field, `O` is zero or one optional field, and `R*`/`O*` are contiguous repeats with that flag. Absent optional fields are omitted, never encoded as placeholder zeroes.

## Registries

| Commands and echoed response IDs | Events |
| --- | --- |
| `0001` capabilities get; `0002` session snapshot get; `0003` session transaction apply; `0006` automation enqueue; `0007` transport get; `0008` transport set; `0009` telemetry configure; `000a` counters get; `000b` diagnostics get; `000c` control list get; `000d` control state get | `8001` session committed; `8002` automation canceled; `8010` transport state; `8020` meter batch; `8021` counter snapshot; `8030` diagnostic |

Message IDs `0004` (parameter metadata get) and `0005` (parameter state get) are **retired** by decision 17 (C4, #1551): the decoder refuses them as `UNSUPPORTED_MESSAGE`, no capability registry lists them, and they are never reallocated (`RETIRED_MESSAGE_IDS`; a conformance row pins each). `CONTROL_LIST_GET` and `CONTROL_STATE_GET` replace them.

Status IDs are: `0` OK, `1` malformed frame, `2` unsupported version, `3` unsupported message, `4` unknown required field, `5` invalid field, `6` limit exceeded, `7` revision conflict, `8` revision exhausted, `9` request ID reuse, `10` replay expired, `11` backpressure, `12` validation failed, `13` not found, `14` unavailable, `15` time in past, `16` automation order, `17` PCM forbidden, `18` internal.

`QueueKind` is control command `1`, automation `2`, reliable response `3`, reliable event `4`, telemetry `5`, replay cache `6`. Diagnostic severity is info `1`, warning `2`, error `3`. Transport state is stopped `1` or playing `2`. Parameter enums are: rack inserts/builtins/console `2`/`4`/`5` (decision 12, #1093: `inserts` kept the retired `dynamic` code and `console` is appended; `1` (SIMD1) and `3` (SIMD2) are retired and refused, never reallocated; `builtins` is the strip's own fixed section, appended by issue #178; the same table is the session edits' rack name and an automation target's rack); channel left/right/both `1..3`; unit dB/Hz/ms/samples/linear/ratio `1..6`. Meter component is left/right/aggregate `1..3`; meter flags are valid/clipped/held bits `0..2`. All unallocated bits reject.

## Common forms

`Diagnostic` is `1:code UTF8 R`, `2:severity U8 R`, `3:path-segment MESSAGE R*`, `4:detail UTF8 O`, `5:operation-index U32 O`, `6:sample U64 O`, `7:provider-sequence U64 O`. A path segment has tag `1:U8 R` and exactly one variant: field name `2:UTF8 O`, index `3:U64 O`, or stable ID `4:UTF8 O`. Provider diagnostic pages/events require sequence.

`Backpressure` is queue kind `1:U8 R`, capacity `2:U64 R`, pre-attempt occupancy `3:U64 R`, requested slots `4:U16 R`, queue generation `5:U64 O`, retry sample `6:U64 O`, requested bytes `7:U64 O`, available bytes `8:U64 O`. Every non-OK response instead has diagnostics `1:MESSAGE R*`, omitted-diagnostic count `2:U32 R`, and backpressure `3:MESSAGE O` exactly when the status is backpressure.

## Commands and success responses

| ID | Command fields | Success response fields |
| ---: | --- | --- |
| `0001` | empty, revision-any | v1 min/max, effective frame/TLV/string/depth limits, batch/page/transaction limits, all queue/replay caps, density/quantum, packed command/event IDs, capability flags |
| `0002` | `1:offset U64 R`, `2:max-bytes U32 R` | `1:total U64 R`, `2:offset U64 R`, `3:canonical-JSON BYTES R`, `4:eof BOOL R` |
| `0003` | `1:SessionEdit MESSAGE R*`; exact revision, nonempty | `1:applied-operations U32 R` |
| `0006` | count `1:U16 R`, stride `2:U16=32 R`, records `3:BYTES R`; exact revision | accepted count `1:U16 R`, occupancy `2:U64 R`, capacity `3:U64 R`, generation `4:U64 R` |
| `0007` | empty | transport snapshot below |
| `0008` | state `1:U8 R`, absolute position `2:U64 O`; exact revision | transport snapshot below |
| `0009` | meter handles `1:PACKED_U32 R`, meter period `2:U32 R`, counter IDs `3:PACKED_U32 R`, counter period `4:U32 R`, diagnostics enabled `5:BOOL R`, minimum severity `6:U8 R`; exact revision | canonical echo of those six fields |
| `000a` | all `1:BOOL R`, IDs `2:PACKED_U32 O` only when not all | observed sample `1:U64 R`, `CounterValue MESSAGE R*` |
| `000b` | after-sequence `1:U64 R`, limit `2:U16 R`, minimum severity `3:U8 R` | last sequence `1:U64 R`, eof `2:BOOL R`, diagnostic `3:MESSAGE R*` |
| `000c` | control list request below | `1:revision U64 R`, `2:structure-epoch U64 R`, `3:last-handle U32 R`, `4:eof BOOL R`, `5:ControlEntry MESSAGE R*` |
| `000d` | control list request below | `1:revision U64 R`, `2:structure-epoch U64 R`, `3:last-handle U32 R`, `4:eof BOOL R`, count `5:U16 R`, stride `6:U16=8 R`, records `7:BYTES R` |

Snapshot continuations, control pages, and diagnostic pages use an exact revision after an any-revision first page. Counter values are `1:id U32 R`, `2:value U64 R`, ascending and non-resetting. Counter IDs run 1 to 16; `16` is `LIVE_VALUES_SUPERSEDED`, the writes to adopted plans that a later write to the same live cell replaced before a block read them (#1312). A control-state record is `{handle:u32, value:f32}`; a meter record is `{handle:u32, component:u16, flags:u16, value:f32, reserved:u32}`. Both fixed arrays are validated schema fields, not opaque bytes.

## The control list (decision 17, C4, #1551)

`CONTROL_LIST_GET` returns pages of rows in strictly ascending handle order: an owner header, then its section headers, then the control rows of each section. `CONTROL_STATE_GET` takes the same request and returns the values of the same rows, without the headers. A handle is nonzero and means a row only inside one `structure_epoch`; the epoch changes when the set of rows changes, never when a value does.

**Request** (both messages): `1:structure-epoch U64 R` (`0` on the first page), `2:after-handle U32 R`, `3:limit U16 R` (`1..=256`), `4:owner-kind U8 O`, `5:owner-id UTF8 O` (present exactly when the owner kind is not the session; stable ID). A request whose epoch is not `0` and not the endpoint's current one is `REVISION_CONFLICT` with the diagnostic `control.list.stale`; the caller starts again from handle `0`. Epoch `0` names no structure to continue in, so a request with epoch `0` and `after-handle` other than `0` is `INVALID_FIELD`.

**Address** (`ControlAddress`): `1:owner-kind U8 R` (session `1`, strip `2`, route `3`, VCA `4`), `2:owner-id UTF8 O` (absent exactly for the session), `3:section U8 R` (inserts `2`, builtins `4`, console `5`, own `6`; the first three are the rack codes above), `4:slot-id UTF8 O` (present exactly for console and inserts), `5:control U32 O`, `6:lane U8 O` (left `1`, right `2`, both `3`; `5` and `6` are present exactly in a control row). An owner header carries the `own` section.

**Entry** (`ControlEntry`): `1:handle U32 R`, `2:row-kind U8 R` (owner header `1`, section header `2`, control `3`), `3:address MESSAGE R`. A header row adds `4:header-kind U8` (owner: track `1`, submix `2`, route `3`, VCA `4`, session `5`, and it must agree with the address's owner kind; section: the section code), `5:name UTF8`, `6:effect UTF8 O` (a console or inserts section header only), and for a route owner header `7:source UTF8`, `8:destination UTF8`, `9:tap U8` (the route tap codes below). A control row adds `10:name`, `11:display-name`, `12:display-unit` (UTF8), `13:unit U8` (dB/Hz/ms/samples/linear/ratio `1..6`), `14:kind U8` (continuous `1`, toggle `2`, enumeration `3`), `15:minimum`, `16:maximum`, `17:disabled-value O`, `18:default` (F32), `19:taper U8` (linear `1`, logarithmic `2`, exponential `3`, decibel `4`, stepped `5`), `20:step F32`, `21:precision U8`, `22:ladder PACKED_F32`, `23:choice MESSAGE O*` (value `1:F32`, label `2:UTF8`), `24:ramp-row U8 O` (the live-ramp rows in declaration order: fader `1`, input trim `2`, send gain `3`, VCA offset `4`, link glide `5`, mute `6`, solo `7`, polarity invert `8`, send mute `9`, send follows-mute `10`, VCA mute `11`, bypass crossfade `12`, pan `13`, matrix `14`, send matrix `15`; absent means a fixed ramp), `25:default-ramp-samples U32`, `26:class U8` (live `1`, rebuild-only `2`, derived `3`), `27:reason U8 O` (new memory `1`, new graph `2`, processing order `3`, latency `4`, glitch `5`, optimisation `6`, correctness `7`, ruled live not built `8`), `28:reason-issue U32 O`, `29:reopen UTF8 O`, `30:scope U8` (model `1`, monitor `2`), `31:automatable BOOL`, `32:family U8` (strip level `1`, strip matrix `2`, strip input `3`, route `4`, effect `5`, setting `6`), `33:active BOOL`, `34:value F32`. Header-only and control-only fields are optional on the wire and required or refused by the row kind: a header with a control field, a control row with a header field, a missing required field or a code outside its table is `INVALID_FIELD`.

A control row's own rules: all numbers are finite and `minimum <= maximum`; choices are present exactly for an enumeration, unique, and hold the default and the value; `reason` and `reopen` are present exactly when the class is rebuild-only, and `reason-issue` exactly with reason `8`; a control is automatable only if it is live and in the model scope.

**Pages.** The list page is `1:revision U64`, `2:structure-epoch U64`, `3:last-handle U32` (the last row's handle, or the request's cursor on an empty page), `4:eof BOOL`, `5:ControlEntry MESSAGE R*` (at most 256, ascending). The state page has the same first four fields, then count `5:U16`, stride `6:U16=8` and records `7:BYTES`: `{handle:u32, value:f32}` per control row of the covered handles, ascending; `last-handle` is the cursor for the next request and may follow the last record (a header row is covered and has no record). A record array whose width, count, order or values break these rules is refused. Handles are revision-scoped; a record's state flags no longer exist, because a value is always valid.

## Events and automation

Reliable `8001` contains sequence, origin request ID, previous revision, and operation count. Reliable `8002` contains sequence, origin request ID, canceled count, cancellation reason, automation queue generation, and optional effective sample. Reliable `8010` contains sequence, state, position, effective sample, and optional origin request ID. Lossy `8020` contains observed sample plus the fixed 16-byte meter array. Lossy `8021` contains observed sample plus ascending counter values. Reliable `8030` contains one diagnostic with provider sequence. Events have header request ID zero; reliable events have endpoint-monotonic payload sequence.

An automation record is exactly 32 bytes: `kind:u8, flags:u8=0, reserved:u16=0, handle:u32, start:u64, end:u64, start:f32, end:f32`. Kinds are point/step/linear/exponential `1..4`. Point has equal time/value; segments have `end > start`; exponential endpoints are nonzero and same-sign. Values are finite and pass the control row's domain; the handle names an automatable control row of the control list. Records are ordered by `(start, handle)` with no overlapping/duplicate records per handle. A segment applies on `[start,end)` and holds its end value until replaced.

## Session edit registry

`SESSION_TRANSACTION_APPLY` carries each edit as `1:opcode U16 R`, `2:payload MESSAGE R`. Payload fields begin at one in the listed order. The mapping is one-to-one with [Session schema v1](SESSION_SCHEMA_V1.md), never TOML, JSON Patch, a string path, or a private-field mutation.

| Opcode | Variant | Payload fields |
| ---: | --- | --- |
| `0001` | set session ID | session ID |
| `0002` | set sample rate | sample rate Hz |
| `0003` | set quantum | quantum frames |
| `0004`–`0005` | set render profile, output profile | one complete respective value |
| `0007` | set console | one complete console declaration |
| `0008` | set control smoothing | `1`: optional control-smoothing message (absent clears the key) |
| `0100` | upsert source | source |
| `0101` | remove source | source ID |
| `0103` | set source content and complete shape | source ID, content, channels, bit depth, frames |
| `0200` | upsert track | track |
| `0201` | remove track | track ID |
| `0202` | set source assignment | track ID, source ID, left/right source channel |
| `0203`–`0204` | set builtins; set rack | strip ID plus builtins; strip ID, rack name, rack |
| `0205` | put effect | strip ID, rack name, final position, effect |
| `0206`–`020c` | remove/order effect; set identity/quality/bypass/link/sidechain | strip ID, rack name, effect ID plus the respective replacement; order repeats effect ID |
| `020d`–`0210` | upsert/remove effect parameter; set fader/matrix | strip ID, rack name, effect ID plus parameter or parameter ID/channel; strip ID plus fader/matrix |
| `0211` | set strip console | strip ID, then the complete ordered console entries (repeated) |
| `0300`–`0301` | upsert/remove submix | submix; submix ID |
| `0400`–`0401` | upsert/remove output | output; output ID |
| `0500`–`0507` | upsert/remove route; set source/destination/matrix/gain/mute/follows-mute | route or route ID plus the respective replacement (`0506`, `SetRouteMute`, is route ID `1` and boolean mute `2`; #1216. `0507`, `SetRouteFollowsMute`, is route ID `1` and boolean follows-mute `2`; #1218: validated on the final candidate like every edit, so `true` on a route whose destination is the output refuses the whole transaction with `schema.invalid_enum` at `$.routes[<i>].follows_mute`) |
| `0600`–`0603` | upsert/remove automation; set target/segments | automation or automation ID plus target; ordered repeated segment |
| `0700`–`0702` | upsert/remove VCA; set VCA fader | VCA; VCA ID; VCA ID `1` plus fader `2` (#1241: `0700` is a plain upsert by ID; `0701` and `0702` at an unknown ID are `session.edit.not_found`; membership, namespace, range and acyclicity are validated on the final candidate, so removing a VCA, track or submix that a VCA still lists refuses the whole transaction with `reference.missing_entity` unless the same transaction rewrites that VCA, and a cycle refuses with `vca.cycle`. A VCA message spends one TLV each on `id` and `fader` and one per member, so under the default `ProtocolLimits::max_tlv_count` (1,024 per message) one `0700` carries at most 1,022 members, the same bound `SetTrackEffectOrder` has; membership has no incremental opcode, so a larger VCA needs both endpoints to raise that configured limit) |
| `0800` | set control | `1`: control address (message, `control` and `lane` required), `2`: value `F32` (any bit pattern; a non-finite value is refused by the validator), `3`: `ramp_samples U32 O` (#1553, below) |

Strip addressing (#1204). Opcodes `0203`–`0211` address a **strip ID**, and nothing else about them changed: no opcode, field, code or wire position moved, and the field once named "track ID" is the same first field with the same type.

- **D1.** A track ID names the track's strip and a submix ID the submix's. The Rust variant and field names (`SetTrackFader { track_id, .. }`) are unchanged internal names; each field's doc says it names a strip.
- **D2.** `0202` (source assignment) stays track-only: a submix ID is `session.edit.not_found`. An output ID is `session.edit.not_found` for every strip opcode in the base model, with two exceptions by design: a structural or declaration edit with rack `console` answers `session.edit.console_slot_fixed` first (D3), and a submix upserted under an output's ID earlier in the same transaction is resolved by a strip edit, after which final validation refuses the whole transaction with a duplicate ID rather than `not_found`.
- **D3.** Rack and knob semantics are unchanged; only the resolution widens. The structural and declaration edits refuse a submix's `console` rack as `session.edit.console_slot_fixed` before any lookup, exactly as for a track (so an unknown ID with rack `console` still answers `console_slot_fixed`); `020a`, `020d` and `020e` with rack `console` edit the submix's entry for the slot; rack `builtins` stays `session.edit.not_found`; and the slot-set rule below (#1202) stands.
- **D4.** A strip ID resolves **tracks first, then submixes**. A validated model has one match at most, because IDs are unique across tracks, submixes and outputs; inside a transaction, before final validation, a track and a submix may briefly share an ID (`0300` is a plain upsert), and then the track wins.
- **VCAs (#1241).** A strip opcode never resolves a VCA ID: `0203`–`0211` at a VCA ID are `session.edit.not_found`. A VCA's fader changes only through `0702` or `0700`.

Control smoothing (#1365, decision 15 D15-1, D15-3). `0008` sets the session's `control_smoothing` (#1054) to the message in field `1`, or clears it when field `1` is absent, so the default table applies; canonical JSON then reads `{ "kind": "explicit", "mute_ms", "fader_ms", "pan_ms" }` or `{ "kind": "default" }`. No prepared plan reads the key, so the edit commits without a plan rebuild and carries no `ramp_samples`: the next live record any later edit produces uses the new lengths, and a ramp already in flight keeps the length it started with.

Set control (#1553, decision 17, C5). `0800` sets any control of the control list by its stable address, with the address message above (`control` and `lane` required; the owner ID, slot ID and codes follow the list's rules, and an address that breaks them is `INVALID_FIELD`). The value is any `F32`; a non-finite value is decoded and refused by the validator, so the refusal names the edit. The core's one validator runs inside the edit, before the model changes and before the compile: a refused edit refuses the whole transaction with `VALIDATION_FAILED` and a diagnostic at `edits[i]` whose code is the control code itself: `control.address.unknown`, `control.read_only`, `control.inactive`, `control.lane`, `control.domain`, `control.ramp.fixed`, `control.ramp.conflict` or `control.monitor` (solo is held by the host and has no session writer). Whether the committed edit is live or a rebuild is the classifier's answer for the resulting model, exactly as for the per-kind edit that writes the same field.

Per-edit ramp length (#1394, decision 15 D15-1, D15-3). Each live-value edit appends one optional field, `ramp_samples` (`U32`), after its last field: an in-place V1 field, with no new opcode and no ABI feature bit. Absent means the session's `control_smoothing`, and an explicit `0` is legal and steps that edit only; the decoder keeps the two apart. A value above the committed `sample_rate_hz` (one second) refuses the whole transaction before anything commits with `VALIDATION_FAILED` and diagnostic `session.edit.ramp_out_of_range` at `edits[i]`, the offending edit's index; the bound uses the rate the transaction commits. The length is transaction metadata: never model state, never in canonical JSON, and the next edit without one uses the session default.

| Opcode | Variant | `ramp_samples` field | Rows it names |
| ---: | --- | ---: | --- |
| `0007` | set console | `2` | console link |
| `0203` | set strip builtins | `3` | input (trim, polarity) |
| `020b` | set effect link mode | `5` | effect link |
| `020f` | set strip fader | `3` | fader, mute |
| `0210` | set strip matrix or pan | `3` | matrix |
| `0211` | set strip console | `3` (after the repeated entries) | link |
| `0504` | set route matrix | `3` | route matrix |
| `0505` | set route gain | `3` | route gain |
| `0506` | set route mute | `3` | route mute |
| `0507` | set route follows-mute | `3` | route mute |
| `0702` | set VCA fader | `3` | VCA fader, VCA mute |
| `0800` | set control | `3` | the one row the address names, if it ramps (below) |

The last edit of a transaction that writes a row decides its length (`Some(n)` sets it, absent removes it), and these clear every length of their target: the four upserts `0200`, `0300`, `0500` and `0700`, the four removals, `PutTrackEffect`, `RemoveTrackEffect` and `SetTrackRack` (#1394 D3). The effect-bypass crossfade (#1341) and effect parameters have no per-edit length. `0800` names one row by its address: the row's descriptor ramp row decides it (builtin fader and mute, `pan` and the matrix rows with the own `matrix_mode` row, trim and polarity as input, a route's gain, mute and follows-mute and matrix rows, a VCA's offset and mute), and every other row has a fixed ramp, which refuses an explicit length as `control.ramp.fixed`. Several `0800` rows share one ramp row (the fader's two lanes, `pan` and the matrix rows, trim and polarity, a route's mute and follows-mute), and the row holds one length: when one transaction gives two such edits two different lengths (an explicit one against another, or against none, which means the session default), the later edit is refused as `control.ramp.conflict` at `edits[i]` before any write. Equal lengths agree. A per-kind edit is not a `0800` write and keeps the last-edit-decides rule.

There are exactly 48 allocated opcodes (#1216 appended `0506`, #1218 `0507`, #1241 opened the VCA family with `0700`–`0702`, #1365 appended `0008`, and #1553 opened the control family with `0800`). Issue #241 retired three: `0006` (set limits), `0102` (set source sample rate) and `0104` (set source mapping), whose subjects left the schema with the `limits` table and the nested source mapping/region. Retired codes are **never reallocated** -- a v1 peer that spells one must be refused, not reinterpreted, which is why `SessionEditOpcode::from_raw` returns `None` for them and a conformance row pins that. A successful atomic transaction replaces the typed `SessionModel`, immutable control-plane `CompiledSession`, and revision together; its canonical snapshot is the committed canonical JSON, never a compiled/render-plan serialization.

The nested model registry is: render/output profile `1:id,2:mode/channels,3:sample-format`; source `1:id,2:content,3:channels,4:bit-depth,5:frames`; console `1*:pre-insert slot,2*:post-insert slot`; control smoothing `1:mute_ms,2:fader_ms,3:pan_ms` (#1365; three required `F32` fields, each finite and in `[0, 1000]` ms, and a message missing one or holding one outside the bounds is refused at decode as `MALFORMED_FRAME`); console slot `1:slot,2:identity,3:quality,4:link-mode` (no sidechain field); track `1:id,2:source,3/4:channels,5:builtins,7:inserts,9:fader,10:matrix/pan,11*:console entry`; submix `1:id,2:builtins,3*:console entry,4:inserts,5:fader,6:matrix/pan` (field 6 is tagged exactly as the track's field 10, #1199; field 3 repeats the track's field-11 console entry, #1202); console entry `1:slot,2:bypass,3*:parameter`; effect `1:id,2:identity,3:quality,4:bypass,5:link-mode,6*:parameter,7:sidechain`; route `1:id,2:source,3:destination,4:matrix,5:gain,6:mute,7:follows-mute` (fields 6 and 7 are required booleans, #1216 and #1218; field 7 may be `true` only on a route into a submix, and `true` on a route into the output is refused at validation, not by the codec); automation `1:id,2:target,3*:segment`; VCA `1:id,2:fader,3*:member` (#1241; a member is a stable-ID string, an empty member list is zero occurrences of field 3, and the session model's root field for `vcas` is `16`, #1240). Route source (also a routed sidechain's source and the `SetRouteSource` value) `1:kind,2:id,3:tap`: kind `1` (track) and kind `2` (submix) both require field 3 (#1203; a tag-2 source without it is refused, and the retired JSON spelling `submix_output` is the same tag 2, now spelled `submix` with a required tap). A route source's tap codes, on a track or a submix, are input `1`, post_input `2`, insert_send `3`, insert_return `4`, pre_fader `5`, post_fader `6`, post_pan `7`. Tagged nested values use field `1:kind`; unknown enum/tag values reject and allocated codes never renumber within v1. The render-profile mode enum has one live code, `1` (`single_thread`); issue #1063 retired `2` (`dependency_waves`), which now refuses like any unallocated code and is never reallocated.

### Decision 12: the session console (#1093, #1094)

Owner decision 12 (`docs/rulings/engine-footprint-2026-09-29.md`, "Wire identity") is an in-place v1 amendment on the #1063 precedent: nothing is renumbered, every retired code is refused and never reallocated, appended codes take the next unallocated ID, and there is no protocol major or minor increment and no `ABI_VERSION` bump. The prelaunch identity stays v1.

- **Renamed in place.** The track taps keep codes `1`–`7` under the spellings above (they were `input`, `post_input_builtins`, `post_simd1`, `post_dynamic`, `post_simd2_pre_fader`, `post_fader`, `post_matrix`). Rack code `2` is `inserts` (was `dynamic`), with the same semantics, and track field `7` carries it.
- **Retired.** Rack codes `1` (`simd1`) and `3` (`simd2`) refuse at decode in every rack-addressed edit and automation target (`MALFORMED_FRAME`). Track fields `6` (`simd1`) and `8` (`simd2`) refuse in either flag form: a mandatory one is `UNKNOWN_REQUIRED_FIELD`, and an optional one is `MALFORMED_FRAME`, never skipped as an unknown optional field would be.
- **Appended.** Rack code `5` (`console`); track field `11` (console entries) and the console, console-slot and console-entry messages; opcodes `0007` (set console) and `0211` (set track console). The session model's own root field for the console is `15`.

Console addressing. A console slot's `slot` ID is session-level and unique across both sections; every strip, track or submix (#1202), carries one entry per slot, in slot order.

- `0007` replaces the whole declaration: both sections and each slot's identity, quality and link mode. It is the only edit that changes the slot set.
- `0211` replaces one strip's whole entry array (a track's or, #1204, a submix's). It edits that strip's knobs and cannot change the slot set: an entry array that adds, drops or reorders a slot refuses at final validation.
- `020a` (bypass), `020d` (upsert parameter) and `020e` (remove parameter) with rack `console` edit one strip's entry, a track's or a submix's (#1204), with the slot ID as the effect ID; a slot the strip does not carry is `session.edit.not_found`.
- `0204`–`0209`, `020b` and `020c` (set rack, put, remove, order, identity, quality, link mode, sidechain) with rack `console` are refused as `VALIDATION_FAILED` with diagnostic `session.edit.console_slot_fixed`, whatever the model holds. A strip, track or submix, cannot add, remove or reorder a slot, a slot's declaration is the session's, and a slot has no sidechain. With rack `builtins`, every rack-addressed edit is `session.edit.not_found` (#178).

A transaction that changes the slot set (adds, removes, renames or reorders a slot) must rewrite every strip's entries in the same transaction: every track's with `0211` or `0200`, and every submix's with `0300` (#1202) or, since #1204, `0211`. The store validates only the final candidate, so the rewrite may follow the declaration in any order; a transaction that skips a strip fails final validation with that strip's `console.entry_missing`, `console.entry_order` or `reference.missing_entity` (at `$.tracks[<i>].console` or `$.submixes[<i>].console`), and is refused whole. Nothing is committed and nothing is acknowledged: the refusal is decided before the response is written, as for every transaction. A declaration change that keeps the slot sequence (a quality, link mode or identity change, or moving a slot across the section boundary without reordering it) leaves every entry valid and needs no rewrite; effect preparation judges the parameters against a changed identity, as it does for `0208` on an insert.
