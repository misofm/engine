# Drop the identity padding section from the stationary EQ cascade


EQ optimisation, slice 1 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at
`6ca203f8`; every `file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`. A timed prototype of exactly this change is
`docs/handoffs/effects-2026-09-27/eq-variant-no-padding.patch`.

## Product outcome

The stationary EQ cascade runs whole depth-2 passes, so it rounds the live-section count up and
keeps identity sections as padding: `cascade_sections` (`crates/parametric-eq/src/lib.rs:1683`,
`let kept = live.div_ceil(depth) * depth;` at `:1701`) and `cascade_sections_mono` (`:1509`,
`:1520`). On every standing console fixture a track has one live bell, so each bank runs the bell
beside the disabled HPF, and half of the executed section arithmetic computes nothing.

Keep only the live sections: run them in pairs, and run an odd last section alone at depth 1. A
prototype measured, on clean builds pinned to one core: `sixty_four_track_eq_only` minus
`sixty_four_track_builtins_only` went from 15.0 to 9.3 us per block natively (`Simd8`) and from
43.2 to 20.5 us in the browser build (wasm `simd128` under V8). All 15 console workloads rendered
the base digests at `Scalar`, `Simd4` and `Simd8` natively and in wasm. These figures are
descriptive; the paired console benchmark runs later, at the batch boundary.

## Lessons carried from #944 (apply them from the start)

1. Run every bit gate in **dev and release** (`cargo test` and `cargo test --release`); release is
   fat LTO and inlines differently.
2. Compare output words as "both NaN, or equal bits". The compiler may commute an addition, and on
   x86 two NaN operands keep the first one's payload.
3. Write the scenario test **first, on the unmodified base**, print its digest, and pin that digest.
4. Record every mutation below red in `crates/parametric-eq/tests/MUTATIONS.md`.

## Invariants

- **Class A.** Every rendered word and every integrator word is unchanged. The dropped section is
  an identity section at `+0.0` state. The elision proof on `cascade_sections` (`:1599-1682`)
  already covers dropping any such section; the padding existed only to keep one depth-2 kernel
  instantiation.
- Section order is the cascade order: the pairs first, the depth-1 section last, and it is always
  the last entry of the list.
- `kept >= EQ_SECTION_COUNT` still returns the full list without scanning the input (`:1702-1704`).
  The non-stationary (ramping) path is untouched.
- Render stays allocation-free. No new field, no new `unsafe`. `EFFECTIVE_CASCADE_DEPTH` stays 2 and
  `Lane::SVF_CASCADE_DEPTH` is untouched.

## Interface contract

