# Issue-005 complete typed BTLV corpus

The shared `complete_schema_corpus()` fixture has 46 canonical frames: all 11 commands (the
session transaction contains all 43 allocated `SessionEditOpcode` values), all 11 successful
responses, all 18 registered non-OK statuses (including typed `BACKPRESSURE`), and all six event
schemas. Optional/boundary values are represented by the transaction's nested fixture, optional
transport position, empty valid pages, and the typed backpressure variant.

The canonical sequence is FNV-1a-64 over each stable frame label followed by its frame bytes:
`95c1ceb68e44f6e2`, pinned once as `COMPLETE_SCHEMA_HASH`. Native mutation, each typed fuzz
decoder, and scalar/simd128 Wasm execution all consume this same public fixture source.

Issue #787 re-pinned this value after the transaction's two same-length source-identity spellings
moved from `sha256:` to `blake3:`; the frame count and total encoded byte count remain unchanged.
Issue #1093 re-pinned it from `e4dec003302d891a` for decision 12: the track message carries the
inserts in field 7 and a console entry in field 11 and no longer the retired fields 6 and 8, and
the rack-addressed edits spell the `inserts` code. Issue #1094 re-pinned it from
`af1b9b71a0a31727`: the transaction appends `SetConsole` (`0x0007`) and `SetTrackConsole`
(`0x0211`), so it carries 41 edits; the frame count is unchanged.
Issues #1199, #1202 and #1203 re-pinned it in turn from `ebf282621550d44a`, through
`ca48855fd3a756b7` and `c0f6ecedbf50920a`: the submix message carries its strip in fields 2, 4, 5
and 6 and its console entries in field 3, and a tag-2 route source carries a required tap.
Issue #1216 re-pinned it from `a1dcc56f2e4a48f9`: route field `mute` (field 6), and the
transaction appends `SetRouteMute` (`0x0506`), so it carries 42 edits; the frame count is unchanged.
Issue #1218 re-pinned it from `39e5a2c1d317a9fe`: route field `follows_mute` (field 7), and the
transaction appends `SetRouteFollowsMute` (`0x0507`), so it carries 43 edits; the frame count is
unchanged.

The value stood at `88a8ee6a6d9e4acc` here until #274. It was correct until `b454b230`, and the
two re-pins that followed (`b454b230`, then #241's `04d291dd`) did not reach this file or the
Wasm runner, because the parity gate over them could not fail; the arithmetic that carries
`88a8ee6a6d9e4acc` to the current value is in `docs/derivations/274-parity-repin.md`.
