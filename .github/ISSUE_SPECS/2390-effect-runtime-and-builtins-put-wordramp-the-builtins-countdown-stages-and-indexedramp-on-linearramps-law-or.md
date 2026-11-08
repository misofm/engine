# Effect runtime and builtins: put `WordRamp`, the builtins' countdown stages and `IndexedRamp` on `LinearRamp`'s law, or record why they cannot

Successor of #2376, which landed the shared lane-vector walk `effect_runtime::ramp::RampLanes` and moved the compressor, the transient shaper and the true-peak limiter onto it. That issue's spec named three encodings outside its slice, and this issue covers them. The lineage is #2368, then #2376, from tracker #2258 item GR-7 ("one ramp state machine has about six encodings").

## Problem
`LinearRamp` (`crates/effect-runtime/src/ramp.rs`) carries the D11 law and its settled invariant: `remaining == 0` implies `current.to_bits() == target.to_bits()` and a `+0.0` step. That invariant is structural. Words from outside enter only through `LinearRamp::from_words`, and a lane-vector advance goes through `RampLanes`, which writes back through `record_advance`. Three other encodings still keep their own copy of the walk, or their own settle check, or both:

1. **`WordRamp`** (`crates/effect-runtime/src/dynamic_band.rs`, `WordRamp` and `advance_word_ramps`). These are the dynamic band's live words in de-esser and dynamic EQ. They are stored as lane words across blocks, not gathered per block.
   - `advance_word_ramps` is an ungated copy of `RampLanes::advance_where`.
   - The restores already validate through `from_words`: de-esser `parse_lane`, the ramp and link chunks; dynamic EQ `parse_lane`, the same.
   - So the walk is the remaining duplicate.
