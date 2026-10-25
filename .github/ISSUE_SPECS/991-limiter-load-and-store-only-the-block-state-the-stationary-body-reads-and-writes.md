# Limiter: load and store only the block state the stationary body reads and writes


Limiter slice 4 of 5 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `6ca203f8`; every
`file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md`, sections 1 and 4 item 4. The prototype is
`V_LEANSTATE | V_NOSCATTER` in `docs/handoffs/effects-2026-09-27/limiter-diagnosis-prototypes.patch`.
It is evidence only; do not commit it.

## Product outcome

Every block, each channel runs `HotChannel::load` (`crates/true-peak-limiter/src/lib.rs:1318`) and
`HotChannel::store` (`:1336`). In the release `bench` binary these are out-of-line functions of
5.5 KB and 2.3 KB. They do four things:

* gather all four fields of both ramps lane by lane through `[f32; 8]` scratch, with a `u16` to
  `f32` conversion per lane (`RampLanes::gather`, `:807`);
* build the window vector lane by lane;
* move the whole `HotChannel`, about 600 bytes, through memory;
* scatter three ramp fields back (`RampLanes::scatter`, `:828`).

In the stationary dispatch (`DISPATCH_STATIONARY`) the frame body reads only `current` of each ramp
(`RampLanes::resting_value`, `:869`) and never writes a ramp word. The scatter therefore writes
back the bits it gathered.

The block-state term (state in and out, excluding the boundary scan and the call) is:

* about 0.50 cycles per lane-sample at native `Simd8` (about 1,000 cycles per bank-block);
* about 0.55 at `Simd4`;
* about 1.3 under V8.

The prototype loads only `current` of the two ramps and splats the uniform window, inline. It skips
the scatter in the stationary dispatch. Measured in process, timing lock, cpu 31:

* **the state term**: -0.28 (`Simd8`), -0.26 (`Simd4`), -0.41 (V8) cycles per lane-sample;
* the whole kernel: -0.2 to -0.5 (`Simd8`), -0.2 to -0.36 (`Simd4`), -0.07 to -0.47 (V8);
* it also helps the worst case.

These figures are descriptive only.

## Invariants

* **Class A.** Every output word, ramp word, snapshot payload and report is unchanged.
* Only the stationary dispatch of the uniform dual and mono bodies changes:
  * `limiter_block_uniform::<DISPATCH_STATIONARY, _>` (`:1925`);
  * `limiter_block_uniform_mono::<DISPATCH_STATIONARY, _>` (`:3313`).

  The ramping dispatch, the runtime oracle (`#[cfg(test)]`, `:1721`) and the per-lane body keep
  `HotChannel::load`/`store` as they are.
* Allocation-free, no `unsafe`.

## Interface contract

1. `HotChannel::load_stationary(state: &ChannelState) -> HotChannel<L>`, `#[inline(always)]`:
   * `history`, `reduction` and `box_sum` as today;
   * `window` as `L::splat(state.lane[0].window as f32)`, since `lanes_uniform` holds on this path;
   * `limit.current` and `release.current` gathered from the lanes' `current`;
   * `target`, `step` and `remaining` set to `L::zero()`. The stationary body never reads them; a
     `debug_assert!` in the body states this.
2. `HotChannel::store_stationary(self, state)`, `#[inline(always)]`: `history`, `reduction` and
   `box_sum` only. No ramp scatter. `ramps_are_stationary` held at entry, and no stationary frame
   advances a ramp.
3. The two stationary bodies call contracts 1 and 2. Every other caller keeps `load`/`store`.
4. In `process_bank_inner` (`:3113`), skip the per-lane `apply_automation` walk when
   `block.automation.is_empty()`. With no spans, the function reads nothing and writes nothing.

## Smallest closable slice

Authorized paths: `crates/true-peak-limiter/src/lib.rs` (the functions named above and the tests
module), `crates/true-peak-limiter/tests/MUTATIONS.md`, and this spec.

Steps:

1. **On the base:** a scenario test (gate 2) and its pinned digest.
2. Contracts 1-4.
3. The gates.

## Non-goals

* The ramping dispatch's gather and scatter.
* Moving the window or ramp words into a different layout.
* The peak scratch's per-block zeroing (2 KB of stack). It is measured-neutral here, and a heap
  scratch did not help.

## Objective gates

1. **Existing identity gates stay green, in dev and in release:**
   * `stationary_dispatch_matches_runtime_oracle_and_observes_selected_body` (`:5020`);
   * `a_uniform_cohort_renders_exactly_the_per_lane_path` (`:4605`);
   * `the_stationary_hoist_reads_what_advancing_would_have_produced` (`:4089`);
   * `automation_retargets_linear_coefficients_and_counts_invalid_spans` (`:6006`);
   * `tests/determinism.rs`, `tests/mono_collapse.rs`.
