# Limiter: compute a linked stereo pair's gain path once


Limiter slice 3 of 5 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `6ca203f8`; every
`file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md`, section 4 item 3. The prototype is
`V_LINKED` in `docs/handoffs/effects-2026-09-27/limiter-diagnosis-prototypes.patch`. It is
evidence only; do not commit it.

## Product outcome

Under `LinkMode::Maximum`, both channels feed the same linked peak `max(p_R, p_L)` to their gain
computers (`limiter_block_uniform`, `crates/true-peak-limiter/src/lib.rs:2022-2024`). Assume each
lane's two channels also carry the same:

* limit and release ramps (all four fields);
* lookahead;
* gain state (`required_ring`, `box_ring`, `prefix`, `box_sum`, `reduction`, `phase`).

Then the two gain paths of `channel_frame_uniform` (`:1621`) compute bit-identical words: the same
operations on the same operands. That covers steps 1-6 of the frozen order and the gain `1 - d`.

This is the standing fixture (`channel: "both"` on every limiter parameter) and the usual stereo
setting. Today the kernel computes that path twice per frame, including two of the frame's four
`vdivps` and the whole van Herk and box traffic of the right channel.

This slice computes it once on the left words. It mirrors every ring write into the right channel,
so the right channel's state stays bit-identical to what a dual run would leave. It then applies the
one gain to both channels' delay lines. Snapshots, observation taps, the silent claim and the mono
collapse see no difference.

Measured on the prototype (in process, timing lock, cpu 31):

| row | native `Simd8` | native `Simd4` | V8 `simd128` |
|---|---:|---:|---:|
| kernel harness, 64 tracks | **-1.2 to -1.5** (-10 to -12 %) | -2.2 to -2.5 | - |
| console minus eq_comp_simd1 | -1.1 to -1.2 | -1.8 to -2.2 | **-3.7 to -5.5 (-14 to -19 %)** |

The figures are cycles per lane-sample. The same saving holds on material that limits every block
(-1.5 at `Simd8`), so this change also lowers the worst case. These figures are descriptive only.

## Invariants

* **Class A.** Every output word, snapshot payload, observation reading and report is unchanged, at
  `f32`, `Simd4` and `Simd8`, on every target. The right channel's gain words are written, not left
  stale. Only the arithmetic that produced them is shared.
* **The linked-agreement invariant.** A new `LimiterCore` field, `gain_linked: bool`, with this
  rule: if it is true, then for every lane the two channels' gain words (listed above) are
  bit-equal.
  * It is **established** in three places:
    * after `reset` and at construction, if and only if every lane's `LaneShape` agrees across the
      channels (`clear_runtime`, `:604`, then writes equal words);
    * after `desymmetrize` (`:2359`), which copies left to right;
    * after `restore_track` (`:2783`), by a full comparison of the gain words of every lane, on the
      control path.
  * It is **kept** by every dual uniform block that renders linked, or renders dual with the
    designed words agreeing under `Maximum`.
  * It is **cleared** by:
    * a dual block whose designed words disagree on any lane;
    * any collapsed block (`process_block_mono`, `:2303`; the right channel goes stale until
      `desymmetrize`);
    * any `restore_track` whose comparison fails.
* **Per-block engagement.** All of the following must hold:
  * `coef.link_max`;
  * `gain_linked`;
  * `lanes_uniform` on both channels;
  * the designed words agree on every lane: `LaneShape` equal, and `limit`/`release` ramps equal in
    `current`, `target`, `step` and `remaining`, by bits, like `designed_channel_symmetry`
    (`:2955`).

  The check is once per block (about 90 word compares per bank) and never per frame.
* The ragged per-lane body, the mono body and `DualMono` banks are untouched.
* Allocation-free, no `unsafe`, no state-layout change. `copy_state_from` needs no change.

## Interface contract

1. `fn designed_gain_agree(left: &ChannelState, right: &ChannelState) -> bool`. It covers all
   lanes and uses bit compares.
2. `fn sliding_minimum_uniform_mirrored<L: Lane>(ring, mirror, ..)`. It is `sliding_minimum_uniform`
   (`:1195`) verbatim, with every `store_ring_lane` of the backward pass also applied to `mirror` at
   the same slot. The reads come from `ring` only.
3. `fn linked_frame_uniform<L: Lane>(..)`. It runs one frame of steps 1-6 on the left
   `HotChannel`/`UniformHot`, in today's order and operands:
   * `required` is stored into both rings;
   * the box term is stored into both box rings;
   * the minimum comes from contract 2;
   * `gain = 1 - d` is formed once;
   * step 7 runs for the left channel and then for the right channel, each on its own
     `main_ring` and its own `x`.