2. **The builtins' countdown stages** (`crates/builtins/src/lib.rs`: `InputStage`, `FaderRampStage`, `MatrixStage`).
   - The authoritative countdown is a `u32` per lane, beside an `f32` countdown residue that the kernel reads. The stage rebuilds that residue from the `u32` before each ramp kernel runs (issue #2377 T2).
   - Lane state enters through each stage's `import_lane`. Its only callers are `builtins-compiler`'s plan-swap carry, from a predecessor plan's `export_lane`, and the builtins' own lane swaps. So the words are engine-produced, not an external payload.
   - Each stage keeps its own settle rule. Examples: `FaderPhase::at_countdown` (#2280); `MatrixStage::import_lane` clamping the countdown to `MATRIX_RAMP_COUNTDOWN_MAXIMUM`; the input stage's `refresh_ramp_flags`.
3. **`IndexedRamp`** (`crates/lane/src/kernels.rs`).
   - This is the 2x2 route-mix law: `c(k)` is a pure function of the ramp index `k`, and a settled ramp is `length == 0` or `k >= length`.
   - Its doc already explains why it is not D11 ("Why it is not D11").
   - Its users are `crates/graph/src/runtime.rs`, the route mix, and the live crossfade in `crates/effect-contract/src/live.rs`.

## Contract
- **`WordRamp`'s walk is `RampLanes`'s walk.** One option is for `advance_word_ramps` to delegate to `RampLanes::advance_where` with an all-lanes gate. The other is for `WordRamp` to become a `RampLanes`, which needs a crate-visible per-lane accessor for the effects' `link_lane`/`set_link_lane`. Either way, the per-sample advance has one body. The restore's `from_words` validation stays.
- **Each builtins stage either shares `from_words` or records why it cannot.** "Shares" means `import_lane` decides a lane's settled phase through `LinearRamp::from_words`, or through one shared rule beside it. "Records" means the stage's doc states why the `u32` plus residue form is not `LinearRamp`'s. In both cases the doc states that `import_lane` reads only engine-produced words, and which writer upholds the invariant.
- **`IndexedRamp` records, in its own doc, that it cannot share `from_words`.** Its law is not D11, and its settled phase is decided by the index, not by a countdown word. The change is to `crates/lane` alone, unless the record finds a validating restore in `crates/graph`, which would then need its own decision.
- **SIMD lane-word encodings stay branch-free, with no enums in lane words.**
- **No rendered bit, state word or payload word moves.**

## Non-goals
- A stationary hoist for builtin gain retargets. It is cost-only. If it is wanted, it gets its own issue, and it must exclude mute moves, which would move signed-zero bits (#2368 finding 2).
- Changing the `IndexedRamp` law, or the builtins' countdown authority.

## Gates
- **Mutation evidence.** Each removed duplicate's settle and advance check is still caught by one test in the crate that owns the logic: `effect-runtime` for the walk, `builtins` for the stages. Equivalent mutants are named with their reason.
- **No bit moved.** For de-esser, dynamic EQ and the builtins sections, rendered output and snapshot or export words are identical to base, scalar and native bank width (PR evidence, not a committed digest).
- **Realtime.** Render stays allocation-, lock- and syscall-free, and `realtime_policy` passes.
- **Downstream.** The graph-compiler fixtures and host-core are green. If a struct size changes, regenerate `graph_fixture` intentionally, or `graph_fixture --check` must pass.
- **Split by encoding** (`WordRamp`, builtins, `IndexedRamp`) if one brief cannot reach a closable slice in half a day.

## Evidence / attempt record

### Attempt 1 (Terra), base `3375b619b`

**Slice.** All three encodings fit one brief, so it is not split. `WordRamp` is unified. The
builtins stages and `IndexedRamp` each record a reasoned "cannot". The builtins also gain a
debug-only check of the half of the invariant they do hold. No release code path in `builtins` or
`lane` changes.

**Claims re-verified on the base.**
- `WordRamp`. `advance_word_ramps` (`crates/effect-runtime/src/dynamic_band.rs:181` at base) is
  `RampLanes::advance_where` with no gate. It computes `last = remaining == 1` without ANDing it
  with `moving`, and that changes nothing: `1.0 > 0`, so `last` already implies `moving` for
  every `f32` word, NaN included. The restores validate through `from_words`: de-esser
  `crates/de-esser/src/lib.rs:1341`, dynamic EQ `crates/dynamic-eq/src/lib.rs:1546, 1579`. The
  claim holds.
- Builtins.
  - The lane-state records have private fields, and only `export_lane` builds one. The only
    callers of `import_lane` are `builtins-compiler`'s `adopt_lane`, which imports the
    predecessor's `export_lane` (`crates/builtins-compiler/src/lib.rs:4839, 5282, 5691`), and the
    stages' own `swap_lanes` (`crates/builtins/src/lib.rs:3057`, `5067`, `5474`). So every word
    an import reads is engine-produced. The claim holds.
  - A new finding: three of the four settled phases keep their step. These settles assign the
    target but leave the step word: `MatrixStage::end_ramp_window` and `MatrixStage::reset`,
    `InputStage::settle` (trim) and `InputStage::settle_filter`. Only the fader's settles clear
    it. A settled step is dead. Each kernel selects the target on a lane whose countdown has run
    out (`crates/lane/src/kernels/builtins.rs`, `done = remaining <= 0`), and every retarget
    derives the step afresh from `current`. But every lane export carries it, so
    `LinearRamp::from_words` would refuse these engine-produced records.
  - The input filter's coefficient ramp is not D11. After 4 leading updates, a word is
    `target - step * remaining` (`filter_ramp_words`, #1407).
  - The spec lists `MatrixStage::import_lane`'s clamp to `MATRIX_RAMP_COUNTDOWN_MAXIMUM` as a
    settle rule. It is not one. It writes the kernel's `f32` countdown residue, which `process`
    rebuilds before every ramp kernel. Mutation S2 below is equivalent.
  - `builtins` does not depend on `effect-runtime`. Its dependencies are `engine`,
    `effect-contract`, `lane` and `math`.
- `IndexedRamp`. Production builds one only through `IndexedRamp::settled` and `IndexedRamp::new`.
  No struct literal and no field write exists outside `crates/lane`. The graph carries a
  `ColumnRamps` by value across a plan swap (`CarriedRoute`), and the live crossfade carries one
  by value too (`BypassShunt::carry_from`, `crates/effect-contract/src/live.rs:1552`). The graph's
  one `from_words` (`RouteControlRecord::from_words`, `crates/graph/src/runtime.rs:1195`) decodes
  control records, not ramps. So there is no validating restore in `crates/graph`, and the change
  is confined to `crates/lane`.
- Restore defect of #2368's class: none found. No encoding builds a ramp from words that came
  from outside the engine without `from_words`.

**Change.**
- `effect-runtime`. `RampLanes` gains two crate-visible conversions, `from_lane_words` and
  `into_lane_words`. `advance_word_ramps` passes each ramp's words through
  `RampLanes::advance_where` with every lane gated in (`L::zero().eq(L::zero())`). That leaves
  one per-sample body. The walk stays branch-free and has no enum. `WordRamp` keeps its public
  fields, because the effects read them per sample.
- New test `tests/ramp_lanes.rs::a_word_ramp_walks_the_scalar_law_word_for_word`. For three ramps
  per call, at every width, it checks all four words against `LinearRamp::next_value` on every
  frame. The defect it catches is the word-ramp path leaving the scalar law: the wrong gate, a
  dropped or swapped word, or a later ramp of the slice skipped. The de-esser and the dynamic EQ
  never scatter these words, so the lane-vector walk test cannot see this path. No existing test
  of the word walk existed, so none is deleted.
- `builtins`. The new `settled_on_target(current, target, remaining)` is the settled half of
  `from_words`'s invariant. Its doc is the record of why the stages do not build their lanes
  through `from_words`, for the four reasons above. Each `import_lane` doc names its writers:
  - input: `set_trim_signed` with no window, `settle`, `apply_prepared_filter` when no word
    moves, `settle_filter`, `reset_with_kind`;
  - fader: `retarget` with no window, `end_ramp_windows`, `reset`;
  - matrix: `set_target_over` with no window, `end_ramp_window`, `reset`.

  Each `import_lane` also debug-asserts `settled_on_target` on every carried word. The step half
  is not asserted anywhere, because a settled step is dead (mutation B2).
- `lane`. `IndexedRamp`'s doc gains "Why it does not share `LinearRamp::from_words`". Its phase
  is the caller's index against `length`, and from `k >= length` on `coefficients_at` and
  `route_mix_ramp_block` assign `target` whatever `start` and `step` hold. So no choice of words
  is a settled ramp off its target. Only `settled` and `new` build one, and a carry moves it by
  value.

**Mutations.** Each mutation was applied, the named tests were run in debug, and the mutation was
reverted.

`effect-runtime` (`ramp_lanes`). "walk" is `a_lane_vector_walks_…`, "rest" is
`a_lane_at_rest_…`, "word" is the new `a_word_ramp_walks_…`.

| mutation | walk | rest | word |
|---|---|---|---|
| W1 `advance_word_ramps` gates no lane in | green | green | RED |
| W2 the step is not written back | green | green | RED |
| W3 only the first ramp of the slice advances | green | green | RED |
| W4 the target word is read from `current` | green | green | RED |
| M4 `advance_where`: the last frame adds the step | RED | green | RED |
| M5 `advance_where`: the step is kept after the snap | green | green | RED |
| M16 `advance_where`: `moving = gate` (no in-flight test) | RED | RED | RED |

M5 was an equivalent mutant in #2376 because the effects there scatter through `record_advance`,
which stores `+0.0` on a settled lane. A `WordRamp` is never scattered, and the stale step would
reach the state payload, where `from_words` refuses it on restore. The new test catches it, so it
is no longer equivalent.

`builtins`. These are the settle rules each stage keeps, and the new import check.

| mutation | result |
|---|---|
| S1 fader `import_lane` copies the phase instead of `at_countdown` | RED `lane_state::an_unmute_holds_the_gain_only_while_its_countdown_runs` |
| S3 input `import_lane` skips `refresh_ramp_flags` | RED `tests::a_reset_and_a_lane_import_republish_the_ramp_flags` and `a_carried_lane_arms_its_joint_flush_on_the_frame_the_exported_lane_would` |
| F fader `retarget` with no window leaves `current` | RED, the new fader import assertion in `lane_state::an_arm_holds_the_lane_until_its_fire_across_a_carry_and_a_reset` and `…imported_mid_fade…`, plus three `fader_ramp` render tests |
| B8 fader `reset` leaves `current` | RED `the_banks_render_their_scalar_sections_under_random_retargets` |
| M matrix `set_target_over` with no window leaves `current` | RED two `matrix` render tests |
| R `settled_on_target` inverted | RED, all three import assertions reached by `builtins` tests: input ×2, fader ×3, matrix ×1 |
| S2 matrix `import_lane` writes `0.0` for the kernel's countdown word | green, `builtins` and `builtins-compiler` |
| B2 fader `end_ramp_windows` keeps the step | green, `builtins` and `builtins-compiler`. A draft import check that also asserted the step went RED in `builtins-compiler`'s `a_carried_automated_lane_renders_the_uninterrupted_bits` |

S2 is equivalent. `MatrixStage::process` rebuilds the countdown word from the `u32` before every
ramp kernel, so the word an import writes is never read.

B2 is equivalent for rendering, for the same reason as the matrix's and the trim's kept steps. A
settled step is never read. Its only effect is an export word: the digest below moves under B2.
That is why the import check asserts only the settled-on-target half.

**No bit moved** (one-time PR evidence, not committed). There are two scratch harnesses, each run
identically on base and fix, and both stay outside the tree.
- Effects: `effect-compiler/tests/zz_digest_2390.rs`. It reuses `tests/reset_lane.rs`'s fixture,
  as in #2376. For each of the 11 launch effects there are two parts, each on the running and the
  resting schedule:
  - scalar: two lanes' draws, run plain, with each reset kind at the boundary, and with a later
    payload restored there;
  - native W8 bank, for the 9 banking effects: untouched, a whole-bank reset of each kind,
    `reset_lane` on three lanes, and two restores of later payloads into rotated lanes.

  The digest covers every output word, rest mask and lane payload after every block, plus every
  restore verdict. It was run in release.
- Builtins: `builtins/tests/zz_digest_2390.rs`. It is issue #1051's randomized differential at 96
  seeds, W4 and W8 banks against scalar strips. A double `swap_lanes` on every bank, a quarter of
  the blocks, drives `export_lane` and `import_lane`. The digest covers every stage's output, both
  bank and scalar, every lane's `export_lane` record for all three banks, the
  `fader_bank_lane_words` / `matrix_bank_lane_words` / `bank_section_words` helpers, and the
  scalar sections' words. It was run in release and in debug; debug runs the new import
  assertions.

| | base `3375b619b` | fix |
|---|---|---|
| builtins (release and debug) | `dfe728a5cdd80b31` | `dfe728a5cdd80b31` |
| compressor scalar / w8 | `918c83ddb8254867` / `87ac19a4bd7fc950` | same |
| de-esser | `57c21c63a2c3fb53` / `e1eb49bda4346b8b` | same |
| delay | `655b7f8f23b36551` / -- | same |
| dynamic-eq | `51540a4992e9d0d1` / `7927f344a24a50ed` | same |
| gate-expander | `c9373442fce5309f` / `01a5c2b78e05689d` | same |
| multiband-compressor | `d0d2da18b9d1618e` / `fa37f97fb1d33d4d` | same |
| parametric-eq | `ad79e6e02faf8582` / `26cbb3a7582c7c68` | same |
| reverb | `d17ac43fb55f96c5` / -- | same |
| soft-clip | `e8cb5c8c220b0172` / `32ee22223ae005cd` | same |
| transient-shaper | `06d752bec221e8b1` / `98e58a4b5a67ef54` | same |
| true-peak-limiter | `45c1fedf8eb5e216` / `c267169b8d5b7b02` | same |

The harnesses see the changed paths:
- With M5 applied, the de-esser and dynamic-EQ digests move, scalar and W8, through `WordRamp`'s
  payload. No other effect's digest moves, because the compressor's gated walk scatters through
  `record_advance`.
- With B2 applied, the builtins digest moves to `0997d1a80d810a48`.

The W4 bank does not bind on this AVX2 host, so the 4-lane walk is covered by the committed
`ramp_lanes.rs`, which runs every width.

**Gates.** All pass on the fix:
- `cargo test --locked`, one crate at a time: effect-runtime, de-esser, dynamic-eq, builtins,
  builtins-compiler, lane, graph-compiler, host-core.
- `cargo clippy --locked --all-targets -- -D warnings` and
  `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` for effect-runtime, builtins and lane.
  The builtins doc also passes with `--document-private-items`.
- `cargo fmt --check`.
- `cargo check --target wasm32-unknown-unknown` with `+simd128` for effect-runtime, builtins,
  lane, de-esser and dynamic-eq.
- `scripts/check-workspace-policy.sh`, `scripts/check-lane-policy.sh` and `realtime_policy`
  (299 marked regions in 48 files).
- `graph_fixture --check` exits 0. No struct changed.

**Follow-ups** (not filed):
1. The builtins treat the kernel's `f32` countdown residue three ways:
   - the fader neither carries nor imports it (#2377 T2);
   - the matrix's import writes it but its export does not carry it (S2: a dead write);
   - the input's export carries it (`InputChannelState::ramp[3]`) and its import writes it,
     though `load_countdown` rebuilds it before every ramp kernel.

   Dropping the matrix write and the input payload word would give one rule. The input change is
   a size change of `InputLaneState`.
2. Four constants are all `1 << 24`: `FADER_RAMP_COUNTDOWN_MAXIMUM`,
   `MATRIX_RAMP_COUNTDOWN_MAXIMUM`, `InputStage::RAMP_COUNTDOWN_MAXIMUM` and effect-runtime's
   `EXACT_COUNTDOWN`.
3. The builtins' block kernels in `crates/lane/src/kernels/builtins.rs` each spell the D11 frame
   walk inline: `remaining - 1`, `done = remaining <= 0`, then `select(done, target,
   ramp_toward(..))`. The kernels are the trim, the fader/mute and the matrix. A lane-level
   helper could hold that walk once. That is beyond this issue's contract, which leaves the
   countdown authority alone.
