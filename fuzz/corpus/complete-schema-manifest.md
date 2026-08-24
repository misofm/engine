# Issue-005 complete typed BTLV corpus

The shared `complete_schema_corpus()` fixture has 48 canonical frames: all 12 commands (the
session transaction contains all 42 allocated `SessionEditOpcode` values), all 12 successful
responses, all 18 registered non-OK statuses (including typed `BACKPRESSURE`), and all six event
schemas. Optional/boundary values are represented by the transaction's nested fixture, optional
transport position, empty valid pages, and the typed backpressure variant.

The canonical sequence is FNV-1a-64 over each stable frame label followed by its frame bytes:
`15b4f16548b072c5`. Native mutation, each typed fuzz decoder, and scalar/simd128 Wasm execution
all consume this same public fixture source.
