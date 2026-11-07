# effect-runtime: LinearRamp's settled invariant is only documented, so three effects' restores admit a settled ramp off its target

Tracker #2258, item GR-7 ("One ramp state machine has about six encodings"), first slice.

## Mission
`effect_runtime::ramp::LinearRamp` documents one invariant -- `remaining == 0` implies the ramp is
at its target -- but its four fields are public, so any crate can build or write a settled ramp
that is not at its target, and three effects' state restores do. Make the invariant structural:
private fields, constructors that cannot produce a settled ramp off its target, a checked
`from_words` for restore, read accessors, and every user migrated. A restore then cannot admit
the illegal state, because there is no way left to build it.

## Verified (base `5864fd249`)
The tracker names three divergences. Each was checked on current `main`:

1. **`MatrixStage::reset` leaves its step words** (`crates/builtins/src/lib.rs` ~4222). True, and
   not observable. A natural settle (`end_ramp_window` ~4122) leaves the step words too. Every
   reader that can see a settled lane ignores its step: the kernel `matrix2x2_ramp_block`
   (`crates/lane/src/kernels/builtins.rs` ~616) assigns `target` on every frame where
   `remaining <= 0`, and `set_target_over` (~3972) rewrites the lane's step before any new ramp.
   The lane export carries the step to `import_lane`, where it is again dead while
   `remaining == 0`. `MatrixLaneState` has no `PartialEq`. **No defect; no change.**
2. **`FaderRampStage::retarget` has no stationary hoist** (~3450). True, and neither do the
   matrix or input stages: no builtin stage hoists. For a gain move the hoist would be
   bit-identical and only save cost (a non-hoisted no-op window costs a ramp body and declines
   `lanes_at_rest`). For a mute move it would **move bits**: `gain_mute_ramp_block` clears the
   mute only on `done` frames, so a no-op mute window on a lane whose gain is already `0.0`
   writes `x * 0.0` (a `-0.0` for a negative `x`) until it settles, where a hoisted settle would
   write `+0.0` at once. **Not a defect; recorded as a successor.**