1. `cascade_sections` and `cascade_sections_mono`: `kept = live` (delete the rounding and the
   padding loop's `padding` bookkeeping; the list is exactly the live sections in cascade order).
   Keep every refusal leg unchanged.
2. `interleave` (`:1756`) and `interleave_mono` (`:1568`): after the `length / DEPTH` pairs, when
   `length % DEPTH == 1`, run one depth-1 pass over `list[length - 1]`:
   - dual: `svf_cascade_interleaved::<L, 2, 1>` (no select) when neither channel's
     `dry_mask(at)` has a lane set, otherwise `svf_cascade_interleaved_with_dry_masks::<L, 2, 1>`;
   - mono: the same with `S = 1`.
   Delete the `debug_assert_eq!(length % DEPTH, 0)` lines; keep
   `debug_assert_eq!(EQ_SECTION_COUNT % DEPTH, 0)`.
3. No public API changes.

## Smallest closable slice

Authorized paths:

- `crates/parametric-eq/src/lib.rs`: the four functions above, their doc comments, and the in-crate
  test modules `interleave_identity` and `elision`.
- `crates/parametric-eq/tests/bank.rs`: gate 1.
- `crates/parametric-eq/tests/MUTATIONS.md`
- `tools/bench/src/floor.rs` (`EQ_LANE_OPS`, `:69`, and its pin at `:620`),
  `scripts/console-benchmark-record-lib.jq` (`eq_lane_ops`, `:69`), and the "EQ inventory" section
  of `docs/rulings/effect-floor-accounting.md`: the floor follows the executed inventory (gate 6).
- This spec.

Steps:

1. **On the unmodified base**, write gate 1 and record its printed digest here, then pin it.
2. Contract 1 and 2.
3. Update the five in-crate tests that pin the rounded count (gate 2).
4. The floor edit (gate 6), then the evidence record.

## Non-goals

- Select-free passes for pairs (EQ-2), the skewed kernel (EQ-3), the elision legs (EQ-4, EQ-5).
- The masked depth-1 pass's `vmaskmovps` (below). Record it; do not fix it here.
- No change to `crates/lane`, the ramped path, the bind, or `SVF_CASCADE_DEPTH`.

## Objective gates

1. **Scenario (public API, pinned on base).** Add `odd_live_counts_render_the_base_bits` to
   `crates/parametric-eq/tests/bank.rs`. For the scalar effect (`W = 1`) and the native bank
   (`native_bank()`), prepare configurations with 1, 3 and 5 live physical sections, including:
   - the HPF live on every lane (enable it with `hpf_target()`-style prepared targets);
   - the HPF live on some lanes only, so the depth-1 section has dry lanes;
   - live general bands through `set_initial` at prepare time.
   Render 32 blocks of hostile input: subnormals, `+0.0`, magnitudes `2^-24..2^25`, and on some blocks
   `-0.0` and one non-finite word (those blocks refuse elision and render all six sections). Fold
   every output word and every lane's state snapshot into one SHA-256 and pin the base digest.
2. **In-crate.** `an_elided_cascade_is_the_full_cascade_bit_for_bit` (over all 64 live masks at
   `f32`, `Simd4`, `Simd8`) stays green, and additionally asserts `kept == live`. These pin the
   rounded count and change to the live count: `the_two_channels_are_judged_together`
   (`lib.rs:3988-3993`), `the_shipped_shape_actually_elides` (`:4006-4014`),
   `a_section_live_on_one_lane_is_not_elided` (`:4054`), `a_negative_zero_input_refuses_elision`
   (`:4094`), `a_non_finite_or_oversized_input_refuses_elision` (`:4131`). Every other test in
   `cargo test -p parametric-eq` stays green unchanged, dev and release.
3. **Rows.** `cargo test --release -p console-workload --test chain_shape` stays green (its four
   pins include `sixty_four_track_console`). Compute the 64-block digest of every `WORKLOADS` row
   before and after (the `digests` harness in
   `docs/handoffs/effects-2026-09-27/eq-diagnosis-prototypes.patch`; apply it in a scratch copy,
   do not commit it) and attach both outputs.
4. **Mutations**, each alone, red:
   - M1: restore `live.div_ceil(depth) * depth` in the dual function only: gate 2's `kept == live`.
   - M2: run the depth-1 section before the pairs: gate 1 (needs the three- and five-section cases).
   - M3: always take the masked depth-1 arm: a `#[cfg(test)]` counter of select-free depth-1 passes
     (add one beside the existing test hooks) must be non-zero on gate 1's HPF-everywhere case. This
     is the only gate that sees a performance-only regression.
5. **Browser artifact.** Build the AudioWorklet wasm with `scripts/build-web-audioworklet.sh`'s own
   cargo line, pipe `wasm-objdump -d` into
   `scripts/check-web-audioworklet-callgraph.py --kernel-shape --kernel-pattern '4wide6f32x[48]'
   --kernel-min 11` and `--callgraph miso_engine_web_v1_render`. Rule 1 must still find exactly one
   arithmetic-carrying `parametric_eq ... 12process_bank` and one `17process_bank_mono` (the
   prototype kept them inline: `vector=312 scalar=0`). The artifact pin is repinned once at the
   batch boundary.
6. **Floor.** `EQ_LANE_OPS` and `eq_lane_ops` move from 53 to 27 (24 for the live section, 3 for the
   boundary scan), and the ruling's EQ inventory states `floor_lane_ops(active) = 25 * 2 *
   floor(active / 2) + 24 * (active mod 2) + 3` for an admitted block and 153 for a refused one.
   `floor.rs`'s own tests pass.
7. **Toolchain.** `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets
   --all-features -- -D warnings`; `bash scripts/check-realtime-policy.sh`,
   `check-lane-policy.sh`, `check-parametric-eq-render-contract.sh`.
8. **Codegen evidence** (recorded, not asserted): in the release `bench` binary,
   `process_bank::<f32x8>` gains a depth-1 select-free loop of about 56 instructions (48 of them
   arithmetic). Quote it.

## Console benchmark rows

- **Can move:** every row that carries the EQ (`eq_only`, `eq_comp_simd1`, `console`, the mono
  rows, `idle` only through its non-silent blocks), native and wasm.
- **Must not move:** `builtins_only`, `dispatch_only`, `gain_pan_only`, `compressor_only`, the ring
  rows.

## Dependencies

None. EQ-2 and EQ-3 build on it.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means no rendered bit moves. Every bit gate is a hard stop, never a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free.
- Run fmt, clippy and the focused tests before every checkpoint; commit on a `codex/<issue>-<slug>`
  branch; touch no path outside the list.
- Do not quote a projected saving; `scripts/run-console-benchmark.sh` is not run by the implementer.

## What the implementer will hit

- **The masked depth-1 pass folds into `vmaskmovps` at `Simd8`.** Its store is
  `select(mask, load(slot), wet)` to the same slot, the #944 pattern. It is still faster than the
  padded pass it replaces (4,519 against 5,883 cycles per bank-block in the replica), so it stays;
  EQ-2 removes it on admitted blocks.
- **The mono path is a second copy of the rule.** `cascade_sections_mono`/`interleave_mono` must
  change too; `crates/parametric-eq/tests/mono_collapse.rs` covers the collapsed body.
- **Do not change `EFFECTIVE_CASCADE_DEPTH`.** It still sets the pair size;
  `the_tuned_depth_is_per_backend_and_divides_the_cascade` (`:3802`) stays.
- **Keep the kernels `#[inline(always)]`.** A second arithmetic-carrying EQ symbol in the wasm
  artifact fails `KERNEL_ROSTER` rule 1.


## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, branch `codex/976-drop-eq-padding-section` from `15414bb6`
(`crates/parametric-eq` byte-identical to the brief's base `6ca203f8`). The verification amendments
on the GitHub issue (VERIFY-EQ.md) were applied as overriding the body: the counter is a
`cfg(any(test, feature = "test-support"))` hook exercised with `--features test-support` and
in-crate; gate 3 used a flag-free digest harness and mandatory wasm guest digests; five live
sections are tested with the dead section first, middle and last; a mono `kept == live` assertion
and mutation exist; `check-builtins-fixtures.sh` is not cited. Host: AMD EPYC 7313P (Zen 3),
`rustc 1.97.1`, `x86-64-v3`. Every build used `CARGO_INCREMENTAL=0` and the worktree's own
`target/`.

Commits: `e5fb8ac8` (gate 1 pinned on the unmodified base), `3e262f1f` (the change, the five pins,
the counter, gate 1 revision), `a1b4f5a7` (floor and mutation record), and this record.

### The change

- `cascade_sections` and `cascade_sections_mono`: the list is exactly the live sections in cascade
  order (`kept = live`); the rounding, the `depth` binding and the padding bookkeeping are gone.
  Every refusal leg is unchanged, and six live sections still return the full list before the scan.
- `interleave` and `interleave_mono`: `length / DEPTH` depth-2 passes as before, then, when
  `length % DEPTH == 1`, one depth-1 pass over `list[length - 1]`:
  `svf_cascade_interleaved::<L, 2, 1>` (mono `<L, 1, 1>`) when neither channel's `dry_mask(at)`
  has a lane set, otherwise `svf_cascade_interleaved_with_dry_masks::<L, 2, 1>` (`<L, 1, 1>`).
  `debug_assert_eq!(length % DEPTH, 0)` became `debug_assert!(length % DEPTH <= 1)`;
  `debug_assert_eq!(EQ_SECTION_COUNT % DEPTH, 0)` stays. `EFFECTIVE_CASCADE_DEPTH` is still 2 and
  still used by `process_channels`/`process_channels_mono`; nothing in `crates/lane`, the ramped
  path, the bind or `SVF_CASCADE_DEPTH` changed. No new field, no `unsafe`, no allocation; the only
  new render-path statement outside test builds is the tail branch.
- Test hook: `SELECT_FREE_TAIL_PASSES` in the existing `cfg(any(test, feature = "test-support"))`
  thread-local block, incremented only in the two select-free tail arms, read in-crate and through
  `test_only_select_free_tail_passes`/`test_only_reset_select_free_tail_passes`
  (`cfg(feature = "test-support")`), the `lib.rs:122-148` pattern.
- Doc comments of the four functions restate the list and tail rule; the elision proof gains one
  paragraph noting it is per dead section, so it already covered the padding.

### Gate 1: `odd_live_counts_render_the_base_bits` (`tests/bank.rs`)

Eight shapes x 8 tracks x 32 hostile blocks (every fifth block 37 frames), scalar (`W = 1`, one
effect per track), the native bank (`native_bank()`: one Simd8 bank here, two Simd4 banks on a
four-lane host) and the same bank through `process_bank_mono`:

| shape | live physical sections | how |
|---|---:|---|
| `HpfEverywhere` | 1 | HPF on both channels of every track through prepared targets (`support::prepare_targets`, the production route) |
| `HpfSomeLanes` | 1 | HPF on the left of even tracks at block 0, on the right of odd tracks at block 12: the tail has dry lanes |
| `BellOnly` | 1 | one general bell through `set_initial` (the console fixture) |
| `ThreeWithDryLpf` | 3 | HPF and a bell at prepare time, then an LPF on some lanes of each channel through prepared targets: the tail is the LPF with dry lanes |
| `ThreeGeneral` | 3 | low shelf, bell, high shelf through `set_initial` |
| `FiveDeadFirst` / `FiveDeadMiddle` / `FiveDeadLast` | 5 | the dead section is the HPF / band 3 (physical 3) / the LPF |

Input words are `+0.0` (1/16), subnormals of either sign (1/16) or normals with magnitude in
`2^-24..2^26`; block `8k + 3` carries one `-0.0` (tracks 1, 4, 7, 2, alternating planes) and block
`8k + 6` one non-finite word (inf, -inf, NaN, inf) on tracks 0 and 4 of one plane. A fault zeroes
and resets a whole bank plane, so faulting one track of every four-lane group makes the bank digest
independent of the bank width. Every output word, every report and every track's state payload after
every block is folded into one SHA-256 per leg (raw bits; a faulted block is zeroed, so no NaN
reaches an output word).

Pinned on the unmodified base (`lib.rs` at `15414bb6`), identical in dev and release:

| leg | digest |
|---|---|
| scalar | `81015a5c841e7fb53b6d7f4968c1706b46902db7055cdc687fa2d7c5366cb852` |
| bank | `247bc0b6fd53e45fe85f15dd481dc65eb6110bc3ca8cd6a056e40a6d9c131d7c` |
| bank-mono | `e904180499a49c1a203eef5da5f6a257b7e9a74c34a1e1ba22fdb7ebcb1e108a` |

After the change: the same three digests in dev and release, with and without
`--features test-support`. The first pin (`e5fb8ac8`: `e57233ea…`, `d6c46894…`, `3821224f…`, also
taken on the base) had every `-0.0` on the right plane, so the collapsed leg never saw one. That was
noticed from the tail counts after the change had already passed the first pins; the generator was
corrected, the three pins above were taken with `lib.rs` checked back out at the base, and the
change then reproduced them (see Deviations).

Select-free depth-1 tail passes per shape (`--features test-support`, in the table's order):
scalar `[236, 0, 244, 56, 244, 244, 244, 244]`, bank `[23, 0, 24, 0, 24, 24, 24, 24]`, bank-mono
`[27, 0, 28, 0, 28, 28, 28, 28]`. The bank's 23 for `HpfEverywhere` is 32 blocks less the ramp block
and the eight refused blocks; the five-live shapes, which never elided before (`kept = 6`), now elide
on every admitted block; the dry-lane tails run masked (0). The gate asserts the `HpfEverywhere`
count is non-zero on every leg (M3).

### Gate 2: in-crate

On the change alone, with the in-crate tests untouched, exactly the five listed tests failed, in dev
and in release (`the_two_channels_are_judged_together`, `the_shipped_shape_actually_elides`,
`a_section_live_on_one_lane_is_not_elided`, `a_negative_zero_input_refuses_elision`,
`a_non_finite_or_oversized_input_refuses_elision`); all 27 other lib tests and every integration
test passed. The new values are derived from the rule, not read from the failures. An admitted plan
keeps exactly the sections that are live on some lane of either channel; six live sections return
the full list; a refusal returns six. Both corpus cases are admissible (case 0 is
`u16 * 2^-15 - 1`, never `-0.0`; case 2 is an impulse of `1.0` then `+0.0`), and every state these
tests set is admissible. So:

| test | configuration | was (`ceil(live / 2) * 2`, capped at 6) | now (`live`) |
|---|---|---|---|
| `the_two_channels_are_judged_together` | every pair of 6-bit live masks at Simd4, `u = popcount(left \| right)` | `min(ceil(u / 2) * 2, 6)` | `u` (moves for `u` = 1, 3, 5) |
| `the_shipped_shape_actually_elides` | masks `000001`, `000011`, `000000`, `000111` | 2, 2, 0, 4 | 1, 2, 0, 3 |
| `a_section_live_on_one_lane_is_not_elided` | section 2 live on lane 5 of the left channel only: not dead, one live section | 2 (and `ran == 2`) | 1 (and `ran == 1`) |
| `a_negative_zero_input_refuses_elision` | mask `000001`; the `+0.0` arm is admitted (the `-0.0` arm still refuses: 6) | 2 | 1 |
| `a_non_finite_or_oversized_input_refuses_elision` | mask `000001`; `1e29` is below the `1e30` ceiling (the refusals still give 6) | 2 | 1 |

`an_elided_cascade_is_the_full_cascade_bit_for_bit` stays green over all 64 masks at `f32`, `Simd4`
and `Simd8`, cold and seeded, and now also asserts dual `kept == live` and runs a collapsed-mono arm
(`process_channels_mono` against the same channel's per-section path) asserting its bits,
integrators and mono `kept == live`. New: `an_odd_tail_without_dry_lanes_skips_the_select` (one and
three live general bands, dual then mono, at every width: exactly one select-free tail pass each).
`the_tuned_depth_is_per_backend_and_divides_the_cascade` is unchanged.

### Gate 3: rows

`cargo test --release -p console-workload --test chain_shape`: 23 passed on the base and after.

Digests: 64 blocks of every `WORKLOADS` row, before (`15414bb6`) and after (`3e262f1f`), with a
flag-free scratch harness (`SessionRuntime::build_with_dispatch(workload, PlanConfig::BASELINE,
backend)`, `render` then `hash_output` per block; the verifier's `zz_digests`, not committed).
Native: all 45 lines (15 rows x `Scalar`, `Simd4`, `Simd8`) identical before and after, and the
base lines equal VERIFY-EQ's `digests_base.txt`; each row's digest is the same at all three
backends. An x86 build binds no four-lane EQ bank, so native `Simd4` does not cover the browser
shape; the wasm guest does: `wasm-console-guest` built with `+simd128` in release at both commits
(modules `3b47f7c9…` and `68832217…`, so the new code is in the second), driven by a digest-only
Node 22.23.2 script (the verifier's source words and 64-block fold, no timing). All 15 guest digests
are identical before and after (12 distinct: rows 2 and 8, 9 and 10, 12 and 13 are twins, as
VERIFY-EQ recorded).

| # | row | native digest, before = after, every backend | wasm guest digest, before = after |
|---|---|---|---|
| 0 | `nine_track_baseline` | `e7c6ef01770ab7da98d4b793a6a817bd50a4dfe682321ecfc023ddc275285a80` | `9fbb1dc74fc186bca65e16516d4380a46f9cb92301ee66bd69590a9c06675a82` |
| 1 | `nine_track_ragged_strip` | `17613a3ab693d3f0dfc457b41f77669f880581fa19c433941a1ef2437684198a` | `f48d412ea357a6f9ff588ee9ca8e0f447d5893aee838dfd598b018fb4f839f3c` |
| 2 | `sixty_four_track_console` | `fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de` | `43574fedb0cabba717064dd7652965fa6dc5c63b6814358821b77f9dca15c28a` |
| 3 | `one_twenty_eight_track_stretch` | `cba2c94f81544caad0945f0720480b568b1a47808d25fd95911f61bd37f5f9b1` | `c8037a62c50d5a5ac1d6237595629e1a22a97203228e3636968d0f47f6f38a69` |
| 4 | `sixty_four_track_eq_only` | `9b2c56a1da62ebda8aef595973870ea077477d209aacdcd6321637e949d5c828` | `c239021bd3780786add6ce633247948b02a1d03f4f6d2726742ec46dc9d77628` |
| 5 | `sixty_four_track_compressor_only` | `95c9375429fbca3449bf9c5134508a6220f17fc9adda0b3267c061b75bb14175` | `52efa19187ecc71482dbcba9e6e0739b69612ed55d97d364dcf29380f3f03c49` |
| 6 | `sixty_four_track_builtins_only` | `b63eccd09c19eb7a6e0608144024ac5b14c7d5f7d1c56012cbbd49d6aad8f7f0` | `099e17c6bd28cb921475ec3985e7eb6148a56d301d17bd081e98e5e79e4c27e1` |
| 7 | `sixty_four_track_dispatch_only` | `15688888612d161e507bc400b9eed356fc1776797c8c66ca52d1e7c9114d3a2d` | `048b58ec0d0b86513a9782b253d6978e87181a9edb942532396a48eb4abe5f42` |
| 8 | `sixty_four_track_idle` | `de2f256064a0af797747c2b97505dc0b9f3df0de4f489eac731c23ae9ca9cc31` | `43574fedb0cabba717064dd7652965fa6dc5c63b6814358821b77f9dca15c28a` |
| 9 | `sixty_four_track_console_legacy` | `f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177` | `3b6dc4ce405703b612b6eb3310ca54cdb82b2139ded499deb8abc3b14c36e3e3` |
| 10 | `sixty_four_track_eq_comp_simd1` | `f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177` | `3b6dc4ce405703b612b6eb3310ca54cdb82b2139ded499deb8abc3b14c36e3e3` |
| 11 | `sixty_four_track_gain_pan_only` | `01e465a797036fb4267e895d9319a911bc108d554705d268d9a84a2e2e2dfdb4` | `dbe1e97b852b629a4eae2e8641ccdc0be785aff9651bc5b325acc547f412c667` |
| 12 | `sixty_four_track_console_mono` | `fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7` | `96c4860be4ff78e31a2bc8995d1224da4c75153eea42e13e019996e9b3a5d6bc` |
| 13 | `sixty_four_track_console_mono_dual` | `fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7` | `96c4860be4ff78e31a2bc8995d1224da4c75153eea42e13e019996e9b3a5d6bc` |
| 14 | `sixty_four_track_console_half_mono` | `4a656cdf63882999b720b7dd765c1b4f7bbcb95e2ab38c10445f71d269665180` | `7ec5e75f7927ef899840020ec9d615991f9d310b0e57e6f5425cda9b0eea7fa0` |

### Gate 4: mutations

Recorded in `crates/parametric-eq/tests/MUTATIONS.md` ("Issue #976"). Each alone, release,
`--features test-support`, `--lib --test bank`:

| # | mutation | red on |
|---|---|---|
| M1 | dual rounding and padding restored | dual `kept == live` (7 in-crate tests); gate 1 counter; gate 1 digests unchanged |
| M1m | the same in `cascade_sections_mono` only | mono `kept == live`, the in-crate counter (1, want 2); gate 1 counter (bank-mono) |
| M2 | dual tail before the pairs | gate 1 scalar `757d054f…` and bank `5d2fa266…`; in-crate bits |
| M2m | the same in `interleave_mono` | gate 1 bank-mono `7760302e…`; in-crate mono bits |
| M3 | dual tail always masked | gate 1 counter; in-crate counter; no bit moves |
| M3m | the same in `interleave_mono` | gate 1 counter (bank-mono); in-crate counter; no bit moves |

### Gate 5: browser artifact

Built with `scripts/build-web-audioworklet.sh`'s cargo line (`RUSTFLAGS="-C target-feature=+simd128
-C strip=debuginfo --remap-path-prefix=$CARGO_HOME=/cargo --remap-path-prefix=$repo=/repo" cargo
build --locked --release --target wasm32-unknown-unknown -p host-web`), at the base and after, and
`wasm-objdump -d` piped into `scripts/check-web-audioworklet-callgraph.py`. Not repinned. The base
artifact is `a3ab44f4…`, which already differs from the committed pin `8934cdd9…` (the pin predates
later batch commits and is repinned at the batch boundary); after is `dc01d9e5…`.

| check | base | after |
|---|---|---|
| `--callgraph miso_engine_web_v1_render` | closure=8 traps=5, one trap owner (`render_inner`) | identical |
| `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11` | ok, kernels=14, f32x4_arith=11725 | ok, kernels=14 (rule 3), f32x4_arith=11849 |
| rule 1, `parametric-eq f32x4 dual` (`12process_bank`) | one kernel, vector=240 scalar=0 | one kernel, vector=312 scalar=0 |
| rule 1, `parametric-eq f32x4 collapsed` (`17process_bank_mono`) | one kernel, vector=120 scalar=0 | one kernel, vector=156 scalar=0 |
| every other roster row | ok | unchanged counts |
| `--callgraph miso_engine_web_v1_meter_poll`, `command_submit --allocation-only` | ok | identical |

The growth is exactly two inlined depth-1 instantiations: the rule counts `f32x4.{mul,add,sub}`,
18 per section per stream (`sub` 1, `d1` and `d2` 3 each, `v1`, `v2` 1 each, the two state
additions 2 each, the mix 5), so the dual body gains 2 x (2 streams x 18) = 72 (240 -> 312) and the
collapsed body 2 x 18 = 36 (120 -> 156). No second EQ symbol appeared.

### Gate 6: floor

`EQ_LANE_OPS` (`tools/bench/src/floor.rs`) and `eq_lane_ops` (`scripts/console-benchmark-record-lib.jq`)
53 -> 27 (24 for the live section as a select-free depth-1 tail, 3 for the boundary scan); the pin
test now asserts 27 and the strip `(69 + 27 + 81.5 + 129.5)`. The ruling's EQ inventory states
`floor_lane_ops(active) = 25 * 2 * floor(active / 2) + 24 * (active mod 2) + 3` for an admitted
block, with the tail at 25 when either channel has a dry lane there (VERIFY-EQ finding 5), and 153
for a refused or all-live block, with a per-count table (27/28, 53, 77/78, 103, 127/128, 153).
`cargo test -p bench floor`: 9 passed. `bash scripts/test-console-benchmark.sh`: PASS.

### Gate 7: toolchain and the listed test runs

All on `a1b4f5a7` (the evidence commit changes only this file), `CARGO_INCREMENTAL=0`:

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| `cargo test --locked -p parametric-eq` (dev; and with `--features test-support`) | 102 passed, 2 ignored; the same |
| `cargo test --locked --release -p parametric-eq` (and with `--features test-support`) | 102 passed, 2 ignored; the same |
| `cargo test --locked -p effect-runtime` | 86 passed, 1 ignored |
| `cargo test --locked -p console-workload` | 39 passed, 2 ignored |
| `cargo test --locked --release -p console-workload --test chain_shape` | 23 passed |
| `cargo test --locked -p builtins-compiler --features test-support` | 79 passed |
| `cargo test --locked -p wasm-gates` | 9 passed |
| `cargo test --locked -p bench floor` | 9 passed |
| `bash scripts/check-lane-policy.sh` | `lane policy: ok` |
| `bash scripts/check-realtime-policy.sh` | `realtime policy: ok (57 marked regions in 16 files)` |
| `bash scripts/check-parametric-eq-render-contract.sh` | PASS |
| `bash scripts/test-console-benchmark.sh` | PASS (0 real runner invocations) |

### Gate 8: codegen (recorded)

Release `bench` binary (fat LTO), `<PreparedParametricEq<f32x8, 8> as PreparedNativeEffectBank>::process_bank`:

| loop | instructions | vector arithmetic | detail |
|---|---:|---:|---|
| depth-2 masked pass (unchanged) | 117 | 100 | `vaddps` 40, `vmulps` 28, `vandps`/`vcmplt_oqps`/`vandnps` 8 each, `vsubps` 4, `vblendvps` 4, 9 `vmovaps`, 4 `vmovups`, 1 `vbroadcastss`, 3 control |
| **depth-1 select-free tail (new)** | **56** | **48** | `vaddps` 20, `vmulps` 14, `vandps`/`vcmplt_oqps`/`vandnps` 4 each, `vsubps` 2, 4 `vmovups` (two plane loads, two stores), 1 `vmovaps`, 3 control |
| depth-1 masked tail (new) | 58 | 48 | the same arithmetic, the stores folded into 2 `vmaskmovps` (the #944 pattern the brief predicted; left for EQ-2) |

The select-free loop opens:

```text
41909f: vmovaps ymm14,ymm13
4190a4: vmovups ymm13,YMMWORD PTR [r10+rdx*1]
4190aa: vsubps  ymm15,ymm13,ymm7
4190ae: vmulps  ymm8,ymm15,ymm0
4190b2: vmulps  ymm9,ymm12,YMMWORD PTR [rsp+0x340]
4190bb: vaddps  ymm8,ymm9,ymm8
...     (56 instructions, 0x41909f-0x4191bf)
```

### Descriptive A/B (not a gate, no projected saving)

Scratch harness (not committed) in clean `git archive` copies of `15414bb6` and `a1b4f5a7`, release,
built outside the lock; each run one hold of
`flock -w 7200 …/scratchpad/timing.lock taskset -c 31 <binary>`, arms alternated three times. Per
run: 1,000 warm-up blocks, then six rounds of 1,500 blocks each of `sixty_four_track_eq_only` and
`sixty_four_track_builtins_only` at `Simd8`, median of the round p50s. Load average 3.8-3.9.

| round | base isolate | after isolate |
|---:|---:|---:|
| 1 | 15.33 us | 9.38 us |
| 2 | 15.63 us | 9.31 us |
| 3 | 15.34 us | 10.05 us |

The same shape as the prototype and the verification (15.0 -> 9.3 and 14.95 -> 9.36 us). The paired
console benchmark remains the batch boundary's.

### Deviations and notes for the verifier

1. **Outside the four functions in `lib.rs`:** the counter and its accessors beside the design-call
   hook (the brief's M3 and amendment 1 ask for it there) and `svf_cascade_interleaved` added to the
   `lane::kernels` import. `process_channels`' inline comment ("retaining one fixed interleaved
   kernel shape") was left untouched: it is outside the authorized functions and still true of the
   pair shape, but no longer of the whole schedule.
2. **Ruling edits outside the "EQ inventory" section:** only the statements that restate the same
   constant as current -- the #368/#805 authority list (EQ 27), the derived-floor table's EQ row
   (27, 0.912) and strip row (307.0, 10.372), the mono-rows paragraph's strip total (307), the
   standing-table note and the Boundary 3 note. Historical tables and their percentages are
   untouched. Leaving them at 53/333 would have had the ruling contradict `floor.rs`.
3. **Gate 1 was re-pinned once, on the base.** Its first version (`e5fb8ac8`) put every `-0.0` on
   the right plane (`block % 8 == 3` is always odd), so the mono leg never saw a refusal. The change
   had already passed those first pins when this was noticed; the placement was fixed, the new
   digests were taken with `lib.rs` checked out at `15414bb6`, in dev and release, and the change
   reproduces them. Both versions of the pins were derived on the base, never from the change.
4. **Three digests, not one.** The scalar and bank legs cannot share a digest (a fault zeroes a whole
   bank plane but one scalar track), so gate 1 pins one SHA-256 per leg (scalar, bank, bank-mono).
   The bank legs are width-independent by fault placement; on a host with no native bank only the
   scalar leg runs.
5. **Floor formula:** the tail is 24 select-free and 25 with a dry lane (VERIFY-EQ finding 5), so the
   inventory table gives both values; the standing fixture is 27.
6. **Five live sections now pay the elision scan** (they returned the full list unscanned before,
   `kept = 6`): exact, covered by gate 1's dead-first/middle/last shapes and the 64-mask sweep.
7. **Not done here:** the artifact repin (batch boundary), the paired console benchmark
   (`scripts/run-console-benchmark.sh` was not run), and the masked tail's `vmaskmovps` (EQ-2).
