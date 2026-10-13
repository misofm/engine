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
   first block after the commit is ramping. Rebuild leg (batch verdict 2, m1): one transaction
   that sets `control_smoothing` to the hard switch, rebuilds the plan with a content edit and
   retargets the carried strips (a mute on the even tracks, a fader level on the odd ones, a matrix on
   the last) renders bit-identically to the same transaction on a session that already says hard
   switch, differs from the same rebuild under the default table, and allocates nothing in any
   render call after the swap. The test is `a_control_smoothing_edit_governs_the_retargets_of_its_own_rebuild`
   (`crates/capi/src/runtime/live_tests.rs`).
5. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). A delta that changes only
   `control_smoothing` is `Ok` with no records. One that also moves a fader carries the new
   length on its `FaderDb` record.
6. **Corpus.** `cargo test --locked -p conformance --test conformance_corpus` (which checks
   `COMPLETE_SCHEMA_HASH`) and `bash scripts/check-protocol-wasm-parity.sh` (every other spelling
   of the hash agrees) pass with the one re-pin (D6), and the controller
   test's boundary row (`controller/tests.rs:327-328`) passes at 47 and 46.
7. **Commands:**
   - `cargo test --locked -p protocol --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-core --features test-support,control-provider --test live_delta`
     (root ruling, 2026-10-12: without `control-provider` the test file is `cfg`ed out and the
     command runs 0 tests),
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
- Gate 4's rebuild leg (batch verdict 2, m1) turns red if the successor's carry retargets are built
  from the pre-commit model's lengths (`LiveRamps::for_session(committed)` in `join_strips`, so a
  rebuild transaction's own `control_smoothing` takes effect one transaction late). The other gate 4
  tests commit live and never rebuild, and the host-core join tests pass the lengths in.
- Gate 5 turns red if the classifier treats a set, a change or a clear of the key as structure, or
  if it does not forward the ramps it is given to its records. It cannot see a caller that builds
  those ramps from the pre-commit model, because `classify_live_delta` takes them as an argument;
  gate 4 catches the old length in the same transaction (root ruling, 2026-10-12, correcting the
  earlier sentence).

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

