# Limiter: end segments at van Herk completion and run the release recursion as its own pass

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
