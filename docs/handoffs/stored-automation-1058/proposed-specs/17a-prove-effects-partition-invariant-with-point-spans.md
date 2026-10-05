# Prove every launch effect partition-invariant with Point spans

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.6) and A5, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R3.

## Product outcome

Every launch effect is proven to render the same bits when a `Point` span arrives at a sample in
the middle of a quantum, in a block that starts at that sample, as when every sample is a block
of its own: per node, and in banks at Simd4 and Simd8 against the scalar oracle. Stored effect
automation (drafts 17b and 18a) processes an effect node in pieces that start at event samples,
and A1.6 relies on this property. Today no test stages a `Point` at a mid-quantum sample, so the
property is unproven. This slice changes the conformance harness only. It lands in batch R3,
with its first user, draft 17b.

## Context

**The contract.**
- A `Block` parameter accepts only a `Point` at the block's `first_sample`
  (`crates/effect-contract/src/lib.rs:1496-1499`; `docs/EFFECT_CONTRACT_V1.md:149-155`).
- A block may be shorter than the quantum (`EffectProcessBlock::new`, `:1299`;
  `validate_automation_block`, `:1473`).
- A bank block holds exactly `frames x lanes` words per plane (`EffectBankProcessBlock::new`,
  `:1359-1397`). The planes are AoSoA, so frames `[a, b)` of every lane are the contiguous words
  `[a·W, b·W)`.

**What the tests cover today.**
- The conformance probe renders each effect in blocks of 1, `q - 1` and `q` frames with no spans
  (`latency_and_partition_probe`, `crates/conformance/src/effect.rs:848-900`; `render_sequence`
  passes `&[]` at `:816`). Every launch effect runs it through `effect_conformance_test!` in its
  `tests/conformance.rs`.
- The randomized bank differential renders some scalar blocks in two pieces (`render_scalar`,
  `crates/conformance/src/randomized.rs:1883-1927`), but it draws a `Block` span only at the
  block's first sample (`draw_spans`, `:1743`, the `Block` arm at `:1765`). So the second piece
  never receives a `Point`.

**How each launch effect takes spans and cuts a block** (read at HEAD):

| Effect | Span entry | Block cutting |
|---|---|---|
| compressor | `apply_automation`, `crates/compressor/src/lib.rs:403-458`: per-lane `LinearRamp` retarget | ramp prefix and settled body; partition test with spans at sample 0 (`tests/partition.rs:59`) |
| gate-expander | `apply_automation`, `crates/gate-expander/src/lib.rs:534-593` | split at the bank-wide `ramp_frames_left`; the no-ramp variant omits only no-op updates (`run_block`, `:597-604`) |
| soft-clip | `apply_automation`, `crates/soft-clip/src/lib.rs:591-650`: converts, then ramps | segments at ramp snaps (`:489-537`) |
| transient-shaper | `apply_automation`, `crates/transient-shaper/src/lib.rs:743-800` | per-sample advance over the ramp prefix (`:541-585`) |
| true-peak-limiter | `apply_automation`, `crates/true-peak-limiter/src/lib.rs:3816-3870`: ramps of linear coefficients, 64 per-sample updates | 32-frame detector chunks with carried history (`:87`, `:2755`) |
| delay | `apply_automation`, `crates/delay/src/lib.rs:1407-1477`: a time `Point` sets `pending_delay` | a crossfade starts only at a chunk start (`:1279-1284`; `begin_transition`, `:504-509`); chunks end where a crossfade ends (`chunk_frames`, `:821-841`) |
| parametric-eq | counts every span invalid (`crates/parametric-eq/src/lib.rs:3513-3521`) | takes prepared targets only (`apply_target_lane`, `:2860-2894`) |
| multiband-compressor | `apply_automation`, `crates/multiband-compressor/src/lib.rs:1228-1279` | splits at bank-wide ramp arrivals (`process_block`, `:956-1012`) and refreshes ratio, attack and release coefficients once per segment from the ramps' current values (`BandCache::refresh`, `:608-617`, called at `:1107`) |

- **The multiband compressor is not partition-invariant with spans.** Its per-segment
  coefficients depend on where block boundaries and other lanes' ramp arrivals cut a segment. That
  is #1069. The differential narrows around it (`Known::RampCutsMoveBits`,
  `crates/multiband-compressor/tests/randomized.rs:36`; ignored reproducer `:43`).
- The other six span-driven effects advance every ramp per sample through `LinearRamp` (or a lane
  form of it) and apply spans only at a call's first sample. By reading, a `Point` at a piece
  start is the same operation as a `Point` at a block start. This slice proves it.
