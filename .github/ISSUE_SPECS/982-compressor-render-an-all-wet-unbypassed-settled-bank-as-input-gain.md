# Compressor: render an all-wet, unbypassed settled bank as `input * gain`


Compressor slice 2 of 5 (research 2026-09-27, base `6ca203f8`; builds on slice 1). Evidence:
`docs/handoffs/effects-2026-09-27/COMPRESSOR-DIAGNOSIS.md`. The prototype is mode 5 of
`compressor-diagnosis-prototypes.patch`, which applies on top of
`compressor-diagnosis-harness.patch`. Neither patch may be committed.

## Product outcome

`mix` defaults to 1.0 (`crates/compressor/src/lib.rs:202-214`), and every track of the standing
fixture uses it. On such a lane the output law `gain_mix` (`crates/compressor/src/kernel.rs:372`)
still computes, per channel-frame:

* the parallel-mix arm, `gain_mix_step`: a subtract, a multiply and an add;
* `smoothed == 0`, masked with `makeup == 0`;
* two mask-ors;
* two selects.

Nine vector operations produce what `input * gain` already is. This slice decides once per block,
from the settled coefficient masks, that no lane of either channel needs anything but the wet arm,
and renders `input * fast_gain_from_db(smoothed + makeup)` alone.

Measured on the prototype, in process, pinned (EPYC 7313P), on top of slice 1:

* compressor-only minus builtins-only, 64 tracks:
  * native `Simd8`: **-15 %** (40.2 to 34.1 us per block);
  * wasm under V8: **-12 %** (72.6 to 64.2 us);
* kernel: 69.1 to 59.6 (`Simd8`), 65.6 to 58.0 (`Simd4`) and 60.3 to 52.4 (V8) cycles per
  channel-frame.

These figures are descriptive only.

## Invariants

* **Class A, stated precisely.** Every output word whose input is not a NaN is unchanged. An
  output word whose input is a NaN is still a NaN, but its payload may differ: x86 quiets a
  signalling NaN in `x * 1.0`, and wasm may canonicalise it. Every block that contains a NaN
  output is rejected by `finish_channel` (`crates/effect-runtime/src/bank.rs:264`). The
  rejection zeroes that channel and resets its state. The output, the recursive words, the reports
  and the rejection lane mask are therefore identical at the effect boundary. The recursive word
  is never affected, because it does not read the output.
* **Why it is exact.** Suppose every lane has `mix == 1` and the block is unbypassed. Then
  `wet_identity` is all-true and `dry_mix_zero` all-false, so
  `dry_identity = (smoothed == 0) & (makeup == 0)`. When that holds:
  * `smoothed` is `+0.0`, because `flush` maps `-0.0` to `+0.0`;
  * `makeup` is `±0`;
  * so the gain is `fast_gain_from_db(+0.0)`, which is exactly `1.0`. This is the sealed tier's
    stated identity (`crates/math/src/fast_db.rs`, "Exactness at the identity points");
  * and `x * 1.0 == x` for every non-NaN `x`, including `-0.0`, subnormals and infinities.
* The arm is chosen once per block, never per frame, in the settled body only: both channels'
  `Coef` masks, and `bypass`. The ramping prefix and every `Silent`/`Sidechain` block keep the
  general law.
* No new field on `Channel`, `Coef` or `Instance`. Render stays allocation-free.
* `process_block::<Simd4>` stays the one roster function (`#[inline(always)]`).

## Interface contract

`crates/compressor/src/kernel.rs` only.

1. `#[inline(always)] fn all_lanes<L: Lane>(m: L::Mask) -> bool { !L::mask_any(L::mask_not(m)) }`.
2. In `settled_main` (slice 1), compute once:

   ```rust
   let wet = !bypass
       && all_lanes::<L>(coef_left.wet_identity)
       && all_lanes::<L>(coef_right.wet_identity);
   ```

   Then run the loop monomorphised on a `const WET: bool`.
