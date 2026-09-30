# Carry the session console and inserts in the control protocol

Slice S1b of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's H4, M5 and amendment 5 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

After S1a, the session model has a session-level `console` and per-track `console` entries and
`inserts`, and the protocol encodes that model. The protocol's session edits cannot express the
change yet:

- `SetTrackRack` (0x0204) and 0x0205-0x020e address an effect by `rack_name`
  (`crates/protocol/src/model.rs`). `rack_name` 1 and 3 are now retired, and nothing edits a
  console declaration or a track's console entry.
- Protocol mutations must update the same typed session model and be snapshot-able back to
  canonical JSON (AGENTS.md).

## Smallest closable slice

1. Rack-addressed edits:
   - The existing effect edits keep working for `rack_name` = `inserts` (2).
   - They refuse the retired codes 1 and 3.
   - For `rack_name` = `console`, `SetEffectBypass` (0x020a), `UpsertEffectParam` (0x020d) and
     `RemoveEffectParam` (0x020e) edit one track's console entry, with the slot as the effect ID.
   - Every structural or declaration edit refuses `console` with a typed status: `SetTrackRack`,
     `PutTrackEffect`, `RemoveTrackEffect`, `SetTrackEffectOrder`, `SetEffectIdentity`,
     `SetEffectQuality`, `SetEffectLinkMode` and `SetEffectSidechain`. A track cannot add, remove or
     reorder a console slot.
2. Console declaration edits. Append new opcodes and reuse no retired code. Proposed:
   - one that replaces the session's `console` declaration;
   - one that replaces a track's whole `console` entry array.

   A transaction that changes the slot set must rewrite every track's entries in the same atomic
   transaction. Otherwise it refuses as a whole, and no partial model is committed. S1b may choose a
   different edit set if it covers the same changes; record why.
3. Registry: `docs/CONTROL_PROTOCOL_REGISTRY.md` and `docs/CONTROL_BTLV_V1.md` record:
   - the renamed tap and rack tokens;
   - the retired rack codes and track fields;
   - the appended IDs and opcodes;
   - the in-place V1 amendment on the #1063 precedent.

   `COMPLETE_SCHEMA_HASH` (`crates/conformance/src/protocol_corpus.rs:249`) is repinned, and the
   corpus covers every new opcode.

Authorized paths: `crates/protocol/**`, `crates/conformance/src/protocol_corpus.rs`, the session
transaction apply path, the two protocol docs, and this spec.

## Owner decisions that bind this slice

Decision 12's "Wire identity": nothing is renumbered, every retired code is refused and never
reallocated, and there is no `ABI_VERSION` or protocol-major bump.

## Dependencies

