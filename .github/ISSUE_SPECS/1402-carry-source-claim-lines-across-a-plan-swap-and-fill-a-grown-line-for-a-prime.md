# Carry source-claim lines across a plan swap and fill a grown line for a prime

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8 (round-5 amendment)).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): the
claim-line carry and the lemma's fill rules L2 and L3. Split from #1287's first slice, which keeps
the compiler and runtime claim lines and the B1 compile gate. Code anchors verified on `main` at
`6fb211594`.

## Product outcome

A source-claim compensation line keeps its pending raw frames through a plan swap. An ordinary
rebuild after a warm growth (an added muted track, a fader-only prepared change) no longer empties
the claim lines, so no source drops out for `P` samples after it (round-4 M1). A line whose
length grows is filled exactly as the lemma says, by one allocation-free routine that *Adopt a warm
successor with a raw-frame prime at the first ready block* (#1355) calls with the primed blocks.

## Context

- #1287 D1 adds the source-claim line: a compensation line of `floor` samples on each claim of a
  source-reading node whose floor exceeds its incoming arrival. It runs on the PDC delay kernel
  (`pdc_delay_block`, `crates/lane/src/kernels.rs:990`): a ring and a cursor per lane.
- The carry program today holds only the source pairs (`GraphCarryProgram`,
  `crates/graph/src/lib.rs:2757-2762`). *Carry compensation lines across a plan swap* (#1283)
  adds the edge lines, keyed by `GraphEdgeId` (its D1), moved when equal (D2); its D3 leaves
  claim lines to this issue.
- Successor preparation and the carry join are in host-core: `SuccessorBase`
  (`crates/host-core/src/prepare.rs:641`), `prepare_host_runtime_successor` (`:909`), and the
  `test-support` successor entry points (`:946`, `:967`).
- The lemma (#1287, binding): L2 fills a W claim line of length `λ'` with the last `λ'` samples of
  `[A's λ pending] ++ [prime]`, where the prime is `k = P / q` consumer blocks on A's own schedule.
  A grown line on a carried node (`λ' = λ + P`) then emits exactly A's samples; a line on a
  restarted strip (`λ' = λ`) takes the shifted fill; an added line has `+0.0` only at its head.
  L3 fills an `Input`-tap line from a restarted strip into a carried node, grown by `P`, with A's
  line pending, A's claim pending and the prime.

## Decisions frozen for this slice

- **D1. Claim-line carry.** #1283's carry location table gains claim lines, keyed by (claiming
  node, claimed source's stable ID).
  - A key present in both plans carries in move mode. With equal lengths and an unchanged
    source-read offset (every ordinary rebuild) that is a swap of the rings and the cursor (#1283
    D2). With a grown length the program records the line for D2's fill at adoption.
  - A claim line holds raw source frames and no processing state, so it carries for a restarted
    strip too. *Duck-swap a strip whose state cannot continue across a plan swap* (#1324) D2
    excludes it from a restart.
  - A key only in W is an added line (L2's N case). The carry takes nothing on a mismatch, like
    the rest of the program.
- **D2. Fill routine.** One allocation-free routine in `crates/graph` fills a W line from A's
  pending samples (and, for L3, A's claim pending) and a prime slice of `P` frames, per L2 and L3,
  for both lanes of a dual-mono claim. Its order for a grown line is A's pending at the read end,
  then the prime. #1283 D4's head-aligned copy emits the pending samples, then `+0.0`; this
  routine puts the prime where that copy puts `+0.0`. This slice tests it with given prime slices;
  #1355 calls it with the replayed blocks.
- **D3. Host-core join.** The inventory records each claim line's key and length, and the carry
  join adds the claim-line rows beside #1283's. A `test-support` preparation takes an explicit
  floor map, so a gate can prepare a plan with lead floors before *Prepare a warm successor whose
  carried nodes lead the predecessor by P* (#1354) computes them.
- **D4. Realtime.** The move and the fill run in the adoption block with no allocation, free, lock
  or syscall; the program's tables are sized at preparation.

## Deliverables

1. D1-D2 in `crates/graph` (the location table, the program section and the fill routine), with
   unit tests (gate 2).
2. D3 in `crates/host-core/src/prepare.rs`.
3. `crates/host-core/tests/latency_growth.rs` (new) with gates 1 and 3.

## Authorized paths

- `crates/graph/src/lib.rs` and `crates/graph/src/runtime.rs` (the claim-line carry and the fill
  routine only)
- `crates/host-core/src/prepare.rs` (the claim-line rows of the carry join, and the
  `test-support` preparation with an explicit floor map, only)
- `crates/host-core/tests/latency_growth.rs` (new)
- `.github/ISSUE_SPECS/1402-carry-source-claim-lines-across-a-plan-swap-and-fill-a-grown-line-for-a-prime.md`

## Non-goals

- No compiler change: the claim lines themselves are #1287's first slice.
- No `warm_lead` or `WarmUnavailable` (#1354). No readiness check, prime or adoption (#1320,
  #1355). No source-read offset change (#1396).

## Objective gates

1. **M1, carry level.** W1 is #1287 gate 2's shape, prepared through D3's `test-support` hook: a
   two-track session with every node floored at its arrival plus `P`. It stands in for a plan after
   a warm adoption, with claim lines of length `P`. After 8 blocks, an ordinary successor of W1 that adds a muted track
   is prepared (floors from W1's inventory) and adopted in move mode. Every claim line carries
   bit-exactly, and the next 64 blocks equal W1's continued run (W1 never swapped), bit for bit.
   All four launch rates, both bank widths. The render-level M1 gate (a growth, an ordinary
   rebuild, then a second growth, equal to A never swapped) is #1355's gate 2.
2. **Fill rules.** Unit tests of D2 with given pending and prime slices:
   - a C line (`λ -> λ + P`) emits A's pending samples, then the prime;
   - an R line (`λ -> λ`) emits the shifted fill;
   - an N line with `λ' > P` emits `+0.0`, then the prime;
   - an L3 line emits A's line pending, A's claim pending, then the prime.

   Each at every predecessor cursor position modulo the block, both lanes.
3. **Realtime.** Gate 1's adoption block makes zero allocations and frees on the render thread
   (`bench_support::alloc`'s current-thread counters, after warm-up).
4. Commands:
   - `cargo test --locked -p graph --features graph/test-support`
   - `cargo test --locked -p host-core --features host-core/test-support --test latency_growth`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: claim lines left out of the carry table restart at rest, so every source drops out for
  `P` samples after the rebuild (round-4 M1). Red.
- Gate 2: #1283 D4's head-aligned copy in a grown line (A's pending samples, then `+0.0` where
  the prime belongs), or an L3 fill without A's claim pending, emits the wrong samples. Red.
- Gate 3: a fill that stages the prime in a buffer allocates on render. Red.

## Dependencies

- *Grow latency during playback by adopting a primed warm successor* (#1287), first slice: the
  source-claim lines.
- *Carry compensation lines across a plan swap* (#1283): the carry location table.
- *Keep every node's latency from dropping during playback* (#1285): the floor map and the
  inventory's recorded arrivals.
