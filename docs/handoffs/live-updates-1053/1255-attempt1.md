Verdict: PASS

Issue #1255, attempt 1. Commit `a2a417c9a` (parent `7436a64f9`), reviewed in the export
`/tmp/claude-1002/v1255-a1`. Checked against the slice spec, umbrella #1053 D1, D3, D9 and G1-G3,
decision 14, AGENTS.md realtime rules and the acked-batch question.

No BLOCKER or MAJOR finding. Two MINOR findings and one NIT.

## Findings

### MINOR-1. D5 has no test: a fader move on a lane that stays muted can be dropped and every test stays green

- Where: `crates/host-core/src/live_delta.rs:177` (code, correct today) and
  `crates/host-core/tests/live_delta.rs:207`, `:628` (no case covers it).
- Evidence: mutation V9 makes `gain_changed[lane]` false when the lane is muted in both models.
  All 14 tests stay green (`test result: ok. 14 passed`).
- Why it matters: D5 says such a move must still emit its `FaderDb`, so the stage remembers the
  gain for a later unmute. If the record is dropped, a later live unmute restores the stale gain.
  The render plane then departs from the committed model (D9), and the acked fader value is lost
  without any error.
  - The "never a redundant record" rule pushes toward exactly this regression: on a muted lane
    the record changes nothing audible.
  - #1054's non-zero ramps add a second push: the record makes the settled zero re-enter the ramp
    kernel.
- Fix: add one case to `fader_records_address_exactly_the_changed_lanes`.
  - `current`: track 1 with `left_mute = true`. `next`: the same with `left_db = -6.0`.
  - Expect `Ok([(id, [FaderDb { Left, -6.0, 0 }], None)])`.
  - Optionally, gate 3 could also push a second, unmuting delta and compare it against a rebuild
    of that second model. That would make the remembered gain audible.

### MINOR-2. The attempt record gives a false reason for not checking `maximum_smoothing_samples`

- Where: the slice spec's attempt record, `.github/ISSUE_SPECS/1255-*.md:354-356`.
- What the note claims: "the post-commit model's admission (#1053 D8) is where a too-long matrix
  smoothing is refused".
  - D8 has no smoothing term. Its three terms are graph bytes, capi bytes and the largest
    allocation.
  - #1257's spec has none either. It only asks its tests to keep smoothing within one quantum.
- The outcome is sound for a different reason (see "Implementer point (1)" below). No production
  cap exists, so there is nothing to refuse.
- Fix:
  - Correct the note: host-core passes `maximum_smoothing_samples: u32::MAX`
    (`crates/host-core/src/prepare.rs:1124`), capi does not override it, and the render setter has
    no smoothing domain.
  - Add one sentence to `classify_live_delta`'s doc: if host-core preparation ever gets a finite
    smoothing cap, the classifier must refuse a smoothing above it as `Domain`. Otherwise a live
    commit could leave a committed model that its own rebuild refuses (D9).

### NIT-1. `checked_fader_gain`'s new doc overclaims

- Where: `crates/builtins/src/lib.rs:4109-4111`.
- The doc says "no caller spells the `[-144, 24]` dB range a second time". The range is still
  spelled for `fader_db` in two other places:
  - `prepare_sections` (`crates/builtins/src/lib.rs:3143`, which preparation reaches through
    `BuiltinChain::new`);
  - `gain_path` (`crates/builtins-compiler/src/lib.rs:4957`).
- Both checks agree with `checked_fader_gain` today, so behaviour does not diverge. Both lines are
  outside this slice's authorized edits.
- Fix: reword the doc to say that the three named callers share the function. Separately, file a
  small follow-up that routes `prepare_sections` and `gain_path` through `checked_fader_gain`.

## The implementer's two points

**(1) No `maximum_smoothing_samples` check in the classifier. Sound in outcome, wrong in its
stated reason (MINOR-2).** An acked record cannot be refused at render or at prepare because of
its smoothing:

- **Render.** `BuiltinMatrixBank::set_target_smoothed` calls `set_target_over`
  (`crates/builtins/src/lib.rs:2870-2906`). It checks only the target, with `checked()`, and the
  lane. It accepts any `u32` window.
- **Prepare.** The only cap is in `strip_parameters`, and every production caller passes
  `u32::MAX`:
  - host-core's `prepare.rs:1124`, used by both capi and host-web;
  - graph-compiler's own callers.
  - capi has no override, and session validation has no smoothing bound.
- **So** a committed model with any smoothing remains preparable, and D9 holds. The hazard is
  latent: it appears only if someone adds a finite cap.

