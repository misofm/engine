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
- Opcodes `0x0001`-`0x0007` and `0x0300`-`0x0702`: *Encode the session, submix, output, route,
  automation and VCA edits in the SDK* (#1385), on the same writer and oracle.

## Objective gates

1. **Bytes equal the codec's.** In `transaction-evals.mjs`, for each method in D2 (each at least
   once; repeated fields with 0, 1 and 3 values), the oracle's re-encoding equals the SDK's bytes
   exactly. This is a wire format, so byte equality is the claim.
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
  integer width makes the codec decode different edits or refuse the frame; it turns red. No
  existing test encodes a transaction in TypeScript.
- Gate 2: an encoder that is byte-valid but maps a spec field to the wrong session field (for
  example, left and right builtins swapped) produces a different snapshot; it turns red.
- Gate 3: a builder that emits a frame the engine must refuse would cost a round trip and hide the
  path of the error; it turns red.

## Dependencies

- none. #1296 and #1385 depend on this issue.
