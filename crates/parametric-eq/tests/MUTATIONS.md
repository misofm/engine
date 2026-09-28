# Red-mutation record for the parametric-EQ gates

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test binary was run, the failure was
recorded, and the mutation was reverted in the same session. Nothing here is a claim about code that
was not run — including the two mutants that survived, which are recorded with the reason.

This record matters more than usual for this crate. The gate it replaces
(`endpoint_conditioned_delta_matches_the_independent_oracle_on_the_complete_grid`) was green while
the shipped kernel was 12.4859 dB out on 483 of its 1,488 rows, because it evaluated a rational
function of the stored words rather than the transfer the kernel computed (#87 F1). A gate that
cannot be made red by breaking the thing it claims to certify is the failure mode this file exists
to rule out.

Host: `x86_64` (Zen 5 class), `rustc 1.97.1`, workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile. Rows 17 and 23 were re-run after the state payload
adopted the shared codec's two-word header inside the current layout (W2-D2's never-bump-twice rule);
every other row is unchanged by that amendment.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p parametric-eq --test <test binary>
# and revert
```

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | `a1` storage instead of `c1`: `c1 = 1 - f32(1 / (1 + t))`, the master plan's pre-A1 text | `src/lib.rs` | `analytic` | RED |
| 1b | the same mutation, measured by the 48 one-second impulses | `src/lib.rs` | `time_domain` | SURVIVED |
| 2 | low shelf without the `sqrt(A)` prewarp: `g = tan(pi f0/fs)` | `src/lib.rs` | `analytic` | RED |
| 3 | high-shelf `m1` sign flipped | `src/lib.rs` | `analytic` | RED |
| 4 | bell damping without the gain factor: `k = 1/Q` instead of `1/(Q A)` | `src/lib.rs` | `analytic` | RED |
| 5 | the snap does not assign the target, it only stops the ramp | `src/lib.rs` | `contract` | RED |
| 6 | the ramped segment loses its pre-advance, so frame 0 uses the pre-event words | `src/lib.rs` | `contract` | RED |
| 7 | `ramp_frames = length` instead of `length - 1`, so the block steps once too often | `src/lib.rs` | `contract` | RED |
| 8 | a block is never cut at a lane's ramp end, so lanes of different ramp ages share one segment | `src/lib.rs` | `bank` | RED |
| 9 | the ramp increment is not stored in the payload (word 8..14 written as zero) | `src/lib.rs` | `contract` | RED |
| 10 | a rejected block is zeroed but the integrators are not reset | `src/lib.rs` | `time_domain` | RED |
| 11 | the identity section is `m0 = 1 - 2^-24` instead of `1.0` | `src/lib.rs` | `contract` | RED |
| 12 | the ramp increment is `2^-6 * (1 + 1e-7)` instead of exactly `2^-6` | `src/lib.rs` | `determinism` | RED |
| 13 | the D7 flush is the identity | `../lane/src/lib.rs` | `time_domain` | RED |
| 14 | `design_svf` drops its spectral-norm guard | `src/lib.rs` | `analytic` | SURVIVED |
| 15 | the automation domain check always validates against the frequency spec | `src/lib.rs` | `contract` | RED |
| 16 | a settled band accepts stored words that disagree with its stored parameters | `src/lib.rs` | `contract` | RED |
| 17 | the payload header stamps invalid version 0 | `src/lib.rs` | `contract` | RED |
| 23 | the descriptor advertises `common_bytes = 0` while the codec stamps a header | `src/lib.rs` | `contract` | RED |
| 18 | `word_spectral_norm` drops the off-diagonal term of `M^T M` | `src/lib.rs` | `analytic` | RED |
| 19 | the corpus reads a lane back from the mirrored AoSoA offset | `src/corpus.rs` | `determinism` | RED |
| 20 | the bypass path renders instead of copying the dry block | `src/lib.rs` | `contract` | RED |
| 21 | the corpus stops staggering its per-lane ramp ends | `src/corpus.rs` | `determinism` | RED |
| 22 | a `-0.0` automation value is not normalised to `+0.0` on the way in | `src/lib.rs` | `contract` | RED |
| 24 | `discontinuity_reset` snaps the section but does not refresh `identity` | `src/lib.rs` | `lib` | RED |

## Recorded failures

### 1 — `a1` storage instead of `c1`

The amendment A1 evidence, reproduced as a mutation.
`svf_words_match_the_independent_oracle_on_the_complete_grid` fails at `analytic.rs:226`: the
low-frequency, high-Q rows exceed the frozen 0.005 dB tolerance, because `t = g(g + k)` is about
4.7e-6 at 10 Hz / Q = 18 / 88.2 kHz and storing `1 - t` instead of `t` spends the whole `f32`
mantissa on the leading one.

### 1b — the same mutation, measured on the impulses: **SURVIVED**

Recorded rather than quietly dropped. The 48 frozen impulse rows are the two parameter *corners*
(10 Hz / -24 dB / Q = 0.1 / S = 0.1 and 20 kHz / +24 dB / Q = 18 / S = 1.0), and neither is where
`a1` storage hurts: at 10 Hz the corner uses Q = 0.1, so `k = 10` and `t = 3.6e-3` — a relative
error of 1.7e-5, not the 0.3 % the Q = 18 row carries; at 20 kHz `g` is large and `t` is order one.
The row that separates the two storages is 10 Hz **with Q = 18**, which lives in the analytic grid
(row 1) and not in the impulse set. The impulse gate is therefore a gate on the realization and the
flush, not on the storage, and row 1 is what protects the storage. The impulse set is deliberately
left as issue #42 froze it.

### 2, 3, 4 — mapping mutations

Each fails `the_f64_mapping_reproduces_the_verified_reference_mapping` at `analytic.rs:144` on the
first shelf or bell row, before any `f32` rounding is involved; 4 additionally fails the `f32` grid
and the PCM-fixture rows. These are the three mutations issue #105 used against its own oracle
(its M1/M2/M3), applied to this crate's side of the mapping.

### 5, 6, 7 — the D11 word ramp

All three fail `automation_starts_a_64_sample_word_ramp`. 5 leaves the words at
`start + 64 * step` instead of exactly `target`; 6 makes frame 0 use the pre-event words, so the
first ramped word is `start`; 7 advances twice on the first frame. 7 also fails
`automation_is_partition_invariant`, because the number of additions then depends on how the block
was cut.

### 8 — one segment for lanes of different ramp ages

`every_width_matches_the_scalar_instantiation` fails at block 2, track 0, frame 0. This is the
mutation that matters for the bank: at `WIDTH = 1` there is only one lane, so no cut is ever needed
and the scalar path is unaffected; at `WIDTH = 8` the lanes whose ramps ended mid-block keep
stepping past their target. A width difference that only appears when ramps of different ages
coexist is exactly what the corpus's staggered case (21) and this gate are for.

### 9 — the increment is not stored

`state_restore_continues_active_ramp_bit_exactly` fails on the continuation block: the restored
effect re-derives an increment from the remaining distance, which is the pre-D11 law, and the two
renders diverge within one block.

### 10 — a rejected block does not reset

`a_non_finite_input_block_is_zeroed_counted_once_and_leaves_the_next_block_clean` fails on the
snapshot: the NaN that entered the integrators survives the zeroing of the output, so the *next*
block is poisoned too and the counter under-reports the fault.

### 11 — a near-identity identity

`disabled_and_zero_db_sections_return_dry_bits_with_zero_state_growth` fails on the first sample.
`1 - 2^-24` is inaudible and still wrong: a disabled slot must be the input bit for bit, or a
cohort's membership becomes observable in its output.

### 12, 19, 21 — the cross-target corpus

12 moves the `cascade/ramped_noise` digest away from its pin (the increment is no longer an exact
power of two, so the ramp accumulates differently); 19 breaks width agreement itself
(`simd4 vs scalar` fails before the pin is even consulted); 21 removes the per-lane stagger, which
moves the ramped digest and would have left mutation 8 undetected by this gate.

### 13 — the flush is the identity

`flush_keeps_decaying_state_out_of_the_subnormal_range` fails inside the impulse loop: the retained
words of the 44.1 kHz / 20 kHz / +24 dB / Q = 18 bell — the row that "recovered" at sample 39,223
under the old predicate — enter the subnormal range as the tail decays. With the flush they are
either exactly `+0.0` or at least `1e-20`.

### 14 — the spectral-norm guard is dropped: **SURVIVED**

Also recorded rather than dropped. `word_spectral_norm(words) > NORM_TOLERANCE` is a guard on a
region that no in-domain parameter set reaches: `word_ramps_are_contractive_on_every_grid_row`
measures a worst norm of `1 + 1.03e-7` over 372 rows and 207,018 convex combinations, and the
10,000 seeded designs reach `1 + 6.7e-8`, both inside the `1 + 2^-22` tolerance. Removing the guard
therefore cannot fail a test that only feeds it legal parameters — which is the *point* of the
measurement, not a hole in it. What must be discriminating is the predicate, and that is mutation
18: `the_spectral_norm_predicate_separates_contractive_from_expansive_words` pins a triple whose
spectral radius is 1 and whose operator norm is 1.4, and dropping the off-diagonal term of `M^T M`
scores it 1.0 and fails. The guard stays because a future parameter-domain change is exactly the
event that would make the region reachable.

### 15, 16, 17, 20 — contract surfaces

15 makes every automation value validate against the frequency domain, so the Q point at 1.0 is
rejected and `malformed_automation_rejects_each_span_without_losing_valid_targets` counts seven
invalid spans instead of six. 16 accepts a settled band whose stored coefficient words disagree
with its stored parameters — a payload that would render something the session does not describe.
17 stamps the wrong version into the header, so the current payload rejects itself — which is the
whole point of the header: the caller's out-of-band `state_layout_version` is a claim, word 0 is
evidence. 23 lets the descriptor advertise a common section the codec does not write, so every
snapshot is refused for length. 20 renders the
signal on the bypass path, and `bypass_copies_dry_bits_and_leaves_the_state_alone` fails because
bypass must preserve the dry bits *and* the latency, not approximate them.

### 22 — `-0.0` is not normalised

`a_negative_zero_automation_value_is_accepted_as_zero` fails on the stored target: the payload
carries `-0.0` and a later restore has to decide what to do with a value five of the eight effect
crates used to reject outright. 83c decision 3 settled it — accepted as a way of writing zero,
normalised on the way in — so nothing downstream of the validator ever sees a negative zero.

### 24 — the reset site is the one ramp end outside the render path

Row 24 is the mutation the round-2 verifier found surviving: deleting `self.refresh_identity(section)`
from `discontinuity_reset` passed the entire suite. The committed code was already correct; what was
missing was a gate over it, because no test drove the shape that makes the site load-bearing.

`identity[section]` is allowed to be stale exactly while a ramp is in flight. `start_ramp` refreshes
the flag while `coef` still holds the words it is leaving, so a section ramping *away from* identity
keeps a `true` flag; `advance_words` then walks `coef` off identity and refreshes nothing. That is
safe only because a ramp in flight makes the bank non-stationary, and `identity` is read on the
stationary path alone. Every ramp that ends inside `process_section` refreshes on the way out.

A discontinuity reset is the one ramp end that happens *outside* the render path. It snaps `coef` to
a live target and clears `remaining` in one step, so the next block is stationary and reads the flag
immediately. Without the refresh the flag still claims identity for a section that is now a real
filter, and `cascade_sections` elides it.

`a_discontinuity_reset_mid_ramp_refreshes_the_identity_flag` drives that shape: four identity
sections, a ramp started on section 0 toward a live corpus design, sixteen frames of it walked off
(the ramp is 64 samples, so it is still in flight), then the `DiscontinuityKeepParameters` reset and
a stationary block. It carries three independent oracles, and the mutation trips all three — the
explicit `identity_flags_agree` assertion, the production `debug_assert` in `process_channels`
(`src/lib.rs:1193`), and the render itself, which elides **0 of 4** cascade positions against the
settled reference's 4 and hands back the block unfiltered. The first and third are ordinary
assertions, so this row is RED in release as well as debug.

The pre-existing reset gate (`resets_restore_defaults_or_only_clear_history`, and the conformance
reset scenario) does not reach it: those reset a section that is live *before* the ramp, where the
flag was already `false` and the snap cannot make it wrong.

## Mono-collapse M2 — the collapsed cascade and the disengage copy

Driver: one mutation at a time, `cargo test -p parametric-eq --test mono_collapse`,
tree restored between rows.

| # | mutation | file | test | result |
|---|---|---|---|---|
| M2-E1 | `PreparedParametricEq::desymmetrize` drops `sections` | `parametric-eq/src/lib.rs` | `a_desymmetrized_bank_is_a_never_collapsed_bank` | RED |
| M2-E2 | the collapsed body hard-codes `stationary = true` instead of reading the one live channel's `no_ramp_in_flight` | `parametric-eq/src/lib.rs` `render_mono` | `the_collapsed_body_renders_the_dual_bodys_left_plane` | RED — 2 failed; a ramping block taking the interleaved cascade renders a coefficient that is not the one in force |

`remaining` and `identity` are on the copy list and are not individually red, and the reason is
worth stating: both are read only to choose a *schedule* — the interleaved cascade over the
per-section one, and the elided cascade over the full one — and this crate's own gates prove all
three schedules render the same bits. A stale copy of either leaves the two channels taking
different schedules to the same words, which is the invariant the whole-state rule protects.

## Issue #976 — no identity padding in the stationary cascade

Driver: one mutation at a time applied to `src/lib.rs`, then
`cargo test --release -p parametric-eq --features test-support --lib --test bank --no-fail-fast`,
tree restored between rows. Host: AMD EPYC 7313P (Zen 3), `rustc 1.97.1`, `x86-64-v3`, release
profile (fat LTO). Gate 1 is `odd_live_counts_render_the_base_bits` (`tests/bank.rs`); its counter
assertion needs `--features test-support`, because a `cfg(test)` counter in `lib.rs` is not compiled
for an integration test. The in-crate rows are in `src/lib.rs`'s `elision` module.

| # | mutation | gate that goes red | result |
|---|---|---|---|
| 976-M1 | `cascade_sections` (dual only) rounds the live count up again: `kept = live.div_ceil(2) * 2` with the old padding loop | `an_elided_cascade_is_the_full_cascade_bit_for_bit` (dual `kept == live`, first at live `000001`: `[2, 2, 2]` against `[1, 1, 1]`), `the_two_channels_are_judged_together`, `the_shipped_shape_actually_elides`, `a_section_live_on_one_lane_is_not_elided`, `a_negative_zero_input_refuses_elision`, `a_non_finite_or_oversized_input_refuses_elision`, `an_odd_tail_without_dry_lanes_skips_the_select`; gate 1's counter (scalar leg) | RED |
| 976-M1m | the same in `cascade_sections_mono` only | `an_elided_cascade_is_the_full_cascade_bit_for_bit` (mono `kept == live`), `an_odd_tail_without_dry_lanes_skips_the_select` (mono count 1, want 2); gate 1's counter (bank-mono leg) | RED |
| 976-M2 | `interleave` runs the depth-one tail before the pairs | gate 1 digests: scalar `757d054f…`, bank `5d2fa266…` against the pinned base; `an_elided_cascade_is_the_full_cascade_bit_for_bit`, `the_two_channels_are_judged_together` (bits) | RED |
| 976-M2m | the same in `interleave_mono` | gate 1 bank-mono digest `7760302e…` against the pinned base; `an_elided_cascade_is_the_full_cascade_bit_for_bit` (mono bits) | RED |
| 976-M3 | `interleave` always takes the masked depth-one arm (`if true \|\| …`) | gate 1's counter (scalar leg first: HPF-everywhere ran 0 select-free tails); `an_odd_tail_without_dry_lanes_skips_the_select` | RED |
| 976-M3m | the same in `interleave_mono` | gate 1's counter (bank-mono leg); `an_odd_tail_without_dry_lanes_skips_the_select` (mono) | RED |

M1 and M3 move no rendered bit, and that is expected rather than a gap: the padding section is an
exact identity (the elision proof covers dropping it), and a select whose mask is empty returns the
wet word. Gate 1's three digests therefore stay at their pins under both (measured in release),
and without `test-support` gate 1 is green for them. They are caught by the two counts that describe the
schedule rather than the audio -- `kept == live` and the select-free tail counter -- which is why
both exist. M2 is caught only by a shape with a pair ahead of the tail: one live section has no
pairs, so the three- and five-section shapes carry that row.

## Issue #980 — the elision gate in its min/max form

Driver: one mutation at a time applied to `block_admits_elision` in `src/lib.rs`, then
`cargo test --release -p parametric-eq --lib elision -- --include-ignored`, tree restored byte for
byte between rows. Gate 1 is `the_min_max_gate_equals_the_rejection_oracle` (every ordered pair of
the sixteen edge patterns, planted at two positions, block lengths 0, 1, 7, 8, 9 and 1,024, against
the `#[cfg(test)]` rejection-accumulator oracle `block_admits_elision_oracle`);
`every_word_gets_the_rejection_oracles_verdict` is the ignored exhaustive run over all `2^32` words.

