# Red-mutation log — `protocol` tests

## Issue #178 — the `builtins` rack token

Driver: one mutation at a time on the committed tree,
`cargo test -p protocol --test session_edit_builtins_rack`, tree restored between rows.

| # | mutation | file | test | result |
|---|---|---|---|---|
| P3-M45 | `rack_mut`'s `RackName::Builtins` arm returns `Ok(&mut track.inserts)` instead of refusing (re-run on #1093's console shape, where `track.simd1` no longer exists) | `protocol/src/model.rs` | `rack_addressed_edits_refuse_the_builtins_token` (named `..._and_console_tokens` until #1094 gave `console` its own refusal) | RED — the first rack-addressed edit, `SetTrackRack` at `Builtins`, reports `Ok(())` instead of `Err(NotFound)` |

The two positive tests in that file are what stop a refusal that refuses everything from passing:
the same edits against `RackName::Inserts` are applied, and the strip is still editable through
`SetTrackBuiltins`, which is the edit that owns it.

A third shape is worth naming and is **not** a mutation, because it is what the arm was written to
avoid: an `unreachable!()` there would abort the control thread on a well-formed wire message. The
wire decodes `RACK = 4` into `RackName::Builtins` by construction (`schema.rs`), so a rack-addressed
edit carrying it is a message a peer can legally send.

## Issue #241 — deleted session-edit opcodes

Applied on 2026-08-29, run, observed RED, and reverted.

| gate | mutation | observed red |
|---|---|---|
| `session_wire::tests::deleted_source_and_limits_opcodes_are_typed_refusals` | alias deleted `0x0006` (`SetLimits`) to the live `SetSourceContent` decoder in `SessionEditOpcode::from_raw` | decode returns `Ok(SetSourceContent { … })` where `Err(InvalidTlv)` is required; the assertion names deleted opcode `0x0006` before payload dispatch |

The same gate independently mutates the opcode TLV to deleted per-source-rate `0x0102` and source
mapping `0x0104`. Its positive neighbor round-trips the new `{content,channels,bit_depth,frames}`
payload canonically, so the refusal cannot pass by disabling source-edit decoding wholesale.

## Issue #1094 — console session edits and retired codes

Driver: a scratch script (PR evidence, not committed), one mutation at a time on `ae4c0561`,
`cargo test --locked -p protocol --no-fail-fast`, file restored after each row. All nine RED.

| # | mutation | file | red tests |
|---|---|---|---|
| M1 | `rack_mut` routes `Console` to the track's inserts | `model.rs` | `console_structural_and_declaration_edits_refuse_with_a_typed_status`, `retired_and_console_refused_codes_meet_their_conformance_rows` |
| M2 | `rack_mut` answers `NotFound` for `Console` | `model.rs` | the same two |
| M3 | knob edits skip the console entry (`Console` falls through to `effect_mut`) | `model.rs` | `console_knob_edits_change_only_that_tracks_entry`, `every_console_edit_round_trips_to_the_document_the_parser_builds` |
| M4 | `parse_track` reads with plain `schema_spec` (optional retired fields skipped) | `session_wire.rs` | `retired_track_rack_fields_are_refused`, `retired_and_console_refused_codes_meet_their_conformance_rows` |
| M5 | rack code `1` decodes as `inserts` | `schema.rs` | `parameter_enum_wire_mappings_are_exhaustive_and_roundtrip`, `retired_and_console_refused_codes_meet_their_conformance_rows` |
| M6 | `SetConsole` applied as a no-op | `model.rs` | `every_console_edit_round_trips_...`, `a_slot_set_change_that_skips_a_track_refuses_whole`, `a_slot_set_change_that_skips_a_track_is_never_acknowledged` |
| M7 | the console decoder reads `pre_insert` from the `post_insert` field | `session_wire.rs` | `console_declaration_and_track_entry_opcodes_round_trip_canonically` and the three above |
| M8 | `ConsoleSlotFixed` reported as `session.edit.not_found` | `controller.rs` | `retired_and_console_refused_codes_meet_their_conformance_rows` |
| M9 | `SetTrackConsole` writes the first track's entries | `model.rs` | `every_console_edit_round_trips_...`, `a_slot_set_change_that_skips_a_track_refuses_whole` |
