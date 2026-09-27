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


## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, branch `codex/990-limiter-linked-pair-gain` from `bbcf8ce1`
(its limiter, effect-runtime, lane, effect-contract and math crates are byte-identical to
`6ca203f8`: `git diff 6ca203f8 bbcf8ce1 -- <those crates>` is empty). Commits: `b88b2c95` (gate 3,
pinned on the unmodified kernel), `fdc295db` (the change, gates 1-2, mutations).

### The change

All in `crates/true-peak-limiter/src/lib.rs`:

* Contract 1: `designed_gain_agree` (every lane: `LaneShape`, and all four words of both ramps, by
  bits), plus `lane_shapes_agree` and `gain_state_agrees` (both gain rings in full, `prefix`,
  `box_sum`, `reduction` by bits, and `phase`).
* Contract 2: `sliding_minimum_uniform_mirrored`, the uniform van Herk pass verbatim with each
  backward-pass store repeated into the mirror ring. The original function is untouched.
* Contract 3: `linked_frame_uniform`: steps 1-6 once on the left words, the required gain and the
  box term stored into both channels' rings, `g = 1 - d` once, step 7 left then right.
* Contract 4: `limiter_block_uniform` takes a block-invariant `linked: bool`; each wrap-free segment
  runs the linked frame loop when it holds, otherwise the unchanged dual loop. Only the left ramps
  advance per frame. At block end the right channel's `prefix`, `phase`, box sum, reduction word
  and both ramps are set from the left's (the ramps by copy, the brief's allowed alternative).
* Contract 5: `LimiterCore::gain_linked`. Established at construction, by `reset` and by the §4.4
  reset iff `lane_shapes_agree` (exact after `clear_runtime`), by `desymmetrize`, and by
  `restore_track` from `gain_state_agrees` over every lane. The per-block decision in
  `process_block` is `link_max && gain_linked && designed_gain_agree`, and the record after the
  block is that decision. `process_block_mono` clears it. `lanes_uniform` still picks the body; the
  per-lane body computes both channels from equal operands, so it keeps the record.

Allocation-free, no `unsafe`, no state-layout change, `copy_state_from` unchanged. The ragged,
mono and `DualMono` bodies render as before.

### Gates

| gate | command | result |
|---|---|---|
| 1 identity (matrix) | `cargo test -p true-peak-limiter --lib the_linked_body_renders_exactly_the_unmodified_kernel`, dev and `--release` | pass. `f32`, `Simd4`, `Simd8` x `Maximum`, `DualMono` x bypass on/off x the 11 block lengths x quiet, +3 dBFS, L/R-asymmetric. Every block: every output word (NaN as both-NaN), every track's payload, the complete state bits (the resident reduction word included), the reports, the silent claim, the non-finite report, and the invariant (`gain_linked` implies `gain_state_agrees`). The oracle is the pre-#990 kernel, kept verbatim in the test module (`tests::reference_block`, reached through a `#[cfg(test)]` `REFERENCE_KERNEL` switch). |
| 1 identity (randomized) | `... randomized_scenarios_render_exactly_the_unmodified_kernel` | pass. 24 scenarios per width in dev, 1,000 in release: every launch rate, both links, bypass, ragged and asymmetric cohorts; quiet, hot, asymmetric, `+0.0`, `-0.0`, subnormal, exact-threshold, spike, log-uniform and non-finite blocks; one- and two-channel, same-value and rejected `Both` retargets; both resets; current, cross-track and hostile-ramp restores (`step` to 0, below 0 or to infinity); collapse runs, one in four without `desymmetrize`. Release: 11,636/31,442 (`f32`), 7,846/31,694 (W4), 7,297/31,834 (W8) dual blocks linked. |
| 2 engagement | `... the_linked_body_engages_exactly_where_the_record_allows` | pass at `f32`, W4, W8: every block on the fixture bank; none under `DualMono`; none with one lane's left ceiling apart (links after `desymmetrize`); none on the landing block of a one-channel retarget or after, including once the other channel is re-equalled and both ramps settle with bit-equal designed words (links after `reset`); stays linked through a two-span retarget (ramping dispatch); none after a whole-bank restore whose sections differ in gain words, links after a symmetric one; none on a collapsed block; links through the silent fast path and after the §4.4 reset; gate 3's pattern (linked except blocks 60-89). |
| 3 scenario | `cargo test -p true-peak-limiter --test linked`, dev and release | pass. Pinned on `b88b2c95` (unmodified kernel): W8 `f4892e75…`, W4 `987746c7…`, scalar `cac0d1fd…`. |
| 4 existing | `cargo test --locked -p true-peak-limiter` dev and `--release` | 50 passed each (D90, `mono_collapse`, `allocation`, conformance included) |
| | `cargo test --locked -p effect-runtime` | 86 passed |
| | `cargo test --locked -p console-workload` | 39 passed (`chain_shape` 23, including both transition tests and the `sixty_four_track_console` 64-block pin) |
| | `cargo test --locked -p builtins-compiler --features test-support` | 79 passed |
| | `cargo test --locked -p wasm-gates`, dev and `--release` | 9 passed each (G5 native, G6) |
| | `gain_pan_profile::digests` (release), base tree against this tree | all 17 native session rows' 64-block digests identical |
| 6 realtime | `tests/allocation.rs`; `bash scripts/check-realtime-policy.sh`; `bash scripts/check-lane-policy.sh` | pass; `realtime policy: ok (57 marked regions in 16 files)`; `lane policy: ok` |
| 6 wasm | the build script's cargo line (`CARGO_TARGET_DIR=target/aw RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo --remap-path-prefix=…" cargo build --locked --release --target wasm32-unknown-unknown -p host-web`), then `check-web-audioworklet-callgraph.py` on base and on this tree, then `check-web-audioworklet.sh` on an assembled artifact directory | pass. "true-peak-limiter f32x4 dual" matches exactly one function (vector 880 to 910, scalar 0); rule 3 kernel count 14 to 14 (`--kernel-min 11`); the render, meter-poll and command-submit closures are unchanged. The two outputs differ only in those two lines. Not repinned. |
| hygiene | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |

Wasm identity: the V8 console guests of the base and of this tree give identical 200-block digests
for `sixty_four_track_console` and `sixty_four_track_eq_comp_simd1`, hot and quiet.

### Mutations

Seventeen mutations, each red. See `crates/true-peak-limiter/tests/MUTATIONS.md`, "Issue #990". Of
the brief's five: M1 and M2 are red in gates 1 and 3, the randomized run and `mono_collapse`. M3 is
red in gates 1 and 2 and in conformance. M5 is red in gate 2, gate 3 and the randomized run. M4 is
tested as the amendment requires, without `desymmetrize` (`a_collapsed_block_unlinks_the_pair_without_desymmetrize`),
and is red there, in gate 2 and in the randomized run. The `chain_shape` claim is dropped. M6-M17
cover the other mirrored writes, the block-end copies, and every site that establishes, keeps or
clears the record.

### A/B (descriptive; cycles per lane-sample; base and #990 built separately)

Native, x86-64-v3, Zen 3, `taskset -c 31`, under the timing lock. Two passes, interleaved
(base-new, then new-base). Each row is 200 warm-up blocks, then the p50 of 1,200 blocks. The core
clock is calibrated in process (3.70 GHz). Load was 5-10.

| row | base (pass 1 / 2) | #990 (pass 1 / 2) | change |
|---|---:|---:|---:|
| kernel, 64 tracks, W8, hot (+3 dBFS noise, every track limiting, deepest reduction 0.703) | 10.11 / 9.93 | 8.88 / 8.83 | **-1.10 to -1.23 (-11% to -12%)** |
| kernel, W8, quiet (0.3 noise, never limits) | 10.12 / 9.95 | 8.87 / 8.78 | -1.17 to -1.25 (-12%) |
| kernel, W4, hot | 18.07 / 18.26 | 15.53 / 15.52 | **-2.54 to -2.74 (-14% to -15%)** |
| kernel, W4, quiet | 18.09 / 18.15 | 15.58 / 15.51 | -2.50 to -2.64 (-14% to -15%) |
| `sixty_four_track_console`, Simd8, hot source | 27.27 / 27.28 | 26.00 / 25.96 | -1.27 to -1.32 (-4.7% of the console) |
| console, Simd8, fixture tone | 27.47 / 27.37 | 26.10 / 26.22 | -1.15 to -1.37 |
| console, Simd4, hot source | 115.26 / 114.98 | 112.38 / 112.61 | -2.37 to -2.88 |
| console, Simd4, fixture tone | 114.86 / 114.97 | 112.74 / 112.63 | -2.12 to -2.33 |
| limiter isolate (console minus `eq_comp_simd1`), Simd8 hot / tone | 10.50 / 10.49; 10.66 / 10.36 | 9.23 / 9.14; 9.29 / 9.36 | -12% |
| limiter isolate, Simd4 hot / tone | 18.80 / 18.05; 18.10 / 18.44 | 15.78 / 16.05; 15.98 / 15.98 | -11% to -16% |