3. Move `gain_mix`'s `fast_gain_from_db(smoothed.add(coef.makeup))` into a private
   `#[inline(always)] fn applied_gain<L>(smoothed, coef) -> L`, and move the existing
   `FAST-DB-CROSSING X2` comment and `#[expect(clippy::disallowed_methods, ...)]` onto that
   function. The seal is `clippy.toml:81-82`; `scripts/check-fast-db-seal.sh` is retired. Then:
   * `gain_mix` calls `applied_gain`;
   * a new private `#[inline(always)] fn settled_output<L, const WET: bool>(input, smoothed, coef,
     inv) -> L` returns `input.mul(applied_gain(smoothed, coef))` for `WET`, and
     `gain_mix(input, smoothed, coef, inv)` otherwise.

   The crossing count stays at eight sites, and the operation order of `gain_mix` is unchanged.
4. A `#[cfg(test)]` thread-local counter `SETTLED_WET_BLOCKS`, which the wet arm increments once
   per block (the witness pattern of `FILTER_PREFIX_KERNEL_FRAMES`, `crates/builtins`).

## Smallest closable slice

Authorized paths:

* `crates/compressor/src/kernel.rs`, including `settled_body_tests`;
* `crates/compressor/tests/MUTATIONS.md`;
* this spec.

Steps:

1. **On the base of this slice (slice 1 landed)**, add gate 2's scenario with its digest empty.
   Record the digest in dev and in release, then pin it.
2. Add the arm (contract 1-4).
3. Gates, then the evidence record.

## Non-goals

* A `mix == 0` or bypass arm, which is output equal to input. They are rare, and it would be a
  separate slice.
* A makeup-zero arm, which would drop `+ makeup`. It saves one operation, and only when makeup is
  zero on every lane.
* The two-pass body (slice 3) and the DualMono arm (slice 4).

## Objective gates

1. **Old law is the oracle.** Extend slice 1's `settled_body_tests` gate at `f32`, `Simd4` and
   `Simd8` with parameter sets on which the arm is taken: every lane `mix = 1`, makeup in
   `{0, +0, -0 normalised, 3, -12}`, ratio in `{1, 4, 20}`, knee in `{0, 6}`, and `bypass = false`.
   * Compare `settled_main` with `frames_loop::<L, false>` word for word on the hostile input.
     **Compare NaN words as "both NaN"**, because the arm may re-quiet a payload. Compare every
     other word, and both recursive words, by bits.
   * Then apply `finish_channel` to both sides. Assert the zeroed outputs, the reset states and the
     returned lane masks are equal by bits.
   * Run in dev and release.
2. **Scenario pinned on base.** The existing corpus never takes the arm at `Simd4` or `Simd8`:
   its track table mixes `mix` values (`corpus.rs:71-79`), so only `f32` lanes 0 and 7 are
   all-wet. Add a 24-block `Simd4`/`Simd8` scenario of eight heterogeneous all-wet tracks, taken
   from the standing fixture's first eight compressors (threshold -6 to -16.5, ratio 1.5 to 6.75,
   makeup 0 to 3). Fold outputs and recursive words into one SHA-256, pinned to the digest recorded
   on the base of this slice.
3. **Dispatch witness.**
   * `SETTLED_WET_BLOCKS` increments once per settled block when every lane is wet.
   * It does not increment when one lane has `mix = 0.999`, when `bypass` is true, when only the
     right channel has a non-wet lane, or in a block's ramping prefix.
   * A block with a ramp that ends mid-block toward `mix = 1` on every lane increments it for the
     tail.
4. **Existing gates stay green.** In particular `identity::unit_mix_is_the_wet_signal_exactly`
   (sidechain, so the general law), `identity::unity_gain_stage_is_the_dry_signal`,
   `identity::bypass_preserves_exact_dry_bits_at_sample_zero`, `nonfinite` (all five),
   `cross_target`, `lane_identity`, `partition`, `oracle`.
5. **Console digests.** All 15 standing workloads, at both dispatch widths, are unchanged. The
   list is slice 1's gate 4.
6. **Mutations.** Apply each alone, record it red, and revert it:
   * M1: the arm ignores `bypass`. `identity::bypass_preserves_exact_dry_bits_at_sample_zero` goes
     red at `f32`, and gate 1 goes red.
   * M2: the arm is taken when any lane is wet (`mask_any`). Gate 1 goes red on a mixed bank.
   * M3: the arm drops `+ makeup`. Gate 2 and `oracle` go red.
   * M4: the arm is never taken. Gate 3 goes red. This is the only gate that sees a
     performance-only regression.
   * M5: only the left channel's mask is tested. Gate 3's right-channel case goes red.
