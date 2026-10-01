# Red-mutation record for the #88 gates

Master plan for issue #83, section 1.6: *every gate is proven red*. A test that has never failed is
not a gate. Each row below was applied to the working tree, the named test binary was run, the
result was recorded, and the mutation was reverted in the same session. Nothing in this file is a
claim about code that was not run.

Host: `x86_64` (AMD Ryzen 7 9700X, Zen 5), workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile unless noted.

## Issue #737 supersession notice

The causal compressor change removed its lookahead/detector history and staged idle body. The
historical rows below that name those retired structures are preserved as evidence, but are no
longer live gates: rows 1 and 4, rows 25--32, and M2-C1, M2-C2 and M2-C6--C8. Their current causal
replacements are the live `lane_identity`, `partition`, `nonfinite`, `mono_collapse`,
`silent_fixed_point`, and `causality` tests. The remaining rows in the original tables retain the
status recorded when their current production behavior was tested.

The current causal-compressor signed-zero regression in `silent_fixed_point` is an equivalence
check, not a gate for the shared predicate: Sol's sign-mask mutation of `block_is_positive_zero`
stayed GREEN in the compressor, while the corresponding EQ test remains the discriminatory gate.
The historical red-mutation records below remain preserved as evidence.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p compressor --test <test binary>
# and revert
```

## Gated

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | the detector gather uses `delay[0]` for every lane | `src/kernel.rs` | `lane_identity` | RED |
| 2 | `flush` dropped from the recursive word | `src/kernel.rs` | `nonfinite` | RED |
| 3 | the `mix == 1` identity select removed | `src/kernel.rs` | `identity` | RED |
| 4 | the ring wrap dropped: `next = write + 1` with no compare-select | `src/kernel.rs` | `partition` | RED |
| 5 | the release coefficient used for both ballistic arms | `src/kernel.rs` | `oracle` | RED |
| 6 | makeup dropped from the gain step | `src/kernel.rs` | `oracle` | RED |
| 7 | the recursive word kept in a local and never written back to the channel | `src/kernel.rs` | `cross_target` | RED |
| 8 | the ramping prefix never runs (`ramping = 0`) | `src/kernel.rs` | `ramps` | RED |
| 9 | the `bypass` term removed from `dry_identity` | `src/kernel.rs` | `identity` | RED |
| 10 | the ballistic coefficient is the retention, not the rate: `exp` for `1 - exp` | `src/design.rs` | `oracle` | RED |
| 11 | the ballistic coefficient designed in `f32`: `0.001 * ms * fs` as an `f32` product | `src/design.rs` | `cross_target` | RED |
| 12 | the per-lane coefficient scatter writes lane 0 for every lane | `src/design.rs` | `lane_identity` | RED |
| 13 | the restored ramp step ignores `remaining` and divides by 64 | `src/state.rs` | `payload` | RED |
| 14 | the left channel is committed before the right section is validated | `src/lib.rs` | `payload` | RED |
| 15 | the backend-availability fallback moved above the per-request validation | `src/lib.rs` | `contract` | RED |
| 16 | a Point ramps over 63 samples instead of the descriptor's 64 | `src/lib.rs` | `ramps` | RED |
| 17 | the boundary check no longer zeroes or resets the failing channel | `src/lib.rs` | `nonfinite` | RED |
| 18 | `offsets_are_ordered` returns `true` unconditionally | `src/lib.rs` | `contract` | RED (index out of bounds — the failure the check prevents) |
| 19 | the corpus renders one block instead of the frozen partition | `src/corpus.rs` | `cross_target` | RED |
| 20 | `inv_two_knee` designed as `1 / knee` instead of `1 / (2 * knee)` | `effect-runtime/src/dynamics.rs` (applied temporarily; that crate is **not** modified by this branch) | `static_curve` | RED |

Row 20 is applied to a foundation crate only to prove that this crate's E1 gate would catch a
change to the shared curve. `effect-runtime` and `lane` are byte-identical
to `origin/main` on this branch.

## Equivalent mutations (applied, GREEN, and recorded rather than gated)

| # | mutation | why it survives |
|---|---|---|
| 21 | the `G == 0 && makeup == +0` term removed from `dry_identity` | Applied and run: **GREEN** in `identity` and `nonfinite`, and it is genuinely equivalent for a finite sample. `gain_from_db(+0.0)` is exactly `1.0` (pinned by gate M1: `0 * LOG2_PER_DB` is `+0.0` and `exp2_lane(0)` is exact), so `wet = z * 1.0` is `z` bit for bit, and `mixed = fma(mix, z - z, z)` is `fma(mix, +0.0, z)`, which is `z` for every `mix`. The select is kept because it is the identity BRIEFS/013 freezes, and because keeping it makes this crate's identity independent of another crate's exactness pin rather than silently dependent on it. |
| 22 | the `Average` link written with an `fma`: `fma(0.5, |l|, 0.5 * |r|)` | Applied and run: **GREEN**, and provably so. `0.5 * x` is exact for every finite `x` (halving only decrements the exponent), so the fused form's internal product is the same number the unfused form rounds to, and both then round one addition. The plan's hazard note expected this to change bits; it cannot, for this multiplier. The unfused form is kept because BRIEFS/013 states the operation order, and because a multiplier that is not exactly `0.5` — which a future link law could have — would make the two differ. |
| 23 | the detector floor's D8 operands swapped: `level_floor.max(detected)` | Applied and run: **GREEN**. Two clamps stand between a NaN detector and the curve, and either alone suffices: `Lane::max(a, b)` returns `b` on an unordered pair, and `log2_lane` clamps its own argument up to `f32::MIN_POSITIVE` the same way. `tests/nonfinite.rs` pins the resulting behaviour — a NaN detector produces the level floor, never a NaN in `G` — rather than the operand order, because the behaviour is what a caller can observe. |
| 24 | the ramping/idle split removed: `ramping = frames`, so the ramping body runs for the whole block | Applied and run: **GREEN** in `partition` and `cross_target`, and it is exactly equivalent. `advance_ramps` is a no-op on a lane whose ramps all have `remaining == 0`: `next_value` returns `current` unchanged and `changed` stays zero, so no coefficient is redesigned. The split is a cost optimisation, not a semantic one — which is the right shape for it to have. The mutation that *is* gated is row 8, `ramping = 0`, which stops the ramps advancing at all. |

## Gates in this crate

| binary | proves |
|---|---|
| `contract` | E13: descriptor rows, zero latency, payload sizes, `scratch_fixed_bytes: 64`, parameter domains against the runtime's specs, the causal bank fallback ordering, the strengthened bank block guard, the three link laws |
| `static_curve` | E1: Giannoulis, Massberg and Reiss equation 4 against an `f64` transcription over a 3x4x3x737 grid — worst deviation **4.578e-6 dB**, gate 1e-4; knee continuity at both edges; the hard-knee threshold sample exact |
| `oracle` | E5: two configurations against the independent `f64` `ReferencePeakCompressor` — worst **4.694e-7** and **1.192e-7**, gate 2e-5 |
| `lane_identity` | E2: a bound bank against `W` scalar instances with per-track parameters — output bits, per-track payload bytes; plus the corpus at `W = 1`, 4 and 8 word for word |
| `partition` | E3: 4,096 frames in blocks of {1, 7, 63, 64, 65, 127, 128, 129, 512}, scalar and bank, output bits and payload bytes identical, with a Point on all seven smoothed parameters of both channels |
| `cross_target` | E4's corpus is finite, busy and has four distinct cases. Its pinned SHA-256 at all three widths is compared by gate G5 alone since issue #1048 (`tools/wasm-gates/tests/g5_native_corpus.rs`, and the wasm guests of `scripts/run-wasm-gates.sh`); the rows above that name `cross_target` for a digest predate that move |
| `identity` | E8: bypass, `mix == 0`, `mix == 1`, `G == 0 && makeup == +0`, the `Average` link's exact level, and that every identity keeps the state warm |
| `ramps` | E6, D11: one division at the event, iterated additions, the exact snap on update 64, a restart from the value reached, automation validation, and that a finished ramp equals a fresh preparation |
| `payload` | E7: idle restore bit-exact against an uninterrupted render, transactional rejection across both channels, the class-B mid-ramp restore, subnormal round trip, both resets |
| `nonfinite` | E9, D7: the boundary check trips once per block and not per sample, the left channel is untouched, the limit row, a NaN detector is clamped, and `flush` brings `G` to exactly `+0.0` |

### E14 (retired, measured once)

`tests/stall.rs` (`f32_release_stall_floor_is_reported`) was print-only — never asserted against a
threshold (issue #359 WP-2, `docs/audits/test-usefulness-2026-09-04/02-dsp-effects.md:68`) — and
was deleted rather than gated. Its retired requirement is BRIEFS/013, handed to issue #046. Before
deletion the number was recorded here once so it stays recoverable without rewriting the test:

At 96 kHz, release 5,000 ms (`c ≈ 2.08e-6`): the envelope settled under a loud burst at
`G = -37.1306 dB`; stepping to a quieter level whose static curve asks for `-13.2804 dB`, `G`
stopped moving at `-13.5093 dB` after 2,180,096 samples — a residual of `0.2289 dB`. The increment
`c * (C - G)` fell below roughly half an `ulp` of `G`, which is inherent to an `f32` state (master
plan D2 leaves an `f64` `Lane64` family open; BRIEFS/013's 0.005 dB envelope gate at the release
maximum needs that decision).

Measured once, release profile, `cargo test --release -p compressor --test stall -- --nocapture`
against `main`'s `crates/compressor/tests/stall.rs` (`git show main:crates/compressor/tests/stall.rs`),
recorded in this commit, 2026-09-04, one shared `x86_64` Zen 5 machine — evidence, not a pin.

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-13 | `PreparedCompressor::gain_reduction` returns a hardcoded zero pair instead of reading `Channel::gain_reduction_db` | `compressor/src/lib.rs` | `gain_reduction::the_compressor_reports_the_reduction_its_kernel_smoothed` | RED (`a signal well over the threshold is audibly reduced: 0`) |

## Issue #143 — the effect observation surface

R5 removed `PreparedNativeEffect::gain_reduction` and its test file is re-expressed on the declared
tap (`tests/observation.rs`); row 140-13 above is superseded by 143-E6-b, which is the same
mutation applied to the same kernel read through the new address. Same host and profile as above.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 143-E6-a | `PreparedCompressor::observe_resident` advances the smoother in the read (`self.instance.left.gain_reduction_db *= 0.9;`) | `compressor/src/lib.rs` | `cargo build -p compressor` | RED — **does not compile**: `observe_resident` takes `&self`, so "resident" is enforced by the signature rather than asserted. This is the `&self` half of E6 |
| 143-E6-b | `observe_resident` writes `0.0` into both lanes instead of reading `Channel::gain_reduction_db` | `compressor/src/lib.rs` | `observation` (whole binary) | RED — 5 of 6 tests fail; `the_compressor_reports_the_reduction_its_kernel_smoothed` reports `0` where reduction was required |
| 143-E2-a | `observe_resident_bank` broadcasts lane 0's reading to every lane | `compressor/src/lib.rs` | `observation::every_bank_lane_reads_its_own_reduction` | RED — `lane 1 left reading is its own, not a neighbour's`, left `0` vs right `3239051021` |

