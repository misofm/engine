# Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Every session carries `control_smoothing`: either `default` or its own mute, fader and pan ramp
lengths in milliseconds. A live record ramps over the session's length for its row, or over the
researched default table when the session says `default`. An explicit 0 in the session stays legal: it is the hard switch that
sample-exact edits and bit-comparison tests use. In this slice the C ABI's live fader, mute and
pan/matrix edits start to ramp; today they step. Decision 14's F7 (the ramp half) is closed for the
C ABI rows that are live today, and every later live row reads the same table. The optional
per-edit length is *Carry an optional per-edit ramp length on live session edits* (#1394).

## Context

- **Owner decision 1** (`docs/rulings/engine-footprint-2026-09-28.md:39-48`): ramp lengths are
  optional session settings, with one documented default table. Decision 15 D15-1 adds that no live
  value steps by omission. Its recorded resolution adds an optional per-edit ramp length, end to
  end (absent = session default, explicit 0 legal); that half is #1394.
- **Defaults.** *Research: default ramp lengths for live mute, fader and pan changes (cited,
  measured, listened)* (#1055) closes with `FINDINGS.md` section 9, the cited and measured default
  for every row. Sections 1-8 already give mute 10 ms, fader 20 ms, pan and raw matrix 20 ms
  (`docs/handoffs/control-smoothing-defaults/FINDINGS.md:11-17`), rounding
  `floor(ms * rate / 1000 + 1/2)` in `f64` from the `f32` value, and bounds `0 <= ms <= 1000`
  (`:19-30`). The blinded listening session is #1388; it can change only these values.
- **The C ABI seam.**
  - `LiveRamps` (`crates/host-core/src/live_delta.rs:33-58`) holds `fader_samples` and
    `mute_samples`; `for_session` returns 0 for both (`:49-50`).
  - `classify_live_delta(current, next, ramps)` (`:211-215`) puts them on the `FaderDb` and `Mute`
    records (`:293-306`). The matrix record carries the post-commit model's own
    `smoothing_samples` (`:271-274`, `:308-313`), through `lower_matrix_or_pan`
    (`crates/builtins-compiler/src/lib.rs:4925-4940`).
  - `commit_live` calls it with `LiveRamps::for_session(next)` once per commit
    (`crates/control-plane/src/control.rs:1201-1211`), where `next` comes from the prepared token's
    `prospective_session()` (`crates/protocol/src/controller.rs:1139`).
- **The model's pan window.** `MatrixOrPan::{Pan, Matrix}::smoothing_samples` is a required `u32`
  (`crates/session/src/model.rs:655-680`); `Submix::unity` writes 0 (`:749`). The prepared window
  is read only by `set_target` (`crates/builtins/src/lib.rs:2999-3006`), which no production path
  calls; live matrix records bring their own (`set_target_over`, `:3008`).
- **Schema.** Root keys are required, explicit and ordered, and every field is explicit
  (`docs/SESSION_SCHEMA_V1.md:35-40`); V1 has no optional fields (`:210`). A tagged value spells a
  "nothing set" choice explicitly, as `"sidechain": { "kind": "none" }` does (`:40`). A key added
  later is required too, so an older document without it is `schema.missing_field`
  (`delay_samples`, `crates/session/src/parse.rs:1061-1063`). The JSON schema lists the root keys
  (`docs/session-v1.schema.json:7`). The SDK's canonical writer has its own root
  key list and float key set (`sdk/src/internal/session-json.ts:13-29`, `:50-65`), and is checked
  against the Rust-generated writer corpus
  (`fixtures/session-canonical/v1/canonical-writer-corpus.json`, from
  `crates/session/src/canonical.rs:505-523`). Its tagged objects take their key order from
  `taggedOrder` (`sdk/src/internal/session-json.ts:91`). The SDK builder emits the root model in
  `sdk/src/core/session.ts:1645-1662`. Visitor root field IDs are at
  `crates/session/src/visit.rs:88`; field 8 is retired (`:110`).
- **Pinned text.** `docs/C_ABI_V1_QUALIFICATION.md:265-267` ("steps until #1054");
  `crates/host-core/tests/live_delta.rs:39-42` (`STEP`) and
  `records_carry_the_ramps_and_the_model_smoothing` (`:603-643`).
- *Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live
  console lanes* (#1053) is the umbrella of the C ABI live path. Its delivered slices (#1255,
  #1257, closed) are what this issue builds on; it does not wait on the umbrella.

## Decisions frozen for this slice

- **D1. Schema.** One **required** tagged root object, `control_smoothing` (SDK
  `controlSmoothing`), in canonical order after `vcas` and before `outputs`, consistent with "V1
  has no optional fields" (`docs/SESSION_SCHEMA_V1.md:210`):
  - `{ "kind": "default" }`: the session uses the researched default table (D2). No other key.
  - `{ "kind": "explicit", "mute_ms": .., "fader_ms": .., "pan_ms": .. }`: all three required, in
    that order, each an `f32` with `0 <= ms <= 1000`.
  - A missing root key is `schema.missing_field`; an unknown kind or key is refused; out of range is
    `NumericOutOfSchemaRange` at `$.control_smoothing.<key>`. Canonical JSON always writes the key
    (`-0.0` stays `-0.0`).
  - The model field is `SessionModel::control_smoothing: Option<ControlSmoothing>`: `None` is
    `default`, `Some` is `explicit`. The visitor walks it as root field 17 (kind 1, then
    `mute_ms` 2, `fader_ms` 3, `pan_ms` 4 for `explicit`).
  - The JSON schema, the SDK canonical writer (`taggedOrder`) and the SDK builder learn the key. The
    builder writes `{ "kind": "default" }` until #1364 D4 lets an author set lengths.
  - Why tagged and not three values written out: the owner's decision 1 makes the lengths an
    optional setting with one documented default table, and #1388's listening result may change
    the defaults. A document that says `default` follows the table; one that wrote numbers keeps
    them. The tag keeps both explicit without an optional field.
- **D1a. Migration.** A one-off rewrite adds `"control_smoothing": { "kind": "default" }` at its
  canonical place in every checked-in session document, fixture and embedded test session. Nothing
  else in any document moves. `default` resolves to the same lengths today's sessions, which carry no
  such key, prepare with, so every render digest stays; a digest of session-document bytes moves by this one key only.
- **D2. Default table and rounding**, in `crates/session`. `CONTROL_SMOOTHING_DEFAULT` holds
  #1055's section 9 values. `SessionModel::control_smoothing_samples(&self) ->
  ControlSmoothingSamples { mute, fader, pan }` applies
  `floor(f64::from(ms) * f64::from(sample_rate_hz) / 1000.0 + 0.5) as u32` to the session's value
  or the default. It is the only conversion; every host calls it.
- **D3. The key each live row uses.** One table, in `docs/SESSION_SCHEMA_V1.md` and on `LiveRamps`,
  copied from #1055 section 9 (if section 9 differs, it wins and this table follows):

  | key | rows |
  |---|---|
  | `fader_ms` | fader dB, input trim, send gain, VCA offset, detector link glide (#1371) |
  | `mute_ms` | mute, solo, polarity invert (`mute_ms` times two; root, 2026-10-05, from #1055), send mute, send `follows_mute` toggle, VCA mute, effect bypass crossfade (#1341) |
  | `pan_ms` | pan, matrix, send matrix |

  - **Polarity invert is twice the mute ramp** (root, 2026-10-05, from #1055; `FINDINGS.md` 9.2,
    9.8): a derived rule on the `mute_ms` key (`2 x mute_ms`, 20 ms at the defaults), not a fourth
    key.
  - **Matrix coefficients through zero** (root, 2026-10-05, from #1055; `FINDINGS.md` 9.1). A raw
    matrix coefficient (in `[-1, 1]`) or a send matrix coefficient (any finite value) that crosses
    zero is a polarity flip of that path on the `pan_ms` ramp. The default table keeps
    `pan_ms >= 2 x mute_ms` (20 >= 2 x 10), so such a crossing clicks no more than a mute. Gate 2
    checks it on the defaults. A session's explicit values may set `pan_ms` lower; it then gets a
    louder crossing.

- **D4. `LiveRamps`.** It gains `pan_samples` and `link_samples`; `for_session(model)` fills all
  four from D2 and D3. It gains `LiveRampRow`, one variant per row of D3's table, and
  `for_row(row: LiveRampRow) -> u32`, which returns that row's key field, and twice `mute_samples`
  for `PolarityInvert` (root, 2026-10-05, from #1055; at most 192,000 samples at 1000 ms and 96 kHz,
  inside the trim kernel's exact `f32` countdown, `2^24`). Every C ABI row reads its length through
  it: this slice's fader, mute and matrix; input trim and polarity (#1261), sends (#1225, #1226),
  VCA (#1247), the link glide (#1371). #1394 adds `resolve(row, edit_ramp)` on top of `for_row`. The
  bypass crossfade (#1341) always uses `mute_samples` (D15-1: the one row with no per-edit length).
  No row has a separate default; the polarity row's length derives from `mute_samples` and has no
  key of its own (root, 2026-10-05, from #1055).
- **D5. Pan and matrix.** Precedence for a matrix record: the post-commit model's
  `smoothing_samples` if non-zero; else `pan_samples`. (#1394 puts an edit's own ramp first.)
  Preparation and the model are unchanged, so canonical JSON still says 0.
- **D6. Explicit 0.** A session with all three keys at 0 renders exactly today's steps.
- **D7-D9. Moved.** The per-edit ramp (wire field, `EditRamps`, `session.edit.ramp_out_of_range`,
  the classifier lookup and the protocol corpus re-pin) moved to *Carry an optional per-edit ramp
  length on live session edits* (#1394). This slice changes no protocol wire and no classifier
  signature.
- **D10. The `-0.0` artefact.** With a non-zero `fader_ms`, a fader move on a lane that stays muted
  retargets 0 to 0 over the ramp, and the kernel turns a negative input into `-0.0` where a rebuild
  gives `+0.0` (`crates/host-core/src/solo.rs:58-66`). Keep the ramp: a step would cut short a mute
  ramp in flight. Bit-identity gates compare after the ramp.
- **D11. Acked-batch question.** No queue changes; records only carry a different length. No ack
  can precede a drop.

## Deliverables

1. D1 and D2 in `crates/session`, with `docs/SESSION_SCHEMA_V1.md` (the root key list at `:35-38`,
   the tagged grammar, bounds, default table, rounding, D3's table with its polarity note and its
   matrix property (root, 2026-10-05, from #1055)), `docs/session-v1.schema.json`,
   the writer corpus regenerated (every document gains `default`; one document carries `explicit`),
   the SDK canonical writer's root key, tagged order and float keys, and the SDK builder's
   `{ "kind": "default" }` emit. D1a's migration lands in its own commit.
2. Every `SessionModel { .. }` struct literal outside `crates/session` gains
   `control_smoothing: None` (mechanical; about 98 sites in tests and helpers).
3. D4 and D5 in `crates/host-core/src/live_delta.rs`. The `commit_live` call
   (`crates/control-plane/src/control.rs:1207-1211`) is unchanged: `for_session(next)` now returns
   the session's lengths.
4. `docs/C_ABI_V1_QUALIFICATION.md:265-267`: live fader, mute and pan edits ramp over the session's
   `control_smoothing`, else the default table.
5. The tests below. Rewrite `records_carry_the_ramps_and_the_model_smoothing`; delete `STEP` if
   nothing uses it.

## Authorized paths

- `crates/session/src/{model,parse,validate,canonical,visit,lib}.rs`, `crates/session/tests/`,
  `docs/SESSION_SCHEMA_V1.md`, `docs/session-v1.schema.json`,
  `fixtures/session-canonical/v1/canonical-writer-corpus.json`, `sdk/src/internal/session-json.ts`,
  `sdk/src/core/session.ts` (the root model emit only; #1364 D4 adds authoring), `sdk/test/`
- Every checked-in session JSON document and embedded test session (D1a's key only), and each
  digest of session-document bytes that the key moves (gate 7)
- `crates/host-core/src/live_delta.rs` (stream B owns it), `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/live_tests.rs` (tests only), `docs/C_ABI_V1_QUALIFICATION.md`
- Struct-literal additions only (deliverable 2), in any file that builds a `SessionModel`
- Root ruling (2026-10-12), a named exception: `crates/capi/tests/resource_lifecycle.rs` (the race
  settle of `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`;
  Attempt record, "Where the spec and the tree differed", item 7)
- Named exceptions, batch follow-ups, recorded after the fact (each necessary or mechanical; the
  Attempt record's "Batch verdict follow-ups" says why):
  - `crates/host-core/src/lib.rs` (the `LiveRampRow` re-export that D4 needs)
  - `crates/host-core/tests/launch_registry_once.rs` (the `LiveRamps` literal gains two fields)
  - `.claude/skills/author-session/SKILL.md` (the root-key list and a `control_smoothing` section)
  - `scripts/check-browser-expected-resources.py` (the self-test's one-byte mutation, the companion of
    the gate 7 re-pin)

## Non-goals

- **Browser and SDK authoring**: *Resolve an absent live ramp to the session default on the
  browser and in the SDK* (#1364): browser commands, `sdk/src/core/live-controls.ts`, the builder's
  `controlSmoothing` and the authoring request, and the cross-host parity gate.
- **The per-edit ramp**: *Carry an optional per-edit ramp length on live session edits* (#1394):
  the wire field, `EditRamps`, its refusal, `LiveRamps::resolve` and the classifier lookup. The
  browser lowering into that field is #1382, the SDK encoding #1383 and #1385.
- **Editing `control_smoothing` by transaction**: *Edit control_smoothing by a session transaction,
  model-only* (#1365). Until it lands, a C ABI session sets the key at boot.
- The C ABI rows that are not live yet (trim, polarity, sends, VCA, link) and the bypass crossfade:
  their issues call `LiveRamps::resolve` (D4).
- No change to preparation, to `lower_matrix_or_pan`, or to any prepared plan.

## Hazards

- C ABI live tests compare a live edit with a rebuilt control. A test whose claim is bit-identity
  to a rebuild authors `control_smoothing` at 0 (D6).
  Otherwise it compares after the ramp (`window`, `crates/capi/src/runtime/live_tests.rs:325-327`,
  already adds the smoothing). Do not weaken a comparison.
- `docs/handoffs/control-smoothing-defaults/` is research evidence. Do not edit it.
- Land it in the order of the deliverables, with a local checkpoint after each compiles.
- *Declare a strip's console link mode in the session, the wire and the SDK* (#1369) also migrates
  every document. Whichever merges second reruns its own one-off rewrite on `main`; never merge the
  two migrations by hand.

## Objective gates

1. **Schema** (`crates/session/tests/`). Accept `default`, and `explicit` with 0, `-0.0` and 1000.
   Refuse, each at its path: a missing root key (`schema.missing_field`), a negative value,
   1000.001, a missing length in `explicit`, a length in `default`, an unknown kind, an unknown
   key, a non-object. `default` and `explicit` sessions round-trip through canonical JSON to
   identical bytes. The writer corpus
   regenerates with the new document, and the SDK writer reproduces it (`builder-evals.mjs`
   corpus loop, run by `check-sdk-headless.sh`).
2. **Rounding** (`crates/session` unit test). 441/480/882/960 for 10 ms and 882/960/1764/1920 for
   20 ms at the four launch rates; 221 for 5 ms at 44.1 kHz (a tie); 0 for `-0.0`;
   `{ "kind": "default" }` gives the default table. `CONTROL_SMOOTHING_DEFAULT` holds
   `pan_ms >= 2 * mute_ms` (D3's matrix property; root, 2026-10-05, from #1055).
3. **Classifier** (`crates/host-core/tests/live_delta.rs`, rewritten test). At 48 kHz with the
   default table: a fader, a mute and a pan change give `FaderDb` 960, `Mute` 480, matrix 960 (96
   when the model says 96). With `mute_ms` 5, `fader_ms` 15 and `pan_ms` 25: 720, 240, 1200. With
   all session keys at 0: 0, 0, 0. `for_row` returns the D3 key's field for every `LiveRampRow`, and
   twice `mute_samples` for `PolarityInvert`: 960 at 48 kHz with the default table, 480 with
   `mute_ms` 5 (root, 2026-10-05, from #1055).
4. **C ABI, the ramp is heard** (`crates/capi/src/runtime/live_tests.rs`, new). A playing session
   with `"control_smoothing": { "kind": "default" }` takes a mute transaction. The first block after the commit equals
   neither the unmuted nor the muted control; from the first block after the ramp, the output is
   bit-identical to a control booted muted. Every render call that carries the commit and the ramp
   allocates nothing (`bench_support::alloc` thread counters, process statics warmed). Root ruling
   (2026-10-12): the commit itself is control-plane work and is not counted; this replaces "The
   commit and the ramp blocks allocate nothing".
5. **C ABI, explicit 0** (same file). The same edit with all session keys at 0 is bit-identical to
   the muted control from the first block.
6. **Commands:**
   - `cargo test --locked -p session`, `cargo test --locked -p protocol --features test-support`,
     `cargo test --locked -p host-core --features test-support,control-provider --test live_delta`
     (root ruling, 2026-10-12: without `control-provider` the test file is `cfg`ed out and the command runs 0 tests),
     `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `cargo build --locked --release -p audit && ./target/release/audit capi`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `bash scripts/check-session-policy.sh`, `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
7. **Migration and digests.** `git diff` of the D1a commit adds only the one key; every migrated
   document passes `cargo run --locked -p session-validator -- validate <file>`. No render digest
   moves: preparation is unchanged and `default` gives the lengths today's key-less sessions prepare with. A
   digest of session-document bytes moves by the key; each is listed with its file and the reason
   "D1a adds the required `control_smoothing` key" and re-pinned on its own, never in bulk
   (decision 15, risk 4). Root ruling (2026-10-12): any pinned digest other than these moves is a
   defect; the unpinned audit capi pcm_digest moves at D4 (cb10fbface44a3a4 -> a9738fa800afcab3)
   because live edits now ramp.

## Test value

- Gate 1 turns red if the parser defaults a missing key, accepts a value over 1000, confuses the
  two kinds, or the Rust or SDK writer drops, reorders or invents the object. No test covers the
  key today.
- Gate 2 turns red if the rounding truncates, rounds half to even or converts in `f32` (each gives
  220 at 5 ms and 44.1 kHz).
- Gate 2 also turns red if a default change puts `pan_ms` below twice `mute_ms`, so a matrix
  coefficient through zero would click more than a mute (root, 2026-10-05, from #1055).
- Gate 3 turns red if the classifier keeps a pan window of 0, ignores the session for one row, or
  maps a row to the wrong key, or gives polarity the plain mute length (root, 2026-10-05, from
  #1055).
- Gate 4 turns red if `for_session` still returns 0 (the mute steps), or a ramp never lands on its
  target.
- Gate 5 turns red if a session 0 is read as "use the default".

## Dependencies

- *Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)* (#1055)

#1055 closes with `FINDINGS.md` section 9 (D2's values and D3's table). *Carry an optional
per-edit ramp length on live session edits* (#1394) depends on this issue.

## Attempt record

**Attempt 1** (implementer, branch `codex/d15-stream-e2`, base `8c0deee2c`). Commits in deliverable
order: D1/D2 (`8a68e257f`), D1a migration alone (`351650a5b`), capi snapshot re-pin, session tests and
writer corpus, D4/D5, C ABI tests and doc, then one commit per re-pinned digest (below), and two test
tidy-ups. The first commit leaves the checked-in documents without the key, so tests that read them
are red at that checkpoint only; D1a is the next commit.

**Section 9 against D3.** `FINDINGS.md` section 9.1's key and default table matches D3 row for row
(fader/trim/send gain/VCA offset/link glide on `faderMs`; mute/solo/send mute/`follows_mute`/VCA
mute/bypass on `muteMs`; polarity `muteMs` x 2; pan/matrix/send matrix on `panMs`; 10/20/20 ms). Nothing
to correct.

**Where the spec and the tree differed (recorded, not improvised around).**

1. *Deliverable 2 is empty.* No `SessionModel { .. }` struct literal exists outside `crates/session`
   (the "about 98 sites" are return types and patterns; every other session comes through
   `parse_session_json`). Nothing was added to any literal. `cargo check --workspace --all-targets
   --all-features` was green with the model field alone.
2. *The `commit_live` call moved* to `crates/control-plane/src/control.rs:1354`; the call is
   unchanged.
3. *Gate 3's command runs 0 tests* as written: `crates/host-core/tests/live_delta.rs` is
   `#![cfg(feature = "control-provider")]` and `--features test-support` alone does not enable it.
   It was run with `--features test-support,control-provider` (33 tests); the workspace debug leg
   unifies the feature.
4. *Gate 2's claim that an `f32` conversion gives 220 at 5 ms and 44.1 kHz is wrong for the two
   natural spellings*: `ms * rate as f32 / 1000.0 + 0.5` and `ms * (rate as f32 / 1000.0) + 0.5` both
   give 221 there (the product is exact). A search of 0.01 ms steps over the four rates found the
   `f32` pipelines differ at 45 ms / 44.1 kHz (1984.5, a tie: 1984 in the second spelling) and at
   263.39 ms / 44.1 kHz (11615.4996: 11616 in the first). Both are in the test, with expected values
   from exact rational arithmetic on the `f32` value (not the crate's formula). The 5 ms tie stays
   (it catches truncation and half-even).
5. *Gate 4's "the commit ... allocate nothing"* cannot hold for the commit itself: the commit is the
   control plane, and `classify_live_delta` documents that it allocates there (the masked clone and
   two canonical JSON strings); measured at 150 allocations for the mute transaction. The test counts
   the render calls that carry the ramp (`bench_support::alloc` thread counters around the `render`
   call alone, process statics warmed by earlier blocks) and asserts exactly 0 allocations and 0
   frees in each of them, the realtime rule of `AGENTS.md`. If the owner meant the commit to be
   counted, that is a different (control-plane) claim and needs its own issue.
6. *The explicit-0 / hard-switch tests need a latency-free session.* Gates 4 and 5 use the
   stateless session (`stateless_session`: no console slot, insert or input filter), asserted at
   latency 0, so "from the first block" is exact; the parity session's EQ latency would make the
   first block a mix of old and new samples.
7. *One edited test is outside the authorized path list.* `crates/capi/tests/resource_lifecycle.rs`
   `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free` compares the raced
   plan's final block with a fresh plan after a settle of `latency + one quantum` blocks; with the
   default table the fader and mute ramps (960 and 480 samples at 48 kHz) outlast that, and the test
   went red (final block `[-0.1245, 0.0068]` against `[-0.1212, 0.0398]`). The spec's hazard names
   `live_tests.rs` only, so this is a gap in its list. The edit adds `race_settle_blocks`, which
   lengthens the settle by the session's longest ramp (`control_smoothing_samples`); the comparison
   itself (bit-identity, every render call allocating and freeing nothing) is untouched. The race
   keeps the default table, so ramps start and retarget under swaps. Reverting the settle makes it red
   again (it was red before the edit).
8. *Editing `control_smoothing` is a structural difference* in `classify_live_delta` (it is not
   masked), so it rebuilds until #1365. `docs/C_ABI_V1_QUALIFICATION.md` says so.

**What changed.** `control_smoothing` is a required tagged root object (parse, validate, visitor field
17, canonical writer, `docs/SESSION_SCHEMA_V1.md`, `docs/session-v1.schema.json`, SDK writer and
builder); `CONTROL_SMOOTHING_DEFAULT`, `ControlSmoothing`, `ControlSmoothingSamples` and the one
conversion `SessionModel::control_smoothing_samples`; `LiveRamps { fader, mute, pan, link }_samples`,
`LiveRampRow`, `LiveRamps::for_row`, the matrix precedence (the model's non-zero window, else
`pan_samples`) in `live_delta.rs`, nothing else there. The author-session skill text gained the key.
Existing C ABI live tests: the bit-exact-to-rebuild ones compare after the session ramp
(`live_pcm_shape` runs under both the default table and an explicit 0; the pending-candidate and
submix tests use the ramp in `window`); the carrying-swap test, whose claim is bit-identity from the
swap block, authors `control_smoothing` at 0 (D6).

**Digests that moved (gate 7), each re-pinned on its own, reason "D1a adds the required
`control_smoothing` key"; no pinned render digest moved** (`ACCEPTED_GRAPH_PCM_SHA256`, `_METERS_`,
the browser module's render digests and the builtins `resources.jsonl` all unchanged). The unpinned
`audit capi` `pcm_digest` did move, at D4 (`a575e74fd`), on purpose: `cb10fbface44a3a4` at the base and
after the D1a migration, `a9738fa800afcab3` after D4, because `tools/audit/src/capi.rs` commits a live
fader/mute/pan edit every 64 calls on a `default` session and those edits now ramp where they stepped
(verified by the verifier at the three commits; no gate pins this digest, and the audit still reports 0
allocations and 0 syscalls). The earlier wording "`audit capi` `pcm_digest a9738fa800afcab3` unchanged"
was wrong and is corrected here (batch follow-up, MINOR 1):

| pin | file | old -> new |
|---|---|---|
| canonical session snapshot length | `crates/capi/src/runtime/tests.rs` `ALL_COMMAND_RESPONSE_VECTORS[1]` | 14,193 (0x3771) -> 14,245 (0x37a5), +52 bytes |
| `session_template_sha256` of `fixtures/session/v1/canonical.json` | `tools/audit/src/fixture_builtins.rs` (generator) and the regenerated `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml`, `MANIFEST.tsv` | `dc19e091f9fd...` -> `4c5756345cf9...`; a fresh `--write` differed from the tree in exactly these three files |
| `ACCEPTED_MANIFEST_SHA256` | `tools/audit/src/builtins_graph.rs` | `0834666e8f2e...` -> `d8d3d6a39d6a...` |
| joined-corpus manifest identity | `tools/audit/src/fixture_builtins.rs` `issue064_checked_corpus_is_read_only_and_complete` | same pair |
| `sessionDocumentBytes` | `hosts/host-web/tests/browser-v1/expected.json` | "1969" -> "2021", +52 |
| the self-test's one-byte mutation of it | `scripts/check-browser-expected-resources.py` | 1970 -> 2022 (kept at pinned + 1) |

`artifacts/steps/*/web-rebuild-cost.jsonl` and `chromium.json` name four migrated fixtures by their old
sha256. They are recorded measurements of past runs and no gate reads them; they were not edited.
`fixtures/session/v1/canonical.json` stops at validator stage 5 with `effect.native.unavailable`, as
`tools/session-validator/tests/validate.rs` expects; the other 20 migrated documents PASS all stages.

**Test value and mutation runs** (defect applied by `python` text edit, test run, file restored; "R" is
red, "G" green). Where a range check exists in both the parser and the validator, a single-site
mutation is an equivalent mutant (the other site refuses the same code at the same path), so the
bound mutations change both.

| Test | Plausible defect (mutation) | Result |
|---|---|---|
| `control_smoothing.rs` refusals | upper bound 1001 (parser and validator) | R `every_malformed_control_smoothing_is_refused_at_its_path`, `a_built_model_with_a_length_outside_the_domain_is_refused` |
| same | lower bound -1 (both) | R, same two |
| same | `default` accepts a length (the `reject_key` loop emptied) | R refusals |
| same | a missing root key means `default` (no diagnostic) | R refusals |
| same | an unknown kind accepted as `default` | R refusals |
| `a_built_model_...` | the validator's range check removed | R |
| `both_kinds_round_trip...` | writer emits the key after `outputs` | R |
| `control_smoothing_samples_round_half_up_in_f64` | truncating `floor(x)`; half-even `round_ties_even`; `f32` pipelines `ms * rate as f32 / 1000 + 0.5` and `ms * (rate as f32 / 1000) + 0.5` | R in all four |
| `the_default_table_is_...`, `the_default_pan_ramp_...` | default `pan_ms` 15; default `mute_ms` 12 | R both tests in both runs |
| `live_delta.rs` (`records_carry_the_session_ramps_and_the_models_own_pan_window`, `a_raw_matrix_without_a_window_...`, `for_row_returns_the_key_field_of_every_row`) | `for_session` returns 0; matrix ignores `pan_samples`; matrix ignores the model window; fader row reads mute key; mute row reads fader key; polarity at the plain mute length; polarity on the fader key; pan rows on the fader key; link glide on the mute key; `link_samples` filled from pan; a session 0 read as the default; the fader record on the mute ramp | R in each (one or more of the three; the polarity and link mutants only in `for_row_...`) |
| `a_live_mute_ramps_over_the_session_mute_ramp_and_lands_on_the_muted_control` (gate 4) | `for_session` returns steps; mute ramp cut to a quarter; mute row on the fader key | R each |
| `a_live_mute_under_an_explicit_zero_is_the_muted_control_from_the_first_block` (gate 5) | a session 0 read as the default (mute 480) | R (also `live_fader_mute_and_pan_edits_change_the_running_plan_bit_exactly` under `HARD_SWITCH`) |
| SDK `builder-evals.mjs` (corpus loop, builder) | writer drops the root key; explicit key order swapped; ms keys missing from the float set; builder omits the key | R each (4, 1, 1, 4 evals) |
| `visit_model.rs` (existing, edited) | covered by the registry assertion `("control_smoothing", 17)` and the root count 9 | not mutated separately |

Which defect each new test catches that no existing test does: the grammar test, a parser that
defaults, over-accepts or reorders the key (no test read it before); the rounding test, a
truncating/half-even/`f32` conversion; the default-table tests, a #1388 value change that breaks
`pan_ms >= 2 x mute_ms`; the classifier tests, a row on the wrong key, polarity at the plain mute
length, or a pan window of 0; the gate 4 test, a mute that still steps (every other C ABI live test
waits the ramp out and cannot hear it) or a ramp block that allocates; the gate 5 test, a session 0
treated as unset.

**Gates run (all on the final tree unless noted).** `cargo test --locked -p session`;
`-p protocol --features test-support`; `-p host-core --features test-support,control-provider --test
live_delta` (33); `-p capi`; the `test-debug-a` leg (`--no-fail-fast`, the CI feature list) and its
doctest leg; `cargo build --locked --release -p audit && ./target/release/audit capi` (0 allocations, 0
syscalls); `cargo test --locked --release -p audit -p bench -p console-workload`;
`scripts/build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
`check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`,
`check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh` (after an offline
`npm ci`, deleted again), `check-sdk-headless.sh` (363 evals), `sdk-package.sh check`;
`check-builtins-fixtures.sh`, `check-console-fixtures.sh`, `check-console-benchmark-fixture.sh`;
`check-session-policy.sh`, `check-host-core-policy.sh`, `check-workspace-policy.sh`,
`check-realtime-policy.sh`, `check-cross-targets.sh`; `cargo clippy --locked --workspace --all-targets
--all-features -- -D warnings`; `cargo fmt --all -- --check`. Not run: the browser (Chromium) legs and
anything AArch64-only.

**Batch follow-ups** (stream E2 batch, after the attempt-1 PASS verdict
`docs/handoffs/decision-15-2026-10-05/verdicts/stream-e2/1054-attempt1.md`).

- *Root rulings (2026-10-12), recorded in the spec text above:* gate 4 counts the render calls that carry
  the commit and the ramp; gate 7 names the unpinned `audit capi` `pcm_digest` move at D4; gate 6's
  host-core command carries `control-provider`; `crates/capi/tests/resource_lifecycle.rs` is a named
  exception in Authorized paths.
- *MINOR 1:* the `audit capi` claim above is corrected.
- *MINOR 2, out-of-path edits the first record left out* (each necessary or mechanical, none weakens a
  check):
  - `crates/host-core/src/lib.rs`: the `LiveRampRow` re-export that D4 needs.
  - `crates/host-core/tests/launch_registry_once.rs`: the `LiveRamps` literal gains `pan_samples` and
    `link_samples`, which the compiler forces.
  - `.claude/skills/author-session/SKILL.md`: the root-key list and a short `control_smoothing` section.
  - `scripts/check-browser-expected-resources.py`: the self-test's one-byte mutation, the companion of the
    `sessionDocumentBytes` re-pin (gate 7).
- *NIT 1:* `for_row_returns_the_key_field_of_every_row` listed its rows by hand, so its doc claim (a new
  row fails to compile) held for `expected()` but not for the list. The rows now come from walking
  `next_row`, an exhaustive `match` beside `expected()`; the doc says exactly that. A row added to
  `LiveRampRow` fails to compile in both. The walk is a chain, so a row nobody links in is still
  untested; that residue needs a person to chain it, and the compile error asks for it.
- *NIT 2:* `tools/audit/src/fixture_builtins.rs`: the #1054 note sat between the #1240 and #1328 notes. The
  two #1054 notes are one note now, after the #1312 note, in the order of the pins.
- *Mutation runs for the changed test* (`-p host-core --features test-support,control-provider --test
  live_delta`; defect applied, run, restored):
  - add a `LiveRampRow::Probe` variant with its `for_row` arm: the test target fails to compile
    (`E0004`, `Probe` not covered, in `expected` and `next_row`); green after the revert (35 passed).
  - polarity row at the plain mute length (`saturating_mul(2)` removed): RED
    `for_row_returns_the_key_field_of_every_row`; green after the revert.
  - The NIT 2 edit is a comment move; no test applies.

**Batch verdict follow-ups** (stream E2 batch verdict, head `29bd7f3ca`, 2026-10-12).

- *n5 (MINOR 2 of the #1054 verdict):* the four out-of-path edits the Attempt record named under "MINOR 2"
  are now named exceptions in Authorized paths (label: "batch follow-ups, recorded after the fact"):
  `crates/host-core/src/lib.rs`, `crates/host-core/tests/launch_registry_once.rs`,
  `.claude/skills/author-session/SKILL.md` and `scripts/check-browser-expected-resources.py`.
- *n2:* the comment on `next_row` in `for_row_returns_the_key_field_of_every_row` said a new row "cannot be
  left out of the test unnoticed". The `match` forces a new row to get an arm, but an arm of `None`, or a
  row no arm chains to, compiles and is never visited. The comment now says that; the NIT 1 record above
  already says the walk is a chain. Comment only.
