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
5. **Realtime and wasm.** `tests/allocation.rs`, `check-realtime-policy.sh`, and
   `check-web-audioworklet.sh`: roster, rule 3 and callgraph unchanged. Scalar loads are not
   counted arithmetic.

## What the implementer will hit

* **`remaining` is gathered as `f32` through `u16`** (`:816`) and scattered back through `as u32`
  (`:838`). The round trip is exact only because `remaining <= 64`. The stationary path simply
  stops doing it.
* **The window splat relies on `lanes_uniform`.** Assert it (`debug_assert!`) at the top of the
  body.

