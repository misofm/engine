# Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A session may carry `control_smoothing`: mute, fader and pan ramp lengths in milliseconds. A live
record that carries no ramp length of its own ramps over the session's length, or over the
researched default table when the session omits the setting. In this slice the C ABI's live fader,
mute and pan/matrix edits start to ramp. Today they step: fader and mute always, and pan or matrix
whenever the model's `smoothing_samples` is 0. An explicit 0 ms in the session stays legal. It is
the hard switch that sample-exact edits and bit-comparison tests use. Decision 14's F7 (the ramp
half) is closed for the C ABI rows that are live today.

## Context

- **Owner decision 1** (`docs/rulings/engine-footprint-2026-09-28.md:39-48`): ramp lengths are
  optional session settings. One default table is documented in the schema, and a single change
  may override it. Decision 15 D15-1 adds that no live value may step by omission: every live row
  takes the session default.
- **Defaults.** #1055's findings (`docs/handoffs/control-smoothing-defaults/FINDINGS.md:11-46`):
  - mute (and solo) 10 ms; fader 20 ms; pan and raw matrix 20 ms;
  - rounding `floor(ms * rate / 1000 + 1/2)`, in `f64`, from the `f32` value;
  - bounds: finite, `0 <= ms <= 1000`, zero legal, and `-0.0` keeps its spelling;
  - 441/480/882/960 and 882/960/1764/1920 samples at the four launch rates.

  The listening result is still pending in #1055. This slice uses the table #1055 closes with.
- **The C ABI seam.**
  - `LiveRamps` (`crates/host-core/src/live_delta.rs:33-58`) holds `fader_samples` and
    `mute_samples`. `for_session` returns 0 for both ("until #1054", `:49-50`).
  - `classify_live_delta` puts them on the `FaderDb` and `Mute` records (`:293-306`).
  - The matrix record carries the post-commit model's own `smoothing_samples` (`:271-274`,
    `:308-313`), through `lower_matrix_or_pan` (`crates/builtins-compiler/src/lib.rs:4925-4940`).
  - The C ABI calls `LiveRamps::for_session(next)` once per commit
    (`crates/capi/src/runtime/control.rs:1071-1075`).
- **The model's pan window.**
  - `MatrixOrPan::{Pan, Matrix}::smoothing_samples` is a required `u32`
    (`crates/session/src/model.rs:655-680`), and `Submix::unity` writes 0 (`:749`).
  - The prepared `MatrixStage` stores it as the lane window
    (`crates/builtins/src/lib.rs:2905-2931`). Only `set_target` reads it (`:2999-3006`), and no
    production path calls `set_target`: its one caller outside `builtins` is a test
    (`crates/builtins-compiler/src/lib.rs:12543`, inside `mod tests` at `:5125`).
  - Live matrix records always bring their own window (`set_target_over`, `:3008`).
  - So the prepared window is not a rendered value. This slice resolves "0 means the session
    default" only where records are produced.
- **Schema.**
  - The root keys are explicit and ordered (`docs/SESSION_SCHEMA_V1.md:35-40`).
  - The visitor's root field IDs are at `crates/session/src/visit.rs:88`. Field 8 is retired
    (`:110`).
  - `SessionModel` is at `crates/session/src/model.rs:89-121`.
