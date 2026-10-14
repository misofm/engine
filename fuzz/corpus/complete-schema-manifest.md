# Issue-005 complete typed BTLV corpus

The shared `complete_schema_corpus()` fixture has 46 canonical frames: all 11 commands (the
session transaction contains all 35 allocated `SessionEditOpcode` values), all 11 successful
responses, all 18 registered non-OK statuses (including typed `BACKPRESSURE`), and all six event
schemas. Optional/boundary values are represented by the transaction's nested fixture, optional
transport position, empty valid pages, and the typed backpressure variant.

The canonical sequence is FNV-1a-64 over each stable frame label followed by its frame bytes:
`0433af905dc618bc`, pinned once as `COMPLETE_SCHEMA_HASH`. Native mutation, each typed fuzz
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
Issue #1241 re-pinned it from `95c1ceb68e44f6e2`: the VCA message (`id` 1, `fader` 2, repeated
`members` 3), and the transaction appends `UpsertVca`, `RemoveVca` and `SetVcaFader`
(`0x0700`-`0x0702`), so it carries 46 edits; the frame count is unchanged.
Issue #1394 re-pinned it from `ab357b6c432f9755`: #1394 appends optional `ramp_samples` to the
live-value edits, and the corpus's `SetTrackFader` sets it, so that edit's bytes grow; the frame
and edit counts are unchanged.
Issue #1365 re-pinned it from `5429f052592101ef`: #1365 appends `SetControlSmoothing` (`0x0008`),
so the transaction carries 47 edits; the frame count is unchanged.
Issue #1551 (decision 17, C4) re-pinned it from `8eb4cc8d075a4355`: `CONTROL_LIST_GET` and
`CONTROL_STATE_GET` (`0x000c`, `0x000d`) replace the retired `PARAMETER_METADATA_GET` and
`PARAMETER_STATE_GET` (`0x0004`, `0x0005`), so the command and response frames of the pair are new
(a list page with every kind of row and optional field, and the state page of its control rows),
and the capability registry lists the new pair; the frame count is unchanged. The retired IDs are
covered by `retired_code_rows()`, not by a frame.
Issue #1553 (decision 17, C5) re-pinned it from `0902357f75bdddcd`: #1553 appends `SetControl`
(`0x0800`), so the transaction carries 48 edits; the frame count is unchanged.
Issue #1313 re-pinned it from `9f13a06993efa99e`: `TransactionApplied` gains the required field 2,
`edit_path` (`live` 1, `model_only` 2, `rebuild` 3), so the transaction response frame carries it;
the frame count is unchanged.
Issue #1561 (decision 17, C5) re-pinned it from `a19a779439660743`: the thirteen per-kind value ops
(`0x0008`, `0x0203`, `0x020a`, `0x020b`, `0x020d`-`0x0210`, `0x0504`-`0x0507`, `0x0702`) are retired
and never reallocated, so the transaction carries 35 edits, one per allocated opcode; the frame
count is unchanged.

The value stood at `88a8ee6a6d9e4acc` here until #274. It was correct until `b454b230`, and the
two re-pins that followed (`b454b230`, then #241's `04d291dd`) did not reach this file or the
Wasm runner, because the parity gate over them could not fail; the arithmetic that carries
`88a8ee6a6d9e4acc` to the current value is in `docs/derivations/274-parity-repin.md`.
