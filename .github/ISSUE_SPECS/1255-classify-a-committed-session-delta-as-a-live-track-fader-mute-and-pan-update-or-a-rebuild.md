# Classify a committed session delta as a live track fader, mute and pan update or a rebuild

Core slice 3 of the umbrella *Deliver value-only fader, mute and pan transactions to the running
C ABI plan through the live console lanes* (#1053), its decisions D1, D3 and D9 and its guards
G1-G3. Anchors verified on `main` at `54b0a1bf8`.

## Product outcome

One pure host-core function reads the committed session model before and after a transaction. It
decides whether the transaction is a live update of track faders, mutes and pans, or needs a plan
rebuild, and for a live update it returns the exact records to push. #1257 calls it on the C ABI.
#1225, #1226 and #1247 widen it later; they must not fork it.

## Context (verified at `54b0a1bf8`)

- **The model.**
  - `Track` (`crates/session/src/model.rs:224-245`).
  - `DualMonoFader { left_db, right_db, left_mute, right_mute }` (`:642-651`).
  - `MatrixOrPan::Pan { left, right, smoothing_samples }` or
    `MatrixOrPan::Matrix { ll, lr, rl, rr, smoothing_samples }` (`:655-678`).
  - `Route::follows_mute` (`:799`) and `RouteSource::Track { track_id, tap }` (`:804-811`).
  - `SessionModel::vcas` (`:113-115`).
- **Validation.** The session checks a fader's dB and a matrix's coefficients only for finiteness,
  and a pan for `[-1, 1]` (`crates/session/src/validate.rs:454-472`). The builtins own the real
  domains:
  - The fader domain `[-144, 24]` dB is the private `checked_fader_gain`
    (`crates/builtins/src/lib.rs:4108-4113`). Preparation calls it (`fader_lanes`, `:4134`), and so
    does the render-side setter (`BuiltinFaderBank::set_fader_db`, `:3695-3711`).
  - The matrix domain is `Matrix2x2::checked` (`:88-103`) and `pan_matrix` (`:4179-4194`).
  - builtins-compiler lowers `MatrixOrPan` inline in `strip_parameters`
    (`crates/builtins-compiler/src/lib.rs:4756-4799`; the match is at `:4777-4789`).
- **The records** (`crates/builtins-compiler/src/lib.rs`).
  - `TrackControlRecord { matrix, smoothing_samples }` (`:105-110`).
  - `TrackFaderRecord::FaderDb { lanes, db, smoothing_samples }` and
    `TrackFaderRecord::Mute { lanes, muted, smoothing_samples }` (`:129-148`).
  - `BuiltinLaneSelector { Left, Right, Both }` (`crates/builtins/src/lib.rs:3939-3947`).
  - host-core re-exports none of the three today.
- **The kernel** (`crates/builtins/src/lib.rs`).
  - On a muted lane, `set_fader_gain` stores the gain and keeps the target at zero (`:2613-2629`).
  - `set_mute` retargets to zero or to the stored gain (`:2631-2652`).
  - Every retarget restarts the ramp from the current value (`:2588-2607`). So the records of one
    strip, drained in one block, reach the same final target in either order.
- **Never a redundant record** (`crates/host-core/src/solo.rs:58-70`).
- **The canonical JSON.** `session::canonical_session_json` (`crates/session/src/canonical.rs:11`).
  Its `f32` spelling keeps every bit, `-0.0` included (`crates/session/src/value.rs:22-37`).
- **The feature.** host-core's `control-provider` feature (`crates/host-core/Cargo.toml:15`). capi
  enables it; host-web does not, so the browser module does not compile the classifier.
  `scripts/check-host-core-policy.sh` allows the feature's name in exactly two `Cargo.toml` lines,
  so a test gates itself with `#![cfg(feature = "control-provider")]`, never with
  `required-features`.

## Decisions

- **D1. The API.** Add the module `crates/host-core/src/live_delta.rs` behind
  `#[cfg(feature = "control-provider")]`, and re-export its items:

  ```rust
  pub struct LiveRamps { pub fader_samples: u32, pub mute_samples: u32 }
  impl LiveRamps {
      /// #1053 D3: zero (a step) until #1054 derives it from the session's `controlSmoothing`.
      pub fn for_session(model: &SessionModel) -> Self;
  }
  pub struct LiveStripRecords<'a> {
      pub strip_id: &'a str,
      /// FaderDb records first, then Mute records; `None` after the last one.
      pub fader: [Option<TrackFaderRecord>; 4],
      pub matrix: Option<TrackControlRecord>,
  }
  pub struct LiveDelta<'a> {
      /// One entry per strip that has at least one record, in normalized order.
      pub strips: Vec<LiveStripRecords<'a>>,
  }
  pub enum LiveRebuild { Vca, Structure, Domain, FollowedMute }
  pub fn classify_live_delta<'a>(current: &SessionModel, next: &'a SessionModel, ramps: LiveRamps)
      -> Result<LiveDelta<'a>, LiveRebuild>;
  ```

  - **The inputs** are normalized models (`CompiledSession::normalized_model()`). The function
    states this in its documentation.
  - **In this slice** only tracks produce entries. Later slices of #1053, and #1225, #1226 and
    #1247, extend the same types: input records on `LiveStripRecords` (#1261, #1262), effect
    records on `LiveDelta` (#1264-#1266), submix strips and routes (#1225). They extend these
    types; they do not add a second classifier.
  - The exact shape may differ (for example, a small fixed-capacity type in place of the array)
    if the claims below still hold.
- **D2. The algorithm.** It is #1053's D1, step for step:
  1. **`Vca`** if `current.vcas` or `next.vcas` is non-empty (G3).
  2. **`Structure`** if the track IDs differ, by count or pairwise in order.
  3. **`Structure`** if the masked model's canonical JSON differs from `current`'s. The masked
     model is a clone of `next` with `revision` set to `current`'s and every track's `fader` and
     `matrix_or_pan` copied from `current`. Compare bytes, never `PartialEq`. A
     `canonical_session_json` error also gives `Structure`.
  4. For each track pair, in order:
     - **`Domain`** if `checked_fader_gain` refuses either post-commit lane's dB, or if the
       post-commit `MatrixOrPan` does not lower. A pre-commit value that does not lower is also
       `Domain`, as a defence; it cannot happen for a prepared model.
     - **`FollowedMute`** if a lane's mute changes and `next` has a route with
       `follows_mute: true` whose source is `RouteSource::Track` with this track's ID (G2).
     - **Otherwise the records.**
       - `FaderDb`: one per lane whose `checked_fader_gain` result changes in bits, or one `Both`
         when both lanes change and their post-commit `db` bits are equal. It carries
         `ramps.fader_samples`.
       - `Mute`: one per lane whose bool changes, or one `Both` when both lanes change to the same
         value. It carries `ramps.mute_samples`.
       - The matrix record: one when the lowered target changes in any coefficient's bits. It
         carries the post-commit `smoothing_samples`.
  5. **`Ok`** with the records. A `LiveDelta` with no entries is a valid live delta.

  G1 needs no code: only track fields are masked, so any submix change gives `Structure`.
- **D3. One authority per domain.** No range is spelled a second time.
  - **builtins.** `checked_fader_gain` becomes `pub`, documented as the one fader-domain authority.
  - **builtins-compiler.** Add `pub fn lower_matrix_or_pan(value: &MatrixOrPan) ->
    Result<(Matrix2x2, u32), BuiltinParameterError>`, moved out of `strip_parameters`, and
    `strip_parameters` calls it. Its `maximum_smoothing` check stays in `strip_parameters`.
  - The classifier calls both functions.
- **D4. Transient allocations.** The masked clone and the two canonical JSON strings are
  allocated and freed on the control thread for every classified transaction. That fits the
  control-plane precedent of #369 (the protocol already compiles a whole session per edit); say so
  in the function's documentation. Nothing the classifier allocates is retained.
- **D5. A fader move on a lane that stays muted** still gets its `FaderDb` record: the stage must
  remember the gain for a later unmute. With a zero ramp it is bit-exact; for a non-zero ramp, see
  #1054's coordination note.
- **D6. Re-exports.** host-core re-exports `TrackFaderRecord`, `TrackControlRecord` and
  `BuiltinLaneSelector`, because #1257 names them from capi, which does not depend on
  builtins-compiler.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (new), `crates/host-core/src/lib.rs`.
- `crates/host-core/tests/live_delta.rs` (new).
- `crates/builtins/src/lib.rs`: `checked_fader_gain`'s visibility and documentation only.
- `crates/builtins-compiler/src/lib.rs`: `lower_matrix_or_pan` and its use in `strip_parameters`
  only.
- This spec.

## Non-goals

- No caller. capi uses the function in #1257.
- No submix, route or VCA liveness. Those are #1225, #1226 and #1247.
- No ramp lengths other than `LiveRamps::for_session`'s zeros. The lengths are #1054's.

## Hazards

- **The iOS memset rule.** host-core and builtins-compiler are product crates.
  `scripts/check-cross-targets.sh` counts `bl _memset_pattern16` per crate against
  `scripts/lib/aarch64-known-defects.py`, so a repeated non-zero fill (for example
  `[None; 4]` of a type whose `None` is not all-zero bits) can fail it. The gate decides.
- **The browser module.** The classifier is compiled out of host-web. The two builtins refactors
  are not. Report whether the module moved (gate 4).

## Objective gates

Run every command from the repository root.

1. **The classifier.** New file `crates/host-core/tests/live_delta.rs`, with
   `#![cfg(feature = "control-provider")]`. Run it with
   `cargo test --locked -p host-core --features control-provider,test-support --test live_delta`;
   the workspace command runs it too. Build the models from a two-track session with an empty
   console, plus the variants named in each case. Each case asserts the exact `Ok` value or the
   exact `LiveRebuild` variant:
   - **(a) Identical values.** Only the revision changes: `Ok` with no entries (`Ok(empty)` below).
   - **(b) The fader.**
     - Left lane to -6 dB: one `FaderDb { Left, -6.0, fader_samples }`.
     - Both lanes to -3 dB: one `Both` record.
     - Both lanes to different values: a `Left` and a `Right` record.
     - `0.0` to `-0.0`, the same gain: `Ok(empty)`.
   - **(c) The mute.**
     - Left lane: one `Mute { Left, true, mute_samples }`.
     - Both lanes to `true`: one `Both` record.
     - `[true, false]` to `[false, true]`: two records.
   - **(d) Order.** A fader change and a mute change on one lane give `[FaderDb, Mute]`.
   - **(e) The pan.**
     - `Pan { 0, 0, 0 }` to `Pan { -1, 1, 64 }`: one matrix record equal to `pan_matrix(-1, 1)`,
       with smoothing 64.
     - `Pan` to the `Matrix` of the same target: `Ok(empty)`.
     - A smoothing-only change: `Ok(empty)`.
   - **(f) The domain.**
     - 24.0 and -144.0 dB are accepted. The next `f32` outward from each, and `NaN` (set directly
       on the model), give `Domain`.
     - A coefficient of 1.0 is accepted; the next `f32` above it gives `Domain`.
   - **(g) The sign of zero.** `trim_db` from `0.0` to `-0.0` gives `Structure`, not `Ok`.
   - **(h) G1.** Any submix fader change gives `Structure`.
   - **(i) G2.** A mute change on a track that a `follows_mute` route sends from gives
     `FollowedMute`. A fader change on the same track gives `Ok`.
   - **(j) G3.** A VCA in `current` only, or in `next` only, gives `Vca`, even for a delta that is
     otherwise live.
   - **(k) Structure.** A track added, removed or renamed, a route gain change and a
     `session_id` change each give `Structure`.
   - **(l) Ramps.** With `LiveRamps { fader_samples: 480, mute_samples: 240 }`, the fader and mute
     records carry those lengths, and the matrix record carries the model's smoothing.

   *Test values, one per group:*
   - *(a) and (e): red if the classifier emits a redundant record, which moves a settled lane's
     zero sign.*
   - *(b), (c) and (d): red if a record goes to the wrong lane, merges lanes wrongly, or changes the
     order.*
   - *(f): red if a value outside the setter's domain is classified live.*
   - *(g): red if the comparison uses `PartialEq` and misses a sign-of-zero edit.*
   - *(h), (i) and (j): red if a guard is dropped, which would render something other than the
     committed model.*
   - *(k): red if a structural edit is classified live.*
   - *(l): red if the ramp seam is ignored.*
2. **The domain matches the render setter.** For every `db` in {-144.0, 24.0, -0.0, 0.0, the next
   `f32` outward from each bound, `NaN`, ±∞}, the classifier accepts the value exactly when
   `BuiltinFaderBank::set_fader_db` accepts it. The same holds for the matrix coefficients
   {-1.0, 1.0, the next `f32` outward, `NaN`} against `BuiltinMatrixBank::set_target_smoothed`.
   *Test value: it turns red if the classifier admits a value the render-side setter refuses. The
   drain would pop that record, fail the render call and lose an acked edit.*
3. **The records equal a rebuild.** On the nine-track EQ fixture
   (`fixtures/session/v1/parametric-eq-nine-track.json`), make the edits of cases (b), (c) and (e)
   on its tracks, with every smoothing 0:
   - Prepare `current` with `prepare_host_runtime_with_live_controls` and depth 16. Push the
     classifier's records with `LiveRamps { 0, 0 }` before the first block.
   - Over 8 blocks fed the same source from sample 0, the output is bit-identical to a plan of
     `next` prepared by `prepare_host_runtime`.

   *Test value: it turns red if a record targets a value other than the one preparation bakes from
   `next` (a wrong lane, a wrong lowering, a dropped `Both`).*
4. **Nothing else moves.**
   - `cargo test --locked --all-targets -p builtins --features builtins/test-support`
   - `cargo test --locked -p builtins-compiler --features test-support`
   - The workspace test command:
     `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - Build the shipped module on this branch and on its base, and report both digests:
     `bash scripts/build-web-audioworklet.sh <dir>` then
     `sha256sum <dir>/miso-engine-v1-audio-worklet.simd128.wasm`.
5. **Workspace and policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh debug` is CI-only here (the `aarch64-debug`
     job); record it as not run locally.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The module digests.

## Dependencies

None. It edits `crates/builtins-compiler/src/lib.rs`, and so do #1253 and #1254, in other places.
Land them one at a time.

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- Never emit a redundant record. "Bit-identical" gates are hard stops.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, base `7436a64f9`)

**What landed.**
- `crates/host-core/src/live_delta.rs` (new, behind `control-provider`): `LiveRamps` (+
  `for_session`, zeros), `LiveStripRecords` (+ `fader_records()` iterator), `LiveDelta`,
  `LiveRebuild { Vca, Structure, Domain, FollowedMute }`, `classify_live_delta`. The D1 shape is
  kept as specified (`[Option<TrackFaderRecord>; 4]`). Steps: VCA guard; track IDs by count and
  pairwise; masked clone (revision, every track's `fader` and `matrix_or_pan` from `current`)
  compared on `canonical_session_json` bytes, an error being `Structure`; then per track, in
  order, `Domain` (pre and post fader through `checked_fader_gain`, pre and post pan/matrix
  through `lower_matrix_or_pan`), `FollowedMute` (a lane mute changes and a `follows_mute` route in
  `next` has `RouteSource::Track` with this ID), then records: `FaderDb` per lane whose
  `checked_fader_gain` bits change (one `Both` when both change and the post-commit dB bits are
  equal), `Mute` per changed lane (one `Both` when both change to the same value), fader before
  mute and left before right, and a matrix record when a lowered coefficient's bits change,
  carrying the post-commit smoothing. D4's transient allocations are documented on the function.
- `crates/host-core/src/lib.rs`: the module and its re-exports (feature-gated); D6 re-exports
  `BuiltinLaneSelector`, `TrackControlRecord`, `TrackFaderRecord` (unconditional). Nothing of
  #1254's was touched.
- `crates/builtins/src/lib.rs`: `checked_fader_gain` is `pub`, documented as the one fader-domain
  authority. No other change.
- `crates/builtins-compiler/src/lib.rs`: `pub fn lower_matrix_or_pan`, moved out of
  `strip_parameters`, which calls it; `maximum_smoothing` stays in `strip_parameters`.
- `crates/host-core/tests/live_delta.rs` (new, `#![cfg(feature = "control-provider")]`).

**New tests and their test values** (`crates/host-core/tests/live_delta.rs`):
- `a_revision_only_delta_is_live_with_no_records` (1a): red if a redundant record is emitted.
- `fader_records_address_exactly_the_changed_lanes` (1b): red if a fader record goes to the wrong
  lane, merges lanes wrongly, or is emitted for `0.0` -> `-0.0`.
- `mute_records_address_exactly_the_changed_lanes` (1c): red if a mute record goes to the wrong
  lane or merges lanes that change to different values.
- `fader_records_precede_mute_records` (1d): red if the fader-then-mute order changes.
- `a_matrix_record_is_emitted_only_when_the_lowered_target_changes` (1e): red if a pan record
  carries a wrong target, or is emitted for a Pan->Matrix of the same target or a smoothing-only
  change.
- `values_outside_the_setter_domain_need_a_rebuild` (1f): red if 24/-144 dB or a 1.0 coefficient
  is refused, or their next `f32` outward or `NaN` is classified live.
- `a_sign_of_zero_trim_edit_is_structural` (1g): red if the mask comparison uses `PartialEq`.
- `a_submix_fader_change_is_structural` (1h, G1), `a_followed_mute_change_needs_a_rebuild` (1i,
  G2; a fader change on the same track stays live, and the same mute is live when the route does
  not follow), `any_vca_needs_a_rebuild` (1j, G3, VCA in `current` only and `next` only): red if a
  guard is dropped.
- `structural_edits_need_a_rebuild` (1k): track removed/added/renamed, route gain, `session_id`.
- `records_carry_the_ramps_and_the_model_smoothing` (1l): red if the ramp seam is ignored; also
  pins `LiveRamps::for_session` to zeros.
- `the_classifier_domain_is_the_render_setters` (gate 2): red if the classifier admits a dB or a
  coefficient that `BuiltinFaderBank::set_fader_db` / `BuiltinMatrixBank::set_target_smoothed`
  refuses, or the reverse, over the listed values.
- `pushed_records_render_the_rebuilt_plan` (gate 3): nine-track EQ fixture, every track settled at
  0 dB / centre pan / smoothing 0 (track 6 pre-muted left), edits of (b), (c), (d), (e) and a
  sign-of-zero fader plus a Pan->Matrix of the same target on track 3 (no record); 8 strips of
  records pushed with `LiveRamps { 0, 0 }` into a `prepare_host_runtime_with_live_controls` plan
  (depth 16); 8 blocks are bit-identical to `prepare_host_runtime(next)`, and differ from
  `current`'s plan. Red if a record targets a value other than the one preparation bakes.

**Mutation runs** (each applied to `live_delta.rs`, the test file run, then reverted; all red):
- M1 VCA guard removed: `any_vca_needs_a_rebuild`.
- M2 masked comparison by `PartialEq` instead of bytes: `a_sign_of_zero_trim_edit_is_structural`.
- M3 `Both` merge removed: 1b, 1c, 1l.
- M4 mute records pushed before fader records: 1d, 1l.
- M5 matrix record always emitted: 1a, 1b, 1c, 1d, 1e, 1i, 1l, gate 3.
- M6 fader domain finiteness-only with a constant gain: 1b, 1d, 1f, 1i, 1l, gates 2 and 3.
- M6b finite out-of-range dB admitted (gain otherwise unchanged): 1f, gate 2 only.
- M7 follow guard removed: `a_followed_mute_change_needs_a_rebuild`.
- M8 a `Both` fader record sent as `Left`: 1b, 1c, 1l, gate 3.
- M9 fader ramp ignored: 1l.
- M10 `matrix_or_pan` not masked: 1e, 1f, 1l, gates 2 and 3.
- M11 fader change detected by dB bits instead of gain bits: 1b, gate 3.
- M12 matrix record carries the pre-commit target: 1e, 1l, gate 3.
- M13 post-commit matrix domain dropped: 1f, gate 2 only.

**Gates** (from the head tree; logs in `/tmp/claude-1002/w1255-a1/logs`):
- Gate 1-3: `cargo test --locked -p host-core --features control-provider,test-support --test
  live_delta`: 14 passed.
- Gate 4: `cargo test --locked --all-targets -p builtins --features builtins/test-support` pass;
  `cargo test --locked -p builtins-compiler --features test-support` pass; the workspace test
  command pass (117 test binaries ok, 0 failed); `cargo test --locked --release -p audit -p bench
  -p console-workload` pass.
- Module digests (`scripts/build-web-audioworklet.sh`, paths remapped): base `7436a64f9`
  `04ce0d44483b5ebce30619c3abcf7991e9dd0d4d69719c3df30def529b3e6f52` (2,858,355 B); head
  `ac35734286d38bab071e5be08a5c42b530aab388bdb69e2c4b73baa3f39fc45b` (2,857,637 B). **The module
  moved** (-718 B): the classifier is compiled out of host-web, so the move is the two builtins
  refactors (`lower_matrix_or_pan` out of line, `checked_fader_gain` public) changing codegen of
  `strip_parameters`. Its behaviour is unchanged: the workspace tests, gate 3 and the worklet
  chain pass.
- Gate 5: `cargo fmt --all -- --check` pass; `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings` pass; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
  --no-deps` pass; host-core, realtime and workspace policy check+test scripts pass;
  `scripts/check-cross-targets.sh` PASS (host-core's iOS `memset_pattern16` stays within its
  #1018 expected-failure row); `run-aarch64-tests.sh debug` not run locally (CI-only).
- Rule-file gates: `scripts/check-capi-abi.sh` and `--self-test` pass; `./target/release/audit
  capi`: 0 allocations, 0 syscalls, 0 violations; worklet chain (`build-web-audioworklet.sh
  --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`) pass.

**Notes for the verifier.**
- No path outside the authorized list changed.
- The classifier does not check `maximum_smoothing_samples`. (Corrected after the verdict,
  MINOR 2: the earlier reason, that #1053 D8's admission refuses a too-long smoothing, was wrong;
  D8 has no smoothing term.) The real reason: no production cap exists. host-core passes
  `maximum_smoothing_samples: u32::MAX` (`crates/host-core/src/prepare.rs`, `strip_parameters`'
  caller), capi does not override it, and the render setter (`set_target_over`) accepts any `u32`
  window, so every committed smoothing stays preparable.
- Domain is checked per track in order, before that track's follow guard, as this spec's D2
  step 4 states.

### Follow-ups applied (after the attempt 1 PASS, verdict MINOR 1-2 and NIT 1)

- MINOR 1 (D5 test): `fader_records_address_exactly_the_changed_lanes` gains a case: track 1 muted
  left in both models, `left_db` moved to -6 -> exactly `[FaderDb { Left, -6.0, 0 }]`. Gate 3
  (`pushed_records_render_the_rebuilt_plan`) now also mutes track 8's left lane in the source, moves
  its `left_db` to -9 while it stays muted, then classifies and pushes a second delta that unmutes
  it, renders the next `BLOCKS` and compares them with the second half of a rebuild of the unmuted
  model rendered over `2 * BLOCKS` (asserted non-silent). Downstream of the fader every stage is
  stateless in this fixture (zero-smoothing pan, output sum), which is what makes the rebuild's
  second half the reference. Test value: red if a fader move on a lane muted in both models is
  dropped as redundant; the follow-on makes the stale gain audible. Mutation V9 (`gain_changed`
  false when the lane is muted in both models): red in both
  (`fader_records_address_exactly_the_changed_lanes` and the gate 3 follow-on assertion); reverted,
  green.
- MINOR 2: the note above is corrected, and `classify_live_delta`'s doc now says that a finite
  smoothing cap in preparation would have to be refused here as `Domain`.
- NIT 1: `checked_fader_gain`'s doc no longer claims that no caller spells the range a second
  time; it names `prepare_sections` and builtins-compiler's `gain_path` as the two remaining
  spellings. Routing those two through `checked_fader_gain` is left as a follow-up issue
  candidate, not done here.