- **A document that says "step".** `docs/C_ABI_V1_QUALIFICATION.md:265-267` ("Fader and mute
  changes are steps until #1054").
- **Tests that pin the step.** `crates/host-core/tests/live_delta.rs:39-42` (`STEP`), and
  `records_carry_the_ramps_and_the_model_smoothing` (`:603-643`), which asserts
  `for_session == STEP` at `:642`.

## Decisions frozen for this slice

- **D1. Schema.**
  - One optional root object, `control_smoothing` (SDK spelling `controlSmoothing`), in canonical
    order after `vcas` and before `outputs`.
  - When present it requires exactly `mute_ms`, `fader_ms` and `pan_ms`, each an `f32` with
    `0 <= ms <= 1000`. Unknown keys are refused. A value out of range is `NumericOutOfSchemaRange`
    at `$.control_smoothing.<key>`.
  - It is the schema's one optional root key, by owner decision 1.
  - Canonical JSON omits it when absent and writes all three keys when present (`-0.0` stays
    `-0.0`).
  - The model field is `SessionModel::control_smoothing: Option<ControlSmoothing>`. The visitor
    walks it as root field 17: a record with fields 1, 2 and 3 in key order. Field 8 stays retired.
- **D2. Default table and rounding**, both in `crates/session`.
  - `CONTROL_SMOOTHING_DEFAULT: ControlSmoothing` holds #1055's closing values.
  - `SessionModel::control_smoothing_samples(&self) -> ControlSmoothingSamples { mute, fader, pan }`
    applies `floor(f64::from(ms) * f64::from(sample_rate_hz) / 1000.0 + 0.5) as u32` to the
    session's value, or to the default.
  - This is the only conversion. Every host calls it.
- **D3. The key each live row uses.** One table, documented in `docs/SESSION_SCHEMA_V1.md` and on
  `LiveRamps`:
  - `fader_ms`: fader dB, input trim, send gain, VCA offset;
  - `mute_ms`: mute, solo, polarity invert, send mute, VCA mute, the effect bypass crossfade;
  - `pan_ms`: pan, matrix, send matrix.

  #1055's decision-15 addition confirms or changes the trim, polarity, send, VCA and bypass rows
  before this slice starts. The table follows its closing record.
- **D4. `LiveRamps`.** It gains `pan_samples`, and `for_session(model)` returns the three values of
  D2. The other C ABI rows read the field D3 names: input trim and polarity (#1261), sends (#1225,
  #1226), VCA (#1247), and the bypass crossfade (#1341). No row gets a separate default.
- **D5. Pan and matrix.** A post-commit model `smoothing_samples` of 0 means "session default":
  the classifier's matrix record then carries `ramps.pan_samples`, and otherwise the model's value.
  Preparation and the model are unchanged (see Context, "The model's pan window"), so canonical
  JSON still says 0.
- **D6. Explicit 0.** A session with all three keys at 0 renders exactly today's steps. This is the
  supported way for a test or a host to ask for sample-exact switches.
- **D7. The `-0.0` artefact.** Once `fader_ms` is non-zero, a fader move on a lane that stays muted
  retargets 0 to 0 over the ramp. The ramp kernel then turns a negative input into `-0.0` where a
  rebuild gives `+0.0` (the mechanism in `crates/host-core/src/solo.rs:58-66`). Keep the ramp,
  because a step there would cut short a mute ramp that is still in flight. Bit-identity gates
  compare after the ramp.
- **D8. Acked-batch question.** No queue changes; the records only carry a different length. So no
  ack can precede a drop that could not do so before.

## Deliverables

1. D1 and D2 in `crates/session` (model, parse, validate, canonical, visit), with
   `docs/SESSION_SCHEMA_V1.md`: the key, its bounds, the default table, the rounding rule and D3's
   table.
2. Every `SessionModel { .. }` struct literal outside `crates/session` gains
   `control_smoothing: None`. This is mechanical: about 98 sites in tests and helpers.
3. D4 and D5 in `crates/host-core/src/live_delta.rs`.
4. `docs/C_ABI_V1_QUALIFICATION.md:265-267`: live fader, mute and pan edits ramp over the session's
   `control_smoothing`, or over the default table.
5. The tests below. Rewrite `records_carry_the_ramps_and_the_model_smoothing`, and delete `STEP`
   if nothing uses it.

## Authorized paths

- `crates/session/src/{model,parse,validate,canonical,visit,lib}.rs`, `crates/session/tests/`,
  `docs/SESSION_SCHEMA_V1.md`
- `crates/host-core/src/live_delta.rs` (stream B owns this file; root sequences the merge),
  `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/live_tests.rs` (tests only), `docs/C_ABI_V1_QUALIFICATION.md`
- Struct-literal additions only (deliverable 2), in any file that builds a `SessionModel`

## Non-goals

- **The browser and the SDK**: *Resolve an absent live ramp to the session default on the browser
  and in the SDK* (#1364), which depends on this slice. It covers:
  - browser commands that resolve an absent length to the session default;
  - the defaults in `sdk/src/core/live-controls.ts`;
  - the SDK builder's `controlSmoothing`;
  - the browser-versus-C ABI parity gate at four rates.
- **Editing `control_smoothing` by transaction**: *Edit control_smoothing by a session
  transaction, model-only* (#1365), which depends on this slice. It covers a new session-edit opcode,
  its wire and corpus rows, and its classification as model-only. Until it lands, a C ABI session
  sets the key at boot.
- The C ABI rows that are not live yet (trim, polarity, sends, VCA) and the bypass crossfade. Their
  own issues read D3's field (D4).
- No change to preparation, to `lower_matrix_or_pan`, or to any prepared plan.

## Hazards

- The C ABI live tests compare a live edit with a rebuilt control. With non-zero defaults, a test
  whose claim is bit-identity to a rebuild must author `control_smoothing` at 0 (D6). Otherwise it
  compares after the ramp: `window` (`crates/capi/src/runtime/live_tests.rs:325-327`) already adds
  the smoothing. Do not weaken a comparison to make it pass.
- `docs/handoffs/control-smoothing-defaults/` is research evidence. Do not edit it.

## Objective gates

1. **Schema** (`crates/session/tests/`).
   - Accept a session with the object. Accept 0, `-0.0` and 1000.
   - Refuse each of these at its path, with `NumericOutOfSchemaRange` or the strict-key code: a
     negative value, 1000.001, a missing key, an unknown key, and a non-object.
   - Omitted and present sessions round-trip through canonical JSON to identical bytes, and an
     omitted one stays omitted.
2. **Rounding** (`crates/session` unit test).
   - `control_smoothing_samples` returns 441/480/882/960 for 10 ms and 882/960/1764/1920 for
     20 ms at the four launch rates.
   - It returns 221 for 5 ms at 44.1 kHz (a tie) and 0 for `-0.0`.
   - An omitted object gives the default table's values.
3. **Classifier** (`crates/host-core/tests/live_delta.rs`, rewritten test).
   - With the default table at 48 kHz, a fader, a mute and a pan change give `FaderDb` with 960
     and `Mute` with 480.
   - They give a matrix record with 960 when the model says 0, and with 96 when it says 96.
   - With all keys at 0, the same delta gives 0, 0 and 0.
4. **C ABI, the ramp is heard** (`crates/capi/src/runtime/live_tests.rs`, new).
   - A playing session with no `control_smoothing` takes a mute transaction.
   - The first block after the commit is neither the unmuted nor the muted control, because it is
     ramping.
   - From the first block after the ramp ends, the output is bit-identical to a control booted
     muted.
   - The commit and the ramp blocks allocate nothing (`bench_support::alloc` thread counters,
     process statics warmed).
5. **C ABI, explicit 0** (same file). The same edit with all keys at 0 is bit-identical to the
   muted control from the first block, as before this slice.
6. **Commands:**
   - `cargo test --locked -p session`,
     `cargo test --locked -p host-core --features test-support --test live_delta`,
     `cargo test --locked -p capi`
   - the workspace debug leg: `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo build --locked --release -p audit && ./target/release/audit capi`
   - `bash scripts/check-session-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
7. **Digests.** No checked-in session or render digest should move: preparation is unchanged, and
   no fixture declares the key. If a digest does move (for example a scripted C ABI edit that now
   ramps), the PR lists it with this reason and re-pins it on its own, never in bulk (decision 15,
   risk 4).

## Test value

- Gate 1 turns red if the parser gives a missing key a default, if it accepts a value over 1000,
  or if the canonical writer drops or invents the object. No test covers the key today.
- Gate 2 turns red if the rounding truncates, rounds half to even, or converts in `f32`: each gives
  220 at 5 ms and 44.1 kHz, or misses a launch-rate value.
- Gate 3 turns red if the classifier keeps a pan window of 0, or ignores `LiveRamps` for one row.
- Gate 4 turns red if `for_session` still returns 0, which steps the mute (the first block then
  equals the muted control). It also turns red if a ramp never lands on its target, which breaks
  the after-ramp equality.
- Gate 5 turns red if an explicit session 0 is read as "use the default": the edit then ramps where
  a step was asked for.

## Dependencies

- *Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)*
  (#1055), closed with its decision-15 addition (D3's row table).
- *Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live
  console lanes* (#1053) provides the C ABI live path that this slice gives ramps to. Its slices
  #1255 and #1257 are on `main`.
