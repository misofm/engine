# MISO Control BTLV v1

This is the normative byte contract for `miso-engine-protocol` major version 1. It is transport-neutral: an adapter supplies exactly one complete frame and may add stream framing outside these bytes. It never transports PCM, media, a render plan, or an opaque provider blob. Message-specific fields are in the [registry](CONTROL_PROTOCOL_REGISTRY.md); endpoint behavior is in [semantics](CONTROL_PROTOCOL_SEMANTICS.md).

## Outer frame

All integer and floating-point values are IEEE little-endian. Implementations decode fields by offset; they must not transmute a Rust or C structure.

| Offset | Bytes | Field |
| ---: | ---: | --- |
| 0 | 8 | ASCII magic `MISOCTL\0` |
| 8 | 2 | major (`1`) |
| 10 | 2 | minor (`0` for the original registry, `1` for the additive named-nudge registry) |
| 12 | 2 | header length (`48`) |
| 14 | 1 | kind: command `1`, response `2`, event `3` |
| 15 | 1 | flags: command bit 0 is `revision-any`; all other bits are zero |
| 16 | 2 | message ID |
| 18 | 2 | status: zero for command/event; registered response status |
| 20 | 4 | padded payload length |
| 24 | 8 | request ID |
| 32 | 8 | revision carrier |
| 40 | 4 | top-level TLV count |
| 44 | 4 | reserved zero |

The exact frame length is `48 + payload_length`; truncation and trailing bytes reject. There is no checksum, compression, alignment promise, transport header, or stream-length prefix. Commands have a nonzero request ID; responses echo it; events encode zero. Commands with `revision-any` set bit 0 and carry zero; exact commands clear it and carry the requested revision. Responses and events clear it and carry the authoritative committed revision.

The empty capability request, request ID 1 and revision-any, is:

```text
4d49534f43544c0001000000300001010100000000000000010000000000000000000000000000000000000000000000
```

## TLV encoding

Every payload field is `field_id:u16`, `wire_type:u8`, `flags:u8`, `value_length:u32`, value bytes, then zero padding to the next eight-byte boundary. `flags & 1` means mandatory; every other flag bit is zero. A `MESSAGE` value starts with `nested_field_count:u32, reserved:u32=0`, then nested TLVs.

| ID | Type | Exact value rule |
| ---: | --- | --- |
| 1–8 | `U8`, `U16`, `U32`, `U64`, `I64`, `F32`, `F64`, `BOOL` | scalar width; bool is one byte `0` or `1` |
| 9–11 | `UTF8`, `BYTES`, `MESSAGE` | validated UTF-8; bounded bytes; nested form above |
| 12–15 | `PACKED_U16`, `PACKED_U32`, `PACKED_U64`, `PACKED_F32` | length is an exact multiple of scalar width |

Fields are nondecreasing by ID. Repeats are contiguous and only permitted by the registered schema; singleton duplicates reject. Padding and all reserved bytes are zero. Known fields use their registered type and mandatory flag. Unknown optional fields are skipped; unknown mandatory fields reject. All arithmetic is checked before slicing, copying, or provider work. The default limits are 1 MiB complete frame, 1,024 TLVs per level, 64 KiB UTF-8 field, and nesting depth four; the endpoint advertises its effective limits in `CAPABILITIES_GET`.

## Codec and FFI ownership

The caller-buffer path borrows input and decode scratch only for the call and writes one canonical frame to caller output. It retains no pointer. A pure codec or already-materialized response/event reports the exact required length on output-too-small and writes no partial frame. Before admitting a new stateful command, the controller instead requires the advertised full response reservation and returns a distinct reservation error; this prevents replay, session, queue, or provider mutation when caller output is short without mislabeling that bound as the eventual frame length. A future byte ABI permits null only for a zero-length span and rejects overlapping input/output. No Rust layout, allocation, panic, unwinding, callback, or trait object crosses that boundary.

After a command is accepted, queued records have been copied into prepared engine-owned storage; on failure the caller keeps its input. The render thread never decodes BTLV or invokes a callback. The future C ABI may use opaque handles only; it must not reinterpret Rust layouts.

## Compatibility

| Peer version | v1 endpoint action |
| --- | --- |
| different major | reject `UNSUPPORTED_VERSION` |
| same major, lower/equal minor | accept known v1 semantics |
| same major, higher minor | accept known semantics and optional unknown fields only |

The encoder emits the negotiated minor. Within a major, IDs, field types, requiredness, enums, statuses, and canonical ordering never change or get reused. Additive optional fields/messages need a minor increment; a semantic or canonical-byte change needs a major increment. Decode then re-encode has exactly one canonical representation. Unknown flag bits and unknown mandatory fields always reject.

The media range `0x6000..=0x6fff` is permanently rejected as `PCM_FORBIDDEN` in major 1. The complete message, status, enum, and session-edit registries are in the [registry](CONTROL_PROTOCOL_REGISTRY.md#registries).