**(2) Domain and follow guard run per track, not as the umbrella's global steps 4 and 5. Sound.**

- The slice spec's D2 step 4 prescribes exactly this per-track order, and the doc comment states it.
- `Ok` is returned only when every track passes every check, so the live output is identical in
  both orders.
- The two orders differ only in which `LiveRebuild` variant is reported. That happens when two
  different tracks trip different guards (a followed-mute change on track i, an out-of-domain value
  on track j > i).
- #1053 D6 step 1 sends every `Err` to the unchanged rebuild path. No consumer branches on the
  variant, and the rebuild reports preparation's own diagnostic.
- Not a finding. The umbrella's D1 could say that the reason is "first failing check in track
  order".

## The acked-batch question: can the classifier emit a record that the render setter refuses or drops?

No.

- **`FaderDb`.** The `db` it carries has passed `checked_fader_gain`, which is the same function
  `BuiltinFaderBank::set_fader_db` calls (`builtins/src/lib.rs:3695-3711`). Gate 2 shows that the
  two agree on ±0, both bounds, the next `f32` outward from each bound, NaN and ±∞.
- **`Mute`.** The setter has no value domain.
- **Matrix.**
  - The target is the output of `Matrix2x2::checked()`, either directly or through `pan_matrix`,
    which ends in `checked()`. `checked()` is idempotent: it keeps finite values in [-1, 1] and
    turns -0.0 into +0.0. So the setter's own `checked()` accepts every lowered target.
  - Smoothing has no render domain (see point 1).
- **Volume.** A strip gets at most 4 fader records and 1 matrix record. That is below depth 16, and
  the room check is #1257's D6 step 4.
- **Remaining cases.** A `LaneLength` refusal or a full queue depends on #1257 resolving strips by
  ID and checking capacity, not on the classifier.

## Scope, decisions and realtime

- **Paths.** Only authorized paths changed.
  - builtins: visibility and documentation of `checked_fader_gain` only.
  - builtins-compiler: `lower_matrix_or_pan` extracted. The match arms are the same as before the
    extraction, and `maximum_smoothing` stays in `strip_parameters`.
- **D1 shape.** Matches exactly: `LiveRamps` with `for_session` returning zeros,
  `LiveStripRecords` with `[Option<TrackFaderRecord>; 4]`, `LiveDelta`, the four `LiveRebuild`
  variants, and the signature.
- **Other decisions.**
  - D3: one authority per domain, as far as this slice's own code goes.
  - D4: the allocation note is in the docs.
  - D5: implemented, but untested (MINOR-1).
  - D6: the re-exports are in place.
- **Mask.**
  - It copies only track `fader` and `matrix_or_pan`, plus `revision`.
  - The comparison is on `canonical_session_json` bytes, and an error gives `Structure`.
  - NaN in a live field reaches `Domain`, not `Structure`, because the mask hides it from the
    serializer.
- **Guards.** G1 holds by construction (V11 red). G2 checks `next`'s routes, which equal
  `current`'s under the mask (V1 and V3 red). G3 checks both models (V12 red).
- **Realtime.** Nothing on the render path. The classifier runs on the control thread, behind
  `control-provider`, so host-web does not compile it.
- **Browser module.** It moved, by -718 B (digests in "Gates" below). The cause is the builtins
  refactor. The worklet chain passes, and `check-browser-expected-resources --artifacts` agrees with
  every expected.json render digest.
- **Test hygiene.** No source-grepping tests, no digest pins.

## Test value (one sentence each)

1. `a_revision_only_delta_is_live_with_no_records`: red if the classifier emits a record for a
   strip whose values did not change (M5), or fails to mask the revision.
2. `fader_records_address_exactly_the_changed_lanes`: red if a fader record goes to the wrong lane,
   merges lanes whose post-commit dB differ into one `Both` (V4), or fires on a `0.0` to `-0.0`
   move that keeps the gain (M11).
3. `mute_records_address_exactly_the_changed_lanes`: red if `[true, false]` to `[false, true]`
   merges into one `Both`, or a mute record goes to the wrong lane (M3).
4. `fader_records_precede_mute_records`: red if mute records are pushed before fader records (M4).
5. `a_matrix_record_is_emitted_only_when_the_lowered_target_changes`: red if the matrix record
   carries the pre-commit smoothing (V6) or target (M12), or is emitted for a change of
   representation or of smoothing only.
6. `values_outside_the_setter_domain_need_a_rebuild`: red if a post-commit dB or coefficient outside
   the setter's domain is classified live (V7, M13), or if a bound value is refused.
