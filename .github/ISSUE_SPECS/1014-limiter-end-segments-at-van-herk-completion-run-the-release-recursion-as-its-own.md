# Limiter: end segments at van Herk completion, run the release recursion as its own pass, and stream a linked pair's steady frames without bounds checks (stationary dispatch only)

Source: `docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS-2.md`, verified in `VERIFY-LIMITER-2.md`. **The Amendments sections supersede the body wherever they conflict.** Draft names map to issues: limiter-2-1 = #1013; limiter-2-2 and limiter-2-3 (merged) = #1014.

**Decisions recorded (root, 2026-09-27):** this slice is limiter-2-2 and limiter-2-3 merged, on the stationary dispatch only (amendments A1 and A2 of limiter-2-2). It lands after #1013 (slice 1) and is measured against it. The `RampLinked` allowance of slice 1 + 2 % for an untouched, token-identical ramping path is adopted provisionally; the owner may rule it to +0 %. Target-specific loop shapes (a wasm-only two-frame detector) are not part of this slice and wait for an owner ruling.

---

## Draft limiter-2-2


Limiter round 2, slice 2 of 3 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at
`49f696c7`; every `file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS-2.md`, sections 2, 4 and 5 item 2. The prototype
is `P_GSPLIT | P_VHSEG` (65552) in `docs/handoffs/effects-2026-09-27/limiter-diagnosis-2-prototypes.patch`,
measured together with `P_SEED` (65556). It is evidence only; do not commit it. Land after
`limiter-2-1` (the seed), which the measurements include.

## Product outcome

`limiter_block_uniform` (`crates/true-peak-limiter/src/lib.rs:2193`) walks each 32-frame chunk in
wrap-free segments (`segment`, `:1705`). Every frame of a segment runs one of two functions:

