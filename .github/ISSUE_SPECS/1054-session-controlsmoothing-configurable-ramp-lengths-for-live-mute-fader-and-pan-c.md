# Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A session may carry `control_smoothing`: mute, fader and pan ramp lengths in milliseconds. A live
record ramps over the session's length for its row, or over the researched default table when the
session omits the setting. An explicit 0 in the session stays legal: it is the hard switch that
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
    (`crates/capi/src/runtime/control.rs:1065-1075`), where `next` comes from the prepared token's
    `prospective_session()` (`crates/protocol/src/controller.rs:1139`).
- **The model's pan window.** `MatrixOrPan::{Pan, Matrix}::smoothing_samples` is a required `u32`
  (`crates/session/src/model.rs:655-680`); `Submix::unity` writes 0 (`:749`). The prepared window
  is read only by `set_target` (`crates/builtins/src/lib.rs:2999-3006`), which no production path
  calls; live matrix records bring their own (`set_target_over`, `:3008`).
- **Schema.** Root keys are explicit and ordered (`docs/SESSION_SCHEMA_V1.md:35-40`); the JSON
  schema lists them (`docs/session-v1.schema.json:7`). The SDK's canonical writer has its own root
  key list and float key set (`sdk/src/internal/session-json.ts:13-29`, `:50-65`), and is checked
  against the Rust-generated writer corpus
  (`fixtures/session-canonical/v1/canonical-writer-corpus.json`, from
  `crates/session/src/canonical.rs:505-523`). Visitor root field IDs are at
  `crates/session/src/visit.rs:88`; field 8 is retired (`:110`).
- **Pinned text.** `docs/C_ABI_V1_QUALIFICATION.md:265-267` ("steps until #1054");
  `crates/host-core/tests/live_delta.rs:39-42` (`STEP`) and
  `records_carry_the_ramps_and_the_model_smoothing` (`:603-643`).