## Round 2 — historical staged idle body and pre-gathered detector taps (superseded by #737)

`kernel::process_block` sends an idle segment to `idle_frames_staged` when every live lane's
detector distance `D` is at least the segment length, and to `frames_loop` otherwise. The claim is
bit identity, and `tests/staged_idle.rs` is its gate: the same input at partitions that straddle
`D` puts the same frames through both bodies, with the 512-frame partition — always the per-frame
body, being longer than the staged bound — as the reference.

Every row below was applied to the working tree, `cargo test -p compressor --test
staged_idle` was run, the result was recorded, and the mutation was reverted in the same session.
Host: `x86_64` (AMD Ryzen 7 9700X, Zen 5), workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile.

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 25 | the legality guard loosened by one frame: `min_delay(..) + 1 >= len` | `src/kernel.rs` | `staged_idle` | RED — `left, D = 64, partition 65`, and `forced true, bank partition 1`. The first is the guard's boundary on the nose: at `D = 64` a 64-frame segment is legal and a 65-frame one is not. The second is the `D == 0` lane, whose tap is the row the frame itself writes |
| 26 | `fill_taps` uses `delay[0]` for every lane | `src/kernel.rs` | `staged_idle` | RED — `forced false, bank partition 1`. The per-lane stride of the pre-gather, which is row 1's mutation applied to the block-level gather |
| 27 | the delay test dropped from the guard, leaving only `len <= MAX_STAGED_FRAMES` | `src/kernel.rs` | `staged_idle` | RED — `left, D = 120, partition 127` and `forced false, bank partition 128` |
| 28 | the ring wrap dropped from `fill_taps`: `first = len` | `src/kernel.rs` | `staged_idle` | RED — `range end index 967 out of range for slice of length 961`, the out-of-bounds read the two-run split prevents. Row 4's mutation applied to the block-level gather |
| 29 | pass A of the staged body binds the linked levels the wrong way round: `let (level_right, level_left) = link_frame(..)` | `src/kernel.rs` | `staged_idle` | RED — `left, D = 960, partition 1` and `DualMono, bypass false, sidechain false, partition 1`. The three link laws, the bypass flag and a connected sidechain are all outside the frozen corpus's reach on the staged path, so they are covered here |
| 31 | the staged body pre-gathers the **right** channel's taps from `channel_right.cursor` instead of the shared left-cursor index (the adversarial verifier's V3) | `src/kernel.rs` | `staged_idle` | RED — `partition 128`, the first staged partition of `a_left_only_rejection_diverges_the_cursors_and_the_staged_body_still_agrees`. Applied to the whole crate, **every other test binary still passes**: this row is the only thing that gates it. The two cursors diverge only after `kernel::finish_channel` resets one channel, and re-converge at the end of the next block, so the window is one block wide |