| # | mutation | gate that goes red | result |
|---|---|---|---|
| 980-M1 | `nearest.min(bits)` (the `^ NEGATIVE_ZERO_BITS` dropped) | gate 1 first at length 1, word `0x00000000`: `+0.0` refused (`false`, oracle `true`); the exhaustive run (word `0x00000000`); also `a_negative_zero_input_refuses_elision`, `the_negative_zero_refusal_is_load_bearing`, `an_elided_cascade_is_the_full_cascade_bit_for_bit` | RED |
| 980-M2 | `largest < ELISION_MAGNITUDE_CEILING` | gate 1 first at length 1, word `0x7149f2ca` (the ceiling itself, `1e30`): refused, oracle admits; the exhaustive run (`0xf149f2ca`) | RED |
| 980-M3 | `largest.max(bits)` (the `& MAGNITUDE_MASK` dropped) | gate 1 first at length 1, word `0x80000001` (a negative subnormal): refused, oracle admits; the exhaustive run; and eight other `elision` tests, because every negative sample now refuses | RED |

M2 is invisible to every test that predates the gate: the ceiling is `BLOCK_LIMIT` itself, and no
earlier fixture places a word of exactly `1e30`. Only the edge set (and the exhaustive run) does.

## Issue #977 — every depth-two pass of an admitted plan runs select-free

