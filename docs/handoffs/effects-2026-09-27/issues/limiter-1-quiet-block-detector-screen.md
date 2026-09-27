# Limiter: skip the Annex-2 detector on blocks whose taps cannot reach the ceiling

Limiter slice 1 of 5 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `6ca203f8`; every
`file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md`, sections 1 and 4. The prototype is
`V_SCREEN` in `docs/handoffs/effects-2026-09-27/limiter-diagnosis-prototypes.patch`. That patch is
evidence only and must not be committed.

## Product outcome

The Annex-2 detector (`detector_chunk`, `crates/true-peak-limiter/src/lib.rs:1750`, called twice
per 32-frame chunk from `limiter_block_uniform`, `:1925`) is half of the limiter's cost:

* 6.0-6.5 of 11.5-12.0 cycles per lane-sample at native `Simd8`;
* 11.7-12.2 of 20.5-21.2 at `Simd4`;
* 14.1 of 26-28 under V8.

Its output is used in exactly one place: the required gain
`r = select(P > limit, limit / P, 1)` (`:1635`). On most blocks of a mix, no peak comes near the
ceiling, and `r` is exactly `1.0` for every frame.

This can be decided exactly, before the detector runs. Every phase of the Annex-2 table satisfies
`|P| <= ||h_p||_1 * max|x|`, and the largest per-phase L1 norm of `ANNEX2_FIR` (`:319`) is
2.022827 (phases 1 and 2). So if every tap the block's detectors will read is below
`0.49 * limit`, then `fl(P) < 0.9915 * limit` even after the twelve roundings. Also
`|h[6]| < limit`, so `r == 1.0` bit for bit. The taps are the twelve history words and every input
frame, of both channels, on every lane.

On such a block, skip both detector passes, use `r = 1.0` directly, and set the history to the
block's last twelve inputs.

Measured on the prototype (kernel harness, 64 tracks, in process, timing lock, cpu 31):

* quiet material, engaged on every block:
  * `Simd8`: 11.8 to 6.1-6.4 cycles per lane-sample (**-48 %**);
  * `Simd4`: 20.5-20.8 to 9.7-10.0 (**-51 to -53 %**);
* the standing console fixture, which sits at the threshold:
  * native `Simd8` +0.8 (it never engaged);
  * native `Simd4` -2.6;
  * **V8 -4.3 to -4.9 (-17 %)**;
* never engaged (+3 dBFS noise): +0.55. That is the unscreened body's code moving, not the scan,
  which is why contract 4 below keeps today's body untouched.

These figures are descriptive only. The paired console benchmark runs at the batch boundary.

## Invariants

* **Class A.** Every output word, every state word (every snapshot payload) and every report is
  unchanged, at `f32`, `Simd4` and `Simd8`, on every target.
* The screen is taken only in the uniform-cohort, stationary dual body: the
  `lanes_uniform && stationary` arm of `limiter_block` (`:1695-1700`). The per-lane body, the
  ramping dispatch and the collapsed mono body (`limiter_block_uniform_mono`, `:3313`) are
  untouched.
* The decision is once per block, whole bank, both channels: never per lane, never per frame. It
  reads data, like the #182 S2 silence admission (`:2203-2209`), and it is exact rather than
  heuristic.
* A NaN or an infinity in any tap declines, by the ordered `lt` (`NaN < t` is false).
* Render stays allocation-free, lock-free and syscall-free. There is no `unsafe`, and only `lane`
  names `wide`. The state layout (`STATE_LAYOUT_VERSION`) and the serialized payload do not change.

## Interface contract

1. `crates/true-peak-limiter/src/lib.rs`, beside `ANNEX2_FIR`:

   ```rust
   /// Largest per-phase L1 norm of `ANNEX2_FIR` is 2.022827; taps strictly below
   /// `SCREEN_FACTOR * limit` bound every rounded phase below `0.9915 * limit`.
   const SCREEN_FACTOR: f32 = 0.49;
   ```

   The doc derives the margin: `0.49 * 2.022827 * (1 + 13 * 2^-24) * (1 + 2^-24) < 0.9915`. That
   covers the γ₁₃ bound of a 12-term dot product and the rounding of `limit * 0.49`.