### Equivalent mutations (applied, GREEN, and recorded rather than gated)

| # | mutation | why it survives |
|---|---|---|
| 30 | `segment_is_stageable` returns `false` unconditionally, so every idle segment takes the per-frame body | Applied and run: **GREEN** in `staged_idle`, and necessarily so — that is the whole point of the guard. The staged body is a cost optimisation with no semantics of its own, exactly as the ramping/idle split of row 24 is, and no bit-identity test can distinguish a renderer from itself. What distinguishes the two is the benchmark: `examples/lane_sample_timing` reports 2.61 → 1.93 ns/lane-sample at `W = 8` with the staged body in and out. The mutation that *is* gated is the one that takes the staged body where it is illegal, which is rows 25 and 27 |
| 32 | row 31's mutation, gated by a test that rejects the **right** channel rather than the left | Applied and run: **GREEN**, and necessarily so. `Channel::clear_state` zeroes the rejected channel's rings as well as its cursor, so when the *right* channel is the one reset every candidate tap row of its detector ring reads `+0.0` and the two cursors cannot be told apart. Only a **left**-only rejection leaves the diverged channel holding real signal, which is why row 31's test injects there. Recorded because the obvious way to write that test is vacuous |

## Mono-collapse M2 — historical collapsed-kernel staging records (superseded where noted by #737)

