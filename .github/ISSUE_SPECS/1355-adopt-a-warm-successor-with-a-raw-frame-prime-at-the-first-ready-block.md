# Adopt a warm successor with a raw-frame prime at the first ready block

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-12, D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): render's
readiness check and the move-mode adoption with its prime. The control plane's classification,
publication and record are *Classify a latency-growth edit and publish its warm successor from the
control plane* (#1403); the outcome word is *Duck-swap the strips a latency growth restarts, and
fall back to the transition when a warm successor cannot adopt* (#1397). Code anchors verified on
`main` at `6fb211594`.

## Product outcome

In host-core, a structural edit that grows a carried node's latency by up to `P` swaps in with no
gap and no jump.
- The warm successor W (#1354) is published `Primed { not_before, lead_blocks }` (#1311). The
  control plane does that in #1403; this slice's tests publish through #1311 D6 directly.
- Render keeps playing the predecessor A, unchanged, until the first block S at which every source
  W carries has its next `P + q` frames queued and playable (`q` is the quantum).
- In that block render claims W, adopts it in move mode, and fills the source-claim lines from A's
  pending frames plus a raw-frame prime of the next `P / q` blocks of each carried source.
- Every carried path continues bit for bit. Nothing is rendered ahead, no state is copied, and no
  candidate is ever handed back by render.
- A live edit committed while W is pending applies at S. A structural edit or a declared stop
  supersedes W in the ordinary way.

## Context

- **The swap today.** `RealtimePlanOwner::enter_block`
  (`crates/engine/src/realtime/plan_exchange.rs:375`) takes a candidate, adopts the clock (`:418`)
  and runs the carry (`carry_from`, `:421`). `GraphExecutor::adopt_predecessor`
  (`crates/graph/src/lib.rs:3139`) moves the sources (`adopt_sources`, called at `:3161-3172`;
  the driver's implementation at `crates/source/src/lib.rs:1851`), then copies the input lanes.
  `PreparedPlanExecutor` is at `crates/engine/src/realtime/plan.rs:269`, and its
  `adopt_predecessor` default at `:311`.
- **Sources at render.** The executor begins the source set at the block's sample
  (`crates/graph/src/lib.rs:3255-3256`). The driver calls `consumer.begin_block_at(first_sample)`
  for each occupied entry (`crates/source/src/lib.rs:1721`), reads a vacant entry as `+0.0`
  (`:1715-1718`) and lends played planes (`played_planes`, `:1793`).
- **Producers.** Host-core keeps each source's `HostChunkProvider` in its `SourceControlSet`
  (`crates/host-core/src/source.rs:25`). A commit moves the persisting producers into the
  successor's set, and `adopt_persisting` (`:286`) moves them back.
- Inputs from earlier slices:
  - #1311: `PlanAdoption::Primed { not_before, lead_blocks }`, with its atomics that render loads
    before it claims (D1, D2), the readiness hook `PreparedPlanExecutor::prime_ready(&self,
    block_start, lead_blocks)` (default `false`) that render calls on the active plan (D3), and
    the claim and adoption in one step (D4);
  - #1320: `PcmSourceConsumer::prime_ready` (D2), `prime_block_at` (D3) and the ring's
    `prime_required` flag (D4);
  - #1354: warm preparation, `warm_lead` over the carried nodes, `lead_samples()` and
    `WarmUnavailable`;
  - #1287's first slice: source-claim lines (#1287 D1);
  - *Carry source-claim lines across a plan swap and fill a grown line for a prime* (#1402): their
    move-mode carry keyed by claiming node and source (D1), and the fill routine for the lemma's
    L2 and L3 (D2);
  - #1396: `source_read_offset` (D1), its inheritance (D2) and `PlanPublisher::render_clock()` (D4);
  - #1314: the per-epoch revision cell (D1) and the watermark advance (D3, D5).

## Decisions frozen for this slice

- **D1. Offset.** W's `source_read_offset` is A's plus `lead_samples` (`P`), set at warm
  preparation on top of #1396 D2's rule. `ΣP` is the running plan's offset.
- **D2. Added sources start at S.** A source W creates has its own fresh ring and consumer in W.
  Render never runs a candidate before it claims it, so that consumer begins its first block at S.
  - Every chunk, generation and seek the host sent after the commit waits in its ring until then.
    A host that starts an added stem as the C ABI header says (`miso_engine_v1_source_seek_at`
    after OK, `crates/capi/include/miso_engine_v1.h:91`, `:96-108`) needs nothing new.
  - If W is withdrawn, the ring is unconsumed, so it passes *Prepare a successor across a
    withdrawn candidate plan* (#1344) D5 as a donation.
  - An added source is never primed and never flagged.
- **D3. Publication is #1403's.** `publish_primed`, the `PrimedCandidate` record and its
  lifecycle moved to #1403 (D3, D4) in round 5. This slice's gates publish W with #1311 D6's
  `reserve_replacement(plan, Primed { not_before, lead_blocks: P / q })`, after the test stores
  `prime_required` (#1320 D4) on each ring W carries and clears it on every other ring A renders.
- **D4. Readiness, on the active plan, before the claim.** This slice implements #1311 D3's hook
  for the graph plan.
  - For a due `Primed` candidate, render calls `prime_ready(S, lead_blocks)` on the **active**
    plan, with `S` the block's start (#1311 D2, D3).
  - `GraphExecutor::prime_ready` computes `source_sample = S + O`, with `O` its own source-read
    offset (#1396 D1), and forwards to a new defaulted
    `GraphPreparedSourceSetDriver::prime_ready(&self, source_sample, lead_blocks) -> bool`
    (default `false`). The source driver returns `true` only if every occupied entry whose
    consumer reads `prime_required()` passes `prime_ready(source_sample, lead_blocks)` (#1320 D2).
  - Only then does render claim (#1311 D2). Otherwise it renders A as usual and looks again at the
    next block.
  - Render keeps no in-flight state, takes nothing back and never returns a candidate. While W is
    pending, the cost per block is at most `lead_blocks + 2` loads per flagged ring.
- **D5. Adoption and prime, in the same block.** The claim adopts W by pointer swap and runs the
  move-mode carry as today (`:418`, `:421`). In `GraphExecutor::adopt_predecessor`, after
  `adopt_sources` (`crates/graph/src/lib.rs:3161-3172`) and the claim-line carry (#1402 D1):
  - for each moved source and each `j` in `0..lead_blocks`, the driver calls
    `prime_block_at(S + O + j * q)` on its consumer, through a new
    `GraphPreparedSourceSetDriver::prime_block_at(&mut self, source_index, sample)`;
  - it hands each primed block's planes to #1402 D2's fill: every claim line of that source (L2),
    and every line of an `Input`-tap sidechain edge from a restarted strip into a carried node (L3);
  - block S then renders as an ordinary block of W, whose driver begins its sources at
    `S + O + P`.

  W is prepared against the plan render runs (#1310 D1), so its carry cannot mismatch. If it does
  anyway, `carry_mismatch_count` counts it as today, a debug assertion fires, and no prime runs.
- **D6. Edits while W is pending.** W is an ordinary pending candidate.
  - A live edit goes to the newest candidate's cells and applies at S (#1053 D7). The retarget
    records are written once, at preparation, in move mode (#1277 D5), so nothing overwrites it.
  - A structural edit supersedes W by #1310, including D5 step 5: `adopt_persisting` returns to
    the running epoch the producers of sources A renders and the superseding plan removed.
  - A host-declared stop supersedes W with a plain rebuild through #1310 (#1323 D2). Its revision
    completes `EXACT`, with `SUPERSEDED` only when W itself folded in earlier revisions (#1323 D4).
  - #1403 gate 3 checks all three through the control plane.
- **D7. Outcome.** A primed adoption completes `EXACT`, the watermark's default (#1314 D5). The
  outcome word and its only non-`EXACT` writer, the transition, are #1397's (round-5 M6).
- **D8. Acked-batch question.** Render never drops a candidate. It claims W only when ready, and
  until then W stays withdrawable in its cell (#1311 D2). The prime plays only blocks A's consumer
  would have played, and leaves every command queued (#1320 D3). An added ring releases nothing
  before S (D2). A live edit written to W's cells applies at S. A withdrawn W is kept until its
  replacement is prepared from the committed model (#1310). An ack can never precede a drop.

## Deliverables

1. D1 in `crates/host-core/src/prepare.rs`.
2. D4 and D5 in `crates/graph/src/lib.rs` (the two driver methods, `GraphExecutor::prime_ready`,
   the prime in the source section) and the source driver's implementations in `crates/source/src/lib.rs`.
3. Gates 1-6 in `crates/host-core/tests/warm_successor.rs`, which #1354 creates.

## Authorized paths

- `crates/graph/src/lib.rs` (the source section and the two driver methods)
- `crates/source/src/lib.rs` (`SourceGraphSourceSetDriver`'s two new methods only)
- `crates/host-core/src/prepare.rs`
- `crates/host-core/tests/warm_successor.rs` (created by #1354)

## Non-goals

- Warm preparation and its refusals (#1354). The duck of restarted strips and the transition
  (#1397). The deadline and ring headroom (#1358).
- Control-plane classification, `Primed` publication and the `PrimedCandidate` record (#1403).
  The outcome word (#1397). The C ABI and the browser (#1360, #1361).

## Objective gates

Unless a gate says otherwise: A is the two-track fixture of
`crates/host-core/tests/successor_swap.rs`. Its caps
(`crates/host-core/tests/support/successor.rs:33`) give every ring 4,096 frames, below today's
5,120-frame default at 48 kHz and quantum 128. So every gate sets each source ring explicitly
through `HostPrepareCaps::source_ring_frames` (`crates/host-core/src/prepare.rs:106`) to
`stall_ring_frames(fs, q)` (#1354 D1, today's default ring) plus #1286 D3's `P_MAX_SAMPLES(fs)` plus
`q`. At quantum 128 that is 6,912, 7,296, 12,800 and 14,080 frames at 44.1, 48, 88.2 and 96 kHz
(4,736 + 2,048 + 128; 5,120 + 2,048 + 128; 9,088 + 3,584 + 128; 9,856 + 4,096 + 128), so #1354 D2's
headroom check passes at every growth. Every source is kept queued at least `P + q` frames ahead.
"Equals A continued" means bit-identical, every block, to A rendered on with no swap and the same
host commands, timed on the source-read clock. Each gate runs at all four launch rates and both bank
widths.

1. **Added-strip growth.** W adds a muted track with a true-peak limiter insert and silent input.
   The swapped output equals A continued. Block `j` of it also equals block `j + k` of a fresh W
   that is compiled with the same floors, has offset 0 and is fed the same PCM from frame 0. The
   watermark reports `EXACT` at S. Second shape: the added track's limiter feeds a submix whose
   arrival grows while the output's does not.
2. **Growth, rebuild, growth.** Gate 1's growth, then an ordinary rebuild that adds another muted
   track, then a second growth (offset `2P`). The output equals A continued throughout.
3. **Input-tap sidechain from a restarted strip.** Track 1 is muted in the model. Track 2 carries a
   compressor whose routed sidechain reads track 1's `input` tap. The edit inserts a true-peak
   limiter on track 1, which restarts track 1 and leaves track 2 carried. The output equals A
   continued.
4. **Readiness.**
   - Frames past each consumer are withheld: no adoption, and the output equals A continued.
   - The frames arrive: W is adopted at the first block where every carried ring is ready
     (checked against #1320 D2), and the output stays equal.
   - Render paused for any wall time: W stays `Full`. Render resumes with frames queued, and the
     output is still equal.
   - A `seek_at` anchored inside `[S + O, S + O + P + q)`: adoption waits until after the seek has
     applied, then the output equals A continued with the same seek.
5. **`input`-tap sidechain from a restarted submix at exactly `a + P`.** Tracks 1 and 2 route to
   submix B. B and track 3 route to the output. Track 3 carries a compressor whose routed sidechain
   reads B's `input` tap. Tracks 1 and B are muted in the model, and A has no latency anywhere. The
   edit inserts a true-peak limiter on track 1 and a compressor (zero latency,
   `crates/compressor/src/lib.rs:295`) on B, which restarts tracks 1 and B. Track 2 is carried at
   `a + P`, so B's `Input` stage arrives at exactly `a + P` (#1354's check), and track 3 stays
   carried. The output equals A continued. (#1354 gates the `g < P` case, which restarts track 3.)
6. **Realtime.** Every pending block's readiness check, and the adoption block of a fourth growth
   whose `ΣP + P` equals #1286 D3 item 1's `P_MAX_SAMPLES(fs)`, make zero allocations and frees on
   the render thread (`bench_support::alloc`'s current-thread counters). Locks and syscalls are
   checked by the realtime policy scripts on these functions and by #1360 D6's `audit capi` leg.
   - The four growths are #1286 D1's growth chain, the one construction: growth `j` (1 to 4) adds
     to track 1 one true-peak limiter (`L = fs / 100 + 6`,
     `crates/true-peak-limiter/src/lib.rs:236-242`) plus `c(fs)` soft clippers (31 samples,
     `crates/soft-clip/src/lib.rs:178`), with `c` = 2, 0, 0 and 1 at 44.1, 48, 88.2 and 96 kHz.
     A has no latency, so track 1 arrives at `j · (L + 31c)` (`j ·` 509, 486, 888 and 997), each
     growth has `P = ceil_q(L)` (512, 512, 896 and 1,024), and `ΣP` after the fourth is 2,048,
     2,048, 3,584 and 4,096, which is `P_MAX_SAMPLES(fs)`. #1286 D1's table shows the arithmetic
     per rate and why no other `c` works (track delay cannot pad an arrival,
     `crates/graph-compiler/src/pdc.rs:12-17`). The test asserts that `ΣP` equals
     `P_MAX_SAMPLES(fs)` first; a mismatch fails it.
7. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support --test warm_successor`
   - `cargo test --locked -p graph --features graph/test-support`, `cargo test --locked -p source`
   - `cargo test --locked -p engine --features realtime-audit`, `cargo test --locked -p capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: an adoption that does not read sources at `S + O + P`, or a prime that leaves `+0.0` in
  the claim lines, repeats or gaps `P` frames. Red.
- Gate 2: claim lines re-created at rest by the ordinary rebuild, or a second prime that fills
  only the new `P`, drops `ΣP` frames. Red. #1283 D4's fill of a grown line (A's pending frames at
  the head, then `+0.0` where the prime belongs) and a grown line left at rest each diverge first
  at block S; that is recorded red once on each mutant as PR evidence.
- Gate 3: an `Input`-tap sidechain line filled with zeros instead of A's pending frames and the
  prime puts a hole in track 2's detector, so its gain steps at S. Red.
- Gate 4: a check that claims before every ring is ready underruns in the prime; one that ignores
  the held seek primes across it. Red.
- Gate 5: a C1 that restarts the consumer of every restarted submix's `input` tap ducks track 3;
  a sidechain line from B's `Input` stage, or track 2's line into it, re-created at rest instead
  of moved puts a hole in track 3's detector at S. Red.
- Gate 6: a prime that stages blocks in a buffer allocates on render. Red.

## Dependencies

- *Grow latency during playback by adopting a primed warm successor* (#1287), its first slice:
  claim lines.
- *Carry source-claim lines across a plan swap and fill a grown line for a prime* (#1402): their
  carry and the fill routine.
- *Let a source consumer check and replay its next blocks for a prime* (#1320).
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354).
- *Give a plan a source-read clock that leads its render clock* (#1396).
- *Adopt a successor plan no earlier than a scheduled sample* (#1311): `Primed`.
- *Let the control thread withdraw an unadopted candidate plan* (#1343).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310), including D5 step 5.
- *Prepare a successor across a withdrawn candidate plan* (#1344), D5.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Carry fader, mute and pan ramps across a plan swap* (#1277), D5.
- *Carry meter and effect observation state across a plan swap* (#1327).
- *Carry spectrum capture state across a plan swap* (#1395).
- *Reset latency floors at a host-declared discontinuity* (#1323), D2: the rule D6 cites.