V8 (Node 22.23.2): both `wasm-console-guest` builds (`+simd128`) ran in one process, alternating
first place each round, over 6 rounds of 600 blocks after 200 warm-up blocks, pinned to cpu 31 at
3.7 GHz nominal. Load was 12-13.

| row | base (median of rounds) | #990 | paired by round |
|---|---:|---:|---|
| console, hot source | 60.48 | 58.26 | #990 faster in 6/6 rounds, by 1.66-2.79 (-2.8% to -4.6% of the console; about -6% to -11% of the limiter's ~26) |
| console, fixture tone | 66.87 | 58.60 | #990 faster in 6/6 rounds. Both arms dropped about 30 us from round 3 on (interference ending). Excluding the round that straddles the drop, #990 is faster by 0.94-1.69. |

Under V8 the `eq_comp_simd1` row has identical code in both guests, yet it moved +6% (hot) and -9%
(tone) between the two modules. So the V8 isolate (hot 26.09 to 21.71, tone 26.34 to 21.57) is
reported but not relied on; the paired console rows are the V8 evidence. There is no regression
on either shape, natively or under V8. Harnesses, raw outputs and binaries were scratch and are
not committed.

### Deviations and notes for the verifier

* **Gate 1's oracle** is a verbatim copy of the pre-#990 `limiter_block`/`limiter_block_uniform`
  in test code (the root's instruction), rather than a forced-dual switch inside the code under
  test. The per-lane body is shared because #990 does not touch it. Gates 1 and 2 live in the
  crate's unit-test module, because a `#[cfg(test)]` switch is invisible to `tests/`.
  `tests/linked.rs` carries gate 3.
* **Gate 3** has one digest per width (W8, W4, and a scalar instance as well), not one combined
  SHA-256. They were pinned on the branch point, not on `6ca203f8`, which has the identical
  limiter crate.
* **Contract 4's single branch** is a block-invariant `linked` flag, tested once per wrap-free
  segment (a handful per block) inside the shared chunk and segment walk. It is not a duplicated
  outer loop. The decision is still taken once per block, in `process_block`.
* **`limiter_block` keeps its 8-argument signature** and never links, because `src/corpus.rs`
  (D90, G5) calls it and is outside the authorised paths. The shipped path is the new
  `limiter_block_linkable`. So D90 and G5 exercise only the dual kernel; gates 1-3 cover the linked
  one.
* **Beyond the brief:**
  * the silent fast path keeps the record. It advances each channel's phase by `(phase, window,
    frames)`, and at silent rest `box_sum == window`, so equal gain words mean equal windows (M16
    guards liveness).
  * a rejected `restore_track` leaves the record unchanged, because it writes nothing.
* **The AudioWorklet pin is already stale on the base.** The base tree's own
  `build-web-audioworklet.sh` produces `820e96dd…` against the pinned `8934cdd9…`. This tree gives
  `15abcca5…`. It was not repinned, as instructed. `check-web-audioworklet.sh` was run on an
  assembled directory, because the build script stops at the pin.
* **The hot console source is +20 dBFS, not +3.** The fixture's -6 dB trim and its 1.5:1
  compressor (threshold -6 dB) would hold +3 dBFS noise under the limiter's -1.5 dBFS limit. At +20
  dBFS about +7 dBFS reaches the limiter. That the console limits is argued from the fixture
  levels, not measured: the armed observation arm published no windows within 64 blocks. The
  kernel harness's hot rows are measured limiting.
* **Dev-profile `process_block` frames** grow as follows (the reference copy is `#[inline(never)]`
  and adds nothing). All are far below the 2 MB test-thread stack.

  | width | base | #990 |
  |---|---:|---:|
  | W8 | about 245 KB | about 274 KB |
  | W4 | about 146 KB | about 164 KB |
  | scalar | 56 KB | 64 KB |

## Sol attempt 1 verdict: PASS

Reviewer: Sol, 2026-09-27, on `cd924ed9` against `bbcf8ce1`. Implementation untouched; the tree
was restored byte-for-byte after every mutation row. Harnesses were scratch and are not committed.

### What was checked, independently of the in-tree oracle

