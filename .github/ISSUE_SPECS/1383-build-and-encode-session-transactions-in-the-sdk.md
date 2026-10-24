# Build and encode session transactions in the SDK

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-11).
Code anchors verified on `main` at `6fb211594`.

This is the smallest closable slice of the SDK transaction builder: the frame, the BTLV writer, and
the source, track and strip-effect edits (opcodes `0x0100`-`0x0211`). The remaining families are
#1385.

## Product outcome

An SDK user builds a session transaction with typed methods, using the same specs the
`SessionBuilder` takes, and gets the exact `SESSION_TRANSACTION_APPLY` frame the engine's protocol
codec would encode for the same edits. `engine.apply(transaction)` (*Apply session transactions from
the browser SDK*, #1296) sends these bytes unchanged. The browser and the C ABI then accept one
transaction format.

## Context

- The wire: `ProtocolCodec::encode_session_transaction`
  (`crates/protocol/src/session_wire.rs:92`) writes a 48-byte outer command header
  (`OUTER_HEADER_BYTES`, `crates/protocol/src/lib.rs:61`) with a nonzero request ID and an `Exact`
  expected revision (`session_wire.rs:74`; `Any` is refused), then one BTLV message of repeated
  `EDIT` fields. Each edit is `{ OPCODE u16, PAYLOAD message }` (`tx_edit_message`, `:330`), and
  the payload follows `schema::session::payload_spec(opcode)` (`tx_edit_payload`, `:338`). The
  opcodes are `SessionEditOpcode` (`crates/protocol/src/model.rs:22`). The format is documented in
  `docs/CONTROL_BTLV_V1.md`.
- The SDK has no session-transaction encoder. It encodes only 48-byte live command records
  (`sdk/src/core/boundary.ts:1211`). Its session model is canonical JSON built by `SessionBuilder`
  (`sdk/src/core/session.ts:679`; `.source` at `:688`, `.track` at `:754`), which validates
  `SourceSpec` and `TrackSpec`.
- Native oracle pattern: `sdk/test/render-evals.mjs:30-38` runs
  `cargo run --locked -q -p host-web --example sdk_render_oracle` and asserts the SDK against its
  answer, never a committed digest (`hosts/host-web/examples/sdk_render_oracle.rs:1-23`).
  `host-web` does not depend on `protocol` today (`hosts/host-web/Cargo.toml:25-33`).
- The per-edit ramp: *Carry an optional per-edit ramp length on live session edits* (#1394 D1)
  appends an optional `ramp_samples: u32` field after the last field of each live-value edit.
  In this slice's opcodes those are `SetTrackBuiltins`, `SetEffectLinkMode`, `SetTrackFader`,
  `SetTrackMatrixOrPan` and `SetTrackConsole`. Absent means the session default; an explicit 0 is
  a step (decision 15, D15-1).

## Decisions frozen for this slice

- **D1. Type.** `SessionTransaction` is an immutable value `{ expectedRevision: bigint; edits:
  readonly SessionEditSpec[]; bytes: Uint8Array }`. Callers never build `bytes` by hand.
- **D2. Builder.** `transaction(expectedRevision: bigint)` returns a `TransactionBuilder` with one
  method per edit in this slice:
  - sources: `upsertSource(id, SourceSpec)`, `removeSource(id)`, `setSourceContent(id, {...})`;
  - tracks: `upsertTrack(id, TrackSpec)`, `removeTrack(id)`, `setTrackSourceAssignment`,
    `setTrackBuiltins`, `setTrackRack`, `setTrackFader`, `setTrackMatrixOrPan`, `setTrackConsole`;
  - strip effects: `putTrackEffect`, `removeTrackEffect`, `setTrackEffectOrder`,
    `setEffectIdentity`, `setEffectQuality`, `setEffectBypass`, `setEffectLinkMode`,
    `setEffectSidechain`, `upsertEffectParam`, `removeEffectParam`.

  The five live-value methods of Context take a last optional argument `{ rampSamples?: number }`.
  When given, it must be an integer in `0..=4294967295` (else `MisoUsageError`) and is written as
  #1394's `ramp_samples` field; when omitted, the field is absent. The SDK does not bound it by the
  rate: the engine refuses a length above one second with `session.edit.ramp_out_of_range`.

  Each method validates its arguments with the same validators `SessionBuilder` uses, and
  `build()` returns the `SessionTransaction`. An empty transaction is refused with a
  `MisoUsageError` (the codec refuses it too).
- **D3. Encoder.** A BTLV writer in `sdk/src/core/transaction.ts` follows `docs/CONTROL_BTLV_V1.md`
  and the payload field order of `session_wire.rs`, opcode by opcode. The request ID is a nonzero
  `u64` that the SDK's engine assigns at send time. The writer leaves the request-ID bytes of
  `bytes` at 1, and the engine rewrites them in place.
- **D4. Oracle.** A new example `hosts/host-web/examples/sdk_transaction_oracle.rs` reads frames on
  stdin. For each frame it decodes with `decode_session_transaction`, re-encodes with
  `encode_session_transaction`, applies the edits with `SessionStore::apply_transaction` to a base
  session given as an argument, and prints `ok <canonical snapshot>` or the typed decode error.
  `protocol` becomes a dev-dependency of `host-web`.
- **D5. Exports.** `transaction`, `TransactionBuilder`, `SessionTransaction` and
  `SessionEditSpec` are exported from the SDK's root and browser entries. `sdk/test/barrel-surface.ts`
  follows.

## Deliverables

1. D1-D3 and D5 in `sdk/src/core/transaction.ts` and the SDK barrels.
2. D4: the oracle example and the dev-dependency line.
3. `sdk/test/transaction-evals.mjs`.

## Authorized paths

- `sdk/src/core/transaction.ts` (new), `sdk/src/index.ts`, `sdk/src/browser/index.ts`,
  `sdk/src/headless/index.ts`, `sdk/test/`
- `hosts/host-web/examples/sdk_transaction_oracle.rs` (new), `hosts/host-web/Cargo.toml`
  (`[dev-dependencies]` only)

## Non-goals

- No `engine.apply` (#1296). No decoder in the SDK.
- Opcodes `0x0001`-`0x0008` and `0x0300`-`0x0702`: *Encode the session, submix, output, route,
  automation and VCA edits in the SDK* (#1385), on the same writer and oracle.

## Objective gates

1. **Bytes equal the codec's.** In `transaction-evals.mjs`, for each method in D2 (each at least
   once; repeated fields with 0, 1 and 3 values; each live-value method with `rampSamples`
   omitted, 0 and 960), the oracle's re-encoding equals the SDK's bytes exactly. This is a wire format, so byte equality is the claim.
2. **Semantics equal the builder's.** A base session from `SessionBuilder`, a transaction that adds
   a source and a track with an EQ insert, and a second transaction that removes a track: each
   oracle snapshot equals the `toJson()` of a `SessionBuilder` that declares the result directly.
3. **Refusals.** An empty transaction, an unknown effect ID and an out-of-domain fader are refused
   by the builder with `MisoUsageError` and produce no bytes.
4. Commands, with an artifact directory built as in `qualification.yml`'s `artifact` job and
   `npm ci` in `sdk/`: `bash scripts/check-sdk-types.sh`,
   `bash scripts/check-sdk-headless.sh <artifacts>` (it runs `test/*-evals.mjs`),
   `bash scripts/check-sdk-generated.sh <artifacts>`, `python3 -B scripts/check-sdk-deletions.py`,
   `bash scripts/sdk-package.sh check <artifacts>`;
   `cargo build --locked -p host-web --examples`, `cargo fmt --all -- --check`,
   `cargo clippy --locked --workspace --all-targets -- -D warnings`,
   `bash scripts/check-workspace-policy.sh`.

## Test value

- Gate 1: a field written out of order, a wrong field count, a missing repeated value or a wrong
  integer width makes the codec decode different edits or refuse the frame; it turns red. So does
  an SDK that writes 0 for an omitted `rampSamples` (the engine would step instead of using the
  session default). No existing test encodes a transaction in TypeScript.
- Gate 2: an encoder that is byte-valid but maps a spec field to the wrong session field (for
  example, left and right builtins swapped) produces a different snapshot; it turns red.
- Gate 3: a builder that emits a frame the engine must refuse would cost a round trip and hide the
  path of the error; it turns red.

## Dependencies

- *Carry an optional per-edit ramp length on live session edits* (#1394), for the
  `ramp_samples` field. #1296 and #1385 depend on this issue.

## Batch verdict follow-ups (stream E2 batch, 2026-10-12)

- *m3, Root ruling (2026-10-12):* the non-goal's opcode range read `0x0001`-`0x0007`, which was stale:
  #1385's Amendment 1 adds `SetControlSmoothing` (`0x0008`) to the opcodes that slice encodes, so the range
  now reads `0x0001`-`0x0008`.

**Decision 17 amendment (2026-10-13).** Every value edit is one `SET_CONTROL` (opcode `0x0800`); the per-kind value ops are retired by #1561. #1563 adds the SDK's `SET_CONTROL` frame encoder (`sdk/src/core/control-frame.ts`) and the per-engine transaction queue; this issue reuses both and encodes only the structural edits.

## Amendment (decision 17, root 2026-10-13)

**Order (root Q7).** This slice lands after #1563.

## Scope split and decisions (root, 2026-10-24; supersedes D1-D5 where they differ)

- **Split.** 1383a (this branch): the BTLV primitives, every entity encoder a track nests, the
  oracle, the source and track methods and their evals. 1383b (later stream): `setTrackRack`,
  `putTrackEffect`, `removeTrackEffect`, `setTrackEffectOrder`, `setEffectIdentity`,
  `setEffectQuality`, `setEffectSidechain`, `setTrackConsole` (with `rampSamples`), on the same
  encoders and oracle.
- **Decision-17 drift.** D2's `setTrackBuiltins`, `setTrackFader`, `setTrackMatrixOrPan`,
  `setEffectBypass`, `setEffectLinkMode`, `upsertEffectParam` and `removeEffectParam` are gone: a
  value is one `SET_CONTROL` (#1561). 14 methods remain; only `setTrackConsole` keeps `rampSamples`.
- **D2 builder.** `transaction(base: SessionBuilder)`, not `transaction(expectedRevision)`: a track
  is validated against the base's rate, sources and console slots. The builder keeps a working
  source and track list, so a later edit sees an earlier one of the same transaction. It is
  immutable, as `SessionBuilder` is.
- **D1/D3 frame.** `SessionTransaction = { edits, payload, fieldCount }`. The request ID and the
  expected revision are stamped at send by `encodeSessionTransaction(tx, { requestId,
  expectedRevision })` (`@internal`, for #1296) through `control-frame.ts`'s `commandFrame`, as
  #1563's queue does. No baked `bytes`, no request ID rewritten in place. The module is
  `sdk/src/core/session-transaction.ts`.
- **One validator.** Edits encode the normalized record of `session.ts`'s own normalizers
  (`normalizeSource`, extracted from `SessionBuilder.source`; `normalizeTrack`, which now also
  refuses unknown track keys; `trackSourceRef`; `resolveLanes`; `builderContext`), exported
  `@internal`. The root barrel now names `core/session.ts`'s public exports instead of
  `export *`, so those internals stay off it; `barrel-surface.ts` pins both.
- **D4 oracle.** Frames on stdin back to back, applied in order to one store; one JSON line per
  frame: `{"bytes", "ok"}`, `{"bytes", "refused"}` or `{"decode"}`. `protocol` was already a
  dev-dependency; no `Cargo.toml` change.
- **D5.** `transaction`, `TransactionBuilder`, `SessionTransaction`, `SessionEditSpec` on the root,
  browser and headless entries (headless exports `session()` too).
- **Gate 3 in 1383a.** Empty, unknown effect, undeclared source, fader out of domain, removing an
  undeclared source. The `rampSamples` refusal is 1383b's.
- **Open owner-level note (not blocking).** After #1386 an "edit the builder, `replaceSession`"
  shape could supersede this encoder; the typed builder stays as ruled (D15-11).

## Tranche 1383b decision record (2026-10-24)

- **Methods.** `setTrackRack(trackId, inserts)`, `putTrackEffect(trackId, finalPosition, effect)`,
  `removeTrackEffect(trackId, slotId)`, `setTrackEffectOrder(trackId, slotIds)`,
  `setEffectIdentity(trackId, slotId, effectId)`, `setEffectQuality(trackId, slotId, "normal")`,
  `setEffectSidechain(trackId, slotId, sidechain | null)`,
  `setTrackConsole(trackId, entries, { rampSamples? })`. No other new export.
- **Strip and rack.** The strip is a track or a base submix (the engine's `strip_mut`, #1204). The
  methods take no rack name and write `inserts` (code 2): the engine refuses `builtins`
  (`NotFound`) and `console` (`ConsoleSlotFixed`) for every one of these edits. An insert is named
  by its stable ID (`effect()`'s `slotId`). The builder keeps each strip's working insert records
  (from `base.toJSON()`, then each edit), so a later edit sees an earlier one.
- **Validators.** `normalizeInserts`, a new shared `normalizeInsert`, `normalizeConsoleEntries`,
  `quality` and a new shared `normalizeSidechain` (now also used by `effect()`) are exported
  `@internal` from `session.ts`. `putTrackEffect` requires `slotId`: a position-derived ID could
  name, and so replace, another insert. `setEffectIdentity` uses a new `retargetInsert`, which
  checks the insert's link mode, sidechain port and every `params` row (parameter ID, unit, lane,
  value) against the new descriptor with `effect()`'s own validators, as the engine's preparation
  refuses a mismatch.
- **`setTrackConsole`.** The engine refuses it unless an earlier `SetConsole` of the transaction
  changed the console (`session.edit.value_op_retired`, #1561 D3). This builder has no
  `setConsole` (#1385), so it validates the strip, the entries against the slots and `rampSamples`
  (`requireU32`), then refuses with that code. The encoder (track ID, repeated entries, optional
  field 3 `ramp_samples` written only when given) is in place, but its bytes and its ramp rows
  (omitted, 0, 960) cannot reach the oracle until #1385's `setConsole` opens it; those rows belong
  to #1385's gate 1.
- **Evals.** Gate 1 gained one frame of all seven rack and effect edits on a track and a submix
  (racks and orders at 0, 1 and 3); gate 2 one snapshot of put, reorder, retarget, reroute, remove
  and a submix rack; gate 3 one test of their refusals and of `setTrackConsole` (no console change;
  `rampSamples` -1, 1.5, 2^32, "960").
