# Effect runtime: put the remaining ramp encodings and the effects' lane-vector ramp helpers on `LinearRamp`'s law

Successor of #2368 (GR-7's first slice), filed when the implicit-state-machine tracker #2258 closed. #2368 made `LinearRamp`'s settled invariant structural (private fields, `from_words`, `record_advance`) and found that the tracker's three listed divergences were not defects.

## Problem
There are still several encodings of one ramp law, each with its own settle and restore checks:
- `WordRamp` (the dynamic band);
- the builtins' `u32` countdown plus `f32` residue stages (fader, matrix, input);
- `IndexedRamp`;
- near-duplicate lane-vector helpers that gather, advance by the law and scatter: the compressor's `RampVec`, the transient shaper's `LinkRamp` and the limiter's `RampLanes`.

Each copy is a place where a restore can admit a settled ramp off its target, which is the defect class #2368 fixed in three effects.

## Contract
- One lane-vector helper in `effect-runtime`, next to `LinearRamp`, that gathers, advances by the law and scatters through `record_advance`. The compressor, transient shaper and limiter use it.
- `WordRamp`, the builtins' countdown stages and `IndexedRamp` either share `LinearRamp`'s validation (`from_words`) or document why their encoding cannot. They are SIMD lane-word encodings: they stay branch-free, with no enums in lane words.
- No rendered bit or state or payload word moves.

## Non-goals
- A stationary hoist for builtin gain retargets. It is cost-only; if it is wanted, it gets its own issue, and it must exclude mute moves, which would move signed-zero bits.

## Gates
- Mutation evidence that each removed duplicate's settle and restore check is still caught by one test in `effect-runtime`.
- No bit moved: per effect, rendered output and snapshot payloads are identical to base (PR evidence).
- Downstream: graph-compiler fixtures and host-core are green; struct-size changes regenerate `graph_fixture`.
- Split by encoding if the brief cannot reach a closable slice in half a day.

## Evidence / attempt record

### Attempt 1 (Terra), base `f24da176c`

**Slice.** The brief's smallest closable slice: the shared lane-vector helper and its three users.
`WordRamp`, the builtins' countdown stages and `IndexedRamp` are not touched; they are proposed as
a successor below.

**Claims re-verified on the base.**
- The three helpers are one law, twice spelled. The compressor's `RampVec`
  (`crates/compressor/src/kernel.rs` ~1292-1358) walks it as gated selects (`advance_where`: a
  lane with one frame left takes its target, any other in-flight lane `ramp_toward`). The
  transient shaper's `LinkRamp` (`crates/transient-shaper/src/lib.rs` ~554-610) and the limiter's
  `RampLanes` (`crates/true-peak-limiter/src/lib.rs` ~1520-1631) walk it as
  `remaining = max(remaining - 1, 0)` plus two selects (`advance`). On a settled lane the two forms
  agree because #2368's invariant puts `current` on its target with a `+0.0` step. Each had its
  own gather and its own scatter through `record_advance`.
- One gather differed: the limiter's coefficient gather narrowed the countdown through `u16`
  (`f32::from(ramp.remaining() as u16)`). Not a defect: a limit or release ramp is armed with
  `RAMP_UPDATES` (64) and a restore bounds `remaining` by it (`ramp_path_inside`, ~5336), so the
  narrowing never cut a word. Its link gather (`gather_link`) already used `as f32`, because a
  glide runs to `2^24`.
- The limiter's stationary load built a lane vector off the invariant (target, step and
  countdown `+0.0` beside a non-zero `current`). Not observable: the stationary dispatch reads
  only `current` and never scatters.
- No restore defect of #2368's class: every helper scatters only through `record_advance`, and no
  helper is restored from words.