7. `a_sign_of_zero_trim_edit_is_structural`: red if the mask is compared with `PartialEq` instead of
   canonical bytes (V2). It is the only test that catches this.
8. `a_submix_fader_change_is_structural`: red if the mask also copies submix faders, which lifts G1
   (V11). It is the only test that catches this.
9. `a_followed_mute_change_needs_a_rebuild`: red if the G2 guard is dropped (V1) or ignores the
   `follows_mute` flag (V3).
10. `any_vca_needs_a_rebuild`: red if the G3 guard is dropped (M1) or checks only one model (V12).
11. `structural_edits_need_a_rebuild`: red if the mask also covers a non-live field, such as
    routes (V10).
12. `records_carry_the_ramps_and_the_model_smoothing`: red if fader or mute records ignore their
    `LiveRamps` field (M9, V8), or if `for_session` stops returning zeros.
13. `the_classifier_domain_is_the_render_setters`: red if the classifier admits a dB or coefficient
    that `set_fader_db` or `set_target_smoothed` refuses (V7). That would be an acked record failing
    the render call.
14. `pushed_records_render_the_rebuilt_plan`: red if a record targets a value other than the one
    preparation bakes from `next`: a `Both` sent as `Left` (M8), a wrong lowering (M12) or gain
    detection on dB bits (M11).

## Mutation runs (in the export, each reverted; restoration verified)

- **The implementer's mutations, re-run:**
  - V1 = M7 (follow guard removed): red, `a_followed_mute_change_needs_a_rebuild`.
  - V2 = M2 (mask compared by `PartialEq`): red, `a_sign_of_zero_trim_edit_is_structural`.
- **My own mutations, red:**
  - V3 (ignore the `follows_mute` flag): 5 red.
  - V4 (fader `Both` merge regardless of dB bits): 1b and gate 3 red.
  - V6 (matrix record carries the pre-commit smoothing): 1e and 1l red.
  - V7 (post-commit fader domain dropped): 1f and gate 2 red.
  - V8 (mute records take `fader_samples`): 1l red.
  - V10 (mask routes too): 1k red.
  - V11 (mask submix faders): 1h red.
  - V12 (VCA guard checks `current` only): 1j red.
- **Survived:** V9 (no `FaderDb` on a lane that stays muted). This is MINOR-1.

## Gates re-run (head export, `CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1255`)

Passed:

- **Gates 1-3:** `cargo test --locked -p host-core --features control-provider,test-support --test live_delta`: 14 passed.
- **host-core, all targets:** `cargo test --locked -p host-core --all-targets --features control-provider,test-support`: 227 passed, 0 failed.
- **Gate 4, builtins:** `cargo test --locked --all-targets -p builtins --features builtins/test-support`: pass (116 passed).
- **Gate 4, builtins-compiler:** `cargo test --locked -p builtins-compiler --features test-support`: pass.
- **Gate 4, module digests:** both reproduce the implementer's figures exactly.
  - Base `7436a64f9`: `04ce0d44483b5ebce30619c3abcf7991e9dd0d4d69719c3df30def529b3e6f52`.
  - Head: `ac35734286d38bab071e5be08a5c42b530aab388bdb69e2c4b73baa3f39fc45b` (2,857,637 B).
- **Gate 5, format and lint:** `cargo fmt --all -- --check` pass; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
- **Gate 5, docs:** `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- **Gate 5, policy:** the host-core, realtime and workspace policy check and test scripts: pass.
- **Gate 5, cross targets:** `scripts/check-cross-targets.sh`: PASS. host-core has 4
  `memset_pattern16` calls, which equals its #1018 row (`aarch64-known-defects.py:67`).
- **C ABI:** `scripts/check-capi-abi.sh`: ok, shared and static. The first run failed only because
  my `CARGO_TARGET_DIR` hid `target/release/libcapi.so`; the re-run uses the script's own target
  dir.
- **Worklet chain:** `build-web-audioworklet.sh --named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py` and
  `test-web-audioworklet.sh`: all pass.

Not re-run:

- **The gate 4 workspace test command and `cargo test --release -p audit -p bench -p console-workload`.**
  - Reason: the disk was at 98-99% during review (8.5 GB free), and I do not doubt the claim.
  - Why it is safe: workspace-wide clippy with `--all-targets --all-features` compiled every test
    target, and the only behavioural change outside the new module is an extraction with the same
    match arms as before.
- **`tools/audit capi`:** no render-path code changed, and the classifier has no caller yet.
- **AArch64:** CI-only.

Logs: `/tmp/claude-1002/v1255-a1/logs/`.