**Digests that moved: none.** `audit capi` `pcm_digest` (the post-D4 value recorded in #1054) (as before this slice), 0
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

| M5 | `join_strips` builds `LiveRamps::for_session(committed)` instead of `for_session(model)` (`crates/host-core/src/prepare.rs`, batch verdict 2, m1) | gate 4's rebuild leg red: `a_control_smoothing_edit_governs_the_retargets_of_its_own_rebuild` fails at "the transaction's own lengths govern its rebuild's retargets". It is the only failing test: `cargo test -p capi --no-fail-fast` (87 tests, every binary), `-p host-core` and `-p control-plane` otherwise pass. Reverted: green |

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

## Amendment 1 note (#1364 attempt 2, under root's 2026-10-12 ruling)

Stream E2 asked whether host-web's cached ramp table must be refreshed when this edit lands. The
table is `SessionRamps` (`LiveRamps::for_session` plus one pan window per strip), built once in
`compile_ready` by #1364. Today the answer is no, and no code is added: the browser cannot commit a
`control_smoothing` change, or a change to a strip's model `smoothing_samples`, without a new boot.
`compile_ready` has one caller, `boot` (`hosts/host-web/src/lib.rs:2274`); host-web holds the
`SessionModel` only inside that call; and none of its entry points edits a model, because the browser
has no session transaction and no committed model (this spec's non-goal; #1309). A new boot builds a
fresh `SessionRamps`. The issue that adds the first browser path to commit a model-only edit is
*Admit browser live edits in the Worker through the committed model* (#1382, after #1309). It must
drop `SessionRamps` (D6 as ruled 2026-10-12: the classifier resolves an absent length with
`LiveRamps::resolve` over the committed model, #1394 D5), and it must add a test that goes red
without that. The earlier option to refresh it on every model-only commit is superseded. Until then a cached table cannot go stale.

**Batch follow-ups** (stream E2 batch, after the PASS verdict
`docs/handoffs/decision-15-2026-10-05/verdicts/stream-e2/1365-attempt1.md`).

- *MINOR 1:* gate 7's host-core command carries `control-provider` (root ruling, 2026-10-12).
- *Gate 5's test-value sentence* is corrected (NIT 2, root ruling, 2026-10-12): gate 4 catches the old
  length in the same transaction; gate 5 catches a classifier that treats the key as structure.
- *NIT 1:* three over-long lines are wrapped (`crates/capi/src/runtime/live_tests.rs` doc of
  `model_only_edits_commit_without_a_plan_rebuild`; `crates/host-core/src/live_delta.rs` step 3 of the
  classifier doc; `docs/C_ABI_V1_QUALIFICATION.md` the model-only item). Comments and prose only.
- The Amendment 1 note about `SessionRamps` is relabelled "Amendment 1 note (#1364 attempt 2, under root's
  2026-10-12 ruling)"; root confirms its content. The duty it names is now in #1382's spec.
- *Mutation run* (the sentence's claim, tested): `commit_live` builds `LiveRamps::for_session` from the
  pre-commit model. RED: `a_control_smoothing_edit_makes_the_next_mute_a_step` and
  `a_control_smoothing_edit_gives_the_next_unmute_its_ramp` (gate 4, `-p capi`); GREEN: gate 5's
  `a_control_smoothing_edit_is_live_and_its_lengths_govern_the_same_transactions_records`. File restored.
  The doc wraps change no code.

**Batch verdict follow-ups** (stream E2 batch verdict, head `29bd7f3ca`, 2026-10-12).

- *n7:* the Amendment 1 note above named "attempt 2". #1365 had attempt 1 only; the note is #1364's
  attempt 2. Its heading now reads "Amendment 1 note (#1364 attempt 2, under root's 2026-10-12 ruling)".

### Rebase onto 88272d08b

Stream E2 was rebased from base `8c0deee2c` (head `cf9df5f89`) onto `origin/main` `88272d08b`, which carries
stream A2 (PR #1526: the strip carry and `join_strips`) and stream B3 (PR #1540: latest-target cells for the
input, route and effect lanes). All 38 commits survive in order, none was dropped, and no commit became empty.
The verdicts and the Attempt records above cite the old hashes; this table maps each to its new one. The old
head is kept as the local branch `backup-e2-pre-rebase-88272d08b`. Fold-in fixups were squashed into the commit
they belong to (the table lists the final hashes), and a commit whose body needed the port's meaning has its
message extended (`b5ca077f6` below).

| old | new | subject |
|---|---|---|
| `8a68e257f` | `371c322a7` | Add the required control_smoothing root key to the session model, grammar, visitor, schema and SDK writer (#1054 D1, D2) |
| `351650a5b` | `d9deebc4d` | Migrate every checked-in session document to carry control_smoothing default (#1054 D1a) |
| `23c9629a2` | `22352f79c` | Re-pin the C ABI snapshot length for the control_smoothing key (#1054 gate 7) |
| `da9f2ff50` | `5d694755e` | Test the control_smoothing grammar, rounding and default table; regenerate the writer corpus (#1054 gates 1, 2) |
| `a575e74fd` | `32d88540a` | Ramp live fader, mute and matrix records over the session's control_smoothing (#1054 D4, D5) |
| `f4d0263c7` | `64b46e076` | Hear the live mute ramp through the C ABI; compare the other live edits after their ramp (#1054 gates 4, 5) |
| `9e0184a76` | `7a3d6448c` | Re-pin the prepare_256_tracks session_template_sha256 for the control_smoothing key (#1054 gate 7) |
| `735ca09df` | `5d0f57168` | Re-pin the builtins-graph ACCEPTED_MANIFEST_SHA256 (#1054 gate 7) |
| `62ffd591d` | `00c445565` | Re-pin the audit unit test's joined-corpus manifest identity (#1054 gate 7) |
| `7ded0025d` | `89eb08e25` | Re-pin the browser staged session document size (#1054 gate 7) |
| `408cdece3` | `a75aa18ad` | Keep the one-byte mutation of the staged document size one byte (#1054) |
| `2c39d189d` | `d86be61a3` | Keep the default-table property assertion out of clippy's constant folding (#1054 gate 2) |
| `d00c310ea` | `7e23d6e95` | Split the default-table test from the pan-at-twice-mute property (#1054 gate 2) |
| `adf234b19` | `686b96e27` | Settle the raced C ABI plan past the session's live ramps; record attempt 1 in the spec (#1054) |
| `6a9f7192e` | `334d379c0` | Add EditRamps, the per-edit ramp lengths of one session transaction (#1394 D2) |
| `70a0a8991` | `356fea5f2` | Carry an optional ramp_samples on the live-value session edits and record it per transaction (#1394 D1, D3, D4) |
| `860e817ad` | `2242114c1` | Re-pin COMPLETE_SCHEMA_HASH for the corpus's SetTrackFader ramp_samples (#1394 gate 5) |
| `434df0140` | `b5ca077f6` | Read every live record length through LiveRamps::resolve with the transaction's edit ramps (#1394 D5, D6) |
| `7cf89f67e` | `48f0577da` | Test a live edit's own ramp length through the C ABI and document it (#1394 gate 6) |
| `5a18bddde` | `0c2b1f788` | Test that clearing one effect's ramp lengths leaves its siblings and its strip (#1394 D2) |
| `5abf225f2` | `ae5954aff` | Record attempt 1 in the #1394 spec |
| `11269ef80` | `0c8a2abae` | Resolve an absent live ramp to the session default in the browser host (#1364 D1, D2) |
| `d0439731d` | `adefbab67` | Default an absent SDK ramp to the session's and author controlSmoothing (#1364 D3, D4) |
| `751523524` | `da8685337` | Test that the browser and the C ABI render the same bits for an absent ramp (#1364 gate 3) |
| `6e4254588` | `e274ca72e` | Record attempt 1 in the #1364 spec and allow the parity test's unsafe calls |
| `bd8a92e9d` | `972df2784` | Clear a ramp entry when an edit rewrites or removes its target (#1394 attempt 2) |
| `9db950add` | `9c6a345b0` | Edit control_smoothing by a model-only session transaction (#1365 D1-D6) |
| `b16fb4d14` | `11dbea2f0` | Record attempt 1 in the #1365 spec |
| `c7382644c` | `744e14a05` | Resolve an absent ramp on a literal LaneEdit by kind, in the writer and the browser transport (#1364 attempt 2) |
| `59eecdc66` | `4c1f2da6f` | Fold the #1054 batch follow-ups: rulings in the spec, for_row walk, note order (stream E2) |
| `3d1909dd8` | `f57034db8` | Fold the #1394 batch follow-ups: pin SetVcaFader's rows, make ramp_samples exhaustive (stream E2) |
| `35f8b908a` | `3b436c6cc` | Fold the #1365 batch follow-ups: gate 7 command, gate 5 sentence, wrapped docs (stream E2) |
| `0029220a6` | `0449fc9a3` | Fold the #1364 batch follow-ups: ramp helpers out of the public exports, README, D3 wording (stream E2) |
| `c1e162b9c` | `6680156c9` | Add the SessionRamps duty to #1382's spec (root ruling 2026-10-12, #1364/#1365 attempt-2 finding) |
| `29bd7f3ca` | `5d1766c9e` | Amend #1385 to encode SetControlSmoothing (root amendment 2026-10-12, #1365) |
| `4b5f58913` | `0bd1d94bc` | Stream E2 batch follow-ups, SDK: u32 check on unramped kinds, shared controlSmoothing helper, subpath leak check |
| `7d00abca1` | `2aa74e168` | Stream E2 batch follow-ups, comments and specs (n1, n2, n5, n6, n7, m3, m4 records) |
| `cf9df5f89` | `a03ada37e` | State that #1385 alone encodes SetControlSmoothing (root ruling 2026-10-12: fold into #1385) |

**What was ported by meaning (#1365).**

- *The model-only mask still holds on the new classifier.* `classify_live_delta` still masks `control_smoothing`
  (`masked.control_smoothing = current.control_smoothing`); with the mask removed `model_only_edits_are_live_with_no_records`
  and the capi live-commit test go red (mutation below).
- *First preparation that bakes a ramp length.* The classifier's doc promised that the first slice to bake a length
  into a plan at preparation would say so. Stream A2's carry retarget is that slice: it writes fader, mute and matrix
  records into the successor's cells at preparation, with the lengths of the successor's own model (and the
  transaction's edit lengths, #1394 D6), so a `control_smoothing` edit in a rebuild transaction governs its retargets
  and a later edit governs only later transitions. The `live_delta.rs` doc now says so, and
  `docs/C_ABI_V1_QUALIFICATION.md` states that a rebuild keeps the transaction's lengths.
- *Corpus:* `SetControlSmoothing` and the `COMPLETE_SCHEMA_HASH` chain are unchanged by main (see the pins above).

**Digests and byte pins on the merged tree.** Each is recomputed on the merged tree and re-pinned on its own, on top
of main's value. No pinned render digest moved: `ACCEPTED_GRAPH_PCM_SHA256`/`_METERS_` and the browser module's
expected-resource digests are unchanged. The unpinned `audit capi` `pcm_digest` moves only because of the ramps (#1054
gate 7); its values and the run that isolates the effect of stream A2's carry are in #1054's Attempt record.

| pin | file | main -> merged tree | reason |
|---|---|---|---|
| `ACCEPTED_MANIFEST_SHA256` | `tools/audit/src/builtins_graph.rs` | `28f109c9...93e9d0` -> `3e151a3c0744742330d44d4a6d22ea0f11886b1892ad3266325ec23d93b9e108` | D1a adds the required `control_smoothing` key (moves the two `prepare_256_tracks` rows); history line on top of the #1346 line |
| joined-corpus manifest identity | `tools/audit/src/fixture_builtins.rs` | same pair | same reason; same history order |
| `session_template_sha256` x2 | `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml` | `dc19e091...72330` -> `4c575634...e6fb` | D1a |
| `MANIFEST.tsv` rows | same two tomls | `4cc2ca53...e996` -> `a729c476...55c8`, `ee6d238c...1111` -> `35ebd642...befe` | D1a |
| snapshot length in the command-response vector | `crates/capi/src/runtime/tests.rs` | 14,193 (0x3771) -> 14,245 (0x37a5) | D1a: the 52-byte `control_smoothing` key |
| `sessionDocumentBytes` (and the self-test's one-byte mutation) | `hosts/host-web/tests/browser-v1/expected.json`, `scripts/check-browser-expected-resources.py` | 1969 -> 2021 (1970 -> 2022) | D1a |
| `COMPLETE_SCHEMA_HASH` | `crates/conformance/src/protocol_corpus.rs` + three mirrors | `ab357b6c432f9755` -> `5429f052592101ef` (#1394: `ramp_samples`) -> `8eb4cc8d075a4355` (#1365: `SetControlSmoothing`) | main did not touch `crates/protocol`, so the chain and its history lines are unchanged |

**Resource ceilings against main** (`resource_lifecycle`, `reference_session_retained_rows_stay_within_their_budgets`):
no ceiling is raised and none trips. Only two rows move: `capi_retained_bytes` 282,287 -> 282,303 of 282,432 (+16
bytes; 129 bytes of headroom left; the retained prepared-token layout grew by the E2 structures, `EditRamps` in
`PreparedSessionTransaction` and the model's `control_smoothing` field, not decomposed further) and the compiled
model estimate 19,434 -> 19,486 of 25,344 (+52, the `control_smoothing` key's bytes). `graph_session_plus_plan_bytes`
405,686, the effect-control resources 19,377 and every other row are equal to main's.

**Gates run on the merged tree** (the port's proof set; the rest of the sweep is deferred to the rebase onto J3's
merge): `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
`-p host-core --features test-support,control-provider --test live_delta` (35); `-p capi`; `cargo test -p host-web
--features test-support`; `-p protocol --features test-support`; `-p conformance`. An earlier full sweep of the tree
before the last test addition was also green: `test-debug-a` and `-b` with doctests, the policy scripts,
`check-cross-targets.sh`, `check-capi-abi.sh`, `check-protocol-wasm-parity.sh`, `audit capi` (0 allocations, 0
syscalls), the builtins fixtures check, the worklet chain (`--named-twin` build, `check-web-audioworklet.sh`,
`check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`) and
the SDK job (383 evals; `node_modules` and `dist` deleted afterwards).

**Mutation runs for each semantic port** (defect applied by a text edit, test run, file restored; "R" red):

| Port | Defect | Result |
|---|---|---|
| classifier reads the transaction's edit ramps | `StripEditRamps::of` replaced by the default | R `a_records_length_is_its_edits_own_when_the_edit_carried_one` |
| successor join reads them too | same, in `join_strips` | R `a_retarget_carries_the_edit_ramp_of_its_transaction_on_every_width_and_form` |
| the control plane hands them to the successor | `SuccessorBase::edit_ramps` set to an empty `EditRamps` | R `a_rebuilds_carried_strip_retargets_over_its_edits_own_ramp` (`-p capi`) |
| matrix rule (#1394 D5, #1054 D5) shared by classifier and join | the model's own window ignored | R 3 `live_delta` tests and 4 `successor_swap` tests |
| mute row | `Mute` record reads the fader length | R 2 `live_delta` tests and 2 `successor_swap` tests |
| #1365 mask | `control_smoothing` not masked | R `model_only_edits_are_live_with_no_records` and the capi live-commit test |
| host-web resolution | the mute sentinel not resolved | R `an_absent_ramp_resolves_to_the_sessions_row_and_a_given_one_is_kept` |
| cross-host strip cells | the browser fader command sends 0 instead of the sentinel | R `both_hosts_drain_two_fader_and_two_pan_edits_alike` |

### Rebase onto 70804d3c2

Stream E2's 39 commits are rebased onto `70804d3c2` (J3, PR #1541). The commit mapping from
`3715d0754` (the head on `88272d08b`; backup branch `backup-e2-pre-rebase-70804d3c2`) is one-to-one and in the
same order, 39 to 39; the rebased tip of that sequence is `2a7cb1798`. Three commits follow it, in this order:
the pointer from the J-batch wording in `STREAMS.md` and #1444 to the 2026-10-13 follow-up-batch note, root's
2026-10-12 ruling for #1382 (D6 drops `SessionRamps`, gate 9 gains a strip-window leg), and root's 2026-10-13 rulings
on the port points. The head after this note is the branch head.

One port. J3 deleted `scripts/check-realtime-policy.sh`, so the E2 row for
`hosts/host-web/tests/control_smoothing_parity.rs` (#1364) moved to `UNSAFE_ALLOWLIST` in
`tools/realtime-policy/src/policy.rs` (20 to 21 entries) with an "Unsafe-code ownership" entry in
`docs/REALTIME_DEPENDENCY_POLICY.md`, in the commit that introduced the row (old `e274ca72e`, now `4705dc0f2`).
Earlier gate lists in this record that name `check-realtime-policy.sh` and `test-realtime-policy.sh` are the
evidence of their time; the gate is now `tools/realtime-policy` (its tests and self-test).

Gates on the final tree (all green, no resource ceiling raised): `cargo fmt --all -- --check`; `cargo clippy --locked
--workspace --all-targets --all-features -- -D warnings`; `./target/release/realtime_policy` and `cargo test -p
realtime-policy`; `scripts/check-workspace-policy.sh`; the `test-debug-a` leg (1858 tests, plus its doctests);
`-p capi`, `-p host-web --features test-support` and `live_delta` with `control-provider` (348 tests);
`scripts/check-capi-abi.sh`; `audit capi` (0 allocations, 0 syscalls, `total_violations` 0); the worklet chain
(`build-web-audioworklet.sh --named-twin`, `strip-wasm-names.py check`, `check-web-audioworklet.sh`,
`check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`); `check-sdk-generated.sh`,
`check-sdk-types.sh` and `check-sdk-headless.sh`.

### Batch verdict 2 follow-ups (stream E2 batch, rebased tree)

Source: `stream-e2-batch-verdict-2.md` (batch head `fcc5c890e`).

- *m1, the rebuild leg of gate 4.* The claim that a `control_smoothing` edit in a rebuild transaction governs its
  retargets (`live_delta.rs`, `docs/C_ABI_V1_QUALIFICATION.md`, the port note above) had no test: the mutant
  `for_session(committed)` in `join_strips` survived every suite. `a_control_smoothing_edit_governs_the_retargets_of_its_own_rebuild`
  (`crates/capi/src/runtime/live_tests.rs`, beside the other gate 4 tests) applies one transaction
  `[SetControlSmoothing(hard switch), content edit, mute on the even tracks, fader level on the odd ones, a matrix on the
  last]` to the default session and compares six rebuilt blocks, bit for bit, with the same transaction without the
  length edit on a session booted on the hard switch; it also asserts a difference from the default table, so the
  comparison is not vacuous, and counts allocations and frees in each of the six render calls (0). Test value:
  red if the successor's carry retargets read the pre-commit model's lengths; no existing test catches it.
  Mutation M5 above: red, and the only red test; reverted: green. `capi_retained_bytes` is unchanged at 282,303 of
  282,432 (a test adds no retained bytes).
- *n9, the stale option.* The Amendment 1 note named "drop `SessionRamps` or refresh it on every model-only commit".
  #1382 D6, as ruled 2026-10-12, drops it; the note now says so and marks the refresh option superseded.
- *Fold-check n1.* The test's doc comment said no render allocates, "the swap" included, but the render right after
  `apply` (the one that swaps the plan in, writing the carried strips' retargets) was not counted. It now runs through
  `render_counted` and asserts 0 allocations and 0 frees, like the six blocks after the seek. A probe had measured 0
  and 0. Mutation: one `Vec::with_capacity(1)` inside the counted window of that render alone: red ("the swap block
  allocated or freed", left `(1, 1)`); reverted: green.

Gates: `cargo fmt --all -- --check`; clippy `--workspace --all-targets --all-features -D warnings`; `-p capi` (87 in
the lib); `audit capi` (0 allocations, 0 syscalls, `pcm_digest` 76525e7ef9e78cf4, as before). The rest of the gate
list is in #1364's record of the same name.
