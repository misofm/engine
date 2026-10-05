# Ramp a lane's detector link between modes

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-7, D15-13 E4).
Slice L3a of *Let a strip override a console slot's link mode* (#1236).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Each of the four linked effects (compressor, gate-expander, transient shaper, true-peak limiter)
can move one lane's detector link from one mode to another while it renders, over a given ramp,
with no click, without moving a bank-mate's bit, and, for the limiter, without any sample over the
ceiling. The engine entry point exists and is tested; *Carry the link record from the edit to the
lane* (#1371) connects it to edits.

## Context

- After #1368, each lane holds its own link mode, and each bank picks its arm per block from the
  lanes' modes.
- The link laws, in each crate's frozen operation order: `own = |x|`; `maximum = max(|l|, |r|)`;
  `average = 0.5|l| + 0.5|r|` (compressor `link_frame`, `crates/compressor/src/kernel.rs:316-339`;
  gate `kernel.rs:205-207`; transient shaper `link`, `crates/transient-shaper/src/lib.rs:286-300`;
  limiter, `maximum` only, through the per-lane `link` mask, `crates/true-peak-limiter/src/lib.rs:2741`).
- The detector feeds a smoothed gain path in each effect, so the output gain is continuous whatever
  the detector does. D15-1 still requires the switch itself to ramp.
- The limiter's ceiling proof needs `g[n] <= r_own[n - N]` on each channel (module doc,
  `crates/true-peak-limiter/src/lib.rs:11-24`). Its linked-pair fast path depends on the record
  `gain_linked` (`:3452`, decided per block at `:3588-3594`, re-established by `gain_state_agrees`).
- Ramps: `effect_runtime::ramp::LinearRamp` (`crates/effect-runtime/src/ramp.rs`) is the shared
  linear ramp with an exact final assignment; state payload helpers `ramp_path_within`,
  `read_f32`/`write_f32` (`crates/effect-runtime/src/state_payload.rs`).

## Decisions frozen for this slice

- **D1. Entry point.** `PreparedNativeEffect` and `PreparedNativeEffectBank` gain
  `retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool`, render-safe (no
  allocation, no lock, bounded). It returns `false`, changing nothing, when the descriptor's
  `lane_link` is false, the mode is not in `supported_link_modes`, or `track` is out of range. The
  default implementation returns `false`. It applies from the next frame the effect renders.
- **D2. Weights.** Each lane holds, per channel, two `LinearRamp`s: `w_link` (0 = own, 1 = linked
  combination) and `w_avg` (0 = maximum, 1 = average). At rest they are exactly 0.0 or 1.0 and
  equal the lane's mode. `retarget_link` sets both targets over `samples` (0 = a step). A retarget
  during a ramp starts from the current words.
- **D3. Blend.** On a frame where either weight of a lane is ramping, that lane's detector is
  `combined = max + w_avg * (avg - max)`, `d = own + w_link * (combined - own)`, as separate `mul`
  and `add` (no fused multiply-add), with `own`, `max` and `avg` computed in the crate's existing
  order. A bank with any ramping lane runs the blended arm for that segment; a lane at rest inside
  it takes the `select` form from its derived masks, so its bits are today's. When a ramp arrives,
  the lane's mode becomes the target and its masks are rederived; the bank's per-block arm choice
  (#1368 D4) reads the new modes.
- **D4. Limiter safety.** The limiter blends only `w_link` (it has no `average`). The blended peak
  is at least the own peak (`max >= own` and `0 <= w_link <= 1`), so each channel's required gain
  stays at or below its own, and the ceiling proof holds in every window that mixes modes.
- **D5. Limiter agreement.** On a switch into `maximum`, `gain_linked` is false. After the ramp
  ends and the lane's ring has been fully rewritten under `maximum` (a per-bank sample counter that
  starts at the switch), the limiter re-establishes `gain_linked` with `gain_state_agrees`, once.
  The fast path never runs on disagreeing words.
