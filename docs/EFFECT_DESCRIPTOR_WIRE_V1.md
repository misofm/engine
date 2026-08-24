# Effect descriptor wire V1

Descriptor records use little-endian fixed-width fields. The public C records have sizes 80, 16,
32, and 48 bytes for parameters, enum choices, ports, and qualities respectively. Unknown enum
values, non-finite numbers, negative zero, nonzero reserved bytes, and noncanonical text reject.

`ParameterId` is a nonzero stable `u32`; a changed meaning, unit, or domain gets a new ID.

The 96-byte header accepts descriptor wire versions 1 and 2. A descriptor with no explicit nudge
declaration remains byte-for-byte V1. If any parameter declares `NudgeLadderV1`, the canonical
encoder emits version 2; this intentionally changes the descriptor bytes and its CID.

V2 gives the parameter record's former reserved bytes one bounded meaning without changing its
80-byte layout:

| Parameter offset | V1 | V2 |
| ---: | --- | --- |
| `72` | zero `u32` | `xs` normalized-space step as canonical finite positive `f32`, or zero when absent |
| `76` | zero `u32` | ratio class in bits `0..7`, presence flags in bits `8..15`, reserved-zero bits `16..31` |

The sole V2 flag is bit 8 (`ladder present`) and the sole ratio class is `HumanV1=1`. Presence zero
requires both words zero. Presence one requires a valid ratio class and `0 < xs <= 1`; unknown
flags/classes and nonzero reserved bits reject. A verified V1 zero-reserved record exposes no
declaration, so the contract registry derives its `(mapping, unit)` default. Record sizes, section
offsets, text rules, and all other V1 bytes are unchanged.