- *Add the session console and per-track inserts to the session schema* (S1a, #1093).

This is batch C3, which is not pushed until S1d passes its gates.

## Objective gates

1. The protocol corpus passes with the repinned `COMPLETE_SCHEMA_HASH`, and native-wasm parity
   holds.
2. Every retired code is refused, not reinterpreted: `rack_name` 1 and 3, track fields 6 and 8, and
   each console-refused edit. Each has a conformance row.
3. Every console edit round-trips: the committed model's canonical JSON equals the document that
   S1a's parser builds from the same edits.
4. A transaction that changes the slot set without rewriting a track's entries refuses atomically,
   and the prior revision stays committed.
5. `cargo test -p protocol -p conformance -p session` pass, as does `bash scripts/check-session-policy.sh`.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance. NaNs fold to one
  value (decision 10).
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value"). A
  one-time "no bit moved" comparison against the pre-change base is PR evidence, not a committed
  test.

## Attempt 1 evidence

Terra, attempt 1, on `codex/1094-protocol-console` from `7ff25845` (S1a after its C3 rebase).
Implementation commit `ae4c0561`; every gate below ran on that tree, x86-64-v3.

### What landed

- **Edit set (the spec's proposal, kept).** `SetConsole` `0x0007` replaces the whole console
  declaration (both sections; each slot's `slot`, `identity`, `quality`, `link_mode`).
  `SetTrackConsole` `0x0211` replaces one track's whole entry array. Each is the next unallocated
  code in its family: `0x0006` stays retired (#241), and `0x0211` follows `0x0210`. Payloads
  mirror the session model's own registries. Console is `1*` pre-insert slot and `2*` post-insert
  slot. A slot is `1` slot, `2` identity, `3` quality and `4` link mode, with no sidechain field.
  An entry is `1` slot, `2` bypass and `3*` param. `SetTrackConsole` is track ID `1`, then
  entries `2*`. There are 41 opcodes.
- **Rack-addressed edits** (`crates/protocol/src/model.rs`).
  - `inserts` works as before.
  - `SetEffectBypass`, `UpsertEffectParam` and `RemoveEffectParam` at `console` edit only the named
    track's entry for the slot in `effect_id`. A slot the track does not carry is `NotFound`.
  - The eight structural and declaration edits at `console` return the new
    `SessionEditError::ConsoleSlotFixed`, which the controller reports as `VALIDATION_FAILED`,
    `session.edit.console_slot_fixed`, with the operation index. The refusal is checked before the
    track lookup, so it is a property of the edit, not of the model.
  - `builtins` is still `NotFound` (#178).
- **Retired codes.**
  - Rack codes 1 and 3 were already refused at decode.
  - Track fields 6 and 8 are now refused in the optional flag form too
    (`Fields::schema_spec_retiring` with `schema::session::track::RETIRED`): `InvalidTlv`, where
    the mandatory form is `UnknownRequiredField`. This closes S1a's verdict info item.
- **Atomicity (choice recorded).** The spec asks that a transaction changing the slot set rewrite
  every track's entries, or refuse whole. No new mechanism was needed: `SessionStore` already
  applies every edit to a candidate and validates only the final model before anything is
  committed.
  - S1a's validation refuses a skipped track as `console.entry_missing` (a slot added),
    `reference.missing_entity` (a slot removed or renamed) or `console.entry_order` (reordered), at
    that track's path. So the rewrite may come in any order within the transaction.
  - A declaration change that keeps the flat slot sequence leaves every entry valid and needs no
    rewrite: a quality, link-mode or identity change, or a slot moved across the section boundary.
    Entries carry only knobs, and effect preparation judges parameters against a new identity, as
    it does for `SetEffectIdentity` on an insert.
  - `SetTrackConsole` cannot change the slot set either: the same validation refuses an added,
    dropped or reordered entry.
  - Acked-batch question: the controller encodes the ack only after `prepare_transaction`
    (validation included) succeeds, and the commit after it cannot fail. No ack precedes a refusal,
    and a replay returns the same refusal.
- **Conformance rows** (`crates/conformance/src/protocol_corpus.rs`). `retired_code_rows()` builds
  37 wire frames against `console_session_fixture()` (the canonical fixture plus one `pre_insert`
  slot `desk-eq` and its entry):
  - rack codes 1 and 3 in all eleven rack-addressed edits and in `SetAutomationTarget`, patched
    only at an asserted mandatory `U8` holding `2` (24 rows, `MALFORMED_FRAME`);
  - track fields 6 and 8, mandatory (`UNKNOWN_REQUIRED_FIELD`) and optional (`MALFORMED_FRAME`),
    spliced into an `UpsertTrack` with every enclosing length grown (4 rows);
  - the eight console-refused edits (`VALIDATION_FAILED`, `session.edit.console_slot_fixed`);
  - one control row, an optional never-allocated field 12 spliced the same way, which must commit.
    A broken splice therefore cannot pass as a refusal.

  Each row runs through the controller's full-frame path. Every refusal leaves revision 7, the
  snapshot and the event queue unchanged.
- **Corpus and pins.** `complete_all_opcode_fixture()` adds one `SetConsole` (a slot in each
  section) and one `SetTrackConsole` (two entries), 41 edits, still 46 frames.
  `COMPLETE_SCHEMA_HASH` `af1b9b71a0a31727` -> `ebf282621550d44a`, with its copies in the parity
  self-test, `docs/CONTROL_PROTOCOL_CONFORMANCE.md` and `fuzz/corpus/complete-schema-manifest.md`.
  The controller's edit-limit boundary row moved 38 -> 40.
- **Docs.** `docs/CONTROL_PROTOCOL_REGISTRY.md` records:
  - the renamed tap tokens (codes 1-7, old spellings named) and rack tokens;
  - the retired rack codes and track fields, with their refusal statuses;
  - the appended rack code, track field, console messages and opcodes;
  - console addressing and the slot-set atomicity rule;
  - the in-place v1 amendment on the #1063 precedent, with no major, minor or `ABI_VERSION` bump.

  `docs/CONTROL_BTLV_V1.md`'s compatibility section names the prelaunch in-place amendment rule
  (#1063, decision 12).

### Files outside the spec's authorized list, with the gate that needed each

- `crates/conformance/src/lib.rs`: re-exports `RetiredCodeRow`, `retired_code_rows` and
  `console_session_fixture` (gate 2's conformance rows).
- `scripts/check-protocol-wasm-parity.sh`: the self-test's two current-pin literals (gate 1's
  `--self-test`).
- `docs/CONTROL_PROTOCOL_CONFORMANCE.md` and `fuzz/corpus/complete-schema-manifest.md`: the
  hash and opcode count, which S1a's repin chain also carried.
- `crates/protocol/tests/MUTATIONS.md` is in `crates/protocol/**`. It records the #1094 rows, and
  the renamed builtins test in P3-M45.

### Gates

| Gate | Evidence |
|---|---|
| 1. Corpus with the repinned hash; native-wasm parity | `conformance_corpus` passes at `ebf282621550d44a`; `check-protocol-wasm-parity.sh`: `ok (simd128)`; `--self-test`: 1 inert-invocation row and 3 red rebuilds refused |
| 2. Every retired code refused, one conformance row each | `retired_and_console_refused_codes_meet_their_conformance_rows` (37 rows, above); `retired_track_rack_fields_are_refused` (both flag forms plus the field-12 control); `opcode_registry_appends_the_console_edits_and_reallocates_nothing` (41 codes; `0x0006`, `0x0102`, `0x0104` still `None`) |
| 3. Every console edit round-trips to S1a's parser | `every_console_edit_round_trips_to_the_document_the_parser_builds`: bypass, param upsert, param removal, `SetTrackConsole`, declaration-only `SetConsole`, an added `post_insert` slot, both sections emptied, and a slot moved across sections. Each goes through encode/decode and a store commit, and the snapshot equals `canonical_session_json(parse_session_json(<hand-written document>))`. Also `console_declaration_and_track_entry_opcodes_round_trip_canonically` (both sections, empty sections, a `cid` identity, every quality and link mode) |
| 4. A slot-set change that skips a track refuses atomically | `a_slot_set_change_that_skips_a_track_refuses_whole`: add, remove and reorder; the refusal names only `$.tracks[1]`; revision, snapshot and model unchanged; the full rewrite commits. `a_track_cannot_change_the_slot_set_through_its_entries`. `a_slot_set_change_that_skips_a_track_is_never_acknowledged` (controller: `VALIDATION_FAILED` `console.entry_missing`, no event, the replay gives the same refusal) |
| 5. `cargo test -p protocol -p conformance -p session`; `check-session-policy.sh` | pass (dev and release); `session policy: ok`, `test-session-policy.sh` PASS |
| `cargo fmt --all --check`; clippy `--workspace --all-targets --all-features -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | pass |
| `cargo test --locked --workspace --all-features --no-fail-fast` (dev) | 293 result lines, 2,103 passed, 0 failed, 40 ignored (capi and host-core size pins unmoved) |
| Release: `cargo test --release -p protocol -p conformance -p session -p console-workload` | 39 result lines, 303 passed, 0 failed |
| Console-workload digests | `gain_pan_profile digests` (64 blocks): all 22 rows identical to base `7ff25845` (a `git archive` build). `console-workload` does not depend on `protocol` or `conformance` |
| Wasm `simd128` | `cargo check --target wasm32-unknown-unknown` with `+simd128` for `target-smoke`, `protocol`, `dsp-reference` and `conformance`: pass |
| AArch64 iOS and Android | `check-cross-targets.sh`: PASS (the known #1018 memset rows are expected failures) |
| Fuzz targets | `cargo check --locked --manifest-path fuzz/Cargo.toml --bins`: pass (the spec names no fuzz run) |
| Policy scripts | pass: protocol-control check and mutation tests; conformance-boundaries check and mutation tests; session; workspace; env-vocabulary; bench; host-core; realtime and audit-leak; artifact-evidence-leak; lane; unfused seal; rack; builtins; graph; effect-runtime and its fixtures; test-support-ci; script reachability; console benchmark fixture; bench preconditions; parametric-EQ contract; release-shape self-test; DSP research. No lint-job script reads the two protocol docs |
| `check-ci-path-routing.py`, `test-ci-path-routing.py` | pass |

Test value: nine one-at-a-time mutations on `ae4c0561` each turned at least one new test red
(`crates/protocol/tests/MUTATIONS.md`, "Issue #1094").

Not run, as scoped: the SDK headless and package suites (S1d, #1097; they send old-schema
documents), live control (S1c, #1096), and the AudioWorklet artifact pin (the batch boundary).
