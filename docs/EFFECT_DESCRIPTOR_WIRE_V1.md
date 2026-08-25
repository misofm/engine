# Effect descriptor wire V1

Descriptor records use little-endian fixed-width fields. The public C records have sizes 80, 16,
32, and 48 bytes for parameters, enum choices, ports, and qualities respectively. Unknown enum
values, non-finite numbers, negative zero, nonzero reserved bytes, and noncanonical text reject.

`ParameterId` is a nonzero stable `u32`; a changed meaning, unit, or domain gets a new ID.

## Observation section (issue #143)

Header bytes `88..92` are `observation_count` and `92..96` are `observation_offset`; the section
sits between the enum-choice table and the string pool, and its 32-byte records carry ascending
tap ids with their strings in the existing pool.

| offset | width | field |
|---|---|---|
| 0 | u32 | `id`, the effect-local tap id |
| 4 | u8 | `kind` |
| 5 | u8 | `unit` |
| 6 | u8 | `cost` |
| 7 | u8 | `cadence` |
| 8 | u8 | `fold` |
| 9 | u8 | `channels` |
| 10 | u8 | `display_name` byte length |
| 11 | u8 | `display_unit` byte length |
| 12 | u32 | `minimum` bits |
| 16 | u32 | `maximum` bits |
| 20 | u32 | `display_name` offset |
| 24 | u32 | `display_unit` offset |
| 28 | u32 | required zero |

The six vocabularies and the two string lengths are single bytes because the record is 32 bytes and
they do not fit otherwise; every string in this workspace is capped at 255 bytes, so a length fits
a byte for the same reason. The C projection (`miso_engine_effect_descriptor_v1_inspect_observations`,
a second export, 56-byte records) widens each of them back to `uint32_t`.

`VERSION` stays `1`. **A descriptor that declares no tap writes both header words zero**, so bytes
`88..96` remain the reserved-zero window the pre-#143 verifier demanded: every existing identity is
byte-for-byte unmoved, and a stale reader *refuses* a tap-bearing descriptor at offset 88 rather
than silently ignoring its menu. Declaring the first tap costs exactly
`32 + len(display_name) + len(display_unit)` and is a `contract_minor` bump;
`state_layout_version` does not move, because the tap reads state that was already there.

## Nudge ladder (issue #127)

The parameter record's eight reserved bytes at offsets `72..80` carry the declared nudge ladder.
The record does not grow: a ladder costs **zero** bytes.

| offset | width | field |
|---|---|---|
| 72 | u32 | declared `xs` bits, `0` with no ladder |
| 76 | u8 | nudge step unit, `0` with no ladder |
| 77 | u8 | nudge ratio class, `0` with no ladder |
| 78 | u16 | required zero |

Byte 76 is the presence bit. **A parameter that declares no ladder leaves all eight bytes zero**,
so a ladder-free descriptor is byte for byte the pre-#127 wire and its identity does not move; a
stale reader refuses a ladder-bearing record on the reserved rule at offset 72 rather than silently
ignoring its ladder. This is the same fail-closed choice issue #143 made with header bytes `88..96`,
and `VERSION` stays `1` for the same reason.

The verifier applies the contract's own three ladder rules through
`check_nudge_ladder_parts_v1` -- the same function `validate_descriptor_v1` calls, so the wire and
the static descriptor cannot drift -- and `bind_effect_descriptor_wire_v1` compares the declared
ladder field by field, so a wire that declares a *different* ladder is not that descriptor.

Declaring a ladder changes the descriptor's bytes and therefore its identity and CID, which is
correct and intended: it describes something different. The launch set's eight identities all
re-pin, with a byte delta of exactly zero. `fixtures/effect-descriptor/v1/comprehensive-d` is
`comprehensive-a` with three ladders and two renamed letters, and the reference implementation
asserts `total(d) == total(a)` alongside the identity change.