* **Separate-build differential.** One public-API harness (factory `prepare` and
  `bind_homogeneous_bank`), built against an export of `bbcf8ce1` and against this branch. After
  every block it folds every output word, every per-track `ProcessReport`, every track's full
  state payload, the resident reduction reading, the per-lane symmetry witness and every restore
  verdict. Scenarios draw: all four launch rates; quanta and ragged frame counts from 1 to 256;
  `Maximum` and `DualMono`; bypass; uniform and ragged cohorts; partial banks (trailing lanes
  silent or fed garbage); left/right asymmetry in ceiling, release, lookahead, or lookahead by one
  ulp (same window). Signals are quiet, +3 dBFS, one-sided, +20 dBFS, tone, per-lane mixed, `+0`,
  `-0`, subnormal, exact and one-below threshold, spikes, log-uniform, and NaN/inf/1e30. Events are
  one- and two-channel retargets, staggered left-then-right retargets, same-value retargets,
  rejected `Both`, out-of-order and lookahead spans, both resets, and restores. The restores
  cover current, stale, cross-track, swapped, mixed-section, one-section-perturbed (ring,
  reduction, prefix, phase, history, ramp), symmetric-perturbed and hostile-ramp payloads, and a
  wrong version. Mono-collapse runs follow the contract, with `desymmetrize` skipped in some.
  * Release: 8,000 scenarios × 128 blocks at each of scalar, W4 and W8 (3.07 M blocks, 834k with
    every lane designed-symmetric under `Maximum`), plus 3,000 × 96. **All digests identical.**
  * Dev profile (debug assertions): 1,200 × 96 blocks per width. Identical, and no assertion fired.
  * wasm32 `+simd128` under Node 22: 6,000 scenarios (scalar and W4). Identical.
* **Guarantees, measured directly** on both builds, with identical output:
  * impulse latency is exactly `N + 6` at every rate and width, at unity gain;
  * the stationary 8x-sinc true peak is at most -0.20 dB against the ceiling (worst ratio 0.978).
* **Admission.** Every writer of the gain words re-derives or keeps the record:
  * construction and reset: `lane_shapes_agree` (`src/lib.rs:2539`);
  * §4.4: `src/lib.rs:2646`;
  * render: the decision is the record (`src/lib.rs:2601-2604`), taken after `apply_automation`;
  * collapse clears it (`src/lib.rs:2685`); `desymmetrize` sets it (`src/lib.rs:2745`);
  * restore compares in full (`src/lib.rs:3197`), and a rejected restore writes nothing;
  * the silent path keeps it, soundly (equal box sums at rest imply equal windows).

  Nothing is stale. The admission is exact, if conservative (see L2).
* **Gate 3's pins are base words.** `tests/linked.rs` passes unmodified on `bbcf8ce1` in dev and
  release.

### Gates (all reproduced)

* fmt, clippy `-D warnings` (workspace, all targets, all features) and doc `-D warnings`: clean.
* `true-peak-limiter`: 50/50 in dev, 50/50 in release.
* `effect-runtime` 86; `console-workload` 39; `builtins-compiler --features test-support` 79.
* `wasm-gates`: 9/9 in dev and 9/9 in release.
* Lane and realtime policies: ok (57 regions).
* AudioWorklet:
  * base and branch built with the script's flags; the branch is `15abcca5…` and base is
    `820e96dd…` (the committed pin `8934cdd9…` is already stale on base);
  * "true-peak-limiter f32x4 dual" matches exactly one function (vector 880 to 910, scalar 0);
  * the rule-3 kernel count stays at 14;
  * the render, meter-poll and command-submit closures are unchanged;
  * `check-web-audioworklet.sh` passes on an assembled directory.
* Session digests: all 17 native 64-block digests match base, as do all 15 V8 wasm-guest digests
  (tone). The V8 console and `eq_comp_simd1` 200-block digests on the +20 dBFS source also match.
* **Mutations:** all 17 rows go red exactly as `tests/MUTATIONS.md` records.
  * My differential independently catches every correctness row (M1-M12, M17).
  * The liveness rows M13-M16 are, correctly, invisible to a class-A differential.
  * An extra output-visible mutation, X1 (the right linked output left ungained), turns 7 limiter
    tests and 11 `console-workload` tests red.

### Performance (descriptive; separate binaries, ABBA, `taskset -c 31`, under the lock)

* **Kernel rig** (64 tracks, fixture parameters), in cycles per lane-sample:
  * hot W8: 9.95-10.50 to 8.70-9.15 (-12%);
  * hot W4: 18.24-18.42 to 15.38-15.91 (-14% to -16%);
  * quiet W8/W4: -14%/-15%; linked ramping W8/W4: -12% to -17%/-20%.
