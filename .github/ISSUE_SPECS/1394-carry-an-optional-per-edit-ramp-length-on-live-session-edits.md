# Carry an optional per-edit ramp length on live session edits

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-3).
Code anchors verified on `main` at `6fb211594`.

Split from *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan
changes* (#1054), which keeps the session key, the default table and `LiveRamps`. This issue owns
the per-edit half: the wire field, `EditRamps`, the refusal and the classifier lookup.

## Product outcome

A host that edits a live value by session transaction may give that one edit its own ramp length
in samples. The C ABI's live fader, mute and pan/matrix records then ramp over the edit's length;
an edit with no length ramps over the session's `control_smoothing` (or the default table), as
#1054 delivers. An explicit 0 on an edit is legal and steps that edit only. A length above one
second at the session rate is refused before anything commits. The length is not model state:
canonical JSON never shows it, and the next edit without a length uses the session default again.

## Context

- **Decision 15, D15-1 recorded resolution**: every live edit carries an optional
  per-edit ramp length end to end; absent means the session default and an explicit 0 is legal. The
  bypass crossfade (*Crossfade the bypass switch over the session ramp*, #1341) is the one row with
  no per-edit length; it always uses the session mute ramp.
- **What #1054 delivers first.** `LiveRamps` (`crates/host-core/src/live_delta.rs:33-58` today)
  gains `pan_samples` and `link_samples`, `for_session` fills all four from the session, and
  `for_row(row)` returns the D3 key field for a row (#1054 D3, D4). The matrix precedence is the
  post-commit model's non-zero `smoothing_samples`, else `pan_samples` (#1054 D5).
- **The transaction.** `SessionStore::prepare_transaction` (`crates/protocol/src/model.rs:917-954`)
  applies each edit with `apply_session_edit` (`:529`) to a candidate model and returns
  `PreparedSessionTransaction { base_revision, compiled, applied_operations }` (`:813-817`). An edit
  leaves nothing but model state, so no per-edit ramp can reach the classifier today.
- **The live-value edits** (`SessionEdit`, `crates/protocol/src/model.rs`): `SetConsole` (`:204`),
  `SetTrackBuiltins` (`:233`), `SetEffectLinkMode` (`:306`), `SetTrackFader` (`:352`),
  `SetTrackMatrixOrPan` (`:358`), `SetTrackConsole` (`:368`), `SetRouteChannelMatrix` (`:397`),
  `SetRouteGainDb` (`:402`), `SetRouteMute` (`:404`), `SetRouteFollowsMute` (`:407`),
  `SetVcaFader` (`:434`). A track ID names a
  strip, a track or a submix (`docs/CONTROL_PROTOCOL_REGISTRY.md:77`).
- **The wire.** Payload specs are in `crates/protocol/src/schema.rs` (for example
  `set_track_fader`, `:1371-1379`); optional fields exist (`FieldSpec::opt`, `:181`). Encode is `tx_edit_payload`
  (`crates/protocol/src/session_wire.rs:338-623`), decode is `parse_edit` (`:1171-1422`). The session-edit table is
  `docs/CONTROL_PROTOCOL_REGISTRY.md:47`.
- **Refusals.** `SessionEditError` (`crates/protocol/src/model.rs:497`) maps to diagnostic codes
  at `crates/protocol/src/controller.rs:3528-3537`.
- **The classifier.** `classify_live_delta(current, next, ramps)` (`live_delta.rs:211-215`) puts
  `ramps.fader_samples` on `FaderDb` records (`:293`) and `ramps.mute_samples` on `Mute` records
  (`:301`); the matrix record carries `smoothing_after` (`:308-313`). `commit_live` calls it once per
  commit (`crates/capi/src/runtime/control.rs:1071-1075`), with `next` from the prepared token's
  `prospective_session()` (`crates/protocol/src/controller.rs:1139`).
- **Crate boundary.** `host-core` depends on `protocol` only under its `control-provider` feature
  (`crates/host-core/Cargo.toml:15`, `:32`), and `live_delta` is always built (`lib.rs:98`). Both
  crates depend on `session`.
- **The corpus.** `crates/conformance/src/protocol_corpus.rs` encodes one edit per opcode, a
  `SetTrackFader` among them (`:186-189`), and pins `COMPLETE_SCHEMA_HASH` with a re-pin history
  (`:715-737`). The hash is also spelled in `scripts/check-protocol-wasm-parity.sh:172-173`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and `fuzz/corpus/complete-schema-manifest.md`.

## Decisions frozen for this slice

- **D1. Wire.** Each live-value edit of Context appends one optional field, `ramp_samples: u32`,
  after its last field (`FieldSpec::opt`). The typed variants gain `ramp_samples: Option<u32>`;
  absent is `None`. This is an in-place V1 protocol field: no new opcode and no C ABI feature bit
  (D15-3 recorded resolution). The registry's session-edit table records the field per opcode.
- **D2. Types, in `crates/session`** (new module `edit_ramp.rs`, so `protocol` and `host-core` share
  them without a new dependency). `EditRampTarget` is `Strip(StableId)`,
  `Effect { strip, rack: RackName, effect }`, `Route(StableId)`, `Vca(StableId)` or `Console`.
  `EditRampRow` is `Fader`, `Mute`, `Matrix`, `Input`, `Link`, `RouteGain`, `RouteMute`,
  `RouteMatrix`, `VcaFader` or `VcaMute`. `EditRamps` is a small owned map from
  `(EditRampTarget, EditRampRow)` to `u32`, with `get(target, row) -> Option<u32>`. Its doc says it
  is transaction metadata, never model state.
- **D3. What each edit records.** One entry per row the edit writes:

  | edit | target | rows |
  |---|---|---|
  | `SetTrackFader` | `Strip` | `Fader`, `Mute` |
  | `SetTrackMatrixOrPan` | `Strip` | `Matrix` |
  | `SetTrackBuiltins` | `Strip` | `Input` (trim and polarity) |
  | `SetTrackConsole` | `Strip` | `Link` (the strip's console-entry overrides) |
  | `SetEffectLinkMode` | `Effect` | `Link` |
  | `SetConsole` | `Console` | `Link` (slot defaults) |
  | `SetRouteGainDb` / `SetRouteMute` / `SetRouteChannelMatrix` | `Route` | `RouteGain` / `RouteMute` / `RouteMatrix` |
  | `SetRouteFollowsMute` | `Route` | `RouteMute` (a follow toggle ramps per the mute key) |
  | `SetVcaFader` | `Vca` | `VcaFader`, `VcaMute` |

  The last edit in the transaction that writes a `(target, row)` decides it: `Some(n)` sets the
  entry, `None` removes it. `UpsertTrack`, `UpsertSubmix`, `UpsertRoute` and `UpsertVca` carry no
  field and remove every entry for their target (and, for a strip, its effects), because they
  rewrite those rows with no length.
- **D4. Transaction.** `prepare_transaction` builds `EditRamps` beside the candidate model and
  stores it in `PreparedSessionTransaction`, exposed as `edit_ramps(&self) -> &EditRamps`. A value
  above the candidate's `sample_rate_hz` (1000 ms, the bound of #1054 D1) refuses the transaction
  with the new `SessionEditError::RampOutOfRange` (`session.edit.ramp_out_of_range`) at that edit's
  index, before anything commits. The check uses the candidate's rate after all edits, so a
  transaction that also changes the rate is checked against the rate it commits.
- **D5. `LiveRamps::resolve`.** `resolve(row: LiveRampRow, edit_ramp: Option<u32>) -> u32` returns
  the edit's value when present, else `for_row(row)` (#1054 D4). After this slice the classifier
  reads every record length through `resolve`; nothing reads a `LiveRamps` field directly. The
  matrix precedence becomes: the edit's ramp; else #1054 D5.
- **D6. Classifier.** `classify_live_delta(current, next, ramps, edit_ramps: &EditRamps)` looks up
  each record's `(target, row)` and passes it to `resolve`. This slice converts every record kind
  the classifier produces on `main` when it merges. A record that a VCA produces on a member strip
  looks up its `Vca` target; a send's follow record looks up its source strip's `Mute` row
  when the source's mute moved, and its route's `RouteMute` row when the `follows_mute` flag moved. The row
  slices that depend on this issue (#1261, #1225, #1226, #1247, #1390, #1371, #1236) read their
  lengths through the same lookup and `resolve`. `commit_live` passes the prepared token's
  `edit_ramps()`.
- **D7. Acked-batch question.** No queue changes; records only carry a different length. The
  refusal runs in preparation, before any push or commit. No ack can precede a drop.

## Deliverables

1. D2 in `crates/session/src/edit_ramp.rs` and its re-export in `lib.rs`.
2. D1, D3 and D4 in `crates/protocol`, with the registry rows.
3. Every construction of a D1 variant outside `crates/protocol` gains `ramp_samples: None`
   (mechanical).
4. D5 and D6 in `crates/host-core/src/live_delta.rs`, and the call change in
   `crates/capi/src/runtime/control.rs`.
5. The corpus: its `SetTrackFader` edit sets `ramp_samples`, with one re-pin (gate 5).
6. `docs/C_ABI_V1_QUALIFICATION.md`: a live edit's own length overrides the session's.
7. The tests below.

## Authorized paths

- `crates/session/src/edit_ramp.rs` (new), `crates/session/src/lib.rs`
- `crates/protocol/src/{model,schema,session_wire,controller}.rs`,
  `crates/protocol/src/session_wire/tests.rs`, `crates/protocol/src/controller/tests.rs`,
  `crates/protocol/tests/` (stream B owns `crates/protocol`; root sequences the merge)
- The re-pin set: `crates/conformance/src/protocol_corpus.rs` (`COMPLETE_SCHEMA_HASH` and its
  history), `scripts/check-protocol-wasm-parity.sh` (the two pinned hash lines),
  `fuzz/corpus/complete-schema-manifest.md`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md`,
  `docs/CONTROL_PROTOCOL_REGISTRY.md`
- `crates/host-core/src/live_delta.rs` (stream B owns it), `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/control.rs` (the `commit_live` call only),
  `crates/capi/src/runtime/live_tests.rs` (tests only), `docs/C_ABI_V1_QUALIFICATION.md`
- Field additions only (deliverable 3), in any file that builds one of D1's variants

## Non-goals

- The session key, defaults, rounding and `LiveRamps` fields (#1054).
- Browser records into transactions: *Admit browser live edits in the Worker through the
  committed model* (#1382) lowers a record's length word into D1's field.
- SDK encoding of the field: *Build and encode session transactions in the SDK* (#1383) and
  *Encode the session, submix, output, route, automation and VCA edits in the SDK* (#1385).
- Rows that are not live on the C ABI yet (trim, polarity, sends, VCA, link): their issues apply D6.
- No per-edit length for bypass (#1341) or for effect parameters (their ramps are the
  descriptor's).

## Hazards

- The prepared token's retained layout is counted by the C ABI resource contract
  (`crates/protocol/src/controller.rs:1129-1134`). If `EditRamps` moves a counted size, the PR
  states the new count and why.
- If *Edit control_smoothing by a session transaction, model-only* (#1365) merges first, the
  corpus carries its edit too; re-pin from the value on `main`, never by hand-editing history.

## Objective gates

1. **Wire** (new tests in the existing `crates/protocol/src/session_wire/tests.rs`). Each D1 edit round-trips with
   `ramp_samples` absent, 0 and 96000.
2. **Recording** (`crates/protocol` model test, new). For a transaction of `SetTrackFader(a, 37)`,
   `SetTrackMatrixOrPan(a, None)`, `SetRouteGainDb(r, 0)`, `SetTrackFader(b, 5)` then
   `SetTrackFader(b, None)`: `edit_ramps()` holds exactly `(Strip a, Fader) = 37`,
   `(Strip a, Mute) = 37` and `(Route r, RouteGain) = 0`. An `UpsertTrack(a)` after the first edit
   leaves no entry for `a`.
3. **Refusal** (same file). At 48 kHz, `ramp_samples = 48000` commits; 48001 refuses with
   `session.edit.ramp_out_of_range` at that edit's index, and the revision and model are unchanged.
4. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). At 48 kHz with the default table
   and a hand-built `EditRamps`: entries 37 on `(Strip, Fader)` and `(Strip, Matrix)` give
   `FaderDb` 37, `Mute` 480, matrix 37 (37 also when the model says 96); an entry 0 on
   `(Strip, Mute)` gives 960, 0, 960; an entry on another strip changes nothing.
5. **Corpus.** `cargo test --locked -p conformance --test conformance_corpus` (which checks
   `COMPLETE_SCHEMA_HASH`) and `bash scripts/check-protocol-wasm-parity.sh` (every other spelling
   of the hash agrees) pass with one re-pin, its history line reading
   "#1394 appends optional `ramp_samples` to the live-value edits".
6. **C ABI** (`crates/capi/src/runtime/live_tests.rs`, new). On a playing session with the default
   table, a mute transaction with `ramp_samples = 0` is bit-identical to a control booted muted from
   the first block; the same edit with `ramp_samples = 96` equals neither control on the first
   block and equals the muted control from the first block that begins 96 frames or more after the
   commit. A following transaction with
   no length ramps over 480 again. The commit and ramp blocks allocate nothing
   (`bench_support::alloc` thread counters, process statics warmed).
7. **Commands:**
   - `cargo test --locked -p session`, `cargo test --locked -p protocol --features test-support`,
     `cargo test --locked -p host-core --features test-support --test live_delta`,
     `cargo test --locked -p capi`, `cargo test --locked -p conformance`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `cargo build --locked --release -p audit && ./target/release/audit capi` and
     `bash scripts/run-protocol-allocation-audit.sh target/release/audit`
   - `bash scripts/check-protocol-control-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the encoder and decoder disagree on the optional field, or read an absent
  field as 0.
- Gate 2 turns red if a later edit without a length keeps an earlier length, if an edit records the
  wrong row, or if an upsert leaves a stale entry. No test covers transaction metadata today.
- Gate 3 turns red if the bound is off by one or checked against the base rate, or if a refused
  transaction commits.
- Gate 4 turns red if the classifier ignores an edit ramp, reads an explicit 0 as "default", or
  applies one row's or one strip's ramp to another.
- Gate 6 turns red if `commit_live` passes no edit ramps, or if a length leaks into the next
  transaction.

## Dependencies

- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes* (#1054)

#1054 delivers `LiveRamps` with `for_row`, the default table and D5's matrix precedence. These
issues depend on this one: #1225, #1226, #1236, #1247, #1261, #1371, #1390 (the C ABI rows), #1382
(browser lowering), #1383 and #1385 (SDK encoding).