2. A private whole-bank predicate, `#[inline(always)]`, per channel:

   ```rust
   fn taps_below<L: Lane>(history: &History<L>, io: &[f32], threshold: L) -> bool
   ```

   It ANDs `abs(tap) < threshold` over the twelve history words and every frame of `io`, and
   returns `!mask_any(mask_not(ok))`. It exits early after every 16 frames once a lane fails.
   `threshold` is `min(limit_left, limit_right) * SCREEN_FACTOR` per lane. Take the minimum of the
   two channels because under `LinkMode::Maximum` each channel compares the linked peak against
   its own limit.
3. `fn history_after<L: Lane>(history: &History<L>, io: &[f32]) -> History<L>`: the history
   `frames` calls of `History::shift` would leave. Taps 0-11 are the last twelve frames when
   `frames >= 12`; otherwise shift the old history.
4. `limiter_block` (`:1680`), in the `lanes_uniform && stationary` arm: evaluate the screen once,
   before any write to `io`, and then dispatch:
   * `true`: `limiter_block_uniform::<DISPATCH_STATIONARY, L, true>`;
   * `false`: `limiter_block_uniform::<DISPATCH_STATIONARY, L, false>`.

   The new `const SCREENED: bool` parameter is threaded to `channel_frame_uniform` (`:1621`). With
   `SCREENED = false`, both functions are today's code, token for token. With `SCREENED = true`,
   the body:
   * runs no detector pass;
   * does not read the peak scratch;
   * takes `required = L::splat(1.0)` in place of step 1 of the frozen order (`:1635`);
   * runs steps 2-7 unchanged;
   * stores `history_after(..)` into `hot_*.history` before the gain loop overwrites `io` in
     place.
5. Test support: a `#[cfg(test)]` counter `screened_blocks` on `LimiterCore`, on the model of
   `silent_engagements` (`:2119`). Also a `#[cfg(test)]` switch that forces the unscreened body,
   which is the oracle for gate 3.

## Smallest closable slice

Authorized paths:

* `crates/true-peak-limiter/src/lib.rs`: contracts 1-5 and unit tests in its `tests` module;
* `crates/true-peak-limiter/tests/screen.rs` (new): gates 3 and 5;
* `crates/true-peak-limiter/tests/MUTATIONS.md`;
* this spec.

Steps:

1. **On the unmodified base:** write gate 5's scenario test, run it with the digest left empty,
   record the printed digest in this spec's evidence, and pin it.
2. Add contracts 1-3 with their unit tests (gates 1 and 2).
3. Add contract 4 and the witness (contract 5).
4. Run the gates and record the evidence.

## Non-goals

* The unity fast path that also skips the gain rings (`limiter-2`, which builds on this).
* Screening the ramping dispatch, the per-lane (ragged) body, or the mono body. Each is a
  successor, because a ramping limit needs a range bound.
* Changing the detector arithmetic, the gain law, the floor inventory or `floor.rs`.

## Objective gates

1. **Bound.** A unit test recomputes each phase's L1 norm from `ANNEX2_FIR` in `f64` and asserts
   `SCREEN_FACTOR as f64 * max_norm * (1 + 13 * 2^-24) * (1 + 2^-24) < 1`.
2. **Worst-case fixture.** For each phase `p`, feed taps `x_k = t * sign(h[k][p])` (the pattern
   that attains the norm), with `t` the largest `f32` below the threshold. Assert, through the
   forced-unscreened oracle, that the peak is below the limit and `r == 1.0`, and that the
   screened block renders the oracle's bits.