Driver as above: one mutation at a time, `cargo test -p compressor --test mono_collapse`,
tree restored between rows.

| # | mutation | file | test | result |
|---|---|---|---|---|
| M2-C1 | the collapsed per-frame body computes the level as `main_left.abs()` instead of `link_frame(detector, slot, main_left, main_left, ..)` — the "the link is a no-op on a mono bank" simplification | `compressor/src/kernel.rs` `frames_loop_mono` | `the_collapsed_body_renders_the_dual_bodys_left_plane` | RED — `LinkMode::Average` is `0.5*|p| + 0.5*|p|`, which is not `|p|` for a subnormal `p`. Every *sample* assertion stays green (the detector floor clamps it); the serialised detector ring is where it shows, which is why this test compares state |
| M2-C2 | the same, in the staged idle body | `compressor/src/kernel.rs` `idle_frames_staged_mono` | the same test | RED |
| M2-C3 | `Channel::copy_state_from` drops `words` | `compressor/src/kernel.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED — redundant *given* `ramps` on a continuously automated session and load-bearing on one whose ramp settled while collapsed, which is why the test retargets once and then stops |
| M2-C4 | `Channel::copy_state_from` drops `ramps` | `compressor/src/kernel.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED — all four fields of every smoothed parameter; only the collapsed channel's were advanced |
| M2-C5 | `Channel::copy_state_from` drops `gain_reduction_db` | `compressor/src/kernel.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED — the one recursive word, and the whole cross-frame dependency |
| M2-C6 | `Channel::copy_state_from` drops `cursor` | `compressor/src/kernel.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED — the shared ring write index, advanced once per frame on the collapsed channel only |
| M2-C7 | `Channel::copy_state_from` drops `main` | `compressor/src/kernel.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED — the delay ring the output is read out of |
| M2-C8 | `Channel::copy_state_from` drops `detector` | `compressor/src/kernel.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED — the detector ring the lookahead tap gathers from |

`a_statically_bypassed_bank_collapses_to_the_dual_bits` carries no mutation of its own: it is
coverage for the one place a **prepared** bypass and a **live** one differ. A live bypass clears the
witness' `UNBYPASSED` term and declines the collapse; a prepared bypass on a live-control-free bank
does not, because `EffectBankStage::lane_symmetry` is the designed-word comparison alone. The effect
contract calls that "a seam the collapse must close", and the test closes it by rendering a bypassed
bank both ways rather than by arguing that the bypass is per plane inside the kernel.

`lookahead_ms` and `delay` are on the copy list and are **not** individually red: no rendered block
writes them. See `copy_state_from`'s doc for why they are copied anyway.

## Issue #1123 supersession notice

The records below describe the implementation and test names at the time of those mutations.
Housekeeping removed the historical #1006 scenario digest gates and replaced the verbatim old
block splitter with the current one-frame scheduling reference. The mixed/compressing grids and
all three seeded differentials retain their generators, arithmetic/state comparisons and dispatch
witnesses. Retargets, resets, restores, hostile inputs and padding have current contract owners;
the complete survivor mapping is recorded in #1123. G5 remains the sole cross-target digest owner.
The old `causality` test compared identical API input blocks throughout its asserted prefix; real
partition and current frame-law gates cover causal scheduling. Historical mutation results stay
as evidence, without implying that a retired gate still runs.

## Issues #981-#985 — the settled-body rewrite

`kernel::settled_body_tests` (in `src/kernel.rs`) is the gate for this series. It keeps the kernel
as it stood before #981 -- `process_block`, `process_block_mono` and their one-pass `frames_loop`
bodies, verbatim -- as `settled_body_tests::reference`, and compares the production kernel with it
block by block: every output word, the recursive words, every coefficient word and every ramp
field, then the `finish_channel` masks and the same words again after the boundary check. Its
three kinds of gate are the deterministic grid (`the_settled_body_is_the_base_body_*`), the seeded
randomized differential (`randomized_differential_{f32,simd4,simd8}`, dual and collapsed), and the
`scenario_*` digests, each pinned on the unmodified base `197db1c9` before its slice landed.

