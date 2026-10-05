# Edit control_smoothing by a session transaction, model-only

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-3).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A C ABI host changes or clears a running session's `control_smoothing` with one session
transaction. The transaction commits without a plan rebuild: no source ring resets, nothing is
re-prepared, and render continues. The next live record any later edit produces uses the new
lengths. The canonical JSON snapshot shows the new value. Owner decision 1's "editing is an
ordinary session transaction" holds.

## Context

- **The setting.** *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and
  pan changes* (#1054) adds `SessionModel::control_smoothing: Option<ControlSmoothing>`. Its only
  reader is `LiveRamps::for_session(next)`, which the C ABI calls once per commit
  (`crates/capi/src/runtime/control.rs:1071-1075`). No prepared plan reads it (#1054 D5; the
  bypass crossfade carries its length in the record, #1341 D1).
- **Opcodes.** `SessionEditOpcode` (`crates/protocol/src/model.rs:22-120`): the root edits are
  `0x0001`-`0x0005` and `SetConsole = 0x0007`; `0x0006` is retired (`:33-35`). The decode table is
  at `:128-140`, the `SessionEdit` enum at `:185-205`, and the opcode map at `:445-450`. Edits are
  applied to the model at `:534-545`.
- **The wire.**
  - Payload specs: `crates/protocol/src/schema.rs:1134-1180` (`set_session_id`, `set_console`),
    selected at `:1566-1571`.
  - Encode: `crates/protocol/src/session_wire.rs:355-366`. Decode: `:1183-1205`.
- **Corpus and its pins.** `crates/conformance/src/protocol_corpus.rs` encodes one edit per
  allocated opcode (`:77-92`) and pins one FNV-1a-64 roll of the whole corpus,
  `COMPLETE_SCHEMA_HASH`, with a re-pin history (`:706-737`). A new opcode moves these files, as
  #1241's opcodes did (commit `210bd252f`):
  - `scripts/check-protocol-wasm-parity.sh:172-173`, the two lines that spell the pinned hash;
  - `crates/protocol/src/controller/tests.rs:327-328`, the all-opcode edit count (46) and the edit
    limit one below it (45);
  - `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`, the hash, the edit count and the re-pin list;
  - `docs/CONTROL_PROTOCOL_REGISTRY.md`, the opcode table and "exactly 46 allocated opcodes"
    (`:83`);
  - `fuzz/corpus/complete-schema-manifest.md:3-4` and its re-pin history (`:30`).
- **Model-only classification.** `classify_live_delta` masks the fields no prepared plan reads
  (`crates/host-core/src/live_delta.rs:230-236`, #1260 D1-D2), so a delta that changes only them
  is live with no records. The C ABI test of that rule is `model_only_edits` and
  `model_only_edits_commit_without_a_plan_rebuild`
  (`crates/capi/src/runtime/live_tests.rs:1419`, `:1481`).

## Decisions frozen for this slice

- **D1. Opcode.** `SetControlSmoothing = 0x0008`, appended to the root group.
  `SessionEdit::SetControlSmoothing { control_smoothing: Option<ControlSmoothing> }`. `Some` sets
  the object; `None` clears it, so the default table applies.
- **D2. Wire payload.** Field 1 is an optional nested message holding `mute_ms`, `fader_ms` and
  `pan_ms` as `f32` fields 1, 2 and 3; it is absent for `None`. Decode refuses a partial message
  and any value outside #1054 D1's bounds. Prospective-session validation would refuse them anyway,
  but decode refuses first with the protocol's existing domain code.
- **D3. Apply.** `session.control_smoothing = control_smoothing.clone()` (`model.rs` apply
  table).
- **D4. Model-only.** The classifier's mask copies `control_smoothing` from `current` with the
  session ID and profile IDs. A delta that changes only it is live with no records, and commits
  with no plan rebuild. The same commit's `LiveRamps::for_session(next)` already uses the new
  value, so records produced by the same transaction ramp over the new lengths.
- **D5. Acked-batch question.** The edit pushes nothing to render. The ack means the committed
  model holds the value, and no record is dropped to reach that.
- **D6. Corpus.** One `SetControlSmoothing` edit with `Some` joins the corpus transaction, so it
  carries 47 edits; the frame count stays 46. The roll is re-pinned once, from whatever value is on
  `main` when this lands (*Carry an optional per-edit ramp length on live session edits*, #1394,
  re-pins it too; whichever merges second re-pins from the first's value), with the history line
  "#1365 appends `SetControlSmoothing` (`0x0008`)". Every pin file of Context moves in the same
  commit: the hash in the parity script, the edit count 47 and limit 46 in the controller test, and the hash, count and
  history in the three documents. This is a wire format, so the pin is allowed.

## Deliverables

1. D1-D3 in `crates/protocol/src/{model,schema,session_wire}.rs`, with the opcode's row (payload
   and field numbers) in `docs/CONTROL_PROTOCOL_REGISTRY.md`'s session-edit table.
2. D4 in `crates/host-core/src/live_delta.rs` (the mask and its doc rule `:181-183`).
3. D6 in `crates/conformance/src/protocol_corpus.rs` and every pin file of Context.
4. `docs/C_ABI_V1_QUALIFICATION.md`: the model-only list gains `control_smoothing`.
5. The tests below.

## Authorized paths

- `crates/protocol/src/{model,schema,session_wire}.rs`, `crates/protocol/src/session_wire/tests.rs`,
  `crates/protocol/src/model.rs` tests. Stream B owns `crates/protocol`; root sequences the merge.
- `crates/host-core/src/live_delta.rs` (stream B), `crates/host-core/tests/live_delta.rs`
- `crates/conformance/src/protocol_corpus.rs`, `scripts/check-protocol-wasm-parity.sh` (the two hash
  lines), `crates/protocol/src/controller/tests.rs` (the count and limit),
  `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md`,
  `fuzz/corpus/complete-schema-manifest.md`
- `crates/capi/src/runtime/live_tests.rs` (tests only)
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- No browser path. The browser has no transaction API until decision 15 D15-11's control-plane
  crate (*Extract the C ABI control plane into a portable crate both hosts call*, #1309), which
  then serves this edit unchanged.
- No change to the schema, the default table or the rounding (#1054).
- No retroactive change to a ramp already in flight: it keeps the length it started with.

## Hazards

- **Prepared transitions.** If a transition slice (*Fade in a strip that a swap adds during
  playback*, #1288, or the duck-swap and removal slices) bakes the mute ramp into a prepared plan,
  that plan reads `control_smoothing` for a transition decided at its preparation only. D4 stays
  correct because a later edit governs later transitions. If one of those slices lands first, its
  PR says so in the mask's doc.

## Objective gates

1. **Wire round trip** (`crates/protocol/src/session_wire/tests.rs`, new). `Some` and `None`
   encode and decode to the same edit. A partial message, a negative value and 1000.5 are refused
   at decode.
2. **Apply** (`crates/protocol` model test, new). A transaction that sets then clears the key
   leaves the canonical JSON first with, then without, `control_smoothing`.
3. **Model-only on the C ABI** (`crates/capi/src/runtime/live_tests.rs`). `model_only_edits` gains
   this edit, and `model_only_edits_commit_without_a_plan_rebuild` passes for it: no new provider
   epoch, no pending plan, render bit-identical to an unedited engine.
4. **New lengths apply** (same file, new). Set `mute_ms` to 0 by transaction, then mute: the next
   block equals a control booted muted, so the step is in effect. Set it to 20 and unmute: the
   first block after the commit is ramping.
5. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). A delta that changes only
   `control_smoothing` is `Ok` with no records. One that also moves a fader carries the new
   length on its `FaderDb` record.
6. **Corpus.** `cargo run --locked -p conformance --example conformance_fixtures -- --check` and
   `bash scripts/check-protocol-wasm-parity.sh` pass with the one re-pin (D6), and the controller
   test's boundary row (`controller/tests.rs:327-328`) passes at 47 and 46.
7. **Commands:**
   - `cargo test --locked -p protocol --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-core --features test-support --test live_delta`,
     `cargo test --locked -p conformance`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-protocol-control-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo build --locked --release -p audit && bash scripts/run-protocol-allocation-audit.sh target/release/audit`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the encoder and decoder disagree on the optional field or accept an
  out-of-range length.
- Gate 3 turns red if the mask misses the key, so the edit rebuilds and resets the source rings.
- Gate 4 turns red if `LiveRamps` is read from the pre-commit model, so the edit takes effect one
  transaction late.
- Gate 5 turns red if the classifier treats the key as structure, or uses the old length in the
  same transaction.

## Dependencies

- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054).
