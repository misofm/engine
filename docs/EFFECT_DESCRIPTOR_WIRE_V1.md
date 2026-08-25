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
