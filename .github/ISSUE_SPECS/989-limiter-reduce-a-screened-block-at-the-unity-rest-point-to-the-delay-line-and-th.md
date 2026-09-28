# Limiter: reduce a screened block at the unity rest point to the delay line and the release


Limiter slice 2 of 5 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `6ca203f8`).
Depends on `limiter-1` (the screen). Evidence:
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md`, section 4 item 2. The prototype is
`V_UNITY` in `docs/handoffs/effects-2026-09-27/limiter-diagnosis-prototypes.patch`. It is
evidence only; do not commit it.

## Product outcome

After `limiter-1`, a screened block still runs the whole gain loop with `r = 1.0` for every frame.
That loop is 4.4-5.0 cycles per lane-sample at `Simd8` and 11.3 under V8. Once enough screened
frames have passed, every required-gain word and every box word is exactly `1.0`. The frame's ring
stages then write `1.0` over `1.0` and read back `1.0`, so they change nothing:

* the required-gain store;
* the van Herk prefix, window minimum and backward pass;
* the quantiser;
* the box store, its running sum (`+1 - 1`, exact on the `2^-14` grid) and its divide
  (`Wb / Wb = 1.0`).

What remains is the delay line and the release recursion with `target = +0.0`:

```text
d' = flush(max(+0.0, c * (+0.0 - d) + d))      (the frozen step 6 with 1 - s = +0.0)
y  = select(bypass, z, z * (1 - d'))            (step 7)
```

This slice makes that reduction a block-level fast path. It follows the model of the #182 S2
silent fixed point (`LimiterCore::process_block`, `crates/true-peak-limiter/src/lib.rs:2184-2233`):
earn the claim by observing the state, keep it by induction, and withdraw it on anything that could
break it.

Measured on the prototype, which implemented only the `d == +0.0` arm (kernel harness, 64 tracks,
timing lock, cpu 31):

* quiet material: **`Simd8` 11.65 to 1.38 cycles per lane-sample (-88 %)**, **`Simd4` 20.8 to
  1.98 (-90 %)**;
* standing console fixture under V8: **-8.4 (-31 % of the limiter)**; together with `limiter-1`,
  `-3`, `-4` and `-5`: -10.7 (-40 %);
* never engaged: +0.11 (`Simd8`), +0.23 (`Simd4`).

The prototype's `d == +0.0` condition does not hold for seconds after any limiting event: the
release takes about 45 time constants to flush to `+0.0`, which is 2.7 s at 60 ms. On a material
mix of one loud block in 32, the prototype therefore never engaged. This slice specifies the
release arm above, which is valid while `d` decays. These figures are descriptive only.

## Invariants

* **Class A.** Every output word, snapshot payload and report is unchanged, at `f32`, `Simd4` and
  `Simd8`, on every target.
* Taken only on a block that `limiter-1`'s screen admits, in the uniform stationary dual body,
  while the claim holds. The mono, per-lane and ramping paths are untouched.
* **The claim.** For every lane of both channels, `required_ring` and `box_ring` are all exactly
  `1.0`, `prefix` is exactly `1.0`, and `box_sum` is exactly `Wb` (bit compares, the
  `all_exactly_one` rule, `:675`).
  * It is **earned** by observation: a full scan of those words (`is_at_unity_rest`, below) after a
    screened block. The scan is tried only after `2R + 2Wb` consecutive screened frames, then every
    `R` frames while it fails.
  * It is **kept** by induction: a unity block writes nothing to those words, and the full body
    would write the same bits.
  * It is **withdrawn** on:
    * an unscreened block;
    * any automation span (the #182 S2 rule, `:3126`);
    * a bypass change;
    * `reset` and `restore_track`;
    * a collapsed (mono) block, whose right channel is then stale;
    * `desymmetrize`;
    * the §4.4 reset.
* The claim is not serialized. After a restore it is re-earned.
* `reduction`, `history`, `main_ring`, the van Herk `phase` and the cursors advance exactly as the
  full body advances them. The phase advance is `(phase + frames) mod Wb` per cohort.
* Allocation-free, no `unsafe`, no layout change.

## Interface contract

1. `ChannelState::is_at_unity_rest(&self) -> bool`, beside `is_at_silent_rest` (`:643`). It is
   the same word list minus `history`, `main_ring` and `reduction`, which carry audio or decay.
2. `LimiterCore` gains `unity_frames: u32` (consecutive screened frames, saturating) and
   `unity_rest: bool`. Both are reset at every withdrawal point listed above.
3. A frame body `unity_frame_uniform::<L>` that does only the delay-line read and write, the
   release recursion and the output, in the frozen order of steps 6-7, with `target = L::zero()`.
   The body `limiter_block_uniform` dispatches to it when screened and the claim holds. The phase
   and cursor advance and `history_after` come from `limiter-1`.
4. The recursion keeps `release.fma(target.sub(d), d)` literally. `+0.0 - d` is `-d` exactly and
   `+0.0 - (+0.0)` is `+0.0`, so there is no shortcut to take and none may be taken.

## Smallest closable slice

Authorized paths: `crates/true-peak-limiter/src/lib.rs`, `crates/true-peak-limiter/tests/screen.rs`
(extended), `crates/true-peak-limiter/tests/MUTATIONS.md`, and this spec.

Steps:

1. **On the base with `limiter-1` merged:** extend gate 5 with the scenarios below, and pin its
   digest.
2. Add contracts 1-4.
3. Run the gates and record the evidence.

## Non-goals

* A claim across ramps, the ragged body, or mono.
* Folding the delay line into the upstream chain (no copy removal beyond what the body does).
* Any change to the release law.

## Objective gates

1. **Identity.** Unity against forced-full, comparing output words and every track's snapshot
   after every block, in dev and in release:
   * widths `f32`, `Simd4`, `Simd8`, with bypass on and off;
   * block lengths `{1, 5, 11, 12, 13, 31, 32, 33, 64, 127, 128}`;
   * scenarios:
     * (a) quiet from the start;
     * (b) one loud block, then quiet, at releases 10, 60, 500 and 2,000 ms, so that `d` decays
       through the unity body;
     * (c) sparse loud blocks, 1 in 16 and 1 in 32;
     * (d) a ceiling retarget during a claimed run;
     * (e) a restore of one track mid-run;
     * (f) the §4.4 reset triggered by an injected NaN.

   NaN output words are compared as "both NaN".
2. **Witness.** A `#[cfg(test)]` counter of unity blocks. It engages in (a) and in (b) before `d`
   reaches `+0.0`. It never engages before the first successful scan, and never on the block of a
   withdrawal.
3. **Digests.** Unchanged: D90 (`tests/determinism.rs`, `wasm-gates` G5), every console
   workload's digest, and `limiter-1`'s pinned scenario.
4. **Mutations**, each recorded red:
   * M1: claim without the scan (counter only). Red in (b) or (c).
   * M2: keep the claim across an automation span. Red in (d).
   * M3: `target = 1.0 - 1.0` computed from a stale `box_sum`, or skip the `flush`. Red in (b).
   * M4: do not advance the phase. The snapshot is red.
   * M5: keep the claim across `restore_track`. Red in (e).
5. **Realtime and wasm.** As `limiter-1` gates 8 and 9. The full body's loops keep their
   instruction counts.

## What the implementer will hit

* **Why earn by a scan and not by counting.** The ring words settle within `2R + 2Wb` screened
  frames. A backward pass writes suffix minima into the next `Wb` cursor slots, which the cursor
  overwrites with `1.0` within `Wb` frames. The scan makes the claim a statement about observed
  state rather than about this argument, exactly as `is_at_silent_rest` does for #182 S2.
* **`is_at_silent_rest` is not reusable.** It requires `main_ring` and `history` at `+0.0`, which
  audio never is.
* **The screen's history-tap rule matters here too.** The first quiet block after a loud one is
  not screened.