Driver: one mutation at a time, `CARGO_INCREMENTAL=0 cargo test --locked -p compressor
--no-fail-fast` (dev), every red test recorded, tree restored before the next row. Host: `x86_64`
(AMD EPYC 7313P, Zen 3), workspace `.cargo/config.toml` pin `-C target-feature=+avx2,+fma`.
"Gate 1" below means the grid and all three randomized differentials.

### #981 — the chunked settled body, recursive words in locals

| # | mutation | red |
|---|---|---|
| 981-M1 | the recursive words are never written back to their channels | gate 1, `scenario_981_heterogeneous_hostile_render_is_pinned`, `cross_target`, and 22 more tests across `conformance`, `identity`, `lane_identity`, `mono_collapse`, `nonfinite`, `observation`, `oracle`, `partition` and `ramps` (27 red) |
| 981-M2 | `start` is ignored: the settled slice begins at frame 0 | gate 1, `scenario_981_*`, `cross_target`, `lane_identity::bank_matches_scalar_per_lane_bits`, `partition::block_partitions_are_invariant`, `partition::bank_block_partitions_are_invariant`, and three `ramps` tests (12 red) |
| 981-M3 | every detector takes `settled_main`, so `Silent` and `Sidechain` blocks read the main input | gate 1 (its `Silent` and `Sidechain` cases under a quiet main), `contract::links_are_exact_and_connected_sidechain_is_distinct_from_main_detection`, `nonfinite::a_nan_in_the_sidechain_alone_is_clamped_to_the_level_floor` (6 red). Neither `causality` nor `partition::linked_sidechain_partitions_are_invariant` went red, which is why the grid carries the sidechain cases |
| 981-M4 | the right channel's `one_frame` updates the left recursive word (`&mut gain_left`), the verification's replacement for "right before left" | gate 1, `scenario_981_*`, `cross_target`, `lane_identity`, four `mono_collapse` tests, two `nonfinite`, `observation`, `oracle` and two `ramps` tests (17 red) |