2. **Scenario pinned on base.** 96 blocks at W8 and W4, comparing every output word and every
   track's payload per block. It goes stationary, then a ceiling retarget whose 64-update ramp
   spans two blocks, then stationary again, then a release retarget on one lane only. This checks
   that the ramp words a stationary block leaves are bit-identical, including a ramp that just
   finished.
3. **Digests.** D90, `wasm-gates` G5 in release, every console workload's 64-block digest.
4. **Mutations**, each recorded red:
   * M1: use `load_stationary` in the ramping dispatch. Gates 1 and 2 go red.
   * M2: have `store_stationary` scatter the lean `RampLanes` back, writing its zero `target`. Gate
     2's payload comparison goes red.
   * M3: skip `apply_automation` when spans exist. The automation test goes red.
5. **Realtime and wasm.** `tests/allocation.rs`, `tools/realtime-policy`, and
   `check-web-audioworklet.sh`: roster, rule 3 and callgraph unchanged. Scalar loads are not
   counted arithmetic.

## What the implementer will hit

* **`remaining` is gathered as `f32` through `u16`** (`:816`) and scattered back through `as u32`
  (`:838`). The round trip is exact only because `remaining <= 64`. The stationary path simply
  stops doing it.
* **The window splat relies on `lanes_uniform`.** Assert it (`debug_assert!`) at the top of the
  body.


## Decision and evidence record (2026-10-25, attempt 1)

* **Scope note wins** (root scope 2026-10-24): authorized paths are `lib.rs` and this spec only.
  Gate 2 (a new pinned scenario) and the M2 mutation are dropped: the restored-ramp hole is closed
  by `ramp_path_inside`, and the stationary scatter wrote back the exact bits it gathered on every
  state (`remaining == 0` at rest is exact through `u16`/`f32`), so skipping it is equivalent.
  No new test; the existing identity gates carry every claim.
* **Link glide (#1370b) checked.** A block's glide frames render through `limiter_block_glide`
  (per-lane body, `DISPATCH_RAMPING`); only the frames after the glide reach
  `limiter_block_linkable`, whose `dual_stationary` test runs after the glide advanced the ramps.
  The uniform bodies read no link weight. So no glide frame takes the lean load.
* **Shipped shape.** `HotChannel::load_stationary` / `store_stationary`, called from both uniform
  bodies under a `const` branch on `DISPATCH == DISPATCH_STATIONARY`; the linked copy of the right
  ramps runs only in the advancing dispatches. `process_bank_inner`'s per-lane automation walk runs
  only when the block carries spans (it shares the existing `silent_fixed_point` branch).

### Measurement (2026-10-25, descriptive)

One A/B, base `3730046f5` (origin/main) against candidate `3a409df87`, each one invocation of
`scripts/operator/run-console-benchmark.sh` (one warmup, two rounds, validator PASS, cpu 31), back
to back in one exclusive window; then one V8 A/B with `run-web-mixing-automation-benchmark.sh`.
The host was shared: both runs are `uncontrolled` (loadavg 1-min 5.9 -> 1.9 over the native window,
6.2 -> 3.7 over the V8 window). Every output digest agrees, base against candidate, on all 39
native rows and both V8 records' six documents and seven arms.

Noise: rows with no limiter (`console_legacy`, `eq_only`, `builtins_only`, `compressor_only`,
`gain_pan_*`, `idle`, `dispatch_only`) moved -0.3 % to +1.0 %. Native Simd8 p50, mean of two rounds:

| row | base us | candidate us | delta |
|---|---|---|---|
| `sixty_four_track_console` | 100.12 | 98.15 | -1.96 % |
| `sixty_four_track_console_mono` | 61.05 | 60.30 | -1.23 % |
| `sixty_four_track_console_mono_dual` | 101.36 | 99.05 | -2.28 % |
| `sixty_four_track_console_metered` | 105.81 | 104.22 | -1.50 % |
| `sixty_four_track_console_sends` | 171.40 | 168.53 | -1.68 % |
| `one_twenty_eight_track_stretch` | 200.46 | 196.45 | -2.00 % |

Every limiter row moved -1.2 % to -2.5 %, outside the non-limiter band. V8 (p50 ns): the six
console documents -0.6 % to +0.1 %, the mixing-automation arms +0.1 % to +0.7 %: within noise.
Close rule (more than 2 % slower) not met; recommendation: ship.
