# Effect descriptor wire V1

Descriptor records use little-endian fixed-width fields. Unknown enum values, non-finite numbers,
negative zero, nonzero reserved bytes, and noncanonical text reject.

`ParameterId` is a nonzero stable `u32`; a changed meaning, unit, or domain gets a new ID.

## Parameter lattice fields (issue #242)

The 80-byte parameter record does not grow. The two words formerly reserved at offsets 72 and 76
are now the persisted-value lattice authority:

| offset | width | field |
|---|---|---|
| 72 | u32 | exact `f32` bits of the positive decimal `step` |
| 76 bits 0..4 | 5 | `xs` integer multiplier |
| 76 bits 5..9 | 5 | `sm` integer multiplier |
| 76 bits 10..14 | 5 | `md` integer multiplier |
| 76 bits 15..19 | 5 | `lg` integer multiplier |
| 76 bits 20..25 | 6 | `xl` integer multiplier |
| 76 bits 26..29 | 4 | canonical decimal precision |
| 76 bits 30..31 | 2 | step unit minus one: absolute, cents, ratio, index |

The multipliers are positive and strictly ascending; `xs..lg <= 31`, `xl <= 63`, precision is
`0..=8`, and the step/unit must match the parameter domain and mapping.

**One lattice has exactly one spelling.** Both words zero decode to the row's derived unit-class
lattice, `default_parameter_lattice(unit, domain, mapping)`. A row whose declaration IS that
derived lattice therefore encodes zeros, and a row that overrides its class encodes the words
explicitly; there is no third case. A verifier **refuses** a non-zero window whose unpacked
lattice equals the derived default -- that would be a second byte sequence for one meaning, and
in a format whose bytes are its identity that is an aliasing bug. The refusal is typed
`Reserved` at offset 72 of the offending record. Exactly one zero word rejects as `Flags`, also
at offset 72.

Because the derived case is spelled as zeros, every descriptor sealed before #242 keeps its exact
bytes and its exact identity. A stale verifier rejects an explicit lattice at offset 72 under its
old reserved-zero rule, so it cannot silently ignore the new authority.

The public C projection names these words `step_bits` and `step_spec` at the same offsets. The
`MISO_ENGINE_EFFECT_PARAMETER_STEP_*_MASK_V1` constants pin the packed derivation.

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
