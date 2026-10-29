# Issue-005 complete typed BTLV corpus

The shared `complete_schema_corpus()` fixture has 61 canonical frames: all 17 commands (the
session transaction contains all 36 allocated `SessionEditOpcode` values, and `RESPONSE_GET` has a
configuration and a strip target), all 19 successful responses, all 16 registered non-OK statuses
(including typed `BACKPRESSURE`), and all seven event schemas (`RESPONSE_READY` in its ready and
refused forms). Optional/boundary values are represented by the transaction's nested fixture, the optional
transport-event origin, empty valid pages, and the typed backpressure variant.

The canonical sequence is FNV-1a-64 over each stable frame label followed by its frame bytes:
`f05688f0cd7e6f28`, pinned once as `COMPLETE_SCHEMA_HASH`. Native mutation, each typed fuzz
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
The batch 4 review fold (decision 17, #1559) re-pinned it from `0433af905dc618bc`: the corpus gains
`response.session_transaction_monitor`, a `TransactionApplied` with `edit_path` `monitor` (4), so the
frame count is 47 and the successful responses are 12.
Issue #1573 (decision 17, C13) re-pinned it from `7de408612b837afd`: `RESPONSE_GET` (`0x000e`)
appends one command frame (an effect configuration with every optional field and three overrides)
and two response frames (a curve reply with sections on both sides, and a lone tag), and the
capability registry lists `0x000e` and sets bit 14, so the frame count is 50, the commands are 12
and the successful responses are 14.
Issue #1627 (draft 21a) re-pinned it from `4f6a80fb52c12aed`: `AUTOMATION_ENQUEUE` (`0x0006`),
statuses 15 and 16 and capability fields 9, 12, 19 and 20 are retired and never reallocated, so the
corpus loses the command frame, its success response and the two status frames (the frame count is
46, the commands are 11, the successful responses 13 and the statuses 16), and the capabilities
response carries 23 fields and lists no command `0x0006`. The retired codes are covered by
`retired_code_rows()` and the decoder tests, not by a frame.
Issue #1628 (draft 21b) re-pinned it from `cfa58ab7b7bbb7c4`: the event `AUTOMATION_CANCELED`
(`0x8002`) is retired and never reallocated, so the corpus loses `event.automation_canceled` (the
frame count is 45 and the events are five), and the capabilities response lists no event `0x8002`.
`retired_code_rows()` gains a row for `0x8002`.
Issue #1584 (D6a) re-pinned it from `4b21d9e6f520afd9`: the reliable event `OBSERVATION_ENDED` (`0x8011`)
is added, so the corpus gains `event.observation_ended` (the frame count is 46 and the events are six),
and the capabilities response lists event `0x8011`.
#1584 D6a fold: one OBSERVATION_ENDED per commit (root ruling A) re-pinned it from `33e4b1cebcbfd902`: the
event carries a repeated key message and the corpus frame carries two keys (the frame count stays 46).
Issue #1678: the `SET_CONTROL` address message gains an optional
seventh field, the native effect the edit was written for, and the transaction appends a `SetControl`
on an insert row that carries it (the frame count stays 46).
#1635 retires the stored transport position, so
`TRANSPORT_SET` field 2, the snapshot's field 2 and the `TRANSPORT_STATE` event's field 3 are refused
and never reallocated (the frame count stays 46); `command.transport_set` carries the state alone.
`retired_code_rows()` gains the `TRANSPORT_SET` field 2 rows, one per flag form.
Batch 13b carries #1678 and #1635 together and re-pins it once from `cc9a7bcc502c4936` (each
alone gave `63ff2e60976e0e21` and `331654b324d076e8`).
Issue #1650 re-pinned it from `0c7196f86abe6d57`: the `Automation` message gains a required field 4,
the nested `AutomationOffset` (`left_db` 1, `right_db` 2), which the transaction's `UpsertAutomation`
carries (the frame count stays 46).

