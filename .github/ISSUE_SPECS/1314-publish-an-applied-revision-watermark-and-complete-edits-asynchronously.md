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
covers it. Submit never waits for render, a swap or a catch-up: completion is observed, never
awaited, and the watermark never uses the reliable event lane.

## Context

- The control thread cannot know when render applies an edit. Live records are popped at block
  entry by whatever render call comes next: each drain takes exactly what was published at its
  entry (`available_at_entry`, `crates/builtins-compiler/src/lib.rs:1072-1075`,
  `crates/graph/src/runtime.rs:896-897`). A rebuild applies when render adopts the candidate
  (`enter_block`, `crates/engine/src/realtime/plan_exchange.rs:375-439`), which a paused host never
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
  (`crates/capi/src/runtime/compile.rs:169`).
- The engine already has a safe-Rust, allocation-free single-writer seqlock with bounded reads:
  `crates/engine/src/realtime/observe.rs` (module doc `:12-18`, `MAXIMUM_READ_ATTEMPTS = 64` at
  `:71`). It is inside the realtime root that `scripts/check-realtime-policy.sh` holds to its
  approved-unsafe list, so it uses no `unsafe`.
- The C ABI commits a live edit at `crates/capi/src/runtime/control.rs:1338-1346` (after every push
  and target publication, then each EQ owner's `commit_owner`) and a rebuild at `:1006-1025`
  (protocol commit, then `reservation.commit()`).
- `miso_engine_v1_plan_resources` (`crates/capi/src/ffi.rs:932`) is the any-thread plan query
  this mirrors; it reads `PlanQueries` (`crates/capi/src/runtime/plan.rs:82-91`), never `PlanState`.
- The reliable event lane has capacity 2 (`crates/capi/src/runtime/compile.rs:128`).
- The feature mask is 63 (`crates/capi/src/abi.rs:66-71`, `crates/capi/include/miso_engine_v1.h:136-142`);
  the frozen exported set is listed in `scripts/check-capi-abi.sh:192-208`.

## Decisions frozen for this slice

- **D1. The revision travels with its plan.** Each mailbox cell of #1343 gains two words beside its
  plan: `revision: AtomicU64` and `superseded: u64`.
  - `revision` is the newest committed revision whose content that cell's plan carries.
  - `superseded` is the number of revisions this candidate folds in from candidates it replaced.
    It is written only while the cell is control-owned, before publication.
  - `UnadoptedCandidate` carries both, so a withdrawn candidate takes its revision with it and
    `republish` writes them into whichever cell it lands in.
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
    does not change which cell is right: the `Full` cell becomes the `Active` one. *Adopt a
    successor plan no earlier than a scheduled sample, with a return queue* (#1311) adds the
    `Returned` cell state after this slice and extends this rule to it (#1311 D6).
  - **Pending candidate held by control.** A candidate the control thread holds outside the
    mailbox carries its own word: `UnadoptedCandidate::set_revision(revision)` (a withdrawn
    candidate, D1, and after #1311 one taken back from `Returned`), `PlanReplacementReservation::set_revision(revision)` (a reserved one,
    written before `commit`), and the owner of any other control-held candidate writes the word it
    will publish with (a catch-up successor: *Hold live edits during a catch-up and apply them at
    the adoption sample*, #1356 D3; a newer candidate held while a copy is in flight: *Supersede a
    running catch-up by a structural edit*, #1357 D1a). `set_revision` on the publisher is not
    called while control holds a candidate.
  - **No candidate pending.** Only then does `PlanPublisher::set_revision` store into the `Active`
    cell. It returns which cell it wrote (`RevisionTarget::{Pending, Active}`), so the control
    plane can debug-assert that it never writes `Active` while it records a pending candidate.
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
  `first_sample`, `flags`, `exact`, `preroll_fallback`, `transition_fallback`, `superseded`. A read
  retries at most `observe.rs`'s 64 attempts, then reports "busy". Reader handle:
  `PlanWatermarkReader`, cloned from `PlanPublisher::watermark_reader()`.
- **D5. Flags and counters.** Bits: `EXACT = 1`, `PREROLL_FALLBACK = 2`, `TRANSITION_FALLBACK = 4`,
  `SUPERSEDED = 8`. An advance's flags are the OR over the revisions it covers. Counters are
  saturating counts of revisions completed per outcome. An advance from `a` to `b` adds `b - a` in
  total.
  - At a claim, render takes the claimed cell's `superseded` value into render-local state. The
    first advance after that claim adds it to the `superseded` counter and sets `SUPERSEDED`.
    `exact` takes the rest of `b - a`, saturating at 0. Later advances on the same plan add none.
  - In this slice every candidate's `superseded` is 0, so every advance publishes `EXACT` and adds
    `b - a` to `exact`.
  - **Who writes `superseded`:** *Supersede an unadopted candidate plan by compare-and-swap*
    (#1310) owns the `SUPERSEDED` outcome. It stores the count beside its successor's revision.
    *Supersede a running catch-up by a structural edit* (#1357) does the same for a catch-up.
  - `PREROLL_FALLBACK` and `TRANSITION_FALLBACK` are set only by the catch-up fallback path, *Fall
    back from a missed catch-up deadline: bounded render-thread pre-roll, then the transition*
    (#1358).
  - A planned D15-9 transition (*Fade in a strip that a swap adds during playback* #1288,
    *Duck-swap a strip whose state cannot continue across a plan swap* #1324, *Remove a strip in
    two phases: ramp out, then a scheduled swap* #1325) is not a fallback and sets no flag: its
    revision completes as `EXACT`.
- **D6. C ABI.** `uint32_t miso_engine_v1_plan_watermark(const miso_engine_v1_plan *plan,
  miso_engine_v1_watermark *out)`, thread: any, concurrent with render, like
  `miso_engine_v1_plan_resources`. Struct (96 bytes, `MISO_ENGINE_V1_WATERMARK_SIZE`):
  `uint32_t struct_size; uint32_t reserved0; uint64_t revision; uint64_t first_sample;
  uint64_t outcome_flags; uint64_t exact_count; uint64_t preroll_fallback_count;
  uint64_t transition_fallback_count; uint64_t superseded_count; uint64_t reserved[4];`.
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

- `crates/engine/src/realtime/watermark.rs` (new), `spsc.rs` (the mailbox cell's two words only),
  `plan_exchange.rs`, `mod.rs`
- `crates/capi/src/abi.rs`, `ffi.rs`, `lib.rs`, `include/miso_engine_v1.h`, `tests/c/abi_smoke.c`,
  `tests/c/header_smoke.cpp`, capi tests
- `crates/control-plane/src/` (the D2 call sites and the reader in its plan state; #1309 moved them
  there)
- `scripts/check-capi-abi.sh` (the frozen symbol list only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- *Add miso_engine_v1_service for bounded control work between edits* (#1348): the session-handle
  entry point, the shared service step and the host's duty text.
- *Publish the applied-revision watermark in the browser status* (#1349): the status words and the
  engine-side hook the Worker of *Run the browser control plane in a Worker and keep the
  AudioWorklet render-only* (#1332) calls.
- Any producer of a nonzero `superseded` or of the fallback flags (#1310, #1357, #1358).

## Hazards

- D2's ordering is the whole correctness argument, and no deterministic test can see a
  reordering. The reviewer checks that the store is the last write of each commit path, including
  the EQ `commit_owner` loop and (after #1312) the cell writes.
- D2's routing is the other half. Writing a revision committed while a candidate is pending (in a
  cell, withdrawn, returned or held by a catch-up) into the `Active` cell would report it in effect
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
   +1. A later live edit on that plan adds 1 to `exact` only.
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
  is not reduced by it.
- Gate 5: red if a field store escapes the odd/even window or the reader skips the second sequence
  check.
- Gate 7: red if the header, the Rust struct and the export drift apart, or the bit is missing
  from the mask.
- Gate 8: red if a revision committed while a candidate is withdrawn or published is written to
  the `Active` cell: the watermark would report it at block 0, while P0, which does not carry the
  candidate's content, still renders.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309): the D2 call
  sites live in the control path it moves.
- *Let the control thread withdraw an unadopted candidate plan* (#1343): the mailbox cells that
  carry the revision words.