4. In `limiter_block_uniform` (`:1925`), one whole-block branch picks the linked segment loop. The
   left ramps are advanced with `ramp_values`. In the ramping dispatch the right ramps are advanced
   too, or copied at block end; they started equal. At block end the right channel's `box_sum`,
   `reduction`, `prefix` and `phase` are set from the left before the write-back.
5. `gain_linked` maintenance, as in the invariants, with the three establishing sites.

## Smallest closable slice

Authorized paths: `crates/true-peak-limiter/src/lib.rs`; `crates/true-peak-limiter/tests/linked.rs`
(new); `crates/true-peak-limiter/tests/MUTATIONS.md`; this spec.

Steps:

1. **On the base:** write gate 3's scenario and pin its digest.
2. Contracts 1-5.
3. The gates, and the evidence.

## Non-goals

* Sharing the gain path under `DualMono`, where it is not the same.
* The ragged body, the mono body, or a lazy right channel. Skipping the mirrored stores would move
  the snapshot and is a separate design.
* Any floor change. The linked-pair count is an owner ruling (diagnosis section 7).

## Objective gates

1. **Identity.** Linked against forced-dual (a `#[cfg(test)]` switch), comparing every output word
   and every track's snapshot after every block, in dev and in release:
   * widths `f32`, `Simd4`, `Simd8`;
   * links `Maximum` and `DualMono` (the latter must never engage);
   * bypass on and off;
   * block lengths `{1, 5, 11, 12, 13, 31, 32, 33, 64, 127, 128}`;
   * signals:
     * quiet;
     * +3 dBFS noise, limiting;
     * L/R-asymmetric material, with one channel loud and one quiet.

   NaN output words are compared as "both NaN".
2. **Engagement witness.** A `#[cfg(test)]` counter. It engages on the fixture-shaped bank. It does
   not engage:
   * under `DualMono`;
   * with one lane's left ceiling different;
   * on the block a one-channel retarget lands and every block after;
   * after a restore whose left and right sections differ;
   * on a collapsed block.

   It re-engages after `reset` and after `desymmetrize`.
3. **Scenario pinned on base.** A bank-API scenario, 128 blocks at W8 and W4:
   * limiting noise;
   * a ceiling retarget of both channels at block 30, as a left span and a right span with equal
     values in one block (it stays linked);
   * a left-only release retarget at block 60 (it disengages);
   * a `reset` at block 90 (it re-engages).

   One SHA-256 of all output words and per-block payloads, recorded on `6ca203f8`.
4. **Existing gates unchanged:**
   * D90, including the `maximum/noise` case, in `tests/determinism.rs` and `wasm-gates` G5 in
     release;
   * `tests/mono_collapse.rs`;
   * `tools/console-workload/tests/chain_shape.rs`, in particular
     `a_live_one_channel_retarget_disengages_on_the_block_it_lands` (`:1275`) and
     `re_equal_designed_words_after_a_one_channel_retarget_never_re_engage` (`:1493`);
   * every console workload's 64-block digest.
5. **Mutations**, each recorded red:
   * M1: skip mirroring the backward pass into the right ring. Gate 1's snapshots go red.
   * M2: skip mirroring the box store. Gate 1 goes red.
   * M3: engage under `DualMono`. Gate 1 goes red.
   * M4: do not clear `gain_linked` on a collapsed block. The `chain_shape` transition tests go
     red.
   * M5: compare only `current` of the ramps. Gate 2's retarget case goes red.
6. **Realtime and wasm.** `tests/allocation.rs`, `check-realtime-policy.sh`, `check-lane-policy.sh`,
   and `check-web-audioworklet.sh`: the roster row matches exactly one function, the rule-3 count
   does not drop, and the callgraph is unchanged.

## What the implementer will hit

* **Mirroring is what keeps this class A without touching the payload.** The saving comes from not
  recomputing (two divides, two minima, the quantiser, the recursion), not from not storing.
* **The right channel's ramps must still advance in the ramping dispatch.** Their words are
  serialized.
* **`lanes_uniform` compares `phase` per channel.** Keep the right phase equal by setting it from
  the left at block end, as the prototype did.
* **Designed agreement can fail after automation that looks symmetric.** Two single-channel spans
  in one block apply in order and produce equal words. A `Both` span is rejected by
  `apply_automation` (`:2399`). Test both.

