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
  (`crates/control-plane/src/control.rs:1207-1211`). No prepared plan reads it (#1054 D5; the
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
  the object; `None` clears it, so the default table applies. In canonical JSON (#1054 D1, a
  required tagged root key) `Some` writes `{ "kind": "explicit", "mute_ms", "fader_ms", "pan_ms" }`
  and `None` writes `{ "kind": "default" }`.
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
   leaves the canonical JSON's `control_smoothing` first `{ "kind": "explicit", ... }`, then
   `{ "kind": "default" }`.
3. **Model-only on the C ABI** (`crates/capi/src/runtime/live_tests.rs`). `model_only_edits` gains
   this edit, and `model_only_edits_commit_without_a_plan_rebuild` passes for it: no new provider
   epoch, no pending plan, render bit-identical to an unedited engine.
4. **New lengths apply** (same file, new). Set `mute_ms` to 0 by transaction, then mute: the next
   block equals a control booted muted, so the step is in effect. Set it to 20 and unmute: the
   first block after the commit is ramping.
5. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). A delta that changes only
   `control_smoothing` is `Ok` with no records. One that also moves a fader carries the new
   length on its `FaderDb` record.
6. **Corpus.** `cargo test --locked -p conformance --test conformance_corpus` (which checks
   `COMPLETE_SCHEMA_HASH`) and `bash scripts/check-protocol-wasm-parity.sh` (every other spelling
   of the hash agrees) pass with the one re-pin (D6), and the controller
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

## Attempt record