The value stood at `88a8ee6a6d9e4acc` here until #274. It was correct until `b454b230`, and the
two re-pins that followed (`b454b230`, then #241's `04d291dd`) did not reach this file or the
Wasm runner, because the parity gate over them could not fail; the arithmetic that carries
`88a8ee6a6d9e4acc` to the current value is in `docs/derivations/274-parity-repin.md`.

Issue #1386 re-pinned it from `5cab2cf3b4e4767f`: `SESSION_REPLACE` (`0x0013`) adds
`command.session_replace` (the document's length and 32-byte SHA-256, never the document) and
`response.session_replace` (the transaction reply with `edit_path` `unchanged`, 5), and the
capabilities response lists command `0x0013`. The frame count is 48.

Issue #1594 re-pinned it from `c85b2a0dff5d2a92`: `SPECTRUM_CONFIGURE` (`0x0010`) adds
`command.spectrum_configure` (a collection of two taps with the hop) and
`response.spectrum_configured` (a monitor rebuild with its plan epoch). The capabilities response
does not list `0x0010` until #1595. The 48 earlier frames are byte-equal (their hash is the old
pin). The frame count is 50.

Issue #1584 re-pinned it from `26579fcd7f959310`: the `Observe` edit (`0x0801`) appends
`edit.observe` to the transaction (an arm of an insert's tap with a window of four blocks; 37
edits). Only `command.session_transaction_apply` moves; the 49 other frames are byte-equal. The
frame count is 50.

Issue #1595 re-pinned it from `a3d6a9dd72195701`: `SPECTRUM_CAPTURE` (`0x0011`) adds
`command.spectrum_capture` (a read of both planes with the raw window) and
`response.spectrum_captured` (a ready one-plane read). The capabilities response does not list
`0x0011` until #1595's second slice. The 50 earlier frames are byte-equal (their hash is the old
pin). The frame count is 52.

Issue #1595's second slice re-pinned it from `c7e010362827dab3`: `SPECTRUM_STREAM` (`0x0012`)
adds `command.spectrum_stream` (a select with a smoothing) and `response.spectrum_streamed` (a
ready one-plane read). The capabilities response lists none of the three spectrum commands and
leaves bit 16 clear, which the invariant allows. The 52 earlier frames are byte-equal (their hash
is the old pin). The frame count is 54.

Issue #1583 re-pinned it from `c146e8fad0b26299`: the meter table replaces the positional meter
handles. `command.telemetry_configure` carries a table with both targets and both metrics (field 1
retired, field 7 repeated), `response.telemetry` the same table and the meter window (field 8),
and `event.meter_batch` the window's span and sequence (fields 5 and 6). Only those three frames
move; the 51 others are byte-equal. The frame count stays 54.

Issue #1369 re-pinned it from `480ffcf546bea7fb`: the console entry appends its required field 4,
`LINK_MODE` (`slot` 1, `dual_mono` 2, `maximum` 3, `average` 4), so the transaction's console
entries carry it. The 53 other frames are byte-equal. The frame count stays 54.

Issue #1585 re-pinned it from `06bb3b6f65f39eec`: `TransactionApplied` gains the optional arm-tag
lists `observe_arm_tags` (field 3) and `removed_arm_tags` (field 4), packed `u64`, and
`response.session_transaction` carries both. Only that frame moves; the 53 others are byte-equal.
The frame count stays 54.

Issue #1586 re-pinned it from `5b6cbc157a1f6154`: `OBSERVATION_READ` (`000f`) adds
`command.observation_read` (a console slot's tap with the left lane and an insert's with the
right) and `response.observation_read` (a ready row with both lanes, a pending armed row and an
unarmed row). The 54 earlier frames are byte-equal. The frame count is 56.

Issue #1578 re-pinned it from `f4c2fe333739f44d`: `RESPONSE_READY` (`8040`) adds
`command.response_get_strip` (a strip target), `event.response_ready` (a ready event with one side
and a member of each section) and `event.response_ready_refused` (`NOT_FOUND` with its
diagnostic), and the static capabilities frame lists `8040` with `000e`, as bit 14 now requires.
Only `response.capabilities` moves; with its event list patched back and the three new frames
filtered out, the 56 earlier frames hash to the old pin. The frame count is 59.

Issue #1690 re-pinned it from `cb8db815ed0103a0`: the meter table gains the master target (`3`,
no strip ID, gain reduction only), so `command.telemetry_configure` and `response.telemetry`
carry a third entry, the master's gain reduction. Only those two frames move; the 57 others are
byte-equal. The frame count stays 59.

Issue #2115 re-pinned it from `dea650f7e506833e`: `LOUDNESS` (`0014`) adds `command.loudness` (a
start) and `response.loudness` (an armed read with every optional level present). The static
capabilities response does not list `0014` and leaves bit 17 clear, which the invariant allows.
The 59 earlier frames are byte-equal (their hash is the old pin). The frame count is 61.
Issue #2206 re-pinned it from `de7d7cf504ca191a`: the control list request carries the optional
field 6, `headers_only` (`true`), so `command.control_list_get` and `command.control_state_get`,
which share it, move. The 59 other frames are byte-equal (with the field absent the corpus hashes
to the old pin). The frame count is 61.