* `channel_frame_uniform` (`:1759`, dual);
* `linked_frame_uniform` (`:1828`, the #990 linked pair).

Each of them calls `sliding_minimum_uniform` (`:1277`) or its mirrored twin (`:1331`), which does
three things per frame:

* branches on `position == 0`;
* branches on block completion;
* at completion, runs a `Wb`-iteration backward pass (241 slots at 5 ms) inside the frame loop.

The frame loop then carries a nested loop. At `Simd8` the register allocator spills about 20 ring
bases and lengths and the recursive word `d` (`hot.reduction`, reloaded from `[rsp+0xca0]` every
frame). The 13-cycle release recursion therefore runs through store forwarding, and the frame costs
119 instructions (linked) or 196 (dual) for 25 or 47 vector operations.

This slice makes three changes, in the uniform dual and linked bodies:

1. **Cut at completion.** Each segment is also cut at the frame where the van Herk block
   completes: `run <= window - phase`, on both channels in the dual body. Every frame before a
   segment's last is a *steady* frame: no `position == 0` branch, no completion, no backward pass.
2. **Pass 1** runs steps 1-5 of the frozen order and the target `t = 1 - s` for the steady frames,
   storing `t` into a stack scratch. The steady frame's van Herk step is
   `running = prefix.min(newest)`, with `prefix` preset to `+inf` when the segment starts at phase
   0. The segment's last frame (completing or not) then runs through today's
   `channel_frame_uniform` / `linked_frame_uniform` steps 1-5, unchanged.
3. **Pass 2** runs step 6 (the recursion) and step 7 (the delay line and the output) for the
   segment's frames, with `d` in a register.

Per-frame instructions at `Simd8`:

| body | today | pass 1 + pass 2 |
|---|---:|---:|
| linked | 119 | 63 + 35 |
| dual | 196 | 111 + 45 |

`Simd4` is the same shape. Under V8 the linked pass 1 is 105 instructions, 68 of them GPR address
work, which slice 3 targets.

Measured with slice 1 applied, against the pristine binary. Figures are cycles per lane-sample, the
minimum over rounds, separate binaries, under the timing lock, and are descriptive only.

| row | native `Simd8` | native `Simd4` | V8 (mean of 3 processes) |
|---|---:|---:|---:|
| kernel rig, +3 dBFS, `dual_mono` | **-8 %** | **-6 %** | **-1.9 %** |
| ditto, linked | -3 % | -5 % | -2.6 % |
| ditto, quiet tone | -2 % | -5 % | not measured |
| ditto, ramping dispatch | **+8 %** | **+3 %** | not measured |
| console minus `eq_comp_simd1` (hot / tone) | -5 % / -4 % | -3 % / -4 % | -3.4 % (`console - nolim`) |

The increment over slice 1 alone:

* native: dual -2 to -3 %, linked +0.5 to +1.6 %;
* V8: dual -3.0 %, linked +0.5 %.

The slice's own value is therefore on dual blocks (`dual_mono`, or asymmetric stereo). It is also
the structural precondition for slice 3: ring streams are uniform only inside completion-free,
wrap-free segments.

## Proofs (the doc comments must carry them)

1. **`+inf` preset.** D8's `min(a, b)` is `select(a < b, a, b)` (`lane` crate, `Lane::min`). With
   `a = +inf`, `a < b` is false for every `b`: no value is greater than `+inf`, and NaN compares
   false. So `min(+inf, newest)` is `newest` bit for bit, including `-0.0`, subnormals and every
   NaN payload. That is exactly `sliding_minimum_uniform`'s `position == 0` result. The preset is
   overwritten by the frame's `prefix = running`, so `+inf` never reaches the arena. A segment has at
   least one frame (`segment` asserts `run >= 1`).
2. **Steady frames never complete.** Frame `s` of a segment has `position = phase + s`, and it
   completes iff `phase + s + 1 == window`. With `run <= window - phase` that is possible only for
   `s == run - 1`, which is not a steady frame. In the dual body, both channels' `window - phase`
   bound `run`. The channels' windows may differ, because `LaneShape` is per channel.
3. **The passes commute.** Within a segment:
   * Pass 1 of frame `s` reads and writes only what pass 1 of frames before `s` left:
     `required_ring`, `box_ring`, `prefix`, `phase`, `box_sum`, the limit ramp and the peak scratch.
     It never reads `io`, `main_ring`, `reduction` or the release ramp.
   * Pass 2 of frame `s` reads `t_s`, `d_{s-1}`, the release ramp, `main_ring` and `io`.
   * Neither pass reads anything the other writes, except `t`.
   * The detector read `io` for the whole chunk before the segment began.
   * Each ramp is advanced exactly once per frame in frame order: the limit ramp in pass 1, the
     release ramp in pass 2. `RampLanes::advance` reads only its own words.

   Every value is therefore computed by the same operations on the same operands as today's fused
   frame.
4. **The linked pair** mirrors exactly the writes `linked_frame_uniform` mirrors, the required gain
   and the box term, in pass 1. The backward pass stays in the last frame (through
   `sliding_minimum_uniform_mirrored`). #990's record is untouched.

## Invariants

* **Class A.** Every output word, state payload, report and observation is unchanged, at `f32`,
  `Simd4` and `Simd8`, on every target.
* **Scope.** Only the uniform dual and linked bodies change, in both dispatches (see "What the
  implementer will hit"). The ragged per-lane body and the collapsed mono body
  (`limiter_block_uniform_mono`, `:3696`) are untouched.
* **Shared functions are frozen.** `channel_frame_uniform`, `linked_frame_uniform`,
  `sliding_minimum_uniform(_mirrored)` and `segment` keep their bodies: #990's test oracle
  (`tests::reference_block_uniform`, `:6534`) calls them. New steady and pass-2 functions are added
  beside them.
* **Allocation-free, no `unsafe`, no state-layout change.** The target scratch is a fixed
  `[f32; DETECTOR_CHUNK * MAXIMUM_WIDTH]` per channel on the stack.
* **Wasm roster.** The row still matches exactly one function (`LimiterCore<f32x4>::process_block`).
  Do not outline the new passes: `#[inline(never)]` bodies moved the timing by 8-15 % in the
  diagnosis and would change the row.

## Interface contract

1. The segment walk takes `run = min(walk.run, window_L - phase_L[, window_R - phase_R])` in the
   uniform body. The `[..]` term is dual only; the linked pair uses the left words.
2. `channel_target_steady` and `linked_target_steady`: steps 1-5 and `1 - s` for a steady frame, as
   in the prototype. `prefix.min(newest)` keeps today's operand order.
3. The pass-1 loop over `run - 1` steady frames, followed by the last frame through today's steps
   1-5, written as a `*_target_uniform` twin of `channel_frame_uniform`'s first half. `phase`
   advances by `run - 1` before the last frame.
4. The pass-2 loop: `release_step` (`fma`, `max`, `flush`, then `1 - d`) and `output_step`
   (read-before-write on `main_ring`, then `select(bypass, z, z * g)`), for each channel. The linked
   pair uses one gain for both.

## Smallest closable slice

Authorized paths:

* `crates/true-peak-limiter/src/lib.rs` (`limiter_block_uniform`, the new functions, the tests
  module);
* `crates/true-peak-limiter/tests/segments.rs` (new);
* `crates/true-peak-limiter/tests/MUTATIONS.md`;
* this spec.

Steps:

1. **On the base:** write gate 3's scenario and pin its digests.
2. Contracts 1-4.
3. The gates, then the evidence.

## Non-goals

* Bounds-check-free streams: slice 3.
* The mono body and the per-lane body.
* `l / max(p, l)` or the link and bypass arms (#992). Adding the first to the steady frames measured
  +5 to +20 % against this slice.
* The fused form: the same cut without the pass split measured +7 to +26 %.

## Objective gates

1. **Identity against the unmodified kernel**, in dev and in release:
   * `randomized_scenarios_render_exactly_the_unmodified_kernel` (`:7365`). Its oracle is the
     verbatim pre-#990 gain loop, so it is independent of this slice.
   * `the_linked_body_renders_exactly_the_unmodified_kernel` (`:7222`);
   * `a_uniform_cohort_renders_exactly_the_per_lane_path` (`:4988`);
   * `stationary_dispatch_matches_runtime_oracle_and_observes_selected_body` (`:5403`).
2. **A new randomized identity test** in the tests module, same oracle, dev (24 scenarios per width)
   and release (1,000). It must hit the cases the existing generator reaches rarely:
   * windows `Wb` in {32, 33, 241, 480, 481}, i.e. lookaheads of 0, 0.67, 5, 9.98 and 10 ms at
     48 kHz (the last is `Wb == R`);
   * left and right windows different (asymmetric lookahead) under `Maximum` and `DualMono`;
   * block lengths 1-256 including 31, 32, 33 and 127;
   * completion landing on a segment's first frame, on its last frame, on a chunk boundary and on a
     ring wrap;
   * the ramping dispatch (retargets of one or both channels);
   * restores that set `phase` to 0 and to `window - 1`.

   Every block compares every output word (NaN as "both NaN") and every track's payload, and
   records how many segments ended at a completion.
3. **Scenario pinned on base** (`tests/segments.rs`). Bank API, W8, W4 and scalar, 128 blocks:
   * +3 dBFS noise;
   * a lookahead mix per channel (L 5 ms, R 9.9 ms), so `R - Wb` is 5 on the right;
   * a ceiling retarget at block 40 (the ramping dispatch);
   * a `reset` at block 80.

   One SHA-256 per width of outputs and payloads, recorded on `49f696c7`.
4. **Existing gates unchanged:**
   * D90 (`tests/determinism.rs`) and `wasm-gates` G5 in release;
   * `tests/{linked,mono_collapse,gain_law,observation,allocation}.rs`;
   * every console workload's 64-block digest;
   * the V8 digests of the shipped artifact on the diagnosis sessions.
5. **Mutations**, each recorded red:
   * M1: no `+inf` preset (the first frame after a completion takes a stale prefix). Gates 1-3 go
     red.
   * M2: let the steady loop include the completing frame (the backward pass is skipped). Red.
   * M3: advance the release ramp in pass 1. Red in the ramping scenario.
   * M4: advance `phase` by `run` before the last frame. Red.
   * M5: cut only at the left channel's completion in the dual body. Red with the asymmetric
     lookahead.
   * M6: the linked pass 1 skips the mirrored box store. Red, through the payload comparison.
6. **Realtime and wasm:**
   * `tests/allocation.rs`;
   * `check-realtime-policy.sh` and `check-lane-policy.sh`;
   * `check-web-audioworklet.sh`: the row matches exactly one function, the rule-3 count does not
     drop, the render callgraph is unchanged. The vector count rises, about 910 to 926.
7. **Timing gate** (freeze the workload first). The method is slice 1's gate 7.
   * **Stationary rows** (`HotLinked`, `HotDualMono`, `QuietLinked`, native `Simd8` and `Simd4`)
     must be ≤ the slice-1 tree, and the V8 isolates' three-process mean ≤ the slice-1 tree + 2 %.
   * **`RampLinked`:** ≤ the slice-1 tree + 0 %, unless the owner rules a bound (diagnosis
     section 8 item 2). The prototype read +8 % (`Simd8`) and +3 % (`Simd4`) there.
   * If the gate fails, record the numbers and stop.
8. **Codegen record.** Quote the pass-1 and pass-2 loop instruction counts, x86 at both widths and
   V8, and the absence of a nested loop in either.

## What the implementer will hit

* **Do not restrict the change to the stationary dispatch to protect the ramping row.** The
  diagnosis tried exactly that (`P_STATONLY`). The ramping body, whose source was then pristine, ran
  +20-25 %, and the stationary rows lost their gain: this kernel's codegen is unstable (diagnosis
  section 4). If the ramping row fails gate 7, report it for the owner's ruling.
* **The dual body's two windows can differ.** `lanes_uniform` compares lanes within a channel, not
  the channels. Take the minimum of both channels' distances to completion.
* **The target scratch has to exist per channel in the dual body.** The linked pair needs one.
* **Keep `x`'s load in pass 2.** Pass 1 must not touch `io`. The detector has already read the chunk.

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-LIMITER-2.md` (F1-F4, F8) and
`verify-limiter-2-raw-timings.txt`. The prototype of the amended slice is
`verify-limiter-2-prototype.patch`. It is evidence only; do not commit it. **These amendments
supersede the body wherever they conflict.**

### A1. This slice absorbs limiter-2-3; alone it fails its own gate 7

Built clean, this draft alone measured against the slice-1 tree (minimum over rounds, load 9-12):

| row | `Simd8` | `Simd4` |
|---|---:|---:|
| `HotDualMono` | -3 % | -4 to -5 % |
| `HotLinked` | **+1 %** | **+1 to +2 %** |
| `QuietLinked` | **+1 to +2 %** | **+1 to +2 %** |
| `RampLinked` | **+8 to +9 %** | **+5 to +6 %** |

Gate 7 requires the stationary rows ≤ slice 1 and `RampLinked` ≤ +0 %, so this draft cannot pass
it alone. The diagnosis's own increment ("linked +0.5 to +1.6 %") said so. For the linked pair,
the cut and the pass split are a precondition, not a product outcome.

* The slice is now **limiter-2-2 + limiter-2-3**.
* Limiter-2-3's stream facts, contracts 1-3, mutations and codegen record become this slice's
  contracts 5-7, mutations M7-M11 and gate 8b.
* Limiter-2-3's two-attempt limit applies to the merged slice.
* Authorized paths are the union of the two drafts'.

### A2. Scope: the stationary dispatch only

Delete "What the implementer will hit", item 1 ("Do not restrict the change to the stationary
dispatch"). Its evidence, `P_STATONLY`, was measured inside the diagnosis's scaffold (VERIFY F1).

* In `limiter_block_uniform`, the new walk runs only when `DISPATCH == DISPATCH_STATIONARY`. Under
  `DISPATCH_RAMPING` the segment loop is today's fused loop, **token for token**.
* `DISPATCH_RUNTIME`, the test-only oracle, takes the new walk.

Built clean this way, every stationary gain stays and the ramping regression goes. Minimum over
rounds, cycles per lane-sample (`raw-k7`, load 11-12):

| row | slice 1 | merged, all dispatches | **merged, stationary only** |
|---|---:|---:|---:|
| `Simd8` dual / linked / quiet | 9.65 / 8.30 / 8.18 | 9.49 / 8.01 / 7.98 | **9.10 / 7.87 / 8.03** |
| `Simd8` ramping | 8.71 | 9.13 (+5 %) | **8.79 (+1 %)** |
| `Simd4` dual / linked / quiet | 17.63 / 14.79 / 14.86 | 16.92 / 14.17 / 14.18 | **16.98 / 14.20 / 14.25** |
| `Simd4` ramping | 15.59 | 15.89 (+2 %) | **15.64 (+0 %)** |

A second clean run (`raw-k8`, load 8-13) reproduced this. Against slice 1, the stationary-only
slice read -7 % on each `Simd8` stationary row and -4 % on each `Simd4` one; `RampLinked` read
-2 % and +0 %.

Under V8, against slice 1 (three processes, load 4-6, `raw-w6`), the merged stationary-only slice
reads:

* `lim - bi` -9.5 %;
* `limdm - bi` -2.4 %;
* `console - nolim` -9.5 %.

The all-dispatch form reads -8.6 %, -2.4 % and -8.6 %.

### A3. Measured and not added: pass 2 over delay-line streams

Step 7 reads each `main_ring` slot and then writes it, frame by frame, and `segment` guarantees
`main_cursor + run ≤ B`. So pass 2 can zip the frames with
`main_ring[main_base·W .. (main_base + run)·W].chunks_exact_mut(W)`, in safe Rust.

The prototype (`p2s` in the verification harness) is bit-identical and passes the crate's suite.
On top of A2 it measured:

* native: -1 to +1 %;
* V8: -0.1 % (`lim - bi`), +0.7 % (`limdm - bi`), +0.2 % (`console - nolim`).

It is not part of this slice. Pass 2's checks were not what cost time.

### A4. Gate 7, for the merged slice

Freeze the workload first. The method is limiter-2-1's gate 7 as amended there (A1: clean builds;
A3: exact statistic). Against the slice-1 tree:

* **Stationary rows** (`HotLinked`, `HotDualMono`, `QuietLinked`), native `Simd8` and `Simd4`:
  each faster.
* **V8** (three processes): `lim - bi` and `console - nolim` faster by more than 2 %, and
  `limdm - bi` faster.
* **`RampLinked`**, whose source is unchanged under A2: at most slice 1 + 2 %, an allowance for
  codegen movement of an untouched path. Two clean runs read -2 to +1 %.
  * The allowance holds only while the review confirms the ramping arm is today's loop token for
    token.
  * The owner may rule it to +0 % (VERIFY section 7, item 2).
* **The V8 sessions have no ramping row.** V8's ramping dispatch is unmeasured. A V8 ramping row,
  with parameter events through `command_submit`, is a harness follow-up, not this slice.
* If the gate fails, record it and stop.

### A5. Mutations, replacing M3 and adding the guard

* **M3 is replaced.** "Advance the release ramp in pass 1" is equivalent if the per-frame values are
  carried to pass 2. Under A2 the release ramp is also resting in the only dispatch that changes.
  **M3′:** pass 2 reads the previous frame's target (`targets[step - 1]`; frame 0 reads the
  segment's last). Red.
* **M12:** drop the dispatch guard of A2, so the new walk also runs under `DISPATCH_RAMPING`.
  Record it as **equivalent (green)**: the all-dispatch prototype is bit-identical on the
  differential and the V8 digests. The guard is a performance choice. Gate 2 must count ramping
  blocks that took the old loop, and that count must be nonzero.

### A6. Codegen record

* Quote the linked stream loop (limiter-2-3's gate 5): at most 25 instructions at `Simd8` and
  `Simd4` in the stationary instantiation, one backward branch, no panic edge. The prototype has 23.
* Quote the V8 loop census of both passes.
* The roster row stays one function, the rule-3 count does not drop, and the render callgraph and
  trap owners are unchanged. The prototype's vector count is 913, with or without A3 (910 pristine,
  878 with the seed).

---

## Absorbed draft limiter-2-3


Limiter round 2, slice 3 of 3 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at
`49f696c7`; every `file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS-2.md`, sections 2 and 5 item 3. The diagnostic
arm is `X_UNCHECKED`, and the failed copy-based prototype is `P_BCF`. Both are in
`docs/handoffs/effects-2026-09-27/limiter-diagnosis-2-prototypes.patch`, as evidence only; do not
commit them.

This slice builds on `limiter-2-2`: its steady frames exist only inside completion-free segments.
**Attempt limit: two.** A smaller limit set by the brief is binding (AGENTS.md). If the timing gate
fails twice, stop and record the evidence.

## Product outcome

Every ring access in the gain loop is `ring_lane` / `store_ring_lane` (`crates/true-peak-limiter/src/lib.rs:1211`, `:1218`): a
`slot * W` multiply, a slice range check and its branch. The loop also spills the ring bases and
lengths that the checks need. An unchecked diagnostic build (`get_unchecked`, not shippable) measures
what that costs, in cycles per lane-sample, minimum over rounds, against pristine:

| row | native `Simd8` | native `Simd4` | V8 (shipped artifact, mean of 3 processes) |
|---|---:|---:|---:|
| +3 dBFS, linked | -6 % | -8 % | **-8.7 %** |
| +3 dBFS, `dual_mono` | -9 % | -10 % | **-6.0 %** |
| console minus the limiter | | | **-8.9 %** |

This is the largest browser lever the diagnosis found. Every detector reshaping stayed inside V8's
±2-4 % noise. The V8 pass 1 of `limiter-2-2` is 105 instructions per linked frame, 68 of them GPR
address work.

This slice removes the checks from the linked pair's steady pass 1, the usual stereo setting. The
arithmetic is unchanged. Only the addressing changes: from indexed slots to exact-length
`chunks_exact(_mut)` views that LLVM iterates with one induction variable. The copy-based prototype
compiled its loop to 24 instructions with no branch (`llvm-mca` 7.1 cycles per frame against about
11 for the checked form). It still measured +4-5 % natively, because it copied three read streams
per segment and perturbed the whole function (diagnosis section 4). This slice specifies the
no-copy form.

## The stream facts (the doc comment must carry them)

Take a steady run of `steady` frames. It starts at the `FrameSlots` of `limiter-2-2`'s segment:
`c` (cursor), `e = c + Wb` (newest), `c + 1` (oldest) and `x = c + (R - Wb)` (expiring), each mod
`R`. It has no wrap and no completion inside it, and satisfies `Wb < R` and `steady <= R - Wb`.

1. **The cursor block `C = [c, c + steady]` (`steady + 1` slots) fits in the ring.** `segment`
   bounds the run by `R - (c + 1)`.
2. **The newest stream `E = [e, e + steady)` is disjoint from `C`.**
   * Unwrapped: `e = c + Wb >= c + 32 > c + steady`.
   * Wrapped: `e = c + Wb - R`, and `e + steady <= c` because `steady <= R - Wb`.
   * So no frame of the run reads a newest slot the run has written. That is exactly why `Wb == R`
     (where `e == c`) is excluded.
3. **The oldest slot of frame `s` is `c + 1 + s`, the next slot of `C`.** Frame `s` reads it and
   frame `s + 1` overwrites it. In a walk that visits each slot of `C` once, reading the slot's old
   value and then writing its new one, the old value of slot `c + 1 + s` is frame `s`'s oldest.
4. **The expiring stream `X = [x, x + steady)` of the box ring is disjoint from the box cursor
   stream `B = [c, c + steady)`.**
   * Unwrapped: `x - c = R - Wb >= steady`.
   * Wrapped: `x = c - Wb`, and `x + steady <= c` because `steady < 32 <= Wb`.

   `Wb == R` (`x == c`) is excluded.
5. **The right channel's rings are separate allocations.** Its mirrored stores go to
   `[c, c + steady)` of each.

`E` and `C` are therefore two disjoint borrows of one ring (`split_at_mut` at whichever of `e` and `c`
is larger), and so are `X` and `B`. Nothing is copied.

## Interface contract

1. `linked_steady_streams<L, DISPATCH>(..)`, called from `limiter-2-2`'s linked pass 1 when
   `Wb < R && steady >= 2 && steady <= R - Wb`. Otherwise the checked steady loop runs, unchanged.
2. **The skewed walk.**
   * **Prologue:** start frame 0. Compute the required gain, store it into `C[0]` and its mirror,
     `newest = E[0]`, then `running_0 = prefix.min(newest)`.
   * **Body:** one zipped loop of `steady - 1` iterations over
     `C[1..steady)`, `E[1..steady)`, the peaks `[1..steady)`, the mirror `[1..steady)`,
     `X[0..steady-1)`, `B[0..steady-1)`, the box mirror `[0..steady-1)` and the targets
     `[0..steady-1)`. Each iteration:
     1. finishes frame `j - 1`: `oldest = old C[j]`, `minimum = oldest.min(running)`, quantise,
        `box_sum = (box_sum + q) - X`, store `q` into `B` and its mirror, then
        `t = 1 - box_sum / window`;
     2. then starts frame `j`: ramp, peak, required gain into `C[j]` and its mirror, `newest`,
        `running`.
   * **Epilogue:** finish frame `steady - 1` with `oldest = old C[steady]`.
   * `prefix`, `box_sum`, `phase` and the limit ramp leave the function as the checked loop leaves
     them.
3. **Operand order per frame** is today's: `limit.div(peak)` inside the same select,
   `prefix.min(newest)`, `oldest.min(running)`, `box_sum.add(q).sub(expired)`. Only the program
   interleaving across frames changes. Finishing `j - 1` before starting `j` is a reorder of
   independent work, and fact 3 is what makes the oldest read identical.

## Invariants

* **Class A.** Every output word, state payload, report and observation is unchanged, at `f32`,
  `Simd4` and `Simd8`, on every target.
* Only the linked steady pass 1 changes, in both dispatches, as in `limiter-2-2`. The dual body and
  pass 2 keep `limiter-2-2`'s form, and the per-lane and mono bodies keep today's. The dual body is a
  follow-up once this one passes its gate.
* Allocation-free, no `unsafe`, no copies of ring data, no state-layout change.
* **Wasm roster.** The row stays exactly one function. Do not mark the new function
  `#[inline(never)]`.

## Smallest closable slice

Authorized paths:

* `crates/true-peak-limiter/src/lib.rs` (the new function, its call site in the linked pass 1, the
  tests module);
* `crates/true-peak-limiter/tests/MUTATIONS.md`;
* this spec.

Steps:

1. Contracts 1-3.
2. Gate 7 first, on the first compiling build.
3. The rest.

## Non-goals

* The dual body, pass 2, and the backward pass.
* A `lane`-crate array view (`[[f32; W]]` rings). If this slice fails, that is the next question,
  in its own issue (diagnosis section 8 item 4).
* `unsafe` or `get_unchecked` in any form.

## Objective gates

1. **Identity**, dev and release: `limiter-2-2`'s gates 1 and 2 (the #990 oracle suite and the
   completion-heavy randomized test), with the generator extended to cover:
   * `Wb` in {32, 241, 470, 476, 480}, so `R - Wb` spans 1-449 (the condition's both sides at
     `steady` up to 31);
   * cursor positions that make `e` and `x` wrap, and ones that do not;
   * linked blocks with `steady` in {0, 1, 2, 3, 31}.

   The test counts runs that took the unchecked streams, the fallback, and each wrap case. All
   counters must be nonzero.
2. **Mirror state.** Every block, the right channel's payload equals the left's words where #990's
   record holds. This comes free through the payload comparison.
3. **Existing gates unchanged:**
   * D90 and G5 (release);
   * `tests/linked.rs` (#990's pins);
   * `limiter-2-2`'s pinned scenario;
   * every console workload's 64-block digest;
   * the V8 digests of the shipped artifact on the diagnosis sessions.
4. **Mutations**, each recorded red:
   * M1: split `E` from `C` at `c` instead of the larger index (a stale-newest view when `e < c`).
     It fails to compile or goes red; record which.
   * M2: read `oldest` from `C[j]` after writing `r_j`. Red.
   * M3: drop the `steady <= R - Wb` leg. Red at `Wb = 476`.
   * M4: skip the mirrored required-gain store in the body. Red, through the payload.
   * M5: finish frame `j - 1` after starting frame `j`, with the oldest read moved after the write.
     Red.
5. **Codegen record.** The body loop at `Simd8`, `Simd4` and under V8 must carry no bounds-check
   branch: exactly one backward branch and no forward jumps to a panic. Quote its instruction count.
6. **Realtime and wasm:**
   * `tests/allocation.rs`;
   * `check-realtime-policy.sh` and `check-lane-policy.sh` (no `unsafe`);
   * `check-web-audioworklet.sh`: the row is one function, the rule-3 count does not drop, the
     render callgraph is unchanged, and the trap owners are unchanged.
7. **Timing gate** (freeze the workload first). The method is `limiter-2-1`'s gate 7, against the
   `limiter-2-2` tree.
   * `HotLinked` and `QuietLinked` at native `Simd8` and `Simd4` must be faster.
   * The V8 `lim - bi` three-process mean must be faster by more than 2 %.
   * Every other row must be ≤ the `limiter-2-2` tree, and V8 ≤ + 2 %.
   * If the gate fails, this attempt fails. After two, stop.

## What the implementer will hit

* **The copy-based form is already measured.** It compiled to a 24-instruction loop and still lost
  4-5 % natively (diagnosis section 5). Do not rebuild it.
* **The whole function's codegen moves when this code is added** (diagnosis section 4). Gate 7 is on
  the whole kernel, never on the new loop in isolation.
* **Prefer `split_at_mut` and `chunks_exact(_mut)` zips.** A zip of `ChunksExact` iterators uses one
  index and no per-element check. Re-slicing inside the loop brings the checks back.

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-LIMITER-2.md` (F4, F8) and
`verify-limiter-2-raw-timings.txt`. **These amendments supersede the body wherever they conflict.**

### A1. Merged into limiter-2-2

Limiter-2-2 alone fails its gate on the linked rows (its amendment A1). This slice is what makes the
linked pair faster.

* This draft's stream facts, contracts, mutations and codegen record move into limiter-2-2, as
  amended.
* The two-attempt limit applies to the merged slice.
* Scope is the stationary dispatch only, per limiter-2-2 A2. The linked streams run only under
  `DISPATCH_STATIONARY`.

### A2. The no-copy form is safe Rust, and it is built

`linked_steady_streams` in `verify-limiter-2-prototype.patch` implements contracts 1-3 as written:

* `split_at_mut` at `max(e, c)` for the required ring and at `max(x, c)` for the box ring;
* eight zipped `chunks_exact(_mut)` iterators;
* a prologue, the skewed body, and an epilogue.

It has no `unsafe`, no copy, and no lane-crate change. The "lane-crate array view" non-goal is moot.

* **Release codegen** (`Simd8` and `Simd4`, stationary): **23 instructions**, one backward branch, no
  panic edge. The checked steady loop it replaces is 64 instructions with 9 branches.
* **Identity.** The #990 differential at W8, W4 and W1 and the V8 digests read unchanged. So does
  the crate's suite (50 tests) in dev and in release.
* **Measured, against the pristine binary, with limiter-2-1 and 2-2 (stationary only):**
  * native linked -9 to -10 % (`Simd8`) and -8 % (`Simd4`);
  * V8 `lim - bi` -10.2 %, `limdm - bi` -3.2 %, `console - nolim` -9.8 %.

  Against slice 1 alone, V8 reads -9.5 %, -2.4 % and -9.5 %.

  The body's `X_UNCHECKED` prize was -5 to -9 % native and -8 % V8 on this batch. The safe form
  meets it on linked blocks.
* Replace "This change is ranked on its prize, not on a measured implementation" with these
  measurements.

### A3. M3 panics rather than rendering wrong words

"Drop the `steady <= R - Wb` leg" makes the `E` or `X` view run past its split point, so safe Rust
panics at the slice. Record M3 as "red (panic)", at `Wb = 476`. In the prototype's form, M1 cannot
compile when `e > c`, because both views would borrow the same half. Record which of the two it does.

### A4. Facts 1-2 need `Wb ≥ steady + 1`, which holds

Fact 2's "`c + Wb ≥ c + 32 > c + steady`" is what lets the cursor block `[c, c + steady]` sit below
`e`. It needs `steady + 1 ≤ Wb`. That holds because `steady ≤ 31` and `Wb ≥ 32`. The doc comment
should state it.

## Attempt 1 evidence

Terra, 2026-09-28. Attempt 1 of the two the merged slice allows. Code commit `bbdf6822` on
`codex/1014-limiter-stationary-walk`. Its parent `bfec4bba` merges the batch head `d09d5024` (#1013
merged, with Sol's verdict) into the branch, as the coordinator asked. That merge adds only #1013's
spec, and the limiter crate there (`6da64966`) is #1013's. Nothing pushed. **Gate 7 passes on clean
builds**: every stationary row is faster natively; under V8 `lim - bi` and `console - nolim` are more
than 2 % faster and `limdm - bi` is faster; `RampLinked` is within the provisional slice-1 + 2 %
allowance (+0.1 to +1.2 %), not within +0 %.

### What changed

* **The source.** The code is the verification's measured stationary-only arm, `vh+bcf+seed+statonly`
  (VERIFY F3's "1 + 2 + 3, stationary only"). It is **not** the committed
  `verify-limiter-2-prototype.patch`, which also carries A3's pass-2 delay streams (`p2s`). It was
  generated by `variants2.py vh+bcf+statonly` on the #1013 tree, and then only comments,
  `#[cfg(test)]` code and tests were added. The release `process_block` of the generated build and
  of the commit are the same instructions at both widths (normalised listings equal).
* **In `limiter_block_uniform`**, the segment loop is `if DISPATCH == DISPATCH_RAMPING { <today's
  loop> run } else { <the walk> run }`. Under `DISPATCH_STATIONARY` and the test-only
  `DISPATCH_RUNTIME` the walk runs:
  * the completion cut;
  * the `+inf` preset;
  * pass 1 (`channel_target_steady`, `linked_target_steady` or `linked_steady_streams`, then the
    last frame through `channel_target_uniform` / `linked_target_uniform`);
  * pass 2 (`release_step`, `output_step`).
* **The ramping arm is token-identical.** A tokenizer that ignores comments and whitespace compares
  the ramping arm with the base's segment loop body. It is the same 581 tokens plus the trailing
  `run`. The only other change in the function is the two `targets_*` scratch declarations
  (24 tokens).
* **The functions #990's oracle calls keep their bodies**: `channel_frame_uniform`,
  `linked_frame_uniform`, `sliding_minimum_uniform(_mirrored)` and `segment`. No `unsafe`, no
  `#[inline(never)]`, no state-layout change.
* **Doc comments.** The four proofs are on `limiter_block_uniform`. The five stream facts, with A4's
  `steady + 1 <= Wb`, and the skewed walk are on `linked_steady_streams`.
* **Test instrumentation.** `SegmentCensus` (`#[cfg(test)]`, two hooks in the walk) counts what the
  walk decided. Non-test builds do not contain it.

### Gates 1-6

| gate | result |
|---|---|
| 1. identity, #990 oracle | Green in dev and in release: `randomized_scenarios_render_exactly_the_unmodified_kernel`, `the_linked_body_renders_exactly_the_unmodified_kernel`, `a_uniform_cohort_renders_exactly_the_per_lane_path`, `stationary_dispatch_matches_runtime_oracle_and_observes_selected_body`. The #990 differential (150 seeds of 96 blocks) gives the same combined digest on both binaries: W8 `c8a711d827569736`, W4 `ff492b1bf5fc6edd`, W1 `36f79d5a6ada2623`. |
| 2. new randomized identity | `the_stationary_walk_renders_exactly_the_unmodified_kernel`: 24 scenarios per width in dev and 1,000 in release, on #990's oracle (`LinkedPair`: every output word with NaN as both NaN, every track's payload and the complete state after every block). Windows are 32, 33, 241, 470, 476, 480 and 481 at 48 kHz (asserted), both channels equal or different, under `Maximum` and `DualMono`. Block lengths run from 1 to 256 (quantum 256). Retargets hit one or both channels, and whole-bank restores set the phase to 0 and to `Wb - 1`. Every counter must be nonzero. In release at W8 (W4 and scalar are alike): 39,317 blocks; 180,142 segments, 46,896 ending at a completion (5,698 on a first frame, 41,198 on a last frame, 3,736 on a chunk boundary, 2,518 on a ring wrap); 15,441 right-channel cuts; linked runs of 0/1/2/3/31 steady frames 2,679 / 886 / 765 / 891 / 9,061; 12,540 stream runs (2,456 with the newest stream wrapped, 9,160 with the expiring stream wrapped, 2,853 with neither); 10,595 checked fallbacks, 9,709 of them refused by `Wb == R` or `steady > R - Wb`; 9,388 ramping blocks on the fused loop; 1,299 / 1,274 phase restores. |
| 3. pinned scenario | `tests/segments.rs`, pinned on `0e4732c0` (limiter crate `6da64966`, the same tree as on `bfec4bba`) in dev and in release: W8 `8b20d428…`, W4 `f72aa857…`, scalar `ea06a84c…`. It has **four arms, beyond the brief's one**: `Maximum` and `DualMono` with L 5 ms / R 9.9 ms (`R - Wb = 5` on the right), plus a linked 5 ms arm and a linked 9.9 ms arm, so that the streams and their refusal are pinned too. Each arm has a two-channel ceiling retarget at block 40 and a reset at block 80, 128 blocks. It was re-run on `bfec4bba` after the merge (dev and release) and is green on `bbdf6822` in both. |
| 4. existing gates | Green in dev and in release: D90, `tests/{linked,mono_collapse,gain_law,observation,allocation,seedless}.rs` (the limiter crate: 54 passed, 1 ignored), `host-core` (187 passed; `limiter_linked_session` at `Simd8`, `Simd4` and `Scalar`), `console-workload` (58 passed, all 64-block digests unchanged). Also green: `run-wasm-gates.sh` G5 (142 cases, 0 mismatches on every leg); the rig's in-place console digests (`Simd8`/`Simd4`, hot and tone) are identical. **V8 digests of the shipped modules, 300 blocks, are identical to base:** `lim` `dea70183`, `limdm` `3ff478a2`, `bi` `b42bcd42`, `console` `a2594b72`, `nolim` `3c42efe0`. |
| 5. mutations | M1-M6 (M3 replaced by M3′) and M7-M11 (limiter-2-3's M1-M5) all red. M7 does not compile (`E0502`). M9 panics at `Wb = 476` in gates 2 and 3. **M12 (the dispatch guard dropped) is equivalent:** every identity comparison is green in dev and release, and only gate 2's "ramping blocks on the fused loop" count falls to zero. The table and notes are in `tests/MUTATIONS.md`. |
| 6. realtime and wasm | `tests/allocation.rs` green. `check-realtime-policy.sh` (57 regions), `check-lane-policy.sh` and `check-env-vocabulary.sh` ok. `run-wasm-gates.sh` ok, including detector residency and the V8 spill gate. `check-web-audioworklet.sh` ok on the commit. The batch pin `8934cdd9…` is stale for both arms, so the gate ran in the scratch worktree with the change module's digest in the pin file, which was then restored. "true-peak-limiter f32x4 dual" matches one function, **878 → 913** (A6: 913). The collapsed row is unchanged at 424. The rule-3 count is 15 on both, minimum 11. The render, meter-poll and command-submit callgraph lines, with their trap owners, are identical. |

Also green: `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`.

### Gate 7 (A4, with #1013's A1 and A3)

**Builds.** Each arm was built from a scratch worktree detached at its commit, whose only other
content was the untracked out-of-tree rig (`limiter-diag-2-harness` from the diagnosis patch, with
the workspace `Cargo.lock` copied in). Native builds started from an empty target directory. The
wasm arms are `scripts/build-web-audioworklet.sh --module-only`, each in a fresh target.
`CARGO_INCREMENTAL=0` throughout.

| arm | commit | kernel rig `ld2` SHA-256 | `host_web.wasm` SHA-256 |
|---|---|---|---|
| base (slice 1) | `bfec4bba` | `59caa96fc8c8d6efaf185680f0555070a2085b968c084be5be8677810fbf9a86` | `0fc453598a790e470d8927cd55b357a5634b6e1996b415149d0e4dec0d44f340` |
| change | `bbdf6822` | `4d367c063bb2870500a16b18e396b8015884d374f04216d2093fb09cc1ad9bf8` | `f4e98eb4434bfae9582fa90ed4a6c09ab77d88d8fe4fa88000e52cbc6d000c0b` |

**Runs from before the merge.** Native runs 1-2 and V8 runs 1-4 were timed on builds of `0e4732c0`
and `ea0f8019`, the same code commit before the merge. Those four artifacts are **byte-identical**
to the ones above: the merge changed a spec, and the code commit's only other difference is two
lines of a test's header comment. Native run 3 and V8 run 5 are builds of `bfec4bba` / `bbdf6822`
themselves. The base binary rebuilt from scratch is byte-identical as well.

**Native.** The kernel rig: 64 tracks, `kbench`, 4 rounds of 800 blocks. Each run is 4 passes, each
arm its own process, forward then reverse, under the lock with `taskset -c 31`. The statistic is the
minimum over all 16 rounds, in cycles per lane-sample, change against slice 1:

| row | run 1 (load 6.4-8.8) | run 2 (load 9.0-9.7) | run 3 (load 6.4-6.7) |
|---|---:|---:|---:|
| `Simd8` `HotDualMono` | 9.703 → 9.194 (**-5.2 %**) | 9.807 → 9.152 (-6.7 %) | 10.126 → 9.188 (-9.3 %) |
| `Simd8` `HotLinked` | 8.330 → 7.930 (**-4.8 %**) | 8.420 → 7.930 (-5.8 %) | 8.397 → 7.934 (-5.5 %) |
| `Simd8` `QuietLinked` | 8.286 → 7.849 (**-5.3 %**) | 8.420 → 7.881 (-6.4 %) | 8.316 → 7.801 (-6.2 %) |
| `Simd8` `RampLinked` | 8.744 → 8.819 (+0.9 %) | 8.799 → 8.808 (+0.1 %) | 8.822 → 8.838 (+0.2 %) |
| `Simd4` `HotDualMono` | 17.554 → 16.852 (**-4.0 %**) | 17.694 → 16.863 (-4.7 %) | 17.866 → 16.871 (-5.6 %) |
| `Simd4` `HotLinked` | 14.818 → 13.992 (**-5.6 %**) | 14.765 → 14.042 (-4.9 %) | 14.993 → 14.065 (-6.2 %) |
| `Simd4` `QuietLinked` | 14.759 → 14.008 (**-5.1 %**) | 14.846 → 13.988 (-5.8 %) | 15.073 → 14.005 (-7.1 %) |
| `Simd4` `RampLinked` | 15.530 → 15.717 (+1.2 %) | 15.569 → 15.687 (+0.8 %) | 15.642 → 15.712 (+0.4 %) |

* **The stationary rows** are faster in every run, by 4-9 %. The change arm's minima are stable
  (7.93 / 9.15-9.19 / 7.80-7.88 at `Simd8`). The spread is the base's: its `Simd8` `HotDualMono`
  minimum moved from 9.70 to 10.13 between runs.
* **`RampLinked`** is +0.1 to +1.2 %, inside the provisional +2 % and outside +0 %. Its source is
  token-identical. Its fused loops moved by 0-3 instructions: `Simd8` 139 → 142 (linked) and
  261 → 263 (dual); `Simd4` 141 → 141 and 251 → 251. VERIFY A2 read -2 % and +0 % for this arm, so
  the owner's ruling on the allowance decides this row.

**V8.** The shipped modules through `miso_engine_web_v1_render`. Each run is three separate Node
22.23.2 processes of 10 rounds of 800 blocks, both arms in each, pinned to cpu 31, under the lock.
The first cell of each isolate is the gate's statistic: the mean over processes of each process's
median isolate, base → change. The second, following Sol's #1013 method, is the **paired** change,
the change arm against base in the same process, per process:

| isolate | run 3 (load 6.8-11.1) | run 4 (load 5.9-6.6, arms reversed) | run 5 (load 6.4-6.8) |
|---|---:|---:|---:|
| `lim - bi` | 21.96 → 20.01 (**-8.9 %**); paired -10.1 / -8.4 / -8.2 | 21.87 → 20.00 (-8.6 %); -8.4 / -8.2 / -9.1 | 21.91 → 20.03 (-8.6 %); -8.3 / -9.0 / -8.4 |
| `limdm - bi` | 24.13 → 23.50 (**-2.6 %**); paired -3.4 / -2.2 / -2.2 | 24.09 → 23.55 (-2.3 %); -2.2 / -2.1 / -2.5 | 24.03 → 23.65 (-1.6 %); -1.1 / -2.0 / -1.7 |
| `console - nolim` | 22.12 → 20.10 (**-9.1 %**); paired -10.0 / -9.1 / -8.3 | 22.10 → 19.99 (-9.5 %); -9.1 / -9.9 / -9.6 | 21.96 → 20.10 (-8.5 %); -7.6 / -8.7 / -9.1 |

* **Every paired comparison is faster**, in every process of every valid run.
* **Runs 1 and 2 are not evidence.** They were taken at load 16.9-18.7, above A3's 15, and were
  repeated once as runs 3 and 4. For the record, they read -8.6 / -1.8 / -8.8 % and
  -9.3 / -2.7 / -8.6 %.
* **V8's ramping dispatch is unmeasured**, as A4 says. The harness has no ramping row.

**A preliminary timing** was taken before the gates, on a temporary commit of the generated
source (never on a branch; the code is the same as the commit's). It read -4 to -6 % on the
stationary rows, +1.1 % / +1.7 % `RampLinked`, and -8.7 / -2.1 / -8.3 % under V8. It is
descriptive only.

### Codegen record (A6, limiter-2-3 gate 5)

The stationary instantiation of `LimiterCore<f32xW>::process_block`, instructions per loop. No loop
of the walk contains a loop; every row below has `nested = 0`.

| loop | x86 `Simd8` | x86 `Simd4` | V8 (TurboFan, wasm `f32x4`) |
|---|---:|---:|---:|
| **linked stream loop** (`linked_steady_streams` body) | **23**, 1 branch (the backward one) | **23**, 1 branch | 85 per two frames (LLVM unrolled by two), 3 branches: the backward one, the unroll exit, and V8's loop stack guard (`cmp rsp,[r13-0x60]; jna` to an out-of-line stub). No trap and no bounds check. |
| linked checked steady (fallback) | 66, 9 branches | 64, 9 | 104, 10 |
| dual steady pass 1 (both channels) | 107, 12 | 107, 12 | 165, 13 |
| linked pass 2 | 35, 3 | 34, 3 | 104 per two frames, 7 |
| dual pass 2 | 49, 3 | 46, 3 | 114 per two frames, 7 |
| the fused frame loops the walk replaces (base, stationary) | linked 118, dual 193, with the backward pass nested | 115, 195 | - |

`process_block` grows from 8,006 to 8,667 instructions (`Simd8`) and from 7,918 to 8,156 (`Simd4`).
The detector loop is unchanged at 129.

### For the verifier

* **The generator (gate 2) is a new test beside #990's.** It reuses `LinkedPair` and its oracle
  unchanged. It counts a phase restore only when both arms accept it: a payload whose main ring
  holds a non-finite word that the §4.4 check has not yet reached is refused by both.
* **Gate 3's added arms and the census hooks** are beyond the brief's letter.
* **Reproduce gate 7.** Apply only the diagnosis patch's `limiter-diag-2-harness/` into a worktree
  of each commit, then `cargo build --release --features console --bin ld2`, then
  `LD2_SHAPES=HotLinked,HotDualMono,QuietLinked,RampLinked ld2 kbench NAME 4 800` per pass. For V8,
  use `web.mjs time` from `verify-limiter-2-harness.patch`. A native run takes 16 s under the lock
  and a V8 run 50 s.
* **Scratch commits.** The code commit was made off-branch in the scratch worktree first. The branch
  was fast-forwarded to it after gate 7 passed.

## Sol attempt 1 verdict: PASS

Sol, 2026-09-28. I judged `94fc8675` (code `bbdf6822`) merged onto the current batch head
`27cf2413`. The batch head moved during the review: it now adds #999 (the EQ and `lane` kernels)
after `d09d5024`, the head the branch had merged. The merge is clean and was made in scratch only.
Host AMD EPYC 7313P, cpu 31 under the timing lock, `CARGO_INCREMENTAL=0`. Nothing was pushed, and
all scratch worktrees and target directories were deleted.

### The questions

1. **Exactness: class A holds everywhere I could reach.**
   * **My own differential against the #1013 kernel.** I built the diagnosis rig on `bfec4bba` and
     on `bbdf6822`, and gave it a second generator of my own beside #990's.
     * My generator draws every launch rate, blocks of 1-256 frames (half of them variable), and
       lookaheads concentrated at the thresholds: 0, 0.67, 5, 9.8-9.99 and 10 ms.
     * It draws symmetric and asymmetric channel windows under `Maximum` and `DualMono`, plus
       ragged and partial banks.
     * It draws retargets of one channel (unlink) and of both, resets, and desymmetrize and
       mono-collapse runs.
     * It restores the whole bank with the van Herk phase moved to 0, 1, 31-33, `Wb - 1`, near
       `N`, or out of range (refused by both arms), sometimes with a new prefix.
     * It draws hostile audio: NaN, ±inf, `±1e30`, `-0.0`, subnormals and limit-edge words.
     * Every block folds every output word, report, payload, observation and restore verdict.
   * **Coverage, all identical.**

     | build | scenarios |
     |---|---|
     | Native release, per width (W8, W4, scalar) | 140,000 of mine and 140,000 of #990's at 96 blocks, plus 7,000 of mine at 1,000 blocks |
     | Native dev (overflow checks and `debug_assert`s on) | 7,000 per width, plus 4,200 of #990's at W8 |
     | V8, the rig built for wasm32 with simd128 (W4 and scalar) | 56,000 of mine and 28,000 of #990's |

   * **The fuzz discriminates the new paths.** Each of these mutations of the change is red in
     every 2,000-seed chunk, at W8 and at scalar:
     * the epilogue reading the wrong oldest slot;
     * the right channel's `+inf` preset removed;
     * the wrapped expiring stream shifted by one slot (wrong words, not a panic);
     * the wrapped newest stream shifted by one slot (a panic).
     The right-preset mutant built for wasm fails 52 of 200 scenarios at W4 and 100 of 200 at
     scalar.
   * **M12 is equivalent under my fuzz too.** With the walk also running under
     `DISPATCH_RAMPING`, 70,000 scenarios of each generator at each width stay identical. That is
     proof 3 (the passes commute, with each ramp advanced in its own pass) tested with ramps
     actually moving.
   * **The proofs check out against the code.**
     * The cut gives `steady + 1 <= Wb - phase <= Wb` by itself, so facts 2 and 4 hold at any
       window, not only at `Wb >= 32`.
     * `phase < Wb` is enforced by restore (`phase as usize >= window` is refused) and kept by
       every advance.
     * Every slice in `linked_steady_streams` is in range under its three preconditions.
2. **The ramping arm is token-identical.** I tokenised the arm myself, with comments and
   whitespace ignored.
   * It is the base's segment loop body, 581 tokens, followed by `run`.
   * With the arm substituted back, the whole function differs from the base only by the two
     `targets_*` declarations (24 tokens).
   * The +2 % allowance therefore applies.
3. **Codegen.**
   * **The stream loop.** At `Simd8` and at `Simd4` it is 23 instructions with one backward
     `jne`, and no other branch, call or `ud2`.
     * The `minps` operand order is D8's.
     * The slice-bound checks sit in the setup, outside the loop, where the stream facts make
       them unreachable.
     * The other loops quoted in the evidence have `nested = 0`: the checked fallback 66, dual
       pass 1 107, linked pass 2 35, dual pass 2 49.
     * In the shipped wasm the stream loop has one `br_if`, no `unreachable` and no call.
   * **The roster.** "true-peak-limiter f32x4 dual" goes from 878 vector / 0 scalar to
     913 / 0.
     * The collapsed row stays at 424 / 0.
     * The kernel count stays at 15 (minimum 11).
     * The render, meter-poll and command-submit analyser reports are byte-identical.
   * **What grows.** Only two functions change:
     * `LimiterCore<f32x4>::process_block` gains 35 vector ops and no scalar ones.
     * `LimiterCore<f32>::process_block`, the scalar lane, gains 35 scalar ops. That is its own
       arithmetic, not a vector kernel scalarising.
   * **So rule 3 holds.**
4. **Gate 7 (A4): passes.**
   * **The arms.** Rig `5e13e01e…` / `a7be6366…`; `host_web.wasm` `0fc45359…` / `f4e98eb4…`, the
     evidence's bytes.
   * **Native.** 4 passes of 4 rounds, forward then reverse, minimum over 16 rounds, change
     against slice 1:

     | row | run 1 (load 5.9-6.3) | run 2 (load 5.0-5.2) | run 3 (load 2.3-2.4) | merge vs `27cf2413` (load 14.3-14.9) |
     |---|---:|---:|---:|---:|
     | `Simd8` `HotDualMono` | -8.5 % | -7.8 % | -8.7 % | -5.6 % |
     | `Simd8` `HotLinked` | -6.3 % | -6.0 % | -6.4 % | -6.9 % |
     | `Simd8` `QuietLinked` | -3.4 % | -6.4 % | -7.0 % | -5.5 % |
     | `Simd8` `RampLinked` | +1.2 % | +0.5 % | -0.4 % | +1.3 % |
     | `Simd4` `HotDualMono` | -5.4 % | -5.2 % | -5.3 % | -3.7 % |
     | `Simd4` `HotLinked` | -5.1 % | -5.8 % | -5.6 % | -3.9 % |
     | `Simd4` `QuietLinked` | -5.7 % | -5.1 % | -4.7 % | -4.1 % |
     | `Simd4` `RampLinked` | -0.4 % | +0.4 % | -0.4 % | +0.3 % |

   * **V8.** Three Node processes each, mean of per-process medians, with the paired per-process
     changes in brackets:

     | isolate | run 1 (load 5.1-5.4) | run 2 (load 4.4-4.7) | run 3 (load 5.5-6.1) | merge (load 13.1-14.1) |
     |---|---:|---:|---:|---:|
     | `lim - bi` | -9.1 % [-9.5 -9.2 -8.7] | -9.3 % [-8.6 -8.9 -10.4] | -9.6 % [-8.9 -9.5 -10.4] | -8.5 % [-8.7 -7.9 -9.0] |
     | `limdm - bi` | -2.3 % [-2.9 -2.4 -1.5] | -0.6 % [-2.2 **+4.0** -3.7] | -3.2 % [-2.4 -3.0 -4.4] | -2.4 % [-2.9 -2.3 -2.0] |
     | `console - nolim` | -8.2 % [-8.9 -8.3 -7.5] | -9.1 % [-8.4 -8.4 -10.5] | -10.0 % [-9.4 -9.2 -11.3] | -8.5 % [-7.5 -8.4 -9.6] |

   * **On the merge.** The limiter's wasm function bodies and its native `process_block` listings
     (addresses normalised) are identical to the branch's. So #999 does not move this kernel.
5. **Other gates, on the merge onto `27cf2413`: green.**
   * The limiter crate (54 passed, 1 ignored), in dev and in release.
   * `limiter_linked_session` at all three widths, and `console-workload` (58 passed), in dev and
     in release.
   * `run-wasm-gates.sh`: G5 has 142 cases and 0 mismatches on every leg, the detector stays
     resident, and the V8 spill gate is ok.
   * `check-web-audioworklet.sh`, with the stale pin replaced by the merge module's digest
     (`c75de10e…`) in scratch, then restored.
   * The realtime, lane, env-vocabulary and workspace policy scripts.
   * `cargo fmt --check`, workspace clippy (`-D warnings`) and `cargo doc`.
   * The V8 digests on the diagnosis sessions are unchanged. Gate 3 passes with `bfec4bba`'s
     `lib.rs` in dev and in release, so its pin is the base's. M5, re-run here, is red on 5
     tests, as recorded.

### Findings

1. **Info: `RampLinked` passes only under the provisional allowance.**
   * **The numbers.** `Simd8` read +1.2, +0.5 and -0.4 % against slice 1, and +1.3 % on the
     merge.
   * **The cause.** The arm is token-identical, so this is codegen movement: its fused loops grow
     by 0-3 instructions.
   * **Failure scenario.** The owner rules the allowance to +0 %. This row then fails in three
     runs of four, on a path whose source did not change.
2. **Info: `limdm - bi` has little margin against V8 placement noise.**
   * **The outlier.** In my run 2, one process rendered the change's `limdm` instance at
     161.1 µs, against 153.6 and 153.8 µs in the other two. That paired value is +4.0 %.
   * **The effect.** It pulls the mean to -0.6 %. The other runs read -2.3 to -3.2 %.
   * **Failure scenario.** One more such outlier turns A4's "faster" criterion red for a slice
     that is faster in 11 of 12 paired processes.
   * **Fix, if wanted.** Judge on paired per-process deltas.

## Owner rulings (2026-09-28)

* The `RampLinked` allowance of slice 1 + 2 % is accepted: "If the implementation is genuinely
  better, I'm okay with 2% slower."
* No target-specific code: the wasm-only two-frame detector is not pursued ("No target-specific
  code please").
