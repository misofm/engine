# Effect descriptor wire V1

Descriptor records use little-endian fixed-width fields. The public C records have sizes 80, 16,
32, and 48 bytes for parameters, enum choices, ports, and qualities respectively. Unknown enum
values, non-finite numbers, negative zero, nonzero reserved bytes, and noncanonical text reject.

`ParameterId` is a nonzero stable `u32`; a changed meaning, unit, or domain gets a new ID.