7. **Browser artifact, toolchain and policy.** As slice 1's gates 6 and 7. The clippy run is
   what enforces the fast-dB seal. It must pass with no new `#[expect]` for the tier: there are
   still exactly eight `FAST-DB-CROSSING` sites.

## Console benchmark rows

As slice 1. The primary rows are `sixty_four_track_compressor_only` (native and wasm) and
`sixty_four_track_console`.

## Dependencies

Slice 1 (`settled_main`).

## Standing rules for the implementer

As slice 1. Class A is the precise statement under Invariants, and "both NaN" is the only
permitted relaxation of bit comparison.

## What the implementer will hit

* **The dry-identity select looks necessary. It is not, for this arm.** Its only job on an all-wet
  lane is `G == 0 && makeup == 0`, where the wet product is already `x * 1.0`. Read "Why it is
  exact" before arguing otherwise. The identity is a property of the form of `fast_exp2`, not of
  its coefficients, so a refit cannot break it.
* **sNaN inputs are where the kernel words differ.** Gate 1 must not compare NaN payloads by bits.
  The boundary check is what makes the difference unobservable. Assert that too (gate 1, second
  half).
* **Keep the decision per block.** A per-frame test, or a mask cached in a new struct field, is
  the stale-cache hazard slice 1 avoided.

## Research findings

* On the standing fixture, mode 1 to mode 5 (prototype) measured 40.2 to 34.1 us natively and
  72.6 to 64.2 us under V8. The replicas agreed (`r_wet` against `r_base`: 68.0 to 59.7 cycles
  per channel-frame natively, 69.2 to 59.2 under V8).
* The base x86 loop spends 9 of its 76 vector ALU operations per channel-frame on the mix and
  identity law. The wet arm keeps one of them. The rest of the saving comes from a shorter chain
  after `fast_exp2`.


## Attempt 1 evidence

Implementer attempt 1, 2026-09-27, on top of #981 (`aa3c0d27`), branch
`codex/981-compressor-settled-body`. The verification amendments were followed: M3 names
`cross_target`, not `oracle`; `all_lanes` is named `every_lane` (a `lane::kernels::builtins::
all_lanes` already exists with another meaning); the relaxation's dependence on the callers is
stated at the arm.

### The change

`crates/compressor/src/kernel.rs` only.

* `applied_gain(smoothed, coef)` is `fast_gain_from_db(smoothed + makeup)` and now carries the
  `FAST-DB-CROSSING X2` comment and the one `#[expect(clippy::disallowed_methods)]`. `gain_mix`
  calls it; its operation order is unchanged. The workspace still has exactly eight crossings
  (X1-X8) and no new `#[expect]` for the tier; clippy, which enforces the seal, passes.
* `settled_output::<L, WET>` returns `input.mul(applied_gain(..))` for `WET` and `gain_mix(..)`
  otherwise. `every_lane(mask) = !mask_any(mask_not(mask))`.
* `settled_main` decides `wet = !bypass && every_lane(left.wet_identity) &&
  every_lane(right.wet_identity)` once per block, from the `Coef` it loads after the ramp prefix,
  and runs `settled_frames::<L, WET>`. The frame loop calls `curve_target`, `ballistic` and
  `settled_output` in `one_frame`'s order. No new field on `Channel`, `Coef` or `Instance`.
* `#[cfg(test)] thread_local SETTLED_WET_BLOCKS`, incremented once per block the arm runs.

### The NaN relaxation, for the owner's acknowledgement

Every output word whose input is not a NaN is bit-identical. An output word whose input is a
signalling NaN may come out as a different NaN on the arm (`x * 1.0` quiets it on x86, and wasm
may canonicalise it); the base's dry-identity select returned the sNaN unchanged. Every block
holding such a word is rejected by `finish_channel` (`lib.rs:544`, `:597`), which zeroes the
channel and resets its state, and the mask depends on NaN-ness only, so output, state, reports and
masks are identical at the effect boundary. This rests on every production caller
(`Instance::render`, `render_mono`) applying `finish_channel` before the output leaves the effect;
the arm's doc says so and that a new caller exposing kernel output must use the general law. The
frozen 013 identity ("`G == 0` and makeup `+0` returns `z` exactly for any mix") therefore now
holds at the effect boundary rather than at the kernel word. **This needs the owner's explicit
acknowledgement**; it cannot be avoided without re-adding the select (3 of the 9 saved operations).

