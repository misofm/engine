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
  `for_row(row)` returns the D3 key field for a row (twice `mute_samples` for `PolarityInvert`;
  root, 2026-10-05, from #1055) (#1054 D3, D4). The matrix precedence is the
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
  commit (`crates/control-plane/src/control.rs:1207-1211`), with `next` from the prepared token's
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
  rewrite those rows with no length. So do the edits that rewrite or end a keyed target the same
  way (attempt 2, after verdict M1): `RemoveTrack`, `RemoveSubmix`, `RemoveRoute` and `RemoveVca` clear
  their target; `PutTrackEffect` and `RemoveTrackEffect` clear their `Effect` target; `SetTrackRack`
  clears every `Effect` entry of that strip and rack. The match over `SessionEdit` is exhaustive, so
  a new edit must be classified. Root ruling (2026-10-12): the clearing on `PutTrackEffect` and
  `SetTrackRack` follows D3's own rule, because both rewrite the effect's `link_mode` with no
  length; the clearing on the five removals (`RemoveTrack`, `RemoveSubmix`, `RemoveRoute`,
  `RemoveVca`, `RemoveTrackEffect`) is ratified (behaviour-neutral hygiene: no record length reads
  it today). `SessionEdit::ramp_samples()` is exhaustive too (no wildcard arm), so a new variant must
  be classified. An existing `=> None` arm that gains a length field still compiles, so a reviewer
  checks the arm of any variant that gains one.
- **D4. Transaction.** `prepare_transaction` builds `EditRamps` beside the candidate model and
  stores it in `PreparedSessionTransaction`, exposed as `edit_ramps(&self) -> &EditRamps`. A value
  above the candidate's `sample_rate_hz` (1000 ms, the bound of #1054 D1) refuses the transaction
  with the new `SessionEditError::RampOutOfRange` (`session.edit.ramp_out_of_range`) at that edit's
  index, before anything commits. The check uses the candidate's rate after all edits, so a
  transaction that also changes the rate is checked against the rate it commits.
- **D5. `LiveRamps::resolve`.** `resolve(row: LiveRampRow, edit_ramp: Option<u32>) -> u32` returns
  the edit's value when present, else `for_row(row)` (#1054 D4), so an absent polarity length
  resolves to twice `mute_samples` with no rule of its own here, and an edit's own length is kept
  as given (root, 2026-10-05, from #1055). After this slice the classifier
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
   `crates/control-plane/src/control.rs`.
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
- `crates/control-plane/src/control.rs` (the `commit_live` call only),
  `crates/capi/src/runtime/live_tests.rs` (tests only), `docs/C_ABI_V1_QUALIFICATION.md`
- Field additions only (deliverable 3), in any file that builds one of D1's variants
- Root ruling (2026-10-12), a named exception: `crates/session/src/model.rs`, the `PartialOrd` and
  `Ord` derives on `RackName` (declaration order Inserts < Builtins < Console, equal to the wire-code
  order), so `EditRamps` keys hold the typed rack. No behaviour changes.

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
     `cargo test --locked -p host-core --features test-support,control-provider --test live_delta`,
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

## Attempt record

**Attempt 1** (implementer, branch `codex/d15-stream-e2`, base `adf234b19`, the end of #1054). Commits in
deliverable order: D2 (`6a9f7192e`), D1/D3/D4 with the mechanical constructions and the registry
(`70a0a8991`), the one corpus re-pin (`860e817ad`), D5/D6 and the `commit_live` call (`434df0140`), the C ABI
test and doc (`7cf89f67e`), one more `EditRamps` unit test (`5a18bddde`), and this record.

**Where the spec and the tree differed (recorded, not improvised around).**

1. *The classifier produces three record kinds on this base:* `FaderDb`, `Mute` and the strip matrix record.
   A VCA rebuilds (`LiveRebuild::Vca`) and no send or follow record exists, so D6 converts those three, all
   on `(Strip, row)`. The `Input`, `Link`, `Route*` and `Vca*` entries are recorded by D3 and read by no
   classifier path yet; the row slices that own those records read them through `EditRamps::get` and
   `LiveRamps::resolve`.
2. *The classifier signature* is `classify_live_delta(current, next, ramps, edit_ramps: &EditRamps)`. A lookup
   key is built per strip only when the transaction recorded a length (`EditRamps::is_empty` otherwise), so
   the common transaction allocates nothing more than before.
3. *Gate 1's corpus has eight live-value edits, not eleven:* `all_opcode_edits_64()` predates `SetConsole`,
   `SetTrackConsole` and `SetVcaFader`. The test builds those three beside the corpus and asserts all
   eleven.
4. *Gate 2 and 3 live in a new integration test,* `crates/protocol/tests/edit_ramps.rs` (authorized path), on
   the canonical fixture plus a second track; a `model::tests` unit test would have needed the same
   fixture. The tracks sort, so the fixture's `vocal` is the second track.
5. *D4's bound is checked after the final compile,* against the compiled model's `sample_rate_hz`: an
   invalid rate or a failed validation outranks the ramp refusal, and an edit error (`NotFound`, ...) at an
   earlier index outranks both. Every edit is checked, not only the one that survives in `EditRamps`: a
   `Some(48001)` followed by a `None` still refuses at the first edit's index.
6. *Gate 6's "the commit and ramp blocks allocate nothing"* holds for the render calls only, as in #1054
   (the commit is the control plane and allocates, `classify_live_delta` documents it). The test counts
   every render call of the 96-frame ramp and of the following 480-frame unmute: 0 allocations, 0 frees.
7. *`EditRamps` is one ordered map, not a map per target kind.* A strip rewrite then clears its own rows
   and its effect instances' rows with one key range, and the token grows by one 24-byte empty map header
   (no per-slot arrays). `size_of::<PreparedStructuralCommand>()`, the row `compile.rs` counts as the
   prepared protocol layout, grows by `EditRamps`'s 24 bytes, to a measured 840 (the field is 8-aligned; the baseline was not built separately). No
   test pins it: `-p capi`, the debug leg and `audit capi` are green with no re-pin.
8. *Files outside the authorized list, forced by the compiler or by a field addition:*
   `crates/host-core/tests/launch_registry_once.rs` (two `classify_live_delta` calls gain
   `&EditRamps::new()`), and `ramp_samples: None` in `crates/capi/src/runtime/tests.rs`,
   `crates/capi/tests/{design_bound_cache,plan_swap_race,resource_lifecycle}.rs`, `tools/audit/src/capi.rs`
   and `crates/protocol/tests/{console_session_edits,session_edit_builtins_rack,submix_strip_edits,vca_edits}.rs`
   (the last four are inside `crates/protocol/tests/`, authorized). No test body changed in them.
9. *Removal and rack edits left entries behind (wrong in attempt 1; corrected in attempt 2).* Attempt 1
   cleared on the four upserts only and recorded that a stale entry for a removed target was unreachable
   because "the upsert that re-creates the ID clears it". That is false for an effect: `PutTrackEffect`
   and `SetTrackRack` re-create an effect with no upsert, and rewrite its `link_mode` with no length, so an
   earlier `SetEffectLinkMode` length survived them (verdict M1; D3's own rule, "the last edit that writes
   a `(target, row)` decides it"). See Attempt 2.

**What changed.** `crates/session/src/edit_ramp.rs` (`EditRampTarget`, `EditRampRow`, `EditRamps` with `get`,
`set`, `clear_target`, `entries`, `is_empty`); `ramp_samples: Option<u32>` on eleven `SessionEdit` variants and
`SessionEdit::ramp_samples()`; field specs and `schema::session::ramp_samples_field`; the encoder emits the
field last and counts it, the decoder reads it with `optional_one` (absent stays `None`);
`record_edit_ramps` in `prepare_transaction`; `SessionEditError::RampOutOfRange` /
`session.edit.ramp_out_of_range`; `PreparedSessionTransaction::edit_ramps()`; `LiveRamps::resolve`; the
classifier reads the three record lengths through it; `commit_live` passes `transaction.edit_ramps()`; the
registry gains the per-edit table; `docs/C_ABI_V1_QUALIFICATION.md` states the override.

**Digests that moved: one re-pin, one reason.** `COMPLETE_SCHEMA_HASH` `ab357b6c432f9755` ->
`5429f052592101ef` (`crates/conformance/src/protocol_corpus.rs` with its history line "#1394 appends optional
`ramp_samples` to the live-value edits", `scripts/check-protocol-wasm-parity.sh` lines 172-173,
`docs/CONTROL_PROTOCOL_CONFORMANCE.md`, `fuzz/corpus/complete-schema-manifest.md`). The corpus's
`SetTrackFader` sets `ramp_samples: Some(480)`; every other edit encodes an absent field to the same bytes,
so that edit is the whole move (frame count 46 and edit count 46 unchanged; confirmed by the corpus staying
green at the old hash with `None` at `70a0a8991`). **No render digest moved:** `audit capi` `pcm_digest`
(the post-D4 value recorded in #1054), the browser module's expected-resource digests (`check-browser-expected-resources.py
--artifacts` green with no edit), and the builtins fixtures are unchanged.

**Test value and mutation runs** (defect applied by a text edit, test run, file restored; "R" red, "G"
green).

| Test | Plausible defect (mutation) | Result |
|---|---|---|
| `session_wire/tests.rs` `live_value_edits_round_trip_their_optional_ramp_length` (gate 1) | decoder reads an absent field as `Some(0)`; encoder drops an explicit 0; field count ignores the field | R, R, R |
| `protocol/tests/edit_ramps.rs` `the_last_edit_to_write_a_row_decides_its_length` (gate 2) | fader edit records the fader row only; `None` does not remove an earlier length; `UpsertTrack` does not clear | R each |
| same file `every_live_value_edit_records_its_rows_on_its_target` (D3 table) | `SetRouteFollowsMute` recorded on the gain row | R |
| same file `a_length_above_one_second_of_the_committed_rate_refuses_the_whole_transaction` (gate 3) | bound `>=`; bound from the base rate; bound `+1`; only the last edit checked | R each |
| same | `rposition` instead of `position` (first vs last offending index) | G, equivalent: the test has one offending edit and the spec does not pin the index when two offend |
| `session/src/edit_ramp.rs` unit tests | explicit 0 not stored; strip clear leaves its effects; effect clear takes its siblings; effect clear takes the whole strip | R each |
| `host-core/tests/live_delta.rs` `a_records_length_is_its_edits_own_when_the_edit_carried_one` (gate 4) | edit ramps ignored; explicit 0 read as default (in the lookup and in `resolve`); the model's pan window beats the edit; mute reads the fader row; fader row ignored; matrix row ignored; strip lookup keyed on another strip | R each (7 runs) |
| `capi/src/runtime/live_tests.rs` `a_live_edits_own_ramp_length_overrides_the_session_and_does_not_outlive_its_transaction` (gate 6) | `commit_live` passes an empty `EditRamps`; explicit 0 read as default; the previous transaction's lengths kept for the next | R each |

Which defect each new test catches that no existing test does: gate 1, a codec that disagrees with itself on
the one optional field or reads absent as 0 (no test carried the field); gate 2, wrong row, stale or kept
length, upsert residue (no test covers transaction metadata); the D3 table test, an edit that writes the
wrong row or target (this is the only place the table is pinned, and the rows it records are read by later
slices); gate 3, an off-by-one, base-rate or last-edit-only bound, or a refused transaction that commits;
gate 4, a classifier that ignores, defaults or misroutes a length; gate 6, an engine that reaches `commit_live`
without the edits' lengths, or lets one leak into the next transaction.

**Gates run (final tree).** `cargo test --locked -p session`; `-p protocol --features test-support` (132 lib
tests, the new integration file, all others); `-p host-core --features test-support,control-provider --test
live_delta` (34; the spec's command without `control-provider` runs 0 tests, as #1054 recorded);
`-p capi`; `-p conformance` (incl. `conformance_corpus`); the `test-debug-a` leg (133 `ok` results, no
failures) and its doctest leg; `cargo test --locked --release -p audit` (33); `cargo build --locked --release
-p audit && ./target/release/audit capi` (0 allocations, 0 syscalls, `pcm_digest` (the post-D4 value recorded in #1054));
`bash scripts/run-protocol-allocation-audit.sh target/release/audit`; `bash scripts/check-capi-abi.sh`;
`bash scripts/check-protocol-wasm-parity.sh` (ok, simd128); `bash scripts/check-builtins-fixtures.sh . target/release/audit`;
`check-protocol-control-policy.sh`, `check-host-core-policy.sh`, `check-workspace-policy.sh`,
`check-realtime-policy.sh` and the three matching `test-*-policy.sh`; `bash scripts/check-cross-targets.sh`
(PASS); `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `cargo fmt --all --
--check`. The worklet chain, because `protocol` is compiled into the browser module: `build-web-audioworklet.sh
--named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py
--artifacts` (32 red mutations, every budget met), and `test-web-audioworklet.sh` under a private `TMPDIR` (rc 0,
nothing left behind). Not run: AArch64 (CI only), the browser matrix.

**Open items.** The SDK does not encode the field (#1383, #1385) and browser records do not lower into it
(#1382); both are non-goals here. `control_smoothing` itself stays a structural difference until #1365.

**Attempt 2** (implementer; fixes verdict M1, m1 and the NITs of `submix-verdicts/1394-attempt1.md`).

*M1.* `record_edit_ramps` (`crates/protocol/src/model.rs`) now clears, in addition to the four upserts:
`RemoveTrack`/`RemoveSubmix` (`Strip` target, which takes the strip's effects with it), `RemoveRoute`,
`RemoveVca`; `PutTrackEffect` and `RemoveTrackEffect` (their `Effect` target); `SetTrackRack` (new
`EditRamps::clear_rack(strip, rack)`: every `Effect` entry of that strip and rack, not the strip's own rows,
not its other racks, not another strip). Every other edit was checked and writes no keyed row:
`SetTrackEffectOrder`, `SetEffectIdentity`/`Quality`/`Bypass`/`Sidechain`, `UpsertEffectParam`/
`RemoveEffectParam` (none writes `link_mode`), `SetRouteSource`/`SetRouteDestination` (leave the route's
gain, mute and matrix), `SetTrackSourceAssignment`, sources, outputs, automation, and the session scalars.
`SetConsole` and `SetTrackConsole` write their own rows with a length and do not rewrite another keyed
target's rows. All were decidable from D3's rule; none needed a ruling. The `match` in `record_edit_ramps`
no longer ends in `_ => {}`: the no-op edits are listed, so a new `SessionEdit` variant fails to compile
until it is classified. Gate 2 gained `an_edit_that_rewrites_or_removes_a_target_clears_its_earlier_lengths`
(eleven cases: set then put; set then `SetTrackRack`; set then remove; remove then put; put then set keeps
the later length; each of the four removals; the rack and put cases also keep the strip's own fader rows and
the other strip's effect entry). Its base gained a submix `bus`, an empty VCA `grp` and no automation (the
fixture's automation targets the `eq`, which the removal cases delete).

*m1.* `controller/tests.rs` `a_ramp_length_out_of_range_answers_its_own_code_at_the_edit_path`: a
two-edit transaction whose second edit carries `u32::MAX` answers `ValidationFailed` with exactly one
diagnostic, code `session.edit.ramp_out_of_range`, path `edits[1]`, `operation_index` 1, and the revision
stays 7.

*NITs.* n1: deleted the first `EditRamps` unit test (gates 2 and 4 catch what it caught). n2: `Key` now holds
the typed `RackName` and `EditRampRow` (both gained `Ord`), so `from_code` and the `filter_map` are gone;
`entries()` has one `unreachable!` for an effect part on a non-strip owner, which `split` cannot produce.
n3: the gate-7 command now carries `control-provider`. n4 (the uncharged control-thread heap of
`EditRamps`) is informational and unchanged.

| Test | Plausible defect (mutation) | Result |
|---|---|---|
| `edit_ramps.rs` `an_edit_that_rewrites_or_removes_a_target_clears_its_earlier_lengths` | `PutTrackEffect` does not clear; `SetTrackRack` does not clear; `RemoveTrackEffect` does not clear; `RemoveTrack`, `RemoveSubmix`, `RemoveRoute`, `RemoveVca` each do not clear; `SetTrackRack` clears the whole strip; `PutTrackEffect` clears the whole strip | R each (9 runs) |
| `session/src/edit_ramp.rs` `clearing_a_rack_takes_that_racks_effects_of_that_strip_only` | `clear_rack` ignores the rack; takes the strip's own rows; takes other strips' entries (no owner bound) | R each (the protocol test cannot reach another rack: link edits on the console and builtins racks are refused) |
| `controller/tests.rs` `a_ramp_length_out_of_range_answers_its_own_code_at_the_edit_path` | the controller maps `RampOutOfRange` to `session.edit.not_found`; the path index is wrong | R each |

Each mutation was applied by a text edit, run, and restored; the restored tree is green.

*Gates run (attempt 2, final tree).* `cargo test --locked -p session`; `-p protocol --features test-support`
(133 lib tests, `edit_ramps` 4); `-p conformance`; `-p host-core --features test-support,control-provider
--test live_delta` (34); `-p capi`; `cargo clippy --locked --workspace --all-targets --all-features -- -D
warnings`; `cargo fmt --all -- --check`; `check-workspace-policy.sh`, `check-realtime-policy.sh`,
`check-protocol-control-policy.sh`; `check-cross-targets.sh` (PASS). The wire and the corpus did not change
(no encoder or schema edit), so the schema hash `5429f052592101ef` is unmoved and the parity script was not
re-run. `protocol` is compiled into the browser module, so the worklet chain ran: `build-web-audioworklet.sh
--named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py
--artifacts` (32 red mutations, every budget met), `check-scalar-oracle-absent.py --wasm`,
`test-web-audioworklet.sh` (rc 0, nothing left in its TMPDIR).

**Batch follow-ups** (stream E2 batch, after the verdicts
`docs/handoffs/decision-15-2026-10-05/verdicts/stream-e2/1394-attempt1.md` (FAIL, M1 fixed in attempt 2)
and `1394-attempt2.md` (PASS) and the probe `1394-attempt1-probe.rs`).

- *Root rulings (2026-10-12), recorded in the spec text above:* D3's clearing rules (put and rack edits by
  D3's own rule, the five removals ratified); the `RackName` derives as a named exception in Authorized
  paths. Gate 7's command already carries `control-provider` (attempt 2's fix; checked, 34 tests ran then
  and 35 now with #1365).
- *m-a:* `SetVcaFader`'s two rows (`VcaFader`, `VcaMute`, target `Vca`) were pinned by no test. A `SetVcaFader`
  entry is now in `every_live_value_edit_records_its_rows_on_its_target`, and in its "bare edit records
  nothing" leg. Defect it catches that no other test does: a `SetVcaFader` that records one row only, which
  the VCA-row slice would then read as a missing mute length.
- *n-a:* `SessionEdit::ramp_samples()` no longer ends in `_ => None`; every non-length variant is listed, so
  a new edit fails to compile until it is classified (as `record_edit_ramps` already does).
- *Mutation runs* (defect applied, run, restored; `-p protocol --features test-support`):
  - record `&[EditRampRow::VcaFader]` only for `SetVcaFader`: RED
    `every_live_value_edit_records_its_rows_on_its_target` (the only red test of the `edit_ramps` file;
    before this follow-up the same mutation was green in the whole crate); green after the revert.
  - remove the `RemoveVca` arm from `ramp_samples()`: `protocol` fails to compile (`E0004`,
    `RemoveVca` not covered); green after the revert.

**Batch verdict follow-ups** (stream E2 batch verdict, head `29bd7f3ca`, 2026-10-12).

- *n1:* D6's sentence and the comment in `SessionEdit::ramp_samples()` claimed a variant that gains a
  `ramp_samples` field cannot lose it. The match has no wildcard, so a new *variant* must be classified
  (47 variants: 11 return their field, 36 return `None`); an existing `=> None` arm that gains a field still
  compiles. Both now say so. Comment and prose only; no code path changed.

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

**What was ported by meaning (#1394).**

- *Where the lookup lives.* Stream A2 made `fader_records` and `matrix_record` the one per-owner derivation, called by
  `classify_live_delta` and by a successor's carry. D5/D6's lookup therefore sits in them: `StripEditRamps::of`
  (`prepare.rs`) reads a strip's `(Strip, Fader)`, `(Strip, Mute)` and `(Strip, Matrix)` entries once, and both
  callers pass it. `LiveRamps::resolve` is in `prepare.rs` beside `for_row`.
- *Record kinds, as D6 asks ("every record kind the classifier produces on main when it merges").* The classifier
  still produces three: `FaderDb`, `Mute` and the strip matrix record; stream B3 added none (its input, route and
  effect cells have no classifier record, and effect record lengths are the descriptor's, a non-goal). **One kind
  is new to the lookup: the successor's carry retargets (`FaderDb`, `Mute`, the strip matrix record), written at
  preparation by A2's `join_strips`.** A retarget is the record of the same edit; without the lookup a rebuild
  transaction's `ramp_samples` would be silently dropped for every carried strip. So `SuccessorBase` gains
  `edit_ramps: &EditRamps` (empty is "none"), `crates/control-plane/src/control.rs` fills it from the prepared
  transaction beside `commit_live`, and the join reads it. Browser command records take their length from the
  command word that #1364 resolves at the single decode site (no B3 command kind).
- *Authorized paths.* `crates/host-core/src/prepare.rs` and the six `SuccessorBase` literals (the `successor_swap`
  and `support/successor.rs` test files and `capi/tests/resource_lifecycle.rs`) are outside the list above; the
  compiler and A2's design force them. `ramp_samples: None` was added to B3's `SetTrackFader`/`SetTrackMatrixOrPan`
  constructions in `capi/tests/plan_swap_race.rs` and `host-web/tests/strip_cells_cross_host.rs`.
- *New tests.* (1) `successor_swap` `a_retarget_carries_the_edit_ramp_of_its_transaction_on_every_width_and_form`: a
  transaction whose edits gave strip X's fader 37 and its pan 100 (against the model's 300) retargets exactly like the
  records the classifier would derive; not equal to the same retarget without the lengths. Defect: a join that reads
  the session's lengths and ignores the transaction's. (2) `-p capi` `a_rebuilds_carried_strip_retargets_over_its_edits_own_ramp`:
  on the default session, a rebuild whose mute edits say `ramp_samples` 0 renders bit for bit like the same rebuild
  on a session that steps every ramp, and the same edits with no length do not. Defect: the control plane preparing
  the successor without the transaction's edit ramps (the host-core test cannot see that; the live-path test never
  rebuilds). Both red under the mutations below.
- *Retained bytes.* The token still carries `EditRamps`; see the ceilings below.

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
