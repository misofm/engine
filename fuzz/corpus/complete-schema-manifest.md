# Issue-005 complete typed BTLV corpus

The shared `complete_schema_corpus()` fixture has 46 canonical frames: all 11 commands (the
session transaction contains all 39 allocated `SessionEditOpcode` values), all 11 successful
responses, all 18 registered non-OK statuses (including typed `BACKPRESSURE`), and all six event
schemas. Optional/boundary values are represented by the transaction's nested fixture, optional
transport position, empty valid pages, and the typed backpressure variant.

The canonical sequence is FNV-1a-64 over each stable frame label followed by its frame bytes:
`bdebb0f81c38ec42`, pinned once as `COMPLETE_SCHEMA_HASH`. Native mutation, each typed fuzz
decoder, and scalar/simd128 Wasm execution all consume this same public fixture source.

The value stood at `88a8ee6a6d9e4acc` here until #274. It was correct until `b454b230`, and the
two re-pins that followed (`b454b230`, then #241's `04d291dd`) did not reach this file or the
Wasm runner, because the parity gate over them could not fail; the arithmetic that carries
`88a8ee6a6d9e4acc` to the current value is in `docs/derivations/274-parity-repin.md`.