3. **dynamic-eq and de-esser rebuild `LinearRamp` from words** (`crates/dynamic-eq/src/lib.rs`
   ~918 and ~1180, `crates/de-esser/src/lib.rs` ~723 and ~955). The words come only from the
   engine (`advance_word_ramps` in `crates/effect-runtime/src/dynamic_band.rs` ~181 zeroes the
   step on the last frame), or from a restore whose `parse_lane` already refuses a settled ramp
   off its target and a settled non-zero step (#2310, #2324). `set_target` reads only
   `current`, so the stale words are never used. **No defect.** The rebuild becomes
   `LinearRamp::fixed(current)` plus `set_target`, which gives the same bits by construction.

**The defect this issue fixes.** #2310 fixed the gate's restore and named, as follow-ups, the
sites with no at-target clause. Reading every `LinearRamp` restore confirms them, plus one more:

- **Compressor** (`crates/compressor/src/state.rs` `validate_channel` ~134-176): parameter
  ramps, rate-coefficient ramps and link weights. None checks that a settled ramp is at its
  target.
- **Transient shaper** (`crates/transient-shaper/src/lib.rs` `read_lane` ~1614-1634): parameter
  ramps. Only its link site has the clause.
- **Soft clip** (`crates/soft-clip/src/lib.rs` `decode_lane_words` ~1003-1024): every parameter
  ramp. It checks neither the at-target rule nor a settled `+0.0` step.

Such a ramp renders its `current` for as long as it stays settled. The committed `target` is
never reached until the next retarget. No engine writer produces it: `fixed`, `set_target`,
`snap` and the last `next_value` all land `current` on `target` with a `+0.0` step. So it is a
word "the engine never holds" (#1411 D1), the class #2310 refused for the gate.

## Decision / model
- **Private fields.** The type keeps four fields. `remaining == 0` is the `Settled` phase and
  `remaining > 0` is `Ramping`. A `Settled|Ramping` enum is not used, for two reasons:
  - The countdown already is the phase discriminant, so an enum would duplicate it.
  - An enum would change the hot `next_value`/`advance_block` match and the layout that
    `[LinearRamp; N]` lane arrays are gathered from.

  Privacy plus checked constructors makes the illegal state unrepresentable outside
  `ramp.rs`. `next_value` and `advance_block` are unchanged.
- **The invariant, exactly.** `remaining == 0` implies `current.to_bits() == target.to_bits()`
  and `step.to_bits() == 0` (`+0.0`). Every engine writer already produces this.
- **Constructors.**
  - `fixed(v)`: settled.
  - `set_target`: unchanged.
  - `from_words(current, target, step, remaining) -> Option<Self>`: `None` exactly when the
    invariant fails. This is the restore path.
  - A named `holding(value, samples)`: the compressor's bounded rate-return carry, which is a
    ramping ramp with step `0`.
- **Accessors.** `current()`, `target()`, `step()`, `remaining()`.
- **One writer for lane-vector advances.** The compressor, transient shaper, limiter,
  multiband and soft clip each gather ramps into lane vectors, advance them by the same law and
  scatter them back. Their scatters wrote the fields directly. They now call
  `record_advance(current, step, remaining)`, which keeps `target` and, for `remaining == 0`,
  stores the target and `+0.0`. A walk of the law already lands there, so this is a plain store
  for every real advance: two scalar selects per lane per block (the compiler may branch: on
  x86-64-v3 each is a `testl`/`je` per lane), never per sample. A `debug_assert!` checks that a
  settled advance already landed on the target, so a lane-law defect that walks off it stays
  visible in debug builds instead of being masked by the store.
- **Rebuild from words.** dynamic-eq, de-esser and gate-expander rebuilt a ramp from its lane
  words only to call `set_target`, which reads `current` alone. The rebuild becomes
  `LinearRamp::fixed(current)` plus `set_target`. This gives the same words by construction.
  gate-expander had the same pattern at three sites, which the tracker did not list.
- **Restore.** `state_payload::read_ramp` returns `Option<LinearRamp>`. Each restore maps
  `None` to the code it already uses for that site. `ramp_path_inside` drops its settled-step
  clause, which the type now guarantees. Delay, reverb and transient shaper normalise `-0.0`
  before `from_words`, so a settled `-0.0`/`+0.0` pair stays accepted where a float compare
  accepted it. Every site that already had an at-target clause (de-esser, dynamic EQ, gate,
  multiband, limiter, reverb, delay, the transient shaper's links) drops it for the structural
  check. Its accepted set does not change.

## Non-goals (successors, recorded here)
- Unifying `WordRamp` (dynamic band), the builtins' `u32` countdown plus `f32` residue stages
  (fader, matrix, input) and `IndexedRamp` onto one type. These are SIMD lane-word encodings and
  stay branch-free.
- A stationary hoist for builtin gain retargets only. It is cost only, and must exclude mute
  moves, which would move signed-zero bits (see 2 above).
- Moving the effects' near-duplicate lane-vector ramp helpers into `effect-runtime`, next to
  `LinearRamp`: the compressor's `RampVec`, the transient shaper's `LinkRamp` and the limiter's
  `RampLanes` (gather, advance by the law, scatter through `record_advance`).

## Gates
- Red-on-base reproducers: compressor, transient-shaper and soft-clip restores refuse a settled
  ramp one ulp off its target.
- A unit test of `from_words` in `effect-runtime`.
- A one-time no-bit-moved comparison of the affected effects against the base, as PR evidence.
- `cargo test --locked` for every crate that builds or reads `LinearRamp`, and their dependents
  (effect-compiler, rack, graph-compiler, host-core).
- clippy `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings`,
  `scripts/check-workspace-policy.sh`, the realtime policy tool, and the plan fixture unchanged.

## Evidence / attempt record

### Attempt 1 (Terra), base `5864fd249`

**Red on base.** The three reproducers were added before the fix and run on the base:
- `transient-shaper` `tests/contract.rs` `a_settled_ramp_off_its_target_is_refused`:
  `ramp 0: settled current 1e-45 off its target 0e0`, `left: Ok(())`,
  `right: Err(StatePayloadError { code: "effect.state.parameter" })`.
- `soft-clip` `tests/state_roundtrip.rs` `a_settled_ramp_off_its_target_or_with_a_step_is_refused`:
  `ramp 0 off its target`, `left: Ok(())`, `right: Err(... "effect.state.parameter" ...)`.
- `compressor` `tests/payload.rs` `a_settled_ramp_off_its_target_is_refused`:
  `ramp 0: ()`. The `expect_err` found the restore `Ok`.

All three are green after the fix.

**New test.** `effect-runtime` `tests/ramp.rs` `words_enter_a_ramp_only_in_phase` covers
`from_words`, `record_advance` and `holding`.

**Tests changed because they built a state the type now forbids.**
- The compressor's `randomized_differential` restores settled ramps at their target (#1006 had
  restored them off target).
- Three compressor `payload.rs` tests modified ramp 0's `current` alone as a "legal" edit. They
  now move `target` with it.
- `a_coefficient_below_zero_or_above_its_design_is_refused`'s settled cases hold
  `current == target`, so they still test the domain rather than the at-target rule.
- The compressor witness-test generator and `perturb` build ramps through `from_words`.
- soft-clip `mono_collapse.rs` splices a countdown word from the mid-ramp payload into the
  at-rest one. Every other word is still spliced the other way.
- `effect-runtime` `ramp_endpoint.rs` redraws forged words that `from_words` refuses.

**Mutations.** Each was applied to `crates/effect-runtime/src/ramp.rs`, its tests were run,
and it was reverted. The columns are the test crates.

| mutation | effect-runtime `ramp` | transient-shaper | soft-clip | compressor |
|---|---|---|---|---|
| M1: `from_words` drops the at-target clause | RED | RED | RED | RED |
| M2: `from_words` compares `current != target` as floats | RED | green | green | green |
| M3: `from_words` drops the settled-step clause | RED | green | RED | green |
| M4: `record_advance` stores a settled `current` verbatim | RED | green | green | green |

**No bit moved** (one-time PR evidence, not committed). A scratch test was run in
`effect-compiler`, release build, identically on base `5864fd249` and on the fix:
- It covers all 11 launch effects, 6 seeds each, as a scalar instance and (for the 10 banking
  effects) a full native-width bank.
- Each run is 96 blocks. The blocks mix partial and full lengths, include silent blocks, and
  carry random lattice automation on random lanes and channels.
- The test snapshots every lane's payload after every block, and restores held payloads into
  random lanes mid-run. It also resets.
- It digests every output sample and every payload word.

All 11 digests are identical: compressor `7da2e7519b7a02d1`, de-esser `6b5284a12c3fa0d8`,
delay `c88b654e12ce48ae`, dynamic-eq `36c77fb57552a98e`, gate-expander `fcdb657e657e0a21`,
multiband `5ec0583e8df0eb71`, parametric-eq `2162cd838d15430d`, reverb `3ce5bb8006072cec`,
soft-clip `acaca3a91436d3fd`, transient-shaper `c51b8acd9596e00d`, limiter `6a8c2ac883e57e19`.

**Codegen.** `next_value` and `advance_block` are textually unchanged. The field order and types
are unchanged, so `size_of::<LinearRamp>()` and every plan fixture are unchanged
(`graph_fixture --check` passes).

**Gates.**
- `cargo test --locked` passes, one crate at a time, for: effect-runtime, compressor, delay,
  de-esser, dynamic-eq, gate-expander, multiband-compressor, reverb, soft-clip,
  transient-shaper, true-peak-limiter, parametric-eq, effect-compiler, rack, graph-compiler,
  host-core, builtins, dsp-reference, math and effect-contract.
- `cargo clippy --all-targets -D warnings` and `cargo doc -D warnings` are clean for the 11
  touched crates.
- `cargo fmt --check` passes.
- `cargo check --target wasm32-unknown-unknown` with `+simd128` passes for the touched crates.
- `scripts/check-workspace-policy.sh` is ok.
- `realtime_policy` is ok (298 marked regions in 48 files).

### Attempt 1 verdict: PASS (minor findings folded)

The adversarial review passed `738256385`. It confirmed four things:
- No live writer produces the newly refused state.
- The adjusted tests lost no coverage.
- The lane-vector arithmetic is unchanged.
- The three "not a defect" conclusions above are sound.

The three minor findings were folded in after merging `origin/main` (`9949664cb`):
1. **The `record_advance` doc** said "branch-free". It now says the compiler may branch (see the
   decision above).
2. **Restore normalisation is now tested.** `transient-shaper`
   `state_restore_validates_version_length_envelope_and_parameters` restores a settled ramp 0
   holding `-0.0` at its `+0.0` target. The restore is accepted, and the snapshot comes back
   `+0.0`. Mutations, each reverted:
   - dropping `normalize_zero(current)` before `from_words` turns it RED;
   - dropping both normalisations turns it RED.

   Delay (`read_carried_ramp`) and reverb (`state.rs` `read_carried_ramp`) normalise by the same
   rule, in the same order. They are covered by that rule and get no row of their own.
3. **`record_advance` asserts in debug** that a settled advance is at its target. The release-only
   store of an off-target settled advance stays in `words_enter_a_ramp_only_in_phase` under
   `cfg!(not(debug_assertions))`. The new `a_settled_advance_off_its_target_asserts_in_debug` is
   RED when the assertion is replaced by `true`. The debug test suites of every crate that calls
   `record_advance` pass, so the assertion never fires on a real advance.