**Attempt 1** (implementer, branch `codex/d15-stream-e2`, base `bd8a92e9d`, after #1394 attempt 2). One
implementation commit (`9db950add`: D1-D6, the five tests, every pin file and the docs, in one commit as
the single re-pin requires), then this record.

**Preflight at the base.** Opcode `0x0008` was free in `SessionEditOpcode` (the only other `0x0008` in the
crate is `TransportSet`, a different registry). The all-opcode count was 46 and the controller test's limit
45, as the spec says; the lines moved (`controller/tests.rs:327-328` is now `:327-329` after the comment,
`model.rs` is longer after #1394), but no number changed. `COMPLETE_SCHEMA_HASH` stood at
`5429f052592101ef` (#1394 attempt 2), and the re-pin runs from it.

**Where the spec and the tree differed (recorded, not improvised around).**

1. *The call site.* `LiveRamps::for_session(next)` is in `crates/control-plane/src/control.rs`
   (`commit_live`, `:1348-1351`), not `crates/capi/.../control.rs:1207-1211`. The code is as D4 assumes
   (the ramps come from `next`); the file is not edited.
2. *`.clone()` is `*`.* `ControlSmoothing` is `Copy`, so D3 is `session.control_smoothing =
   *control_smoothing` (clippy refuses `.clone()` on a `Copy` type).
3. *Decode refusal code.* "The protocol's existing domain code" is `DecodeError::InvalidTlv`
   (`MALFORMED_FRAME`), the code `read_f32_exact` and the bit-depth decoder already use for a value outside
   its domain. A partial message is refused by the schema's required-field check, as every other nested
   message is.
4. *`record_edit_ramps`.* The exhaustive match in `crates/protocol/src/model.rs` lists
   `SetControlSmoothing` with the edits that write no keyed row (a no-op), as the brief said. The edit has
   no `ramp_samples`, so `SessionEdit::ramp_samples()` returns `None` for it by its `_` arm.
5. *No compiler-forced edit outside the authorized paths.* host-web, the SDK and the rest of the workspace
   compile and pass unchanged.

**The re-pin.** `COMPLETE_SCHEMA_HASH` `5429f052592101ef` -> `8eb4cc8d075a4355`, history line "#1365
appends `SetControlSmoothing` (`0x0008`)". The corpus transaction gains one edit (`Some`, lengths 5, 15 and
25 ms, so a field cannot be read as its neighbour) after `SetConsole`, so it carries 47 edits; the frame
count stays 46. Moved together: the constant and its history (`protocol_corpus.rs`), both spellings in
`scripts/check-protocol-wasm-parity.sh` (`:172-173`), the count 47 and limit 46 in
`controller/tests.rs`, the allocated-opcode count in `model.rs`'s registry test (46 -> 47, with an
assertion for `0x0008`), the hash, count and history in `docs/CONTROL_PROTOCOL_CONFORMANCE.md`,
`fuzz/corpus/complete-schema-manifest.md` and the "exactly 47 allocated opcodes" line of
`docs/CONTROL_PROTOCOL_REGISTRY.md`. The reason for the re-pin is the new opcode; nothing else moved. The
parity script and `conformance_corpus` agree on the new value.

**Digests that moved: none.** `audit capi` `pcm_digest a9738fa800afcab3` (as before this slice), 0
allocations, 0 syscalls; `check-browser-expected-resources.py --artifacts` agrees with the rebuilt
simd128 module.

**Tests added.** Each answers "which plausible defect turns this red that no existing test catches?".

| Gate | Test | Defect it catches |
| --- | --- | --- |
| 1 | `protocol` `session_wire::tests::set_control_smoothing_round_trips_and_decode_refuses_a_partial_or_out_of_range_message` | encoder and decoder disagree on a length's field or on the optional message; a partial message completed with a default; a length outside `[0, 1000]` (or non-finite) accepted at decode and left to validation |
| 2 | `protocol` `model::tests::set_control_smoothing_sets_then_clears_the_key_in_the_committed_snapshot` | the apply arm ignores `None`, merges into the old lengths or writes a length into its neighbour; canonical JSON spells the key wrongly |
| 3 | `capi` `model_only_edits_commit_without_a_plan_rebuild` (its edit list gains the edit) | the mask misses the key, so the edit rebuilds: new provider epoch, pending candidate, source-ring reset |
| 4 | `capi` `a_control_smoothing_edit_makes_the_next_mute_a_step`, `a_control_smoothing_edit_gives_the_next_unmute_its_ramp` | `LiveRamps` read from the pre-commit model, so the edit takes effect one transaction late; the key read as the default table; the ramp not landing on its control |
| 5 | `host-core` `live_delta` `a_control_smoothing_edit_is_live_and_its_lengths_govern_the_same_transactions_records`, plus the edit joins `model_only_edits_are_live_with_no_records` | the classifier treats the key as structure (set, change and clear are each tried) |

Gate 5's second half (the fader record carries the new length) is the classifier fed
`LiveRamps::for_session(next)`, the call the control plane makes; the classifier itself only forwards the
ramps it is given, so the "old length in the same transaction" defect is a caller defect and is caught by
gate 4's same-transaction row, which runs through the C ABI.

**Mutation runs** (apply the defect: red; revert: green; each run reverted before the next).

| # | Defect applied | Red |
| --- | --- | --- |
| M1a | `parse_control_smoothing` checks only `is_finite`, no range | gate 1: `-0.001 in field 1` accepted (`Ok(ControlSmoothing { mute_ms: -0.001, .. })` where `Err(InvalidTlv)`) |
| M1b | `parse_control_smoothing` swaps the fader and pan fields | gate 1 red on the round trip |
| M2a | apply arm `control_smoothing.or(session.control_smoothing)` (`None` does not clear) | gate 2 red at the third commit (left `Some(..)`, right `None`) |
| M2b | apply arm swaps `fader_ms` and `pan_ms` | gate 2 red (fader 25, pan 15 against 15, 25) |
| M3 | delete `masked.control_smoothing = current.control_smoothing` in `classify_live_delta` | gate 3 (`model_only_edits_commit_without_a_plan_rebuild`), gate 4 (both tests, the edit rebuilds) and gate 5 (both host-core tests) red |
| M4 | `commit_live` builds `LiveRamps::for_session` from the pre-commit model | gate 4 red: `a_control_smoothing_edit_makes_the_next_mute_a_step` fails in its "in the mute's transaction" row (block 0 is not the muted control), and `a_control_smoothing_edit_gives_the_next_unmute_its_ramp` fails (the unmute steps) |

Every mutated file was restored from a copy and the tree was clean afterwards (`git status` showed only the
intended files); the full suites below ran after the last revert.

**Gates run (all green).**

- `cargo test --locked -p protocol --features test-support` (135 lib tests and the integration tests),
  `-p conformance` (with `conformance_corpus`), `-p host-core --features test-support,control-provider
  --test live_delta` (35 passed, so the feature is on), `-p capi` (86, 5, 3, 13).
- The `test-debug-a` leg (workspace debug tests and doctests with the CI feature list) and the `test-debug-b`
  leg (DSP crates and `conformance`, doctests, `conformance_fixtures --check`).
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --
  --check`.
- `check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-protocol-control-policy.sh`,
  `check-host-core-policy.sh`, `check-cross-targets.sh`, `check-protocol-wasm-parity.sh`,
  `check-capi-abi.sh`; `cargo build --release -p audit` with `audit capi` and
  `run-protocol-allocation-audit.sh`.
- The worklet chain from `qualification.yml`: `build-web-audioworklet.sh --named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py
  --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh` (private TMPDIR left empty).
  Not run here: the V8 spill gate, which needs the exact Node 22.23.2 and reads the EQ cascade loops this
  slice does not touch. AArch64 runs only in CI.

**Open items.** None for this slice. The browser has no transaction API for the edit (non-goal; #1309).

## Amendment 1 (root, 2026-10-12)

Stream E2 asked whether host-web's cached ramp table must be refreshed when this edit lands. The
table is `SessionRamps` (`LiveRamps::for_session` plus one pan window per strip), built once in
`compile_ready` by #1364. Today the answer is no, and no code is added: the browser cannot commit a
`control_smoothing` change, or a change to a strip's model `smoothing_samples`, without a new boot.
`compile_ready` has one caller, `boot` (`hosts/host-web/src/lib.rs:2274`); host-web holds the
`SessionModel` only inside that call; and none of its entry points edits a model, because the browser
has no session transaction and no committed model (this spec's non-goal; #1309). A new boot builds a
fresh `SessionRamps`. The issue that adds the first browser path to commit a model-only edit is
*Admit browser live edits in the Worker through the committed model* (#1382, after #1309). It must
drop `SessionRamps` (the classifier resolves an absent length with `LiveRamps::resolve` over the
committed model, #1394 D5) or refresh it on every model-only commit, and it must add a test that goes
red without that. Until then a cached table cannot go stale.
