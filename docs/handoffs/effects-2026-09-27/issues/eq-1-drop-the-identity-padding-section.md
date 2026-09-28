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