* **Unlinked shapes** over 4 interleaved passes:
  * `DualMono` +0.2%/+0.2%;
  * asymmetric -1.1%/-0.1%;
  * asymmetric ramping -0.2%/-2.8%;
  * ragged (per-lane body, untouched) +0.7% W8 and +1.3% W4 (see I1).

  One early W8 `DualMono` pass read +3% to +6% and did not reproduce in four later passes.
* **Console, +20 dBFS source:**
  * W8 27.41-27.80 to 26.24-26.60 (-3% to -5.5%);
  * W4 115.0-115.4 to 112.4-112.7 (-2.3%);
  * tone: W8 -1.0 to -1.4 and W4 -2.4 cycles per lane-sample;
  * `eq_comp_simd1` is flat, so the limiter isolate falls about 11% (W8).
* **V8, one process, 6 rounds each:**
  * console +20 dBFS: 59.77 to 55.89 (-6.5%, faster in 6/6 rounds);
  * tone: -6.3% (6/6);
  * `eq_comp_simd1`: ±0.2%.

  This meets or beats the claims.
* **How much the hot console limits** (point 3, now measured, not argued): with observation
  armed (it needs `control: true`, because `arm_observation` pushes through the control producers
  at `tools/console-workload/src/lib.rs:1185`; this is why the attempt's arm published nothing),
  the limiters with reduction at steady state are:

  | source | limiters with reduction | ≥ 1 dB | median | deepest |
  |---|---:|---:|---:|---:|
  | +20 dBFS | 64/64 | 31 | ~0.95 dB | 21.3 dB |
  | fixture tone | 6/64 | – | – | 0.41 dB |
  | +3 dBFS | 63/64 | 14 | – | – |

### Findings (severity-ranked; none blocks)

1. **M1 (Medium; coverage, follow-up issue).** Session-level tests reach the linked body, but
   they cannot see the right channel's mirrored state.
   * *What reaches it:* `console-workload` renders compiled sessions whose W8 limiter banks link
     on every block. `chain_shape::the_select_free_matrix_arm_renders_the_base_bits`
     (`tools/console-workload/tests/chain_shape.rs:872`) pins `sixty_four_track_console`'s
     pre-#990 64-block digest, and X1 turns it and 10 others red.
   * *What it cannot see:* M1 stays green there, and M8 is caught only by the collapsed-tap
     observation test. It stays green even on a +20 dBFS source with a hard left-only retarget
     that unlinks a cohort. Three reasons:
     * `SourceSignal::Injected` freezes one 128-frame block (`tools/console-workload/src/lib.rs:1756`). That period is
       shorter than the 241-sample window, so the window minimum is constant and the post-unlink
       difference falls below one ulp through the release step.
     * The fixture tone barely limits.
     * No session API exposes effect state payloads.
   * *Meanwhile:* D90 and G5 run only the dual kernel, because `limiter_block`
     (`crates/true-peak-limiter/src/lib.rs:1899`) is called from `corpus.rs:246` with no record.
   * *Needed test:* a `console-workload` (or `host-core`) test that renders the standing console
     from an **aperiodic, multi-block** source that limits every track (for example a
     per-observation +20 dBFS noise table; this needs a tooling change to `SourceSignal`). It
     arms observation, pushes a Left and Right ceiling retarget and later a Left-only one through
     `push_parameter`, and pins the output and readings, recorded on `bbcf8ce1`, at Simd8, Simd4
     and in the wasm guest. The test must turn M1, M2, M6 and M7 red. My output-only aperiodic
     bank differential shows it can: 23-124 of 400 scenarios per width.
   * Separately, route `corpus::run_case` through `limiter_block_linkable` with a record, so D90
     and G5 link. That needs `corpus.rs` authorised.
2. **L2 (Low; liveness).** Rendering never re-establishes the record. After any one-channel
   retarget, the pair stays dual until reset, restore or `desymmetrize`. That holds even once both
   channels settle at silent rest, where `is_at_silent_rest` on both, plus equal phases and shapes,
   would re-prove agreement cheaply. This is as briefed; it is worth a follow-up, since hosts
   rarely reset.
3. **I1 (Info).** The ragged per-lane body, which #990 does not change, measured +0.7% at W8 and
   +1.3% at W4, with a consistent sign. That is within codegen variance, but it should be
   watched in the weekly pass.
4. **I2 (Info).**
   * Contract 4's "one whole-block branch" is a per-segment test of a block-invariant flag
     (`src/lib.rs:2275`). That is acceptable as measured.
   * The AudioWorklet pin was stale before #990, and needs a repin at the batch boundary.