3. **Identity.** Screened against forced-unscreened, from fresh banks, comparing every output word
   and every track's snapshot payload after every block, in dev and in release:
   * widths `f32`, `Simd4`, `Simd8`;
   * links `Maximum` and `DualMono`, bypass on and off;
   * block lengths `{1, 5, 11, 12, 13, 31, 32, 33, 64, 127, 128}`;
   * signals:
     * quiet noise at 0.45, 0.48 and 0.495 times each lane's limit;
     * samples exactly at the threshold;
     * a loud block, then a quiet one (the history tap case);
     * `-0.0`, subnormals, and `2^-24` to `2^0`;
     * one NaN and one infinity injected in one lane of one channel;
     * per-lane ceilings that differ, including left and right differing on one lane.

   NaN output words are compared as "both NaN", every other word by bits.
4. **Engagement witness.** The counter increments on a quiet block, and does not increment for:
   * one loud sample in one lane of either channel;
   * a loud history tap alone;
   * a ceiling ramp in flight;
   * a ragged cohort;
   * a collapsed (mono) block.
5. **Scenario pinned on base.** A bank-API scenario of 96 blocks at W8 and at W4. It alternates
   quiet and loud blocks and retargets one lane's ceiling at block 40. It pins one SHA-256 of every
   output word and every block's payloads, recorded on `6ca203f8` in step 1.
6. **Digests.** Unchanged:
   * `tests/determinism.rs`, all five D90 cases at every width;
   * `cargo test --release -p wasm-gates` (G5 replays the same pins);
   * every console workload's 64-block digest: the `chain_shape.rs` pins, and the before and
     after outputs of the digest harness from the 944 patch, run in a scratch copy.
7. **Mutations**, each alone, each recorded red in `MUTATIONS.md`:
   * M1: `SCREEN_FACTOR = 0.51`. Gates 1 and 2 go red. Gate 2's phase-1 pattern then peaks at
     1.03 times the limit.
   * M2: drop the history taps from the screen. Gate 3 goes red on the quiet block after a loud
     one.
   * M3: `max` instead of `min` of the two channels' limits. Gate 3 goes red with asymmetric
     ceilings under `Maximum`.
   * M4: do not update the history on a screened block. Gates 3 and 5 go red.
   * M5: screen during a ramp. Gates 3 and 4 go red.
8. **Realtime and wasm.**
   * `tests/allocation.rs`, `bash scripts/check-realtime-policy.sh`, `check-lane-policy.sh`.
   * `bash scripts/check-web-audioworklet.sh`: the roster row "true-peak-limiter f32x4 dual" still
     matches exactly one function, the rule-3 kernel count does not drop, and the render callgraph
     is unchanged.
9. **Codegen (recorded).** In the release `bench` binary, today's detector loop and uniform frame
   loop keep their instruction counts: 133 per channel-frame and 223 per frame at `Simd8`, ±2. Quote
   both loops. The prototype that shared one body cost +0.55 cycles per lane-sample when not
   engaged.

## Console benchmark rows

* **Moves:** `sixty_four_track_console` and the limiter-carrying mono rows. The effect differs by
  width, because the fixture's levels sit at the threshold.
* **Must not move, bit or unit:** every row without a limiter.

## What the implementer will hit

* **Today's body must not change.** A shared body with a runtime `bool` perturbed LLVM's register
  allocation of the unscreened loops (+0.55). Keep the screened body a separate monomorphisation.
* **`io` is overwritten in place** by the gain loop (`:1665`). The screen and `history_after` must
  read it first.
* **The silence path runs first** (`:2210`). A silent block never reaches the screen.
* **`History` is newest-first**: `t0` is `x[n]` and `t11` is `x[n-11]` (`:1013`).
* **The limit is per lane and per channel.** Read `hot.limit.current` (stationary: current equals
  target).
* **The corpus calls `limiter_block` with 1,024-frame blocks** (`src/corpus.rs:246`). The screen
  and `history_after` must be correct for any `frames`.