- **D6. Payload (D15-7).** Each lane's state payload gains its mode and the two ramps (4 words
  each), so a swap carries a link in flight. Restore validates the ramps with `ramp_path_within`
  and the mode against the descriptor. `STATE_LAYOUT_VERSION` stays 1 (nothing persists state; the
  word-count header refuses a stale length). These codecs are stream A's code in the same crates:
  coordinate with the #1279/#1280 owner.

## Effect evidence (AGENTS.md list)

- **Equations:** D3. **Update rule:** linear in the weights over `samples`, exact assignment on
  arrival. **Stability:** the detector is a convex combination of nonnegative magnitudes, so it
  stays in `[own, max(own, max)]`; no recursive coefficient changes. **Latency, tail:** unchanged.
  **Smoothing:** the ramp; the effect's own ballistics follow. **NaN/denormal:** the weights are in
  `[0, 1]` and finite; a NaN detector input is handled as today (D8 clamps, D7 boundary check).
- **Citations:** Giannoulis, Massberg and Reiss, JAES 60(6), 2012 (detector and link);
  ITU-R BS.1770-5 Annex 2 (true peak).
- **Listening:** a blinded A/B of a `dual_mono` to `maximum` switch over the default ramp against
  a step, on a wide stereo bus, recorded in the PR.

## Deliverables

1. D1 in `crates/effect-contract/src/lib.rs`.
2. D2-D6 in the four effect crates.
3. Tests per gate, and an `f64` reference of D3's blend in `crates/dsp-reference`.

## Authorized paths

- `crates/effect-contract/src/lib.rs` (the two trait methods)
- `crates/compressor/`, `crates/gate-expander/`, `crates/transient-shaper/`,
  `crates/true-peak-limiter/` (`src/` and `tests/`)
- `crates/dsp-reference/src/`

## Non-goals

- The control record, the classifier, any host path (#1371).
- The multiband compressor (#1367).

## Objective gates

1. **Blend matches the reference.** Per effect, at `Scalar`, `Simd4` and `Simd8`: a switch in each
   supported direction over 64 samples, started mid-block, on a two-channel signal whose channels
   differ by 20 dB. The detector words match the `f64` reference of D3 within the crate's existing
   SIMD/scalar tolerance, and from the arrival frame on, each detector word equals the target
   mode's law (the `select` form) bit for bit.
2. **Bank-mates keep their bits.** A full bank where one lane switches and the others rest or ramp
   parameters: every other lane is bit-identical to its scalar instance.
3. **Limiter ceiling.** Every switch direction, at every ramp length in {0, 1, 64, 4800}, on a
   full-scale fixture with intersample peaks: no output sample's true peak exceeds the ceiling.
4. **Limiter agreement returns.** After a switch into `maximum` on a uniform bank, the linked-pair
   path (test-support counter) runs again within one ring length after the ramp ends, and output
   is bit-identical to the per-lane body over the same blocks.
5. **Payload.** A lane snapshotted mid-switch and restored renders bit-identically to the
   uninterrupted one; a forged weight outside `[0, 1]` is refused.
6. **Realtime.** `retarget_link` and a switching block make zero allocations and frees
   (`bench_support::alloc` thread counters, statics warmed).
7. Commands:
   - `cargo test --locked --all-targets -p effect-contract -p compressor -p gate-expander -p transient-shaper -p true-peak-limiter -p dsp-reference --features math/lane,lane/test-support`
   - the touched effects' `KERNEL_ROSTER` rows; the browser legs as in #1368
   - `bash scripts/check-cross-targets.sh`; `scripts/run-aarch64-tests.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
     `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: a blend in the wrong order, or a lane that keeps the old mode after arrival, turns it red.
- Gate 2: a bank-wide ramping arm that blends resting lanes too moves their bits; it turns red.
- Gate 3: a blend that can fall below a channel's own peak overshoots the quieter channel; it
  turns red. No existing test switches a limiter's link.
- Gate 4: an agreement record never re-established loses the fast path for good (counter stays 0);
  one re-established too early renders the mirror on disagreeing words (bits differ).

## Dependencies

- *Lower the link mode to per-lane state in the linked effects' banks* (#1368).