Measured: in release the three randomized differentials took the arm in 13,547 (`f32`), 9,728
(`Simd4`) and 10,617 (`Simd8`) settled blocks and saw 307, 2,004 and 5,304 NaN-payload
differences, every one in a block the boundary check rejected (asserted per block), and none in an
accepted block. Only makeup-zero tables produce them (the dry-identity case); the grid saw 93 on
each of the makeup `0` and raw `-0.0` tables and none on the others.

### Gates

* **Gate 1** (`the_all_wet_arm_is_the_base_body_on_all_wet_tables`): #981's grid (`f32`, `Simd4`,
  `Simd8`; every link mode; both `bypass`; every detector; starts 0, 1, 18, 40; the hostile input)
  on all-wet tables with makeup `0` (which is also what the parameter layer delivers for `-0`), a
  raw `-0.0` word, `3` and `-12`, the lanes crossing ratio `{1, 4, 20}` with knee `{0, 6}`, and on
  the fixture's first eight compressors. NaN words compare "both NaN" only where the witness says
  the arm ran; the masks, finished planes and state after `finish_channel` compare by bits, and a
  payload difference must be in a rejected channel. The differentials apply the same rule. Dev
  and release.
* **Gate 2** (`scenario_982_all_wet_render_is_pinned`): 24 all-wet heterogeneous blocks (the
  standing fixture's tracks 0-7), `Simd4` and `Simd8`, DualMono and Average, hostile input; NaN
  words fold canonically before the boundary check and by bits after it. Recorded on B0
  (`197db1c9`) and on this slice's base (#981): `cd2d5b11da315893f13e5585bf82fcbb71f9026046497626544a6573ed97c8cf`,
  dev and release; unchanged after the arm. #981's digest is unchanged.
* **Gate 3** (`the_all_wet_arm_is_taken_exactly_when_every_lane_is_wet`, `f32`, `Simd4`, `Simd8`):
  once per settled all-wet block of 1, 7, 32 and 128 frames; never with `bypass`, with one left
  lane at mix 0.999, with only the right channel non-wet, or in a block that is all ramping
  prefix; once for the tail of a block whose ramp to mix 1 on every lane ends at frame 24.
* **Gate 4**: `identity` (6), `nonfinite` (5), `cross_target`, `lane_identity`, `partition`,
  `oracle`: green. `cargo test --locked -p compressor`: 90 passed in dev and in release.
* **Gate 5**: all 30 console digests identical to B0.
* **Gate 7**: browser artifact rule 3, roster (compressor dual matches one function: vector 350,
  scalar 0), `meter_poll`, `command_submit` and boot budget PASS; artifact 3,351,106 bytes (+2,377
  over #981, +5,491 over B0). fmt, workspace clippy, rustdoc, the other crates' tests, the wasm
  gates and the policy scripts: see the table below.

| gate | command | result |
|---|---|---|
| toolchain | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| compressor | `cargo test --locked -p compressor`, and `--release` | 90 passed, 0 failed, each |
| other crates | `-p effect-runtime` (86), `-p console-workload` (39), `-p builtins-compiler --features test-support` (79) | all passed |
| wasm | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` (113 passed); `bash scripts/run-wasm-gates.sh` | PASS, 0 mismatches on the native, wasm and wasm `simd128` legs |
| policy | `check-lane-policy.sh`, `check-realtime-policy.sh`, `check-unfused-seal.sh` | PASS |

### Mutations (gate 6)

`MUTATIONS.md`, section "#982": M1-M5 all red (9, 7, 7, 5 and 5 red tests). M4 is caught by gate 3
and by gate 1's dispatch assertion; no bit-exactness test can see it.

### Codegen (recorded)

x86 `Simd8` release: the wet settled loop is 195 instructions per frame (3 scalar), against 220
for the general settled loop in the same function (and 239 for B0's settled loop).

### A/B

Measured once as the set #981-#985 against a separately built B0; see #985's evidence.
