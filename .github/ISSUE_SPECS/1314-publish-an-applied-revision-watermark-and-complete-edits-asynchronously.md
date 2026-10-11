# Publish an applied-revision watermark and complete edits asynchronously

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-3, D15-17).
Code anchors verified on `main` at `6fb211594`.

This is the smallest closable slice of plan item 7: the watermark with its outcome flags and
counters, published by render and read through a C ABI query. `miso_engine_v1_service` is
*Add miso_engine_v1_service for bounded control work between edits* (#1348). The browser status
field and the engine-side hook the Worker calls are *Publish the applied-revision watermark in the
browser status* (#1349).

## Product outcome

A C host can ask, from any thread, "which committed revision is audible, and since which render
sample?" `miso_engine_v1_plan_watermark` returns `(revision, first sample fully in effect, outcome
flags)` plus saturating per-outcome counters. The revision is the highest one that is in effect
together with every revision before it. Every committed revision is *pending* until the watermark
covers it. Submit never waits for render or a swap: completion is observed, never awaited, and the watermark never uses the reliable event lane.

## Context

- The control thread cannot know when render applies an edit. Live records are popped at block
  entry by whatever render call comes next: each drain takes exactly what was published at its
  entry (`available_at_entry`, `crates/builtins-compiler/src/lib.rs:1072-1075`,
  `crates/graph/src/runtime.rs:896-897`). A rebuild applies when render adopts the candidate
  (`enter_block`, `crates/engine/src/realtime/plan_exchange.rs:375-437`), which a paused host never
  reaches.
- *Let the control thread withdraw an unadopted candidate plan* (#1343) replaces publication with
  a two-cell mailbox: one cell is `Active` (the plan render runs), the other `Empty` or `Full` (the
  published candidate). Render adopts by one compare-and-swap that makes the `Full` cell `Active`
  and the old `Active` cell `Empty`. Control can withdraw a `Full` candidate, and a withdrawn
  candidate can be republished into whichever cell is `Empty`. So a cell index says nothing about
  which epoch it holds over time, and a candidate's epoch is consumed even if it is withdrawn.
- `RealtimePlanOwner` (`plan_exchange.rs:80`) renders through `render_contiguous` (`:448`) and
  `render` (`:473`); both call `enter_block` first. `plan_exchange_resource_report` (`:163`) is the
  exchange's exact retained-byte projection, which the C ABI's resource check uses
  (`crates/control-plane/src/compile.rs:197`).
- The engine already has a safe-Rust, allocation-free single-writer seqlock with bounded reads:
  `crates/engine/src/realtime/observe.rs` (module doc `:12-18`, `MAXIMUM_READ_ATTEMPTS = 64` at
  `:71`). It is inside the realtime root that `scripts/check-realtime-policy.sh` holds to its
  approved-unsafe list, so it uses no `unsafe`.
- The C ABI commits a live edit at `crates/control-plane/src/control.rs:1474-1482` (after every push
  and target publication, then each EQ owner's `commit_owner`) and a rebuild at `:1142-1161`
  (protocol commit, then `reservation.commit()`).
- `miso_engine_v1_plan_resources` (`crates/capi/src/ffi.rs:932`) is the any-thread plan query
  this mirrors; it reads `PlanQueries` (`crates/control-plane/src/plan.rs:86-95`), never `PlanState`.
- The reliable event lane has capacity 2 (`crates/control-plane/src/compile.rs:156`).
- The feature mask is 63 (`crates/capi/src/abi.rs:66-71`, `crates/capi/include/miso_engine_v1.h:136-142`);
  the frozen exported set is listed in `scripts/check-capi-abi.sh:192-208`.

## Decisions frozen for this slice

- **D1. The revision travels with its plan.** Each mailbox cell of #1343 gains three words beside
  its plan: `revision: AtomicU64`, `superseded: u64` and `outcome: u32`.
  - `revision` is the newest committed revision whose content that cell's plan carries.
  - `superseded` is the number of revisions this candidate folds in from candidates it replaced.
    It is written only while the cell is control-owned, before publication.
  - `outcome` is the flag the candidate's own revisions complete with: `EXACT` unless written.
    Like `superseded`, it is written only while control owns the candidate, before publication
    (`PlanReplacementReservation::set_outcome`, `UnadoptedCandidate::set_outcome`). Its only
    writer of another value is the transition fallback (*Duck-swap the strips a latency growth
    restarts, and fall back to the transition when a warm successor cannot adopt*, #1397).
  - `UnadoptedCandidate` carries all three, so a withdrawn candidate takes its revision and outcome
    with it and `republish` writes them into whichever cell it lands in.
  - The initial plan's revision is written into cell 0 (`Active`) by `plan_exchange`, which also
    writes the initial watermark `(initial revision, 0, EXACT)` with every counter 0.
  - The words are part of the cells, so `plan_exchange_resource_report` charges them with the
    cells. Nothing is allocated per plan, and no table is indexed by epoch.
- **D2. Control writes: the revision goes to the newest pending candidate, wherever it is.** A
  committed revision is in effect only when the plan that carries it renders, and revisions
  complete in order, so every revision committed while a candidate is pending belongs to that
  candidate, never to the running plan.
  - **Pending candidate in a cell.** `PlanPublisher::set_revision(revision)` loads the state word
    and stores `revision` (`Release`) into the `Full` cell, if there is one. A concurrent claim
    does not change which cell is right: the `Full` cell becomes the `Active` one. A candidate
    published `NoEarlierThan` or `Primed` by *Adopt a successor plan no earlier than a scheduled
    sample* (#1311) is a `Full` cell like any other; render adopts in the same step as it claims,
    so no other cell state exists.
  - **Pending candidate held by control.** A candidate the control thread holds outside the
    mailbox carries its own word: `UnadoptedCandidate::set_revision(revision)` (a withdrawn
    candidate, D1) and `PlanReplacementReservation::set_revision(revision)` (a reserved one,
    written before `commit`). `set_revision` on the publisher is not called while control holds a
    candidate.
  - **No candidate pending.** Only then does `PlanPublisher::set_revision` store into the `Active`
    cell. It returns which cell it wrote (`RevisionTarget::{Pending, Active}`), so the control
    plane can debug-assert that it never writes `Pending` while it records no pending candidate
    (Amendment 1: the converse is racy, because render may adopt the candidate between the
    command's epoch synchronisation and the commit, and `Active` is then correctly its cell).
  - The C ABI writes the revision as the **last write** of a live or model-only commit: after
    every record push, target publication and `commit_owner`.
  - For a rebuild it sets the reservation's revision before `reservation.commit()`.
- **D3. Render reads the `Active` cell, then drains.** In `render_contiguous` and `render`, after
  `enter_block` and before `render_inner`, render loads the `Active` cell's `revision` with
  `Acquire`. It never reads the other cell. Every drain inside `render_inner` then sees every
  record released before that store. If the loaded revision is above the published one, render
  publishes, after `render_inner` returns `Ok`: `(revision, this block's first absolute sample,
  flags)`, and it adds the covered revision count to the counters. Otherwise it writes nothing. A
  block that errors publishes nothing. "In effect" means a live value's ramp has started at that
  sample, and a rebuilt plan renders from it.
- **D4. Record and reads.** New module `crates/engine/src/realtime/watermark.rs`, the
  `observe.rs` seqlock pattern in safe Rust, with one writer (render). Words: `revision`,
  `first_sample`, `flags`, `exact`, `transition_fallback`, `superseded`. A read
  retries at most `observe.rs`'s 64 attempts, then reports "busy". Reader handle:
  `PlanWatermarkReader`, cloned from `PlanPublisher::watermark_reader()`.
- **D5. Flags and counters.** Bits: `EXACT = 1`, `TRANSITION_FALLBACK = 2`, `SUPERSEDED = 4`.
  There is no pre-roll outcome (D15-8 (round-5 amendment)), so no bit is left unassigned. An
  advance's flags are the OR over the revisions it covers. Counters are saturating counts of
  revisions completed per outcome. An advance from `a` to `b` adds `b - a` in total.
  - At a claim, render takes the claimed cell's `superseded` value into render-local state. The
    first advance after that claim adds it to the `superseded` counter and sets `SUPERSEDED`.
    `exact` takes the rest of `b - a`, saturating at 0. Later advances on the same plan add none.
  - At the same claim render takes the cell's `outcome`. If it is `TRANSITION_FALLBACK`, the
    first advance after that claim sets that flag instead of `EXACT`, and the rest of `b - a`
    goes to `transition_fallback` instead of `exact`. Later advances on the same plan are `EXACT`.
  - In this slice every candidate's `superseded` is 0 and its `outcome` `EXACT`, so every advance
    publishes `EXACT` and adds `b - a` to `exact`.
  - **Who writes `superseded`:** *Supersede an unadopted candidate plan by compare-and-swap*
    (#1310) owns the `SUPERSEDED` outcome. It stores the count beside its successor's revision,
    for a superseded warm candidate as for any other.
  - `TRANSITION_FALLBACK` is set only when a warm successor falls back to the transition: on
    `WarmUnavailable` at submit (*Duck-swap the strips a latency growth restarts, and fall back to
    the transition when a warm successor cannot adopt*, #1397) or at the readiness deadline (*Fall
    back to the transition when a warm successor is not ready by its deadline*, #1358).
  - A planned D15-9 transition (*Fade in a strip that a swap adds during playback* #1288,
    *Duck-swap a strip whose state cannot continue across a plan swap* #1324, *Remove a strip in
    two phases: ramp out, then a scheduled swap* #1325) is not a fallback and sets no flag: its
    revision completes as `EXACT`.
- **D6. C ABI.** `uint32_t miso_engine_v1_plan_watermark(const miso_engine_v1_plan *plan,
  miso_engine_v1_watermark *out)`, thread: any, concurrent with render, like
  `miso_engine_v1_plan_resources`. Struct (96 bytes, `MISO_ENGINE_V1_WATERMARK_SIZE`):
  `uint32_t struct_size; uint32_t reserved0; uint64_t revision; uint64_t first_sample;
  uint64_t outcome_flags; uint64_t exact_count; uint64_t transition_fallback_count;
  uint64_t superseded_count; uint64_t reserved[5];`.
  - Wrong `struct_size` or nonzero reserved words: `MISO_ENGINE_V1_INVALID_ARGUMENT`.
  - A read that gives up: `MISO_ENGINE_V1_BACKPRESSURE`, out untouched; the host retries.
  - The query writes nothing through the plan handle, its diagnostic word included: like
    `miso_engine_v1_plan_resources` it is pure (`miso_engine_v1.h:26-30`).
  - Flag macros `MISO_ENGINE_V1_OUTCOME_EXACT` ... `_SUPERSEDED`.
  - Feature bit `MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK`: a new symbol, so a new bit, the next free
    bit when this merges. `FEATURE_MASK` grows to include it. The spec names no bit number; gates
    use the symbol.
  - The frozen symbol list in `check-capi-abi.sh` gains `miso_engine_v1_plan_watermark`.
- **D7. Header contract text** (beside the live-edit paragraph): submit is synchronous only for
  what can fail (validate, classify, prepare a rebuild and reserve its credits, commit), then
  returns; every committed revision is pending until the watermark covers it; a paused host's
  revisions stay pending until render resumes; poll the watermark, never the event lane.
- **D8. Acked-batch question.** No queue is added. The watermark is a level: render overwrites it,
  never waits, never drops a command. An ack still precedes nothing that can be dropped, because
  D2's store comes after the last infallible write of the commit, a withdrawn candidate's
  revision leaves with it (D1) instead of being reported by a cell it no longer occupies, and a
  revision committed while any candidate is pending is written to that candidate (D2), so the
  running plan never reports it before the candidate that carries it renders.

## Deliverables

1. D1-D5 in `crates/engine/src/realtime/` (`watermark.rs`, `spsc.rs` cell words,
   `plan_exchange.rs`, `mod.rs` exports).
2. D2 call sites in the C ABI control path; the reader in `SharedPlanState` / `PlanQueries`.
3. D6 entry point, struct, constants, feature bit; header, `abi_smoke.c`, `header_smoke.cpp`,
   `check-capi-abi.sh` frozen list; D7 text; the `C_ABI_V1_QUALIFICATION.md` amendment paragraph.

## Authorized paths

- `crates/engine/src/realtime/watermark.rs` (new), `spsc.rs` (the mailbox cell's three words only),
  `plan_exchange.rs`, `mod.rs`
- `crates/capi/src/abi.rs`, `ffi.rs`, `lib.rs`, `include/miso_engine_v1.h`, `tests/c/abi_smoke.c`,
  `tests/c/header_smoke.cpp`, capi tests
- `crates/control-plane/src/` (the D2 call sites and the reader in its plan state; #1309 moved them
  there)
- `scripts/check-capi-abi.sh` (the frozen symbol list only), `docs/C_ABI_V1_QUALIFICATION.md`
- Amendment 1: `scripts/check-realtime-policy.sh` (its file and region floors only, raised to the
  measured counts; the script's own rule requires the raise in the change that adds a marker), and
  `scripts/test-realtime-policy.sh` (its fixture padding and the three expected floor messages,
  which sit exactly on the floors).

## Non-goals

- *Add miso_engine_v1_service for bounded control work between edits* (#1348): the session-handle
  entry point, the shared service step and the host's duty text.
- *Publish the applied-revision watermark in the browser status* (#1349): the status words and the
  engine-side hook the Worker of *Run the browser control plane in a Worker and keep the
  AudioWorklet render-only* (#1332) calls.
- Any producer of a nonzero `superseded` or of the fallback flag (#1310, #1358, #1397).

## Hazards

- D2's ordering is the whole correctness argument, and no deterministic test can see a
  reordering. The reviewer checks that the store is the last write of each commit path, including
  the EQ `commit_owner` loop and (after #1312) the cell writes.
- D2's routing is the other half. Writing a revision committed while a candidate is pending (in a
  cell or withdrawn) into the `Active` cell would report it in effect
  at the next block of the old plan; gate 8 is built to catch that.
- Render must read only the `Active` cell (D3). Reading "the newest cell" or a cell chosen from the
  epoch would report a candidate's revision before render adopts it; gate 1 is built to catch that.

## Objective gates

1. **Adoption, not publication (engine unit test).** Initial plan P0 at revision 1 (cell 0).
   - Publish A at revision 7 (cell 1); render block 0: watermark `(7, 0, EXACT)`.
   - Publish B at revision 8 (it lands in cell 0, P0's old cell), withdraw it, render blocks 1-2:
     the watermark stays `(7, 0, EXACT)`.
   - Republish B, render block 3: `(8, 3 * quantum, EXACT)`, `exact` 7 in total.
   - Publish C at revision 9 and do not render: the watermark stays at 8.
2. **Live edit on a pending candidate.** C ABI test: commit a rebuild (revision r1), then a live
   fader edit (r2) before any render. Render the swap block `k`: the watermark jumps from the
   initial revision straight to `(r2, k * 128, EXACT)` and `exact_count` grows by 2. Before the
   swap block it never reports r1 or r2.
3. **Live edit timing.** C ABI test, one thread: render blocks 0..3, commit a live edit (r), render
   block 4: watermark `(r, 512, EXACT)`; render block 5: unchanged.
4. **Superseded accounting (engine unit test).** A candidate published with `superseded = 2` at
   revision `a + 3`: its adoption block publishes `EXACT | SUPERSEDED`, `superseded` +2, `exact`
   +1. A later live edit on that plan adds 1 to `exact` only. Then a candidate at revision `b + 2`,
   with `b` the watermark's revision and `outcome = TRANSITION_FALLBACK`
   (`PlanReplacementReservation::set_outcome`), is withdrawn and republished before render claims
   it: its adoption block publishes `TRANSITION_FALLBACK` without `EXACT`, `transition_fallback`
   +2 and `exact` +0, and a later live edit adds 1 to `exact` only.
5. **Torn reads.** Stress test in `watermark.rs` modelled on `observe.rs`'s: a writer thread
   publishes records whose words are all derived from one counter; a reader never returns a record
   whose words disagree, and a busy result is the only alternative.
6. **Render stays clean.** `./target/release/audit capi` keeps 0 allocations, 0 deallocations and
   `"total_violations":0` with watermark publication on.
7. **ABI.** `abi_smoke.c` checks `MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK` against the queried mask
   with `&`, calls the query on a live plan with a valid struct (OK) and with a wrong
   `struct_size` (`INVALID_ARGUMENT`). `header_smoke.cpp` static-asserts the size 96 and the field
   offsets. The frozen list includes the new symbol.
8. **A revision follows a held candidate (engine unit test).** Initial plan P0 at revision 1.
   Publish A at revision 7 and withdraw it, so control holds it. Commit a model-only revision 8:
   it is written with `UnadoptedCandidate::set_revision`, and the publisher reports no write to
   `Active`. Render blocks 0-2: the watermark stays `(1, 0, EXACT)`. Republish A, render block 3:
   `(8, 3 * quantum, EXACT)`. Then publish B at revision 9 (`Full`) and commit a model-only
   revision 10: `PlanPublisher::set_revision` reports `Pending`, and the watermark stays at 8
   until B is adopted, then reads `(10, .., EXACT)`.
9. Commands:
   - `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p capi`; `cargo test --locked -p control-plane --features test-support`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-cross-targets.sh`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if render reads any cell but the `Active` one, if a withdrawn candidate's revision
  stays in the cell it left, or if render publishes at publication instead of adoption.
- Gate 2: red if the revision is one session-wide word instead of one per plan (it would report r2
  while the old plan still renders), or if counters count advances instead of revisions.
- Gate 3: red if render loads the word after `render_inner` (an edit committed during a render
  would be claimed one block early) or publishes on every block.
- Gate 4: red if `superseded` is added on every advance of a plan instead of once, or if `exact`
  is not reduced by it; red if a withdrawn candidate's `outcome` stays in the cell it left, so the
  republished transition reports `EXACT`.
- Gate 5: red if a field store escapes the odd/even window or the reader skips the second sequence
  check.
- Gate 7: red if the header, the Rust struct and the export drift apart, or the bit is missing
  from the mask.
- Gate 8: red if a revision committed while a candidate is withdrawn or published is written to
  the `Active` cell: the watermark would report it at block 0, while P0, which does not carry the
  candidate's content, still renders.

## Amendment 1 (root, 2026-10-05)

Two rulings on the attempt-1 verdict. D2's debug assertion checks the race-free direction the code
uses (a `Pending` write while no pending candidate is recorded), not the racy converse. The
realtime-policy floors in `scripts/check-realtime-policy.sh` are raised to the measured file and
region counts, so deleting a new marker turns the gate red; the raise lands in stream B's batch 1
after its last slice, at the counts measured then.

## Amendment 2 (root, 2026-10-05)

Root's binding requirement (2026-10-05): every live value of one committed revision takes effect
in the same block, and the watermark's `first_sample` is exact. Root accepted the revision-bounded
cell design (#1432 Amendment 1) and ruled that the revision gate lives in each plan, one gate per
plan. D1-D8 above stay the record of what this issue delivered. They describe the landed code.

**The truth on `main` once batch 1 merges.**

- The watermark is never early: it never reports a revision before that revision is in effect.
  `first_sample` is the start of the first block that begins after the commit's last write, so at
  most one block after the submit returns (while a replacement plan is pending: the block
  that adopts it). A live value of that revision can apply earlier, in any block that
  ran between the submit's first push and its revision store.
  - Render loads the running plan's revision word before `render_inner` (D3), but each strip
    drains its live lane at its own node inside `render_inner`, and the control plane releases
    one record at a time and stores the revision word last.
  - So a record pushed after the load and before that strip's drain renders in this block, while
    the watermark reports it at a later block's first sample (the attempt record's batch
    follow-up, F2).
- One transaction's values can spread across the blocks that ran between the submit's first
  push and its revision store, until #1502, #1503, #1504, #1312 and #1345 have all landed, which
  together make it exact.
- D3's sentence "Every drain inside `render_inner` then sees every record released before that
  store" holds. Its converse does not: a drain can also see records released after the load.
- These places state this bound: `crates/capi/include/miso_engine_v1.h` (the Completion paragraph),
  `crates/capi/src/abi.rs` (`Watermark::first_sample`), `crates/capi/src/ffi.rs` (the doc comment
  on `miso_engine_v1_plan_watermark`), `docs/C_ABI_V1_QUALIFICATION.md`, AGENTS.md (the
  block-snapshot sentence) and the D15-2 ruling. None claims `first_sample` is the exact first
  block in effect.

**The successors that make both exact.**

1. *Add the latest-target cell primitive and its loom model* (#1432) adds the revision gate and
   the revision-bounded cell.
2. *Give each plan its own revision gate and take each block's live snapshot from it* (#1502)
   replaces this issue's mailbox revision words with a gate per plan, following root's ruling.
   - Gone: I7's words, `MailboxWriter::store_revision`, `MailboxReader::active_revision`, the
     revision argument of `MailboxPermit::commit`, the revision in `MailboxWithdrawal::Withdrawn`,
     `RevisionTarget`, the routing in `PlanPublisher::set_revision`,
     `PlanReplacementReservation::set_revision`, `UnadoptedCandidate::{revision, set_revision}`,
     and Amendment 1's debug assertion.
   - Render takes the block's snapshot `S` with one read-modify-write on the running plan's gate
     and advances the watermark to it.
   - The control plane publishes each revision on the newest provider epoch's gate: one routing.
   - D5-D8 (flags, counters, the C ABI query and the header text) are unchanged.
3. *Bound committed revisions at the plan gate's ceiling* (#1503) bounds every revision at
   `2^63 - 1`.
4. *Hand each block's live snapshot to every live drain* (#1504) passes `S` to every drain.
5. Each lane family then reads its cells under `S`, and for that family the watermark becomes
   exact and a transaction lands in one block: #1312 (strip fader, mute and matrix), #1345
   (effect lanes), #1346 (input) and #1347 (routes).
   - On the C ABI both properties are whole once #1312 and #1345 have landed. The input and route
     lanes reach the C ABI only through #1261 and #1225, after #1346 and #1347.
   - *Prove under a racing render that a live transaction lands in one block and the watermark
     names it* (#1505) asserts the converse this issue's race test leaves out.

**Landed-code correction.** #1311 N4 (`cd5030923`) folded `MailboxPermit::write_revision` and
`write_adoption` into `MailboxPermit::commit(value, revision, adoption)`. The attempt record is
corrected to name the HEAD form.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309): the D2 call
  sites live in the control path it moves.
- *Let the control thread withdraw an unadopted candidate plan* (#1343): the mailbox cells that
  carry the revision words.

## Attempt record

### Attempt 1 (implementer, on `codex/d15-stream-b` after #1343)

**What landed.**

- `crates/engine/src/realtime/watermark.rs` (new): the `observe.rs` seqlock in safe Rust, one
  writer (render). `PlanWatermark`, `PlanWatermarkReader::read() -> Result<_, WatermarkBusy>` (64
  attempts), `CandidateOutcome`, `OUTCOME_*`. Render-local `pending_superseded` and
  `pending_outcome` are taken at a claim and consumed by the first advance (D5); claims between two
  advances accumulate, so no outcome is lost to a block that errored.
- `spsc.rs` (D1 only): each mailbox cell has a revision word (`revisions: [AtomicU64; 2]`,
  invariant I7). The cell's revision is stored before the publishing `Release`: at HEAD it is
  the `revision` argument of `MailboxPermit::commit(value, revision, adoption)`, into which
  #1311 N4 (`cd5030923`) folded the separate `MailboxPermit::write_revision` this attempt added
  (Amendment 2);
  `MailboxWriter::store_revision` (`Full` cell if any, else `Active`; `Release`),
  `MailboxReader::active_revision` (`Acquire`, the reader's own `active` index, set by its last
  claim), and `MailboxWithdrawal::Withdrawn(T, u64)` (at HEAD `Withdrawn(T, u64, PlanAdoption)`,
  with #1311's schedule), which takes the cell's word with the payload.
  #1343's tests changed only their `Withdrawn(value)` patterns to `Withdrawn(value, _)`.
- `plan_exchange.rs`: `superseded` and `outcome` ride in the payload (written only before
  publication); `RevisionTarget`, `PlanPublisher::{set_revision, watermark_reader}`,
  `PlanReplacementReservation::{set_revision, set_superseded, set_outcome}`,
  `UnadoptedCandidate::{revision, set_revision, superseded, outcome, set_outcome}`; `republish`
  writes the candidate's words into whichever cell it lands in. Render loads the `Active` word
  after `enter_block` and before `render_inner` and advances only after `render_inner` returns
  `Ok`. `plan_exchange_resource_report` charges the larger mailbox and a new row for the watermark
  record.
- Control plane: `compile_children` uses `plan_exchange_at_revision(plan, store.revision().0, ..)`;
  `SharedPlanState.watermark` and `PlanQueries::watermark()`; the rebuild sets the reservation's
  revision after the protocol commit and before `reservation.commit()`; a live or model-only
  commit calls `publish_committed_revision()` as its last write (after every push, target
  publication, `commit_owner` and the readback).
- C ABI: `miso_engine_v1_plan_watermark`, `miso_engine_v1_watermark` (96 bytes),
  `MISO_ENGINE_V1_WATERMARK_SIZE`, `MISO_ENGINE_V1_OUTCOME_*`, `MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK`
  (the next free bit, 64; mask 127); the thread-ownership text, the D7 "Completion" paragraph,
  `abi_smoke.c`, `header_smoke.cpp`, the frozen symbol list and the qualification amendment.

**Deviations from the spec's letter, for review.**

1. `plan_exchange(initial, config)` keeps its signature as revision 0 ("a host that numbers no
   revisions"), and `plan_exchange_at_revision(initial, revision, config)` is the D1 entry point the
   control plane calls. Changing `plan_exchange` itself would edit eight callers outside the
   authorized paths (`tools/audit`, `crates/source`, `crates/graph/tests`, `crates/host-core/tests`).
2. D2's debug assertion is the sound direction only: a `Pending` write while the control plane
   records no pending provider fails. The spec's direction ("never writes `Active` while it records
   a pending candidate") is racy in the C ABI: render may adopt the candidate after the command's
   `synchronize_plan_epochs` and before the commit's `set_revision`, and then the `Active` cell is
   correctly the candidate's while `pending_providers` still lists it.
3. A reservation without `set_revision` carries the newest revision the publisher stored or
   published (so an engine-level publication adds no revision). The C ABI always sets it.
4. `PlanReplacementReservation::set_superseded` is added because gate 4 publishes a candidate with
   `superseded = 2`; #1310 is its producer.
5. Gate 2 cannot tell "one session-wide word" from "one word per plan" in a single-threaded C ABI
   run: render adopts the candidate at the first block after the commit, so no block of the old
   plan runs in between. Gate 1 (engine) catches that defect (row `g1-session-wide-word`). Gate 3
   likewise cannot see "load after `render_inner`" on one thread; the loom model holds the
   store/load pairing, and gate 3 catches the timing defects listed below. Gate 2 also checks a
   rebuild with no live edit after it.

**Test value and mutation evidence.** Every row: the named defect applied, the named test red;
reverted, green. No pre-existing test reads the watermark or a revision word, so none catches
these defects.

| Test | Defect applied | Result |
|---|---|---|
| gate 1 `a_revision_completes_at_adoption_not_at_publication` | render loads the non-`Active` cell | red (block 0 read 1, not 7) |
| gate 1 | one session-wide word (all cell indices 0) | red (block 1 read 8 while A renders) |
| gate 1 | a withdrawn candidate loses its revision (`revision: 0`) | red (block 3 read 7, not 8) |
| gate 1 | publication also writes the `Active` cell (reported at publication) | red (block 1 read 8) |
| gate 1 | `render` publishes the next block's sample as `first_sample` | red |
| gate 4 `superseded_and_fallback_revisions_complete_once_per_adoption` | `superseded` added on every advance | red |
| gate 4 | `exact` not reduced by `superseded` | red (exact 3, not 1) |
| gate 4 | a withdrawn candidate's outcome reset to `Exact` | red (flags 1, not 2) |
| gate 8 `a_revision_follows_a_held_or_published_candidate` | `store_revision` writes `Active` while a cell is `Full`, still reporting `Pending` | red (adoption read 9, not 10) |
| gate 8 | the same, reporting `Active` | red (`RevisionTarget`) |
| gate 8 | `UnadoptedCandidate::set_revision` drops the revision | red (block 3 read 7, not 8) |
| gate 5 `a_watermark_read_is_one_whole_publication_or_busy` (release profile) | `superseded` stored after the closing counter | red (126191 of 236973 reads torn) |
| gate 5 | reader skips the second counter check | red (1433 of 26585 reads torn) |
| loom `spsc_loom_plan_mailbox_revision_follows_the_pending_candidate` | `store_revision` `Relaxed` | red ("revision 8 read without its record") |
| loom | `active_revision` `Relaxed` | red (same) |
| loom | pending revision written to the `Active` cell | red |
| gate 2 `a_live_edit_on_a_pending_candidate_completes_at_the_swap` | pending revision written to the `Active` cell | red (swap read r1) |
| gate 2 | counters count advances, not revisions | red (exact 1, not 2) |
| gate 2 | the rebuild does not set the reservation's revision | red (second swap read r2, not r3) |
| gate 3 `a_live_edit_completes_in_the_next_block` | the live commit stores no revision | red |
| gate 3 | an advance publishes on every block (`<` for `<=`) | red (first_sample moved with no advance) |
| gate 3 | `render_contiguous` publishes the next block's sample | red (640, not 512) |
| `plan_watermark_refuses_reserved_words_and_is_pure` | no reserved-word check | red |
| same | the query clears the plan's render diagnostic | red |
| gate 7 (`check-capi-abi.sh`) | `FEATURE_PLAN_WATERMARK` left out of `FEATURE_MASK` | red (abi_smoke) |
| gate 7 | the query refuses a valid struct | red (abi_smoke) |
| gate 7 | header `reserved[4]` | red (header_smoke size) |
| gate 7 | header swaps `exact_count` and `transition_fallback_count` | red (header_smoke offsets) |
| gate 7 | symbol left out of the frozen list | red (symbol set differs) |

Re-pins, each for one reason: `FEATURE_MASK` 0x3f -> 0x7f in `abi.rs`'s
`masks_and_result_codes_are_frozen` and `ffi.rs`'s `version_and_capabilities_are_exact`, and
`MISO_ENGINE_V1_FEATURE_MASK == 127` in `abi_smoke.c`, because the new bit joins the frozen mask.

**Gates (x86_64 Linux).**

- `cargo test --locked -p engine --features engine/realtime-audit`: ok (48 + 4 + 1).
- loom (`RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom`): 5 passed.
- `cargo test --locked -p capi`: ok (76 + 2 + 11); `cargo test --locked -p control-plane --features test-support`: ok.
- `bash scripts/check-realtime-policy.sh`: ok (91 regions, 26 files); `bash scripts/test-realtime-policy.sh`: ok.
- `bash scripts/check-capi-abi.sh`: ok (shared and static); `--self-test`: ok.
- `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`: 0 allocations,
  0 deallocations, 0 locks, 0 syscalls, `"total_violations":0` (the audit's live edits and its
  carrying swap advance the watermark during the 100000 audited calls).
- `bash scripts/check-cross-targets.sh`: PASS.
- `bash scripts/check-workspace-policy.sh`: ok; `cargo clippy --locked --workspace --all-targets -- -D warnings`: clean; `cargo fmt --all -- --check`: clean.
- Worklet chain (the engine crate is browser-compiled, though host-web does not use the plan
  exchange): `build-web-audioworklet.sh --named-twin` ok (module sha256 `87c03c64...28cf`),
  `check-web-audioworklet.sh --without-metadata-regeneration` ok,
  `check-browser-expected-resources.py --artifacts` ok, `test-web-audioworklet.sh` ok.
- Neighbours of the exchange: `graph` `rt11_swap_carry_alloc`, `source` lib: ok.

**D8 (acked-batch question).** No queue is added. The watermark is one overwritten record; render
never waits on it and drops nothing. The revision store is each commit's last write, after every
fallible step and every push; a withdrawn candidate takes its revision word with it; and a
revision committed while a candidate is pending goes to that candidate. So no ack precedes a drop,
and the running plan never reports a revision before the plan that carries it renders (gates 1,
2 and 8, and the loom model).

### Batch follow-up (2026-10-05)

Folds the attempt-1 verdict (`PASS`; MINOR F1-F4, NIT N3). F5 (realtime-policy floors), N1, N2
and N4 are root's items and are not touched here.

- **F1.** `cargo doc -D warnings` refused two links from public docs to private items. The public
  `WatermarkBusy` doc (`watermark.rs`) now names `MAXIMUM_READ_ATTEMPTS` in plain backticks, and so
  does the `From<CompileLimits> for ControlLimits` doc for the private `limits_are_valid`
  (`crates/capi/src/runtime/mod.rs`, from #1309, a cross-slice fix the verdict asked for here).
- **F2.** `crates/capi/tests/plan_swap_race.rs` gains
  `a_watermark_advance_names_a_block_that_applied_the_edit`. A control thread under
  `bench_support::producer::render_while_producing` commits 320 live fader edits on `eq0` of the
  stateless fixture (1024-frame blocks), one at a time, alternating -6 dB and 0 dB. A `RenderGate`
  handshake parks the render thread before a render call and starts it from the measured commit
  and render durations, so the revision stores sweep from a quarter call before the render call
  to its end, in either build profile (debug: render about 4.3 ms, commit about 1.1 ms; release:
  about 18 us and 88 us). After each block the render thread reads the watermark; at an advance to
  `r` it must name the block's first sample, and the block's last samples must be `r`'s steady
  level, measured single-threaded beforehand (the fader ramp ends within one block, which the
  measurement asserts). A guard requires more than a tenth of the stores inside a render call
  (debug about 240, release about 200, of 320). The shared source-submit code became
  `submit_dc_block`, which the existing `Control::feed` now calls.
  - Mutation M14 (the revision loaded after `render_inner` in `render_contiguous`, the C ABI's
    path; also with `render` moved): red in 10 of 10 debug runs and 10 of 10 release runs
    ("block 8: the watermark advanced to edit 9 in a block that did not apply it"). Reverted: 20 of
    20 debug runs and 5 of 5 release runs green.
  - Finding for root (not a defect in this slice): the converse does not hold, and the test does
    not assert it. A strip drains its live lane inside `render_inner`, at its node, so an edit
    whose record is pushed after render's revision load but before that drain renders in this
    block, while the watermark reports it at the next block's first sample, one block late. The
    first draft of this test compared against a single-threaded replay at the reported blocks and
    went red on unmodified code for exactly this reason. D3's "in effect" sentence reads as exact;
    the watermark is conservative in this direction only (never early).
- **F3.** `realtime::tests::watermark::a_block_that_errors_publishes_nothing` (engine): a block that
  `render_inner` refuses with `OutputShape` publishes nothing on either render path, even after it
  claimed a candidate (`render`) or with a live revision pending (`render_contiguous`), and the next
  rendered block publishes the revision at its own first sample. Mutation M15 (`advance` before the
  error propagates), applied to `render` alone and to `render_contiguous` alone: each red (engine
  54 passed, 1 failed). Reverted: 55 + 4 + 1.
- **F4.** The loom model `spsc_loom_plan_mailbox_claim_races_withdrawal` now stores revision 1 in the
  initial cell and 7 in the candidate's, and render reads `active_revision()` after its block: 1 when
  the withdrawal wins, 7 when the claim wins. Mutation L3 (`self.active = observed.full` above the
  claim compare-and-swap): only this model red ("a lost claim moved the Active cell"), the other 7
  loom models green. Reverted: 8 passed.
- **N3.** `plan_watermark_refuses_reserved_words_and_is_pure` (`ffi.rs`) poisons `reserved0` and each
  of the five `reserved` words alone. Mutation (the query checks only `reserved0` and
  `reserved[4]`, the two words the old test poisoned): red. Reverted: green.

Test value:
- `a_watermark_advance_names_a_block_that_applied_the_edit`: render loading the revision word after
  `render_inner` (M14), which reports a commit landing after the strip's drain at a block that did
  not apply it; no single-threaded test can place a commit inside `render_inner`.
- `a_block_that_errors_publishes_nothing`: render publishing before a refused block's error
  propagates (M15), on either path; every suite was green under it.
- the extended loom claim-race model: a lost claim that still moves render's cached `Active` index
  (L3), so render would report a withdrawn candidate's revision; loom, engine and capi were green
  under it.
- the extended reserved-word test: a query that checks only some of the reserved words.