Driver: one mutation at a time applied to `src/lib.rs`, then
`cargo test --release -p parametric-eq --features test-support --lib --test bank --no-fail-fast`,
tree restored byte for byte between rows. Gate 1 is `admitted_blocks_render_the_base_bits_without_selects`
(`tests/bank.rs`; digests pinned on the unmodified base: scalar `9316456b…`, bank `d4a1dc9d…`,
bank-mono `f442a0d3…`). Gate 2 is its masked depth-two pass counter (`test_only_masked_pair_passes`,
so `--features test-support`): zero on the admitted stationary blocks and non-zero on the refused
ones of the two switching shapes. The in-crate rows are
`elision::an_admitted_plan_runs_every_pair_select_free` (which also pins the tail's #976 rule
through the select-free tail counter) and `elision::a_non_finite_state_in_a_live_section_refuses_elision`.
Rows re-run for attempt 2 (the tail keeps #976's rule; see the issue's attempt 2 evidence).

| # | mutation | gate that goes red | result |
|---|---|---|---|
| 977-M1 | `interleave`: `admitted = true` | gate 1 scalar `cbbb3c83…` and bank `5f519f25…` (the poisoned dry lane's `NaN` state reaches the output on its refused blocks and the plane faults); gate 2 (refused blocks ran 0 masked pairs); in-crate (a refused block ran 0) | RED |
| 977-M1m | `interleave_mono`: `admitted = true` | gate 1 bank-mono `ded71138…`; gate 2 (mono refused 0); in-crate; and `interleave_identity::disabled_cuts_preserve_all_high_pass_signed_zero_against_four_section_oracle` (`+0.0` where the oracle keeps `-0.0`) | RED |
| 977-M2 | `interleave`: `admitted = false` | gate 2 (scalar DryHpfInPair: 228 masked pairs on admitted blocks); in-crate (2 masked pairs on an admitted block); no digest moves | RED |
| 977-M2m | `interleave_mono`: `admitted = false` | gate 2 (bank-mono: 26); in-crate (mono); no digest moves | RED |
| 977-M3 | `interleave`: the tail made select-free on admitted blocks too (`if !admitted && (…dry…)`), attempt 1's rule | in-crate: `Simd4 hpf 00000000 lpf 01100110: the tail keeps #976's rule` (1 select-free tail, want 0); no digest moves | RED |
| 977-M3m | the same in `interleave_mono` | in-crate: `the mono tail keeps #976's rule` | RED |
| 977-M4 | leg (c) without the finiteness term (`-0.0` only, as briefed before the amendment) | gate 1 scalar `f14e7956…` and bank `c79e11df…`; `a_non_finite_state_in_a_live_section_refuses_elision` | RED |

M2, M2m, M3 and M3m move no rendered bit: a select whose every lane returns the wet word is
invisible, so the counters are their only gates. M3 is attempt 1's tail rule: exact, and in the
shipped `simd128` artifact the change that let V8 keep an integrator of the one-band tail loop in a
stack slot, so its gate is the counter that pins the rule rather than a bit. (Attempt 1's M3 row,
"the tail skips its `admitted` arm", has no subject in attempt 2: the tail has no such arm.) M1 and
M4 are the correctness rows. The brief expected M1 to show as a `-0.0` rendered `+0.0` on a dry lane
of a refused block; in gate 1 that `-0.0` is rewritten to `+0.0` downstream by the dead general
bands a refused block executes, so M1 shows instead through the poisoned dry lane (and, in mono,
through the older all-high-pass signed-zero oracle, whose general bands are all live). In mono the
poisoned HPF is a dead section, refused by leg (b), so M4 moves only the dual legs.

## Issue #979 — an identity section with a frozen, inert state stays elidable

Driver: one mutation at a time applied to `lane_is_inert` in `src/lib.rs`, then
`cargo test --release -p parametric-eq --features test-support --lib --test bank --no-fail-fast`,
tree restored byte for byte between rows. Gate 1 is `a_cut_switched_off_keeps_the_bank_eliding`
(`tests/bank.rs`; pinned on the unmodified base: scalar `a34ce034…`, bank `2e0845c6…`, bank-mono
`26a755c1…`, and the restored-overflow leg faults its first block once per leg). Gate 2 is
`elision::a_band_switched_off_keeps_the_bank_eliding`, gate 3
`elision::a_non_inert_state_in_a_dead_section_refuses_elision` (formerly
`a_non_zero_state_in_a_dead_section_refuses_elision`). Re-run after the rebase onto #977's attempt 2,
on the branch-free body (`offset = magnitude.wrapping_sub(FLOOR)`, `inert &= (word == 0) | (offset
<= CEILING - FLOOR)`), which agrees with the range form on all `2^32` words: the same rows go red
with the same messages and digests.

| # | mutation | gate that goes red | result |
|---|---|---|---|
| 979-M1 | back to the exact `+0.0` test (`*word == 0` only) | gate 2: `Scalar section 3: 0 of 9 later blocks elide` (the shipped rule's cliff); gate 3: `1.0` refused | RED |
| 979-M2 | accept magnitudes below `FLUSH_EPS` (the floor set to `1`) | gate 3: `1e-30` elided, integrators differ (the executed section flushes it to `+0.0`); also `a_tiny_restored_disabled_cut_state_refuses_elision_but_preserves_old_bands` and `the_interleaved_cascade_renders_the_per_section_path_bit_for_bit` (seeded subnormal states) | RED |
| 979-M3 | accept `-0.0` (`word & MAGNITUDE_MASK == 0` for the zero term) | gate 3: `-0.0` elided, integrators differ (the executed section flushes it to `+0.0`) | RED |
| 979-M4 | accept above the cap (`[FLOOR, 0x7f80_0000)`, the rule as briefed before the amendment) | gate 3: the word above the ceiling (`1.0000001e30`) admitted; gate 1: scalar `c060ba98…`, bank `b0063058…`, bank-mono `7698bb8b…`, and the restored-overflow leg reports no fault (the band restored with `ic2 = -f32::MAX` is elided and the block renders audio) | RED |

Gate 2 is green under M2, M3 and M4 (a switched-off band's frozen state is always inert), and gate 1
under M1-M3 (the fixtures never restore a sub-`FLUSH_EPS` or `-0.0` word); each row is caught by the
gate built for it. On the unmodified base gate 2 reports 0 of 9 at every width and section, in dev
and release; after the change, 9 of 9.

## Issue #1005 — a ramping block runs only live or ramping sections

Driver: one mutation at a time applied to `ramping_sections`, `ramping_sections_mono` or
`ramp_keeps_unit_m0` in `src/lib.rs`, then `cargo test -p parametric-eq --lib ramping_elision`
(dev: 40 scenarios × 96 blocks per width and body), tree restored byte for byte between rows. M3,
M5-dual and M7 were re-run in release (300 scenarios). Gate 1 is the bank differential
`ramping_elision::a_ramping_block_renders_the_batch_head_bits_{scalar,simd4,simd8}` (dual and
collapsed; the oracle arm renders ramping blocks through `Channel::process_block`, the batch-head
path and the unit-test default of the `RAMPING_LIST` switch, which only the candidate arm sets).
Gate 2 is the list itself,
`ramping_elision::the_unsafe_ramp_rule_keeps_every_dead_section_after_it` and
`ramping_elision::a_ramping_identity_section_is_never_dead`, at `f32`, `Simd4` and `Simd8`. Gate
2b is `ramping_elision::a_restored_subnormal_live_state_refuses_the_list`: a live high shelf at
`m0 = m2 = 0.5` holding restored integrators of `-2^-149`, behind a ramping HPF toggle, rendered
through the list and through the batch-head path.

| # | mutation | gate that goes red | result |
|---|---|---|---|
| 1005-M1 | leg (a) dropped from both lists (`if false`) | gate 1 at every width (`W8 seed 2 block 10 mono false: left output`, W4 seed 2, W1 seed 23); gate 2 (`a -0.0 input refuses`) | RED |
| 1005-M1d | leg (a) dropped from the dual list only | the same as M1 | RED |
| 1005-M1m | leg (a) dropped from the collapsed list only | gate 1 collapsed at every width (`W1 seed 4294967305 block 47 mono true: left output`); gate 2 | RED |
| 1005-M2 | both lists: `dead` without the "not ramping" term (a ramping identity section treated as dead) | gate 1 at every width (`W8 seed 0 block 13: state`, `W1 …: right output`, `W4 …: report`); gate 2 (`LPF toggle`, `the left ramp keeps the HPF`) | RED |
| 1005-M3 | the unsafe-ramp rule dropped (`unsafe_before` never set) | gate 2 only (`ramping high shelf: the dual list`); gate 1 stays green in dev and in release | RED |
| 1005-M4 | the collapsed list's `dead` without the "not ramping" term | gate 1 collapsed at every width (`W1 seed 4294967296 block 15 mono true: state`); gate 2 (`the ramp keeps the HPF (collapsed)`, `LPF toggle: the collapsed list`) | RED |
| 1005-M5 | leg (b) skipped for dead sections in both lists | gate 1 at every width (`… : state`: the executed dead section flushes a restored `-0.0` or sub-`FLUSH_EPS` integrator, the elided one keeps it); gate 2 (`a -0.0 dead state refuses`) | RED |
| 1005-M5d | leg (b) skipped in the dual list only | gate 2; gate 1 at W8 in dev (`W8 seed 26 block 26 mono false: state`), and at every width in release | RED |
| 1005-M5m | leg (b) skipped in the collapsed list only | gate 1 collapsed at every width; gate 2 | RED |
| 1005-M6 | the dual list's `dead` without the "not ramping" term | as M2, dual rows | RED |
| 1005-M7 | `ramp_keeps_unit_m0` checks `coef.m0` only, not `step.m0` | gate 2 only (`identity to high shelf: the dual list`); gate 1 stays green in dev and in release | RED |
| 1005-M8 | leg (c) back to the stationary gate's form (`section_state_is_finite_without_negative_zero`) in both lists | gate 2b only: `Scalar mono false: the list moved a bit behind a restored subnormal` (the elided dead band passes `-0.0`, the executed one writes `+0.0`) | RED |
| 1005-M8d | the same, dual list only | gate 2b (`mono false`) | RED |
| 1005-M8m | the same, collapsed list only | gate 2b (`mono true`) | RED |

M3 and M7 are the rows the random differential cannot see, as the automation diagnosis found for M3:
dropping a dead section after an unsafe ramp is wrong only when that ramp emits `-0.0`, which needs a
constructed underflow the scenarios never produce. Gate 2 exists for them. M7's shape is reachable in
production -- a high shelf at 0 dB designs `m0 = A^2 = 1` exactly, and a ride away from 0 dB moves it
-- and gate 2 asserts that shape as well as the identity-to-shelf ramp. M8 is the same kind of row:
its counterexample is one `f32` gain and a restored payload, which no random draw reaches.