- The delay's time crossfade cannot restart. A time `Point` that lands while a crossfade runs
  takes effect at that crossfade's end, which is a chunk boundary in pieces and in 1-frame blocks
  alike.
- The EQ stages no span. Its stored events are prepared targets at piece starts (draft 20), so this
  slice proves it with targets.

## Decisions frozen for this slice

- **D1. The per-node probe.** `partition_with_points_probe` in `crates/conformance/src/effect.rs`,
  beside `latency_and_partition_probe`, run by `effect_conformance_test!` for every effect:
  - It renders the reference input with a fixed schedule of `Point`s on every `Block` cell of the
    descriptor, at offsets that are not block starts at `q = 128` (for example 1, 37, 63, 64, 100,
    127, 129, 191), spaced closer than the smoothing length so ramps are cut in flight. Values
    alternate between two legal values of each parameter's domain.
  - Form (a): 1-frame blocks, each `Point` in the block of its sample. Form (b): blocks cut
    exactly at the `Point` samples, quantum blocks otherwise. Form (c): form (b) plus a cut at every
    `q - 1` boundary.
  - The three outputs equal bit for bit and the three final snapshot payloads byte for byte, or the
    failure `process.partition_invariance_points`. Every report counts zero invalid spans.
  - An effect with target preparation (the EQ) takes the same schedule as prepared targets from
    its own `prepare_targets`, applied with `apply_prepared_target` at the same samples.
- **D2. The bank differential.** In `crates/conformance/src/randomized.rs`, a chunked block may
  also draw `Block` `Point`s at `first + cut`. The scalar oracle stages them in its second piece.
  The bank renders the same two pieces through `process_bank` (and `process_bank_mono` when the
  scenario collapses), with per-lane offsets per piece, at every width the build binds. The
  coverage report gains a count of blocks chunked with a mid-block `Point`, and `assert_reached`
  requires it nonzero at each bound width.
- **D3. No effect changes here.** A failure of D1 or D2 in an effect is that effect's defect. It
  gets its own issue, as #1069, and this slice records the failure and waits; it never narrows the
  harness around a new defect.

## Deliverables

1. D1 in `crates/conformance/src/effect.rs`.
2. D2 in `crates/conformance/src/randomized.rs`.
3. The multiband compressor's `known` list empty (after #1069).

## Authorized paths

- `crates/conformance/src/effect.rs`, `crates/conformance/src/randomized.rs`.
- `crates/multiband-compressor/tests/randomized.rs` (only to remove a `known` entry that #1069
  leaves).

## Non-goals

- The engine's piece loop (draft 17b). Compiling cells (draft 18a).
- Any change to an effect crate's source (D3).
- Sample-rate parameters: no launch descriptor has one.

## Hazards

- **#1069.** The multiband compressor fails D1 and D2 until #1069 lands. This slice depends on it.
- **The EQ's targets.** The EQ refuses a target whose `enabled` or `kind` differs from the prepared
  one until #1337 (`apply_target_lane`, `crates/parametric-eq/src/lib.rs:2869-2883`). D1's schedule
  for the EQ changes only numeric values.
- **Probe cost.** D1 renders three forms per prepared configuration. Keep the schedule to a few
  quanta so the per-effect conformance tests stay inside the `test-debug-b` budget.

## Objective gates

1. **Per node** (D1). Every launch effect's `tests/conformance.rs` passes with the new probe:
   `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   (the `test-debug-b` step).
2. **Banks against the scalar oracle** (D2). Each effect's `tests/randomized.rs` passes at full
   strength, with the new count nonzero at each bound width, in the same command.
3. **The probe can fail (PR evidence).** Plant a change that applies a `Point` one sample late in
   one effect: gate 1 turns red with `process.partition_invariance_points`; revert.
4. **Policy.** `bash scripts/check-conformance-boundaries.sh && bash scripts/test-conformance-boundaries.sh`;
   `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1 turns red if an effect applies a `Point` at a mid-quantum block start differently from one
  at a quantum start, or if a cut inside a ramp moves a bit. No test stages a `Point` at a
  mid-quantum sample today.
- Gate 2 turns red if a bank applies a lane's mid-block `Point` to another lane, or if a bank's
  piece render differs from its scalar instances'. The differential compares only whole-block spans
  today.

## Dependencies

Batch R3. Direct dependencies:

- *Multiband compressor: a ramp's cut moves a lane's bits in a bank* (#1069).

Draft 17b *Process an effect node in pieces at automation events* depends on this draft and lands
in the same batch.