The brief's original M4, "right is processed before left", is not recorded as a gate: the
verification applied it and it is equivalent (the two channels' recurrences share nothing), which
is the same independence #983's pass 2 relies on.

### #982 — the all-wet arm, `input * gain`

Gate 1 compares NaN words as "both NaN" only in a block whose witness `SETTLED_WET_BLOCKS` says the
arm ran, and then requires `finish_channel` to have rejected that channel; everything else, and
everything after the boundary check, compares by bits. Gate 1 also asserts the witness moved
exactly when the block was an all-wet, unbypassed, settled `Main` block, so the grid and the
differentials see dispatch as well as bits.

| # | mutation | red |
|---|---|---|
| 982-M1 | the arm ignores `bypass` | `identity::bypass_preserves_exact_dry_bits_at_sample_zero`, gate 1 (`the_all_wet_arm_is_the_base_body_on_all_wet_tables`, the three differentials), gate 3, `conformance`, `contract::every_launch_rate_processes_scalar_and_supported_bank_at_zero_latency`, `mono_collapse::a_statically_bypassed_bank_collapses_to_the_dual_bits` (9 red) |
| 982-M2 | the arm is taken when any lane is wet (`mask_any`) | gate 1 on the mixed corpus banks (`the_settled_body_is_the_base_body_on_the_corpus_table`, `randomized_differential_simd4`/`_simd8`), gate 3, `scenario_981_*`, `cross_target`, `lane_identity::every_width_produces_the_same_words` (7 red) |
| 982-M3 | the arm drops `+ makeup` | gate 1, `scenario_982_all_wet_render_is_pinned`, `cross_target` (its `f32` lane 7 has mix 1 and makeup 6), `lane_identity::every_width_produces_the_same_words` (7 red). `oracle` stays green, as the verification found |
| 982-M4 | the arm is never taken | gate 3 (`the_all_wet_arm_is_taken_exactly_when_every_lane_is_wet`) and gate 1's dispatch assertion (5 red). No bit-exactness test can see it: it is a performance-only regression |
| 982-M5 | only the left channel's mask is tested | gate 3's "only the right channel has a non-wet lane" case, and gate 1 (5 red) |

### #983 and #984 — the two-pass settled body and its DualMono detector arm

Gate 1's grid gained the 97-frame count, so its settled bodies start at frames 0, 1, 18 and 40 of
blocks of 1, 7, 31, 32, 33, 97 and 128 frames: chunks that are short, exact, and misaligned to the
block. `scenario_983_chunk_straddling_render_is_pinned` adds blocks of 31, 32, 33, 64, 97 and 128
frames with ramps ending at frame 40.

| # | mutation | red |
|---|---|---|
| 983-M1 | pass 2 reads `targets[k + 1]` (`targets.iter().cycle().skip(1)`) | gate 1, all three scenarios, `cross_target`, `partition` (both), `lane_identity`, four `mono_collapse`, `conformance`, `contract` and two `ramps` tests (20 red) |
| 983-M2 | the last, short chunk is skipped (`chunks_exact_mut` on the outer loop) | gate 1, all three scenarios, `cross_target`, `partition::block_partitions_are_invariant`, `partition::bank_block_partitions_are_invariant`, `conformance`, `contract` (13 red) |
| 983-M3 | pass 1 runs only for the first chunk; later chunks reuse its targets | gate 1, all three scenarios, `cross_target`, `partition` (both), `lane_identity`, `conformance` and three `ramps` tests (16 red) |
| 983-M4 | `gl` and `gr` swapped in pass 2 | gate 1, all three scenarios, `cross_target`, `lane_identity`, two `nonfinite`, `conformance` and two `ramps` tests (15 red) |
| 984-M1 | the `abs` arm is taken for every link mode | gate 1, all three scenarios, `cross_target` (`maximum_link`, `average_link`), `identity::average_link_is_two_products_and_an_add`, `contract::links_are_exact_and_connected_sidechain_is_distinct_from_main_detection` (11 red) |
| 984-M2 | the arm returns `(|left|, |left|)` | gate 1, all three scenarios, `cross_target`, `lane_identity`, `nonfinite`, `conformance`, `contract` and two `ramps` tests (15 red) |

### Equivalent and performance-only mutations (applied, GREEN, recorded)

| # | mutation | why it survives |
|---|---|---|
| 984-M3 | the `abs` arm is never taken (`dual_mono = false`) | Applied and run: **GREEN** in every test, and necessarily so: the arm's bits are `link_frame`'s under DualMono (the `linked` mask is all zero and `select` is bitwise). It is a performance-only regression, caught by the recorded codegen evidence (#984 gate 7: DualMono pass 1 loads `abs` straight into the level floor with no link `vblendvps`), not by a test, as #944's M2 was |

### #985 — the collapsed body's settled rewrite

From #985 on, gate 1's collapsed half (`Mono::block` in the grid and the differentials) applies the
same witness-keyed "both NaN" rule as the dual half, and the grid runs a third parameter set,
`COMPRESSING_TRACKS`.

| # | mutation | red |
|---|---|---|
| 985-M2 | the short last chunk is skipped (`chunks_exact_mut` on the outer loop), the verification's replacement for "the body reads the right plane", which cannot be written | gate 1 (its 31-, 33- and 97-frame blocks: the grid on every table and the three differentials), `scenario_985_collapsed_render_is_pinned` (7 red) |
| 985-M3 | the recursive word's write-back is dropped | gate 1, `scenario_985_*`, and four `mono_collapse` tests including `the_collapsed_body_renders_the_dual_bodys_left_plane` (11 red) |
| 985-M4 | the collapsed wet arm ignores `bypass` | `mono_collapse::a_statically_bypassed_bank_collapses_to_the_dual_bits`, gate 1 (7 red) |

| # | mutation | why it survives |
|---|---|---|
| 985-M1 | the Average case takes the `abs` arm in the collapsed body | Applied and run: **GREEN** everywhere, including `mono_collapse::a_halved_subnormal_does_not_come_back` and gate 1's subnormal-only input, and it is equivalent. On one plane `0.5|m| + 0.5|m|` differs from `|m|` only where `0.5|m|` is inexact, which needs `|m| < 2 * f32::MIN_POSITIVE`; the linked value's only consumer is `curve_target`'s `max(detected, 1e-8)` floor, which maps both to `1e-8`, and NaN and `inf` agree. It was red in the retired design because a detector ring stored the linked value (M2-C1). The body keeps `link_frame` for Average and Maximum anyway, as the brief freezes, so the collapsed arithmetic stays the dual body's word for word |
| 985-M5 | Maximum also takes the `abs` arm | Applied and run: **GREEN**, as the verification predicted: `max(|m|, |m|)` is `|m|` on every backend. Recorded, never listed as red |

## Issue #995 — the sidechained settled body

A connected sidechain, present (`Detector::Sidechain`) or absent (`Detector::Silent`), now renders
its settled frames through `settled_sidechain`, the two-pass body with the sidechain as pass 1's
source, instead of the one-pass `frames_loop::<L, false>`. The gates are #981-#985's: the grid
and the three randomized differentials already ran `Silent` and `Sidechain` blocks against
`settled_body_tests::reference` (the one-pass body, verbatim), and they now also assert that
sidechained settled bodies started mid-block. Nothing is relaxed for them: a sidechained block
never takes the all-wet arm, so its words, NaN payloads included, compare by bits.
`scenario_995_sidechain_render_is_pinned` (both sources, `f32`, `Simd4` and `Simd8`, every link
mode, both tables, chunk-straddling blocks and ramps ending at frame 40) was pinned on the
unmodified batch head `fc43c97d`, dev and release: `25b39c7a...`.

Driver as for #981-#985: one mutation at a time on a scratch copy of the tree,
`CARGO_INCREMENTAL=0 cargo test --locked -p compressor --no-fail-fast` (dev), every red test
recorded. "Gate 1" is the grid on every table
(`the_settled_body_is_the_base_body_on_the_corpus_table`,
`the_all_wet_arm_is_the_base_body_on_all_wet_tables`,
`the_collapsed_settled_body_is_the_base_body_on_the_three_parameter_sets`) and the three
differentials.

| # | mutation | red |
|---|---|---|
| 995-M1 | the sidechain's first pass reads the main planes (`Detector::Sidechain(..) => Detector::Main` in `settled_sidechain`) | gate 1, `scenario_995_*`, `contract::links_are_exact_and_connected_sidechain_is_distinct_from_main_detection`, `nonfinite::a_nan_in_the_sidechain_alone_is_clamped_to_the_level_floor` (9 red) |
| 995-M2 | the sidechain offset never advances, so every chunk detects on the settled slice's first chunk (`offset += 0`) | gate 1, `scenario_995_*`, `partition::linked_sidechain_partitions_are_invariant` (8 red) |
| 995-M3 | the sidechain planes are not sliced to the ramp prefix's end (`[..settled.len()]`) | gate 1 (its settled bodies starting at frames 1, 18 and 40), `scenario_995_*` (7 red) |
| 995-M4b | an absent sidechain detects the main planes (`Detector::Silent => Some(chunk)`) | gate 1, `scenario_995_*` (7 red) |
| 995-M5 | a sidechained block takes the all-wet arm when #982's predicate holds | `the_all_wet_arm_is_the_base_body_on_all_wet_tables`, `randomized_differential_simd4`/`_simd8`, `scenario_995_*` (4 red). The difference is a quieted signalling NaN in a block the boundary check then rejects, which is why the gates hold sidechained blocks to bits before `finish_channel` |
| 995-M6 | the DualMono `abs` arm for every link mode | gate 1, `scenario_995_*` (7 red) |

| # | mutation | why it survives |
|---|---|---|
| 995-M4 | an absent sidechain's one target is never computed, so pass 2 reads the zero-filled scratch | Applied and run: **GREEN** everywhere, and equivalent for every legal parameter set. A silent detector floors to `1e-8`, whose level (about -160 dB) is at least 80 dB under any legal threshold (`>= -80`) and so past any legal half knee (`<= 12`): `gain_delta_db` takes its `under` arm, `+0.0`, and both clamps keep it, which is the zero fill's word. The body computes the target anyway, so it is `link_frame`'s and `curve_target`'s value by construction rather than by a range argument over the parameter domains |
| 995-M7 | the DualMono arm is never taken for a sidechain (`if false`) | Applied and run: **GREEN**, necessarily: under DualMono `link_frame`'s result is `abs` of the word it read (984-M3's argument). Performance-only |

The independent head-against-candidate differential of the attempt evidence (spec #995) is red on
M1, M2 and M5 at `f32`, `Simd4` and `Simd8`; on M1 and M2 also through the public factory; on M5
only at the kernel word, as the boundary check predicts.

## Issue #1006 — the ramping prefix as the two-pass body with lane-wide ramps

The main detector's ramping prefix, dual (`ramping_main`) and collapsed (`ramping_main_mono`),
now advances each channel's ramps as lane vectors (`ChannelRamps`) inside the settled body's two
passes: the threshold, ratio and knee ramps and the curve redesign in pass 1, attack, release,
makeup and mix in pass 2. The oracle is `settled_body_tests::reference`'s one-pass
`frames_loop::<L, true>` and `frames_loop_mono::<L, true>`, verbatim. The grid and the three
randomized differentials assert, per block, that the two-pass prefix ran (dual and collapsed
witnesses) exactly on a `Main` block with an open window, and that it took the all-wet arm exactly
when the block was unbypassed, no mix ramp was open and every lane's mix was `1`; #982's NaN
relaxation is granted only in a block whose witness shows an arm ran (#1065 later made every
comparison class-A, every NaN one value, in every block). The differentials also
retarget both channels with one value in one block, and restore ramps through the payload codec at
`remaining = 0` with `current != target`. Two scenarios were pinned on the unmodified batch head
(`081fdc6c`) before the change: `scenario_1006_ramping_prefix_is_pinned` (kernel, `f32`, `Simd4`,
`Simd8`, dual and collapsed, payload words) and `ramping_prefix_scenario.rs` (the bank contract at
this build's width, dual and collapsed, reports and payloads).

Driver as for #981-#985, dev profile, one mutation at a time on a scratch copy.

| # | mutation | red |
|---|---|---|
| 1006-M1 | the `remaining == 0` hold dropped: a ramp at rest takes its target, as on its last sample | `randomized_differential_simd4`/`_simd8` (their payload restores at `remaining = 0`), both #1006 scenarios (4 red) |
| 1006-M3 | the output ramps (attack, release, makeup, mix) advanced in pass 1 as well, the brief's "attack in pass 1" | gate 1, both #1006 scenarios, `partition::block_partitions_are_invariant`, `payload::an_active_attack_restore_*`, `ramps::an_attack_cancel_to_current_*`, `native_points`, `cross_target`, two `mono_collapse` tests and `ramps::a_both_channel_point_on_every_bank_lane_*` (`bench_ramp`'s preflight until #1027 ported it) (16 red) |
| 1006-M5 | the wet arm taken while a mix ramp is open (both bodies) | gate 1 (its witness and its bits), both #1006 scenarios, `partition`, `lane_identity::every_width_produces_the_same_words`, `cross_target` (9 red) |
| 1006-M6 | the words scattered from before the prefix (the dual prefix's word write-back dropped) | gate 1, `scenario_981`, `982`, `983` and `1006`, the bank scenario, #982's witness gate, `cross_target`, two `ramps` tests, `mono_collapse` (16 red) |
| 1006-M7 | the collapsed prefix skips the curve redesign on a moved lane | gate 1, `scenario_985`, `scenario_1006`, the bank scenario, two `mono_collapse` tests (11 red) |
| 1006-M8 | the collapsed prefix takes the wet arm while a mix ramp is open | gate 1, both #1006 scenarios (6 red) |
| 1006-M9 | the prefix's DualMono `abs` arm for every link mode | gate 1, `scenario_981`, `982`, `983` and `1006`, the bank scenario (11 red) |
| 1006-M10 | the threshold ramp never advanced in the prefix (a moving parameter dropped) | gate 1, `scenario_981`, `982`, `983` and `1006`, the bank scenario, four `ramps` tests, `payload::a_mid_ramp_restore_*`, `cross_target` (17 red) |
| 1006-M11 | #986's "the mask from one channel": the right channel's ramping set taken from the left's | gate 1, `scenario_1006` (7 red) |
| 1006-M12 | #986's "a changed word not reloaded": makeup and mix advance but their words are not written | gate 1, both #1006 scenarios, #982's witness gate, `ramps::a_ramp_onto_an_identity_boundary_*`, `native_points`, `cross_target` (12 red) |

| # | mutation | why it survives |
|---|---|---|
| 1006-M2 | the rate ramps gated by their own `remaining` rather than their parameter ramp's | Applied and run: **GREEN**, and equivalent. Every write site keeps a rate ramp's `remaining` at `0` or at its parameter ramp's: `set_parameter_target` arms both with the same window (and re-arms the parameter ramp when only the rate ramp would move), `restore_rate_ramps` copies the parameter's `remaining`, and the resets and `copy_state_from` move both together. So a rate ramp is never in flight on a lane whose parameter ramp is at rest, and the two gates select the same lanes. The prefix keeps `advance_ramps`' gate anyway |
| 1006-M4 | the redesigned curve written on every lane rather than the moved lanes | Applied and run: **GREEN**, and equivalent by an invariant, not by construction: every write site keeps `words[0..4] == GainComputerCoef::new(ramps[0..3].current)` on every lane (VERIFY-AUTOMATION section 4), so rewriting an unmoved lane rewrites the word it holds. This is C2b's invariant, which is out of this slice pending the owner's ruling; the prefix keeps `design_lane`'s `changed` rule, so it does not depend on it |

The independent head-against-candidate differential of the attempt evidence (spec #1006) is red on
M3, M5 and M6 at `f32`, `Simd4` and `Simd8`, and on M1 at `Simd4` and `Simd8`. M1 cannot fail at
`f32`: the prefix advances a parameter only when some lane of it is in flight, and a one-lane
parameter at rest is never advanced, so the hold is exercised only by a resting lane beside a
moving one.

## Issue #994 — a knee whose `1 / (2 W)` overflows, through the compressor's entry points

The fix is in `effect-runtime` (`dynamics::knee_coefficients`); this crate's source is unchanged
and its reduction clamp in `kernel.rs` is untouched. The mutations were applied to the shared
design temporarily, as for row 20, to prove this crate's gates catch it. Debug profile,
`CARGO_INCREMENTAL=0 cargo test --locked -p compressor --test knee_overflow --no-fail-fast`; the
file was restored byte for byte from a saved copy.

| # | mutation | test | result |
|---|---|---|---|
| 994-C1 | `knee_coefficients` without its `is_finite` test (the pre-#994 design) | `knee_overflow` | RED, 4 of 5: the prepare, automation, restore and bank tests. First failure `prepare: W 3e-45 (0x00000002) R 4 frame 0: 0.11473124 vs f64 reference 1` — the NaN target clamped to -100 dB and a 0.1 ms attack duck the very first full-scale sample by 18.8 dB. The automation test is red although both of its endpoints (0 and `1e-38`) are outside the overflow band: the `Linear 64` ramp passes through it |
| 994-C2 | the rule as a constant bound one ulp too low: `knee_db >= f32::from_bits(0x0010_0000)` | `knee_overflow` | RED, 2 of 5: the prepare and bank tests, the two whose widths include `2^-129` itself |

`the_narrowest_soft_knees_do_not_duck_a_sample_at_the_threshold` stays green under both: it is a
preservation gate for widths the rule must not move, not a red gate.