- *Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live
  console lanes* (#1053) is the umbrella of the C ABI live path. Its delivered slices (#1255,
  #1257, closed) are what this issue builds on; it does not wait on the umbrella.

## Decisions frozen for this slice

- **D1. Schema.** One optional root object, `control_smoothing` (SDK `controlSmoothing`), in
  canonical order after `vcas` and before `outputs`. When present it requires exactly `mute_ms`,
  `fader_ms` and `pan_ms`, each an `f32` with `0 <= ms <= 1000`; unknown keys are refused; out of
  range is `NumericOutOfSchemaRange` at `$.control_smoothing.<key>`. Canonical JSON omits it when
  absent and writes all three keys when present (`-0.0` stays `-0.0`). The model field is
  `SessionModel::control_smoothing: Option<ControlSmoothing>`; the visitor walks it as root field
  17 (fields 1, 2, 3 in key order). The JSON schema and the SDK canonical writer learn the key.
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
  | `mute_ms` | mute, solo, polarity invert, send mute, VCA mute, effect bypass crossfade (#1341) |
  | `pan_ms` | pan, matrix, send matrix |

- **D4. `LiveRamps`.** It gains `pan_samples` and `link_samples`; `for_session(model)` fills all
  four from D2 and D3. It gains `LiveRampRow`, one variant per row of D3's table, and
  `for_row(row: LiveRampRow) -> u32`, which returns that row's key field. Every C ABI row reads its
  length through it: this slice's fader, mute and matrix; input trim and polarity (#1261), sends
  (#1225, #1226), VCA (#1247), the link glide (#1371). #1394 adds `resolve(row, edit_ramp)` on top
  of `for_row`. The bypass crossfade (#1341) always uses `mute_samples` (D15-1: the one row with no
  per-edit length). No row has a separate default.
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

1. D1 and D2 in `crates/session`, with `docs/SESSION_SCHEMA_V1.md` (key, bounds, default table,
   rounding, D3's table), `docs/session-v1.schema.json`, one writer-corpus document with the key,
   and the SDK canonical writer's root and float keys.
2. Every `SessionModel { .. }` struct literal outside `crates/session` gains
   `control_smoothing: None` (mechanical; about 98 sites in tests and helpers).
3. D4 and D5 in `crates/host-core/src/live_delta.rs`. The `commit_live` call
   (`crates/capi/src/runtime/control.rs:1071-1075`) is unchanged: `for_session(next)` now returns
   the session's lengths.
4. `docs/C_ABI_V1_QUALIFICATION.md:265-267`: live fader, mute and pan edits ramp over the session's
   `control_smoothing`, else the default table.
5. The tests below. Rewrite `records_carry_the_ramps_and_the_model_smoothing`; delete `STEP` if
   nothing uses it.

## Authorized paths

- `crates/session/src/{model,parse,validate,canonical,visit,lib}.rs`, `crates/session/tests/`,
  `docs/SESSION_SCHEMA_V1.md`, `docs/session-v1.schema.json`,
  `fixtures/session-canonical/v1/canonical-writer-corpus.json`, `sdk/src/internal/session-json.ts`
- `crates/host-core/src/live_delta.rs` (stream B owns it), `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/live_tests.rs` (tests only), `docs/C_ABI_V1_QUALIFICATION.md`
- Struct-literal additions only (deliverable 2), in any file that builds a `SessionModel`

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

## Objective gates

1. **Schema** (`crates/session/tests/`). Accept the object, with 0, `-0.0` and 1000. Refuse, each at
   its path: a negative value, 1000.001, a missing key, an unknown key, a non-object. Omitted and
   present sessions round-trip through canonical JSON to identical bytes. The writer corpus
   regenerates with the new document, and the SDK writer reproduces it (`builder-evals.mjs`
   corpus loop, run by `check-sdk-headless.sh`).
2. **Rounding** (`crates/session` unit test). 441/480/882/960 for 10 ms and 882/960/1764/1920 for
   20 ms at the four launch rates; 221 for 5 ms at 44.1 kHz (a tie); 0 for `-0.0`; an omitted
   object gives the default table.
3. **Classifier** (`crates/host-core/tests/live_delta.rs`, rewritten test). At 48 kHz with the
   default table: a fader, a mute and a pan change give `FaderDb` 960, `Mute` 480, matrix 960 (96
   when the model says 96). With `mute_ms` 5, `fader_ms` 15 and `pan_ms` 25: 720, 240, 1200.
   With all session keys at 0: 0, 0, 0. `for_row` returns the D3 key's field for every `LiveRampRow`.
4. **C ABI, the ramp is heard** (`crates/capi/src/runtime/live_tests.rs`, new). A playing session
   with no `control_smoothing` takes a mute transaction. The first block after the commit equals
   neither the unmuted nor the muted control; from the first block after the ramp, the output is
   bit-identical to a control booted muted. The commit and the ramp blocks allocate nothing
   (`bench_support::alloc` thread counters, process statics warmed).
5. **C ABI, explicit 0** (same file). The same edit with all session keys at 0 is bit-identical to
   the muted control from the first block.
6. **Commands:**
   - `cargo test --locked -p session`, `cargo test --locked -p protocol --features test-support`,
     `cargo test --locked -p host-core --features test-support --test live_delta`,
     `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `cargo build --locked --release -p audit && ./target/release/audit capi`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `bash scripts/check-session-policy.sh`, `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
7. **Other digests.** No session or render digest should move: preparation is unchanged and no
   fixture declares the key. Any that moves is listed with its reason and re-pinned on its own,
   never in bulk (decision 15, risk 4).

## Test value

- Gate 1 turns red if the parser defaults a missing key, accepts a value over 1000, or the Rust or
  SDK writer drops or invents the object. No test covers the key today.
- Gate 2 turns red if the rounding truncates, rounds half to even or converts in `f32` (each gives
  220 at 5 ms and 44.1 kHz).
- Gate 3 turns red if the classifier keeps a pan window of 0, ignores the session for one row, or
  maps a row to the wrong key.
- Gate 4 turns red if `for_session` still returns 0 (the mute steps), or a ramp never lands on its
  target.
- Gate 5 turns red if a session 0 is read as "use the default".

## Dependencies

- *Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)*
  (#1055), closed with `FINDINGS.md` section 9 (D2's values and D3's table).

Dependent: *Carry an optional per-edit ramp length on live session edits* (#1394) depends on this
issue.
