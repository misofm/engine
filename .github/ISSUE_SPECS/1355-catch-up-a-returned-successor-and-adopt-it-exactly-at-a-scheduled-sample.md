# Catch up a returned successor and adopt it exactly at a scheduled sample

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 steps 3-4, D15-12, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287), W3, W4, the drain deferral of W5 and the bound of W6. Code anchors verified
on `main` at `6fb211594`.

## Product outcome

In host-core, a structural edit that grows a node's latency by up to `P` swaps in with no gap and
no jump.
- A warm successor returned at B (#1354) is rendered forward off the render thread through the
  ring peeks, then adopted exactly at a sample S.
- Every unchanged path continues bit for bit.
- Live edits written to the successor during the catch-up apply exactly at S, never earlier.
- A source the edit adds starts at S, exactly as it does in an ordinary swap: its first block,
  its generation and any seek the host sent after the commit are its consumer's own.
- The watermark reports the revision's outcome from the word the control thread gave its epoch.

## Context

- The graph driver reads every source at the block's sample
  (`consumer.begin_block_at(first_sample)`, `crates/source/src/lib.rs:1721`). The consumer's own
  `next_frame` decides which PCM plays; the block sample decides only when an anchored seek applies
  (`begin_block_at`, `:1115`).
- `PreparedRenderPlan` has one clock, `next_absolute_sample` (`crates/engine/src/realtime/plan.rs:609`),
  passed to the executor as `RenderTime` (`:943`). `render_contiguous` (`:870`) refuses any other
  sample, so the render clock is the host's and never jumps.
- The graph's source set (`SourceGraphSourceSetDriver`, `crates/source/src/lib.rs:1611`) reads a
  vacant entry as `+0.0` and reports an underrun (`begin_block`, `:1700`, the vacant branch at
  `:1715-1718`); `played_planes` (`:1793`) lends a played block's planes.
- `RealtimePlanOwner::enter_block` (`crates/engine/src/realtime/plan_exchange.rs:375`) claims
  candidates; `PlanPublisher` (`:67`) is the control side.
- Inputs from earlier slices:
  - *Give a plan a source-read clock that leads its render clock* (#1396, split from this
    issue): `source_read_offset`, its inheritance (D2), and `PlanPublisher::render_clock()` (D4);
  - #1321's `OffThreadPlanRenderer::render_slice`;
  - #1320's `PcmSourcePeek` reads (D5), its lifecycle (D3) and `advance_past_peek` (D8);
  - #1354's `CopyAndReturn`, `warm_lead` and `WarmUnavailable` (D1, D3);
  - #1311's `ExactlyAt(S)`, else return (D3);
  - #1314's per-epoch revision cell (D1) and watermark advance (D3, D5);
  - #1322 D3: after a copy, move-mode adoption runs only the source section;
  - #1327 D4: observers publish only windows the predecessor did not publish;
  - #1277 D5: in copy mode the retarget records stay on the prepared successor and are written at
    its first publication (#1356 D3).

## Decisions frozen for this slice

- **D1. Offset of a warm successor.** Its `source_read_offset` is its predecessor's plus
  `lead_samples` (#1354 D3), set at warm preparation on top of #1396 D2's rule. `ΣP` is the
  running plan's offset.
- **D2. Peek-backed entries.**
  - At warm preparation, each carried source's successor entry takes its ring's peek
    (`take_peek`, #1320 D3) instead of staying vacant. During the catch-up the driver reads it as
    it reads a consumer: `played_planes` lends the peek's planes.
  - Before each block the catch-up checks that every peek holds the block's frames; `NotYet` ends
    the slice without rendering.
  - `Diverged` abandons the catch-up. The catch-up takes every carried peek out of the
    successor's entries, `abandon`s it and drops it on the control thread (#1320 D3, D8), so
    `peek_outstanding` is clear and the next warm preparation's `take_peek` succeeds. It keeps
    the successor itself, without peeks, as the donor of D10, which re-prepares the edit as a new
    warm successor from a new B, until #1358's deadline. The successor is dropped only after that
    donation, or kept if D10 is refused.
  - **An added source stays closed until adoption.** A source the successor creates fresh has no
    predecessor consumer, and the catch-up never reads it. Its entry is marked `added`, and
    `SourceGraphSourceSetDriver` gains `added_open: bool`, false on a warm successor and true on
    every other plan. While it is false, an added entry reads `+0.0`, `played_planes` returns
    `None` for its claims, its consumer's `begin_block_at` is not called, and it raises no
    underrun (no block of it is due yet). So the consumer begins no block before adoption: every
    chunk, the generation and every seek the host sent stay in its ring, and *Prepare a successor
    across a withdrawn candidate plan* (#1344) D5 accepts the ring as a donation. The off-thread
    catch-up and #1358's render-thread pre-roll read it the same way. No peek is taken on an
    added ring.
  - So a host that starts an added stem as the C ABI header says (`miso_engine_v1_source_seek_at`
    after OK, `crates/capi/include/miso_engine_v1.h:91`, `:96-108`) needs nothing new: the seek
    waits in the ring and the consumer applies it from its first block, with the lateness rule of
    the header if the anchor has passed. A seek never makes an added source diverge. An added
    strip's fade-in arm (*Fade in a strip that a swap adds during playback*, #1288 D3) sees no
    played planes before S, so it fires at the first block at or after `S + D`.
- **D3. Alignment.** The catch-up starts and fills exactly as #1287's recorded proof states. Per
  that proof:
  1. A prime phase reads source frames `[B, B + P)` through the peeks into each source-claim line,
     without advancing any node downstream of the lines.
  2. The successor then renders render samples `B ...` with sources read at
     `render + source_read_offset`.
  3. Its render clock equals the predecessor's throughout.
- **D4. Live-lane drains wait for adoption.** `PreparedRenderPlan` gains `live_lanes_open: bool`,
  false on a warm successor. While it is false the executor drains no live lane: no latest-target
  cell (#1312, #1345, #1346, #1347) and no live queue. `render_slice` never opens them. Render sets
  it true in the adoption block, just before it renders block S (D6; #1358's pre-roll sets it after
  its pre-rolled blocks, before the adopted block). So the retargets written at publication and
  the live edits written to the successor during the catch-up (#1356 D1, D3) apply at S, even
  when S is missed, the candidate returns, and the catch-up renders on. Ramps already running at B are state: the copy carries them and they continue.
- **D5. Publication.** The catch-up type is `host_core::CatchUp`, which owns the
  `OffThreadPlanRenderer`.
  - `CatchUp::service(&mut self, max_blocks) -> CatchUpState` renders while the successor's clock
    is behind `render_clock() + 2 * quantum`.
  - Then, in this order, it runs #1356 D3's publication writes (the kept retargets at the first
    publication only, and the revision word), then writes the epoch's outcome word (D8), then
    publishes `ExactlyAt(S)` with `S` the successor's next sample.
  - While the catch-up holds the successor, `CatchUp::set_revision(revision)` raises the revision
    word it will be published with (#1314 D2's control-held candidate rule); #1356 calls it.
  - A returned candidate stays in its cell until the catch-up takes it with `withdraw()`
    (`Withdrawal::Returned { reason: Late }`, #1311 D6); it goes back into the loop and is
    published again later. What was written stays written and undrained (D4).
- **D6. Adoption at S.** Render adopts by pointer swap. The move-mode source section, for each
  carried source:
  1. moves the consumer into the successor and calls `advance_past_peek(lead_samples)`;
  2. moves the peek out of the successor's entry into the retiring predecessor's entry, in
     exchange for the consumer (#1320 D3). The predecessor is reclaimed off render, and its drop
     ends the peeks.

  Observers take the predecessor's producers under #1327 D4. A ring whose advance refuses fails the
  whole section and moves nothing; the candidate is returned, never adopted. Then render sets
  `added_open` (D2), so each added consumer begins its first block at S, and opens the live lanes
  (D4).
- **D7. When no warm successor can be prepared.** Warm preparation adds two `WarmUnavailable`
  reasons to #1354's: `LeadBound` when `ΣP + P > p_max`, and `PeekOutstanding` when a carried
  ring's `take_peek` returns `None`. It never refuses a valid edit: the caller takes the transition
  (*Duck-swap the strips a latency growth restarts, and fall back to the transition when no
  catch-up can finish*, #1397, split from #1358). `p_max` comes from the warm configuration;
  #1358 D2 fixes its default.
- **D8. Per-epoch outcome word.** #1314 D1's per-epoch revision cell gains a sibling outcome word.
  `PlanPublisher::set_outcome(epoch, flags)` writes it before publication; it is `EXACT` unless
  written. When render adopts an epoch, the watermark advance it publishes (#1314 D3) ORs in that
  epoch's word and adds the covered revisions to that word's counter. A warm adoption at S writes
  `EXACT`. #1358 and #1397 write the fallback words.
- **D9. Acked-batch question.** A returned or abandoned successor drops no committed content: the
  edit stays committed and is published again or re-prepared. `advance_past_peek` releases only
  frames the successor consumed. An added ring releases nothing before adoption (D2). Edits held
  in the successor's cells cannot drain before S (D4). An ack can never precede a drop.
- **D10. Re-preparation inside `service`.** Whenever a `service` call prepares the committed model
  again (D2 here; #1358's transition; #1357 and #1359 reuse the rule):
  - it prepares against the running plan, with the displaced successor as donor (#1310 D2): the
    rings of sources the edit added are donated to the new candidate, never dropped or allocated
    again. They pass #1344 D5's check because the catch-up never opened them (D2), so every chunk
    and seek acked since the commit is still in them;
  - its model, base and lead are the ones submit already prepared successfully, and its floors are
    no higher, so the re-prepared plan's resource row is no larger than the displaced successor's,
    and host-core's per-plan ceilings pass again;
  - the control plane's cross-plan admission is not run here. The warm submit already admitted the
    re-preparation peak: the running plan plus the successor's row twice, with #1398 D4's model
    terms (*Size the C ABI's plan capacities and resource admission for a superseding candidate*,
    #1398 D4, wired at the warm submit by #1360 D1). The displaced successor is not released
    first, because it is the donor;
  - if it is refused anyway, that is a defect: the displaced successor and its donated rings are
    kept, `catch_up_reprepare_refusals` (saturating) rises, a debug assertion fires, and the next
    `service` call retries. The revision stays pending and visible. It never completes as nothing.

## Deliverables

1. D1 and D4 in `crates/engine/src/realtime/plan.rs` and the graph executor's drains.
2. D2 in `crates/source/src/lib.rs`.
3. D3, D5, D7 and D10 in a new `crates/host-core/src/catch_up.rs` section, beside #1321's
   renderer.
4. D6 and D8 in `crates/engine/src/realtime/plan_exchange.rs`, the watermark module of #1314 and
   the graph's source section.
5. The proof's alignment, with any correction, back in #1287's decision record.
6. Gates in `crates/host-core/tests/warm_successor.rs`.

## Authorized paths

- `crates/engine/src/realtime/plan.rs`, `plan_exchange.rs`, `watermark.rs` (#1314's), `mod.rs`
- `crates/source/src/lib.rs`, `crates/graph/src/lib.rs` (source section, claim-line prime, the
  live-lane drain switch)
- `crates/host-core/src/catch_up.rs`, `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs`
- `crates/host-core/tests/warm_successor.rs`

## Non-goals

- Held live edits (#1356), supersession (#1357), the deadline and fallbacks (#1358), and stop
  (#1359).
- Control-plane and C ABI wiring (#1360), and the browser (#1361).
- Gates 1-6 use existing sources; gates 7 and 8 cover a source the edit adds (D2).

## Hazards

- **Added-source ring depth.** An added ring is not consumed until adoption, as for any pending
  candidate today. The host may fill it to its configured depth; after that its submits get the
  existing typed `Full` until adoption. Nothing is dropped, and the catch-up never waits on it.

- **Size.** This is the largest stream C slice even after the #1396 split. If the prime phase (D3)
  needs a partial-render executor call, move it into its own slice before this one and report
  that.

## Objective gates

1. **Output growth, gap-free (the old gate 1).**
   - Predecessor A is the two-track fixture of `crates/host-core/tests/successor_swap.rs`.
     Successor B adds a muted track with a true-peak limiter insert and silent input.
   - With the copy after block 6 and the catch-up serviced from a second thread, block `j` of the
     swapped run equals block `j + k` of a fresh B. That fresh B is compiled with the same floors
     and offset 0 and fed the same PCM from frame 0. This holds for every `j`.
   - Run at all four launch rates and both bank widths.
2. **Submix growth (the old gate 2).** The same, with the limiter inside a submix whose arrival
   grows while the output's does not.
3. **Missed S keeps held writes for S.** In gate 1's setup, write a fader retarget record and a
   mute record into the successor's cells at publication, and force one return (render skips `S`). The catch-up renders 4 more
   blocks and publishes again at `S'`. The output equals a run in which both records are pushed to
   the successor just before its block `S'`, and the watermark reports `first_sample == S'` and
   `EXACT`.
4. **Outcome word.** An epoch whose word is set to `TRANSITION_FALLBACK` before publication
   advances the watermark with that flag and adds its covered revisions to
   `transition_fallback_count`; an unwritten word gives `EXACT`.
5. **No refusal, and the peek comes back.** Warm preparation with `p_max` one quantum below
   `ΣP + P` returns `WarmUnavailable::LeadBound`, and the model and revision are unchanged. After
   gate 1's adoption, once the predecessor is reclaimed, a second growth edit takes every peek
   again and reaches gate 1's equality with offset `2P`.
6. **Realtime.** The adoption block makes zero allocations and frees on the render thread
   (`bench_support::alloc`).
7. **Abandon.** A seek during the catch-up yields `Diverged`. The producer's admission depth returns
   to the configured value, and the re-prepared successor (D10) reaches gate 1's equality.
   - With an edit that also adds a source `c`, fed from frame 0 during at least 4 catch-up
     blocks before the divergence: `c`'s consumer has not begun a block, `check_donation` accepts
     it, and the re-prepared successor holds the same ring. From its adoption `c` plays every
     chunk submitted before and after the divergence, from frame 0.
   - The re-prepared plan's resource row is at most the displaced successor's, and no
     re-preparation refusal is counted. The second warm preparation's `take_peek` succeeds on
     every carried ring.
8. **An added stem started by `seek_at` after OK.** Gate 1's edit also adds source `c`. Right
   after the submit returns OK, while the catch-up runs, the host calls
   `seek_at(c, 2, F, A)` with `A` two quanta past the last rendered block and submits generation 2
   from `F`. The catch-up never reports `Diverged`, the adoption is at S with `EXACT`, and gate 1's
   equality holds on every other path. From S, `c`'s played planes equal those of a lone consumer
   of a ring fed the same commands and chunks that begins its first block at the same source-read
   sample, including the header's lateness rule when `A` has passed.
9. Commands: those of #1354, plus `cargo test --locked -p source` and `cargo test --locked -p capi`.

## Test value

- Gate 1: an adoption that does not advance the consumers by `P`, or a prime that leaves `+0.0` in
  the lines, repeats or gaps `P` frames. Red.
- Gate 2: a lead sized from the output alone leaves the submix's paths skipping. Red.
- Gate 3: a successor that drains its cells during the catch-up applies the mute before
  `S'` while the watermark says `EXACT`. Red.
- Gate 4: an advance that ignores the epoch's word reports every fallback as `EXACT`. Red.
- Gate 5: a bound that refuses the edit changes the revision; a peek never handed back makes the
  second growth fail `PeekOutstanding`. Red.
- Gate 7: a divergence that keeps the gate armed starves the producer; a re-preparation that
  prepares a new ring instead of reusing the donated one loses the chunks already submitted; a
  catch-up that begins the added consumer releases those chunks, so the donation is refused and the
  re-preparation never succeeds; a divergence that keeps the old peeks makes the new `take_peek`
  fail `PeekOutstanding`. Red.
- Gate 8: a catch-up that reads the added source at generation 1 waits forever for PCM the host
  never sends and falls back; one that starts it at B instead of S, or skips the waiting seek,
  differs from the lone consumer. Red.

## Dependencies

- *Give a plan a source-read clock that leads its render clock* (#1396), split from this issue.
- *Snapshot a running plan into a returned successor at a block* (#1354).
- *Give the source ring a read-only peek cursor that gates release* (#1320).
- *Render a successor plan off the render thread with a pinned floating-point environment* (#1321).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310), D2: the donor.
- *Hold live values in latest-target cells on both hosts* (#1312): a drain D4 defers.
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345): a drain D4
  defers.
- *Hold strip input-lane values in latest-target cells* (#1346): a drain D4 defers.
- *Hold route-lane values in latest-target cells* (#1347): a drain D4 defers.
- *Carry fader, mute and pan ramps across a plan swap* (#1277), D5.
- *Carry meter and effect observation state across a plan swap* (#1327), D4.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Prepare a successor across a withdrawn candidate plan* (#1344), D3 and D5: the donation D10
  runs and its unconsumed-ring check.
- *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398),
  D4: the re-preparation peak the warm submit admits.
