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