**Change.**
- `effect_runtime::ramp::RampLanes<L>`, beside `LinearRamp`: private fields; `gather` and
  `scatter` over any iterator of ramps (the first `L::WIDTH`, scatter through `record_advance`);
  `resting(value)`; `current()`; `in_flight()`; and the two walks, `advance` and `advance_where`,
  whose bodies are moved unchanged. The countdown is an `f32` integer of at most `2^24`
  (debug-asserted on gather). No enum, and no per-frame branch.
- The compressor (`ChannelRamps`, `LinkWeights`), the transient shaper (`gather_link_weight` and
  `scatter_link_weight` over `ramps[l][weight]`) and the limiter (`HotChannel`, the glide weights)
  use it. Their three copies are deleted. The limiter's stationary load is now
  `RampLanes::resting(current)`, which is also the vector a full gather of at-rest ramps loads, so
  no lane vector is off the invariant any more. Its `u16` narrowing goes; both gathers are one.
- Tests. The new `effect-runtime` `tests/ramp_lanes.rs`:
  - `a_lane_vector_walks_the_scalar_law_word_for_word` checks, at every width, every frame's value,
    the in-flight mask, the gated walk's moved mask, and all four scattered words against
    `LinearRamp::next_value`. It splits the walk at random frames, and its ramps cover every phase:
    at rest, parameter windows, link glides past `u16::MAX`, restored steps and holding.
  - `a_lane_at_rest_advances_to_itself` checks that at rest, signed zeros and NaN included, the
    walks are the identity and `resting` is the vector its fixed ramps gather.
- Two limiter tests are deleted because their claims moved with the code:
  `the_lane_ramp_reproduces_the_scalar_ramp_bit_for_bit` (E13) and
  `the_stationary_hoist_reads_what_advancing_would_have_produced`. `tests/MUTATIONS.md` row 13 now
  points at the new gate. The two glide tests that read the private `remaining` word now read
  `in_flight()`.

**Mutations.** Each was applied to `RampLanes` in `crates/effect-runtime/src/ramp.rs`,
`tests/ramp_lanes.rs` was run, and the mutation was reverted. "walk" is
`a_lane_vector_walks_the_scalar_law_word_for_word` and "rest" is `a_lane_at_rest_advances_to_itself`.

| mutation | walk | rest |
|---|---|---|
| M1 `advance`: the settle adds the step (no D11 snap) | RED | RED |
| M3 `advance_where`: `gate` ORed instead of ANDed (ungated lanes move) | RED | RED |
| M4 `advance_where`: the last frame adds the step | RED | green |
| M6 `gather`: countdown narrowed through `u16` | RED | green |
| M7 `gather`: target read from `current` | RED | green |
| M9 `in_flight`: `remaining > 1` | RED | green |
| M10 `scatter`: the last lane skipped | RED | green |
| M11 `resting`: target `+0.0` | green | RED |
| M12 `scatter`: `current` not written | RED | green |
| M13 `scatter`: `remaining` not written | RED | green |
| M2 / M5 the step left in force after the settle (`advance` / `advance_where`) | green | green |
| M8 `scatter`: the step not written | green | green |

M2, M5 and M8 are equivalent mutants. A settled lane never reads its step again, because both walks
select the target or the unchanged word there. Its scatter stores `+0.0` through `record_advance`.
While a lane is in flight, the walk never changes its step. So no output word and no state word can
see these mutations.

**No bit moved** (one-time PR evidence, not committed). A scratch test in `effect-compiler`, run in
a release build identically on base `f24da176c` and on the fix, reuses `tests/reset_lane.rs`'s
fixture. For every one of the 11 launch effects it runs:
- a scalar instance on two lanes' draws, plain, with both resets at the boundary, and with a
  mid-run payload restored there;
- for the 9 banking effects, a full native-width (W8) bank: untouched, whole-bank reset of both
  kinds, `reset_lane` of three lanes, and two restores at the boundary of later payloads into
  rotated lanes.

Each runs both schedules (running and resting), with per-lane lattice automation, prepared
targets and 4096-sample link glides in flight across the boundary. It digests every output
sample, every rest mask, every lane's payload after every block, and every restore verdict.

