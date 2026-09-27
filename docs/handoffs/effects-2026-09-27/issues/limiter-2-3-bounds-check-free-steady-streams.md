# Limiter: run a linked pair's steady frames over bounds-check-free ring streams

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