All 11 digests are identical (`scalar` / `w8`):

| effect | scalar | w8 |
|---|---|---|
| compressor | `4b356c2dafa46b3e` | `2029f1796ff48685` |
| de-esser | `b318e5fdbbd167e1` | `6b36ff66d70d55e1` |
| delay | `58974a74698da544` | -- |
| dynamic-eq | `0375785f428decea` | `2baedf91291071a1` |
| gate-expander | `94cf6c5a3a03a682` | `74ad1e6cc69dd1e4` |
| multiband-compressor | `18770fa98a5968a8` | `3cbe508a2ddff251` |
| parametric-eq | `fc5527ebd2c187d8` | `f0fabb5dd706343e` |
| reverb | `d9e147ccc89ac8e4` | -- |
| soft-clip | `a78d30dda34e155f` | `70a01191304d3e08` |
| transient-shaper | `dbdf06adabb83830` | `566ee63d93ca1654` |
| true-peak-limiter | `94f25ffa52cb3d20` | `29f9fd22b94caab1` |

The harness sees each user. With M7 applied, the compressor, transient-shaper and limiter digests
all move, both scalar and W8, and no other effect's digest moves. With M1 and M4 applied together,
the compressor's and the limiter's digests move but the transient shaper's does not. That is
expected: a transient-shaper lane whose glide arrives on a frame takes its prepared law on that
frame (`glide_level`'s select), and the scatter stores the target, so the snap word is never read
there. The `W4` bank does not bind on this AVX2 host, so the 4-lane body is covered by the
committed `ramp_lanes.rs` and by each effect's lane-identity tests, not by the digest.

**Gates.** All pass on the fix:
- `cargo test --locked`, one crate at a time, for: effect-runtime, compressor, transient-shaper,
  true-peak-limiter, effect-compiler, rack, graph-compiler and host-core.
- `cargo clippy --all-targets -D warnings` and `cargo doc -D warnings` for the four touched crates.
- `cargo fmt --check`.
- `cargo check --target wasm32-unknown-unknown` with `+simd128` for the four touched crates.
- `scripts/check-workspace-policy.sh` and `scripts/check-lane-policy.sh`.
- `realtime_policy` (298 marked regions in 48 files).
- `graph_fixture --check`, which exits 0: no plan struct changed, because `RampLanes` lives only
  in block-loop registers and `LinearRamp` is unchanged.

**Proposed successor (not filed): the remaining encodings.**
- `WordRamp` (`effect_runtime::dynamic_band`, ~153-191). The dynamic band of de-esser and
  dynamic EQ stores its ramps as lane words across blocks; it does not gather them per block.
  `advance_word_ramps` is a third spelling of `advance_where` with no gate.
  - Option: store `RampLanes` there, which needs a crate-visible lane accessor for
    `link_lane`/`set_link_lane`.
  - Option: route `parse_lane`'s at-target and settled-step clauses (de-esser ~1306, dynamic EQ
    ~1506) through `LinearRamp::from_words` per lane.
- The builtins' fader, matrix and input stages (`crates/builtins/src/lib.rs`, `FaderRampStage`
  ~3325, `MatrixStage` ~3845, `InputStage` ~1051). Their authority is a `u32` countdown, beside a
  kernel `f32` countdown residue; their lane state enters through `import_lane` (~2975, ~3808,
  ~4201). Share `from_words` there, or document why the residue form cannot.
- `IndexedRamp` (`crates/lane/src/kernels.rs` ~1660). It is not D11: `c(k)` is a pure function of
  the ramp index, and its doc already says why ("Why it is not D11"). Its owner is `crates/graph`
  (`runtime.rs`) and the effect-contract live crossfade. Most likely it needs only a documented
  "cannot share `from_words`" line, but that touches `crates/graph`, which another stream owns
  at the time of this attempt.
