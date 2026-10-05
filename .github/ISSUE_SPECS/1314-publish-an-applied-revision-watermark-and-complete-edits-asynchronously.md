# Publish an applied-revision watermark and complete edits asynchronously

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-3, D15-17).
Code anchors verified on `main` at `6fb211594`.

This is the smallest closable slice of plan item 7: the watermark with its outcome flags and
counters, published by render and read through a C ABI query. `miso_engine_v1_service` is
*Add miso_engine_v1_service for bounded control work between edits* (#1348); the browser status
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
  `crates/graph/src/runtime.rs:896-897`). A rebuild applies when `enter_block` adopts the candidate
  (`crates/engine/src/realtime/plan_exchange.rs:375-439`), which a full retirement queue defers
  (`SwapOutcome::DeferredRetirementFull`, `:385-404`) and a paused host never reaches.
- `RealtimePlanOwner` (`plan_exchange.rs:80`) renders through `render_contiguous` (`:448`) and
  `render` (`:473`); both call `enter_block` first. Publication goes through
  `PlanPublisher::reserve_replacement` (`:265`) and `PlanReplacementReservation::commit` (`:304`).
  `plan_exchange_resource_report` (`:163`) is the exchange's exact retained-byte projection, which
  the C ABI's resource check uses (`crates/capi/src/runtime/compile.rs:169`).
- The C ABI creates its exchange with publication and retirement capacity 1
  (`compile.rs:755-761`). At most `publication_capacity + 1` plans are unretired at once: the active
  one and the queued candidates.
- The engine already has a safe-Rust, allocation-free single-writer seqlock with bounded reads:
  `crates/engine/src/realtime/observe.rs` (module doc `:12-18`, `MAXIMUM_READ_ATTEMPTS = 64` at
  `:68`). It is inside the realtime root that `scripts/check-realtime-policy.sh` holds to its
  approved-unsafe list, so it uses no `unsafe`.
- The C ABI commits a live edit at `crates/capi/src/runtime/control.rs:1338-1346` (after every push
  and target publication, then each EQ owner's `commit_owner`) and a rebuild at `:1006-1025`
  (protocol commit, then `reservation.commit()`).
- `miso_engine_v1_plan_resources` (`crates/capi/src/ffi.rs:932`) is the any-thread plan query
  this mirrors; it reads `PlanQueries` (`crates/capi/src/runtime/plan.rs:82-91`), never `PlanState`.
- The reliable event lane has capacity 2 (`crates/capi/src/runtime/compile.rs:128`).
- The feature mask is 63 (`crates/capi/src/abi.rs:64-71`, `crates/capi/include/miso_engine_v1.h:136-142`);
  the frozen exported set is 15 symbols (`scripts/check-capi-abi.sh:192-208`).

## Decisions frozen for this slice

- **D1. Revision cells, one per unretired epoch.** The exchange owns a table of
  `publication_capacity + 1` `AtomicU64` revision cells, allocated once in `plan_exchange` and
  counted in `plan_exchange_resource_report`. Epoch `e` uses cell `e % (publication_capacity + 1)`.
  A cell is never shared by two unretired epochs: epoch `e + n` cannot be reserved while epoch `e`
  is active, because the queue then holds `n - 1` candidates. No allocation per plan.
- **D2. Control writes.** `PlanPublisher::set_revision(epoch, revision)` stores the cell with
  `Release`; `PlanReplacementReservation::set_revision(revision)` does the same for the reserved
  epoch before `commit`. `plan_exchange` takes the initial plan's revision and writes cell 0 and the
  initial watermark `(initial revision, 0, EXACT)` with every counter 0. The C ABI calls
  `set_revision` on the newest epoch (the pending candidate's while one waits, else the current
  one; `newest_providers`, `control.rs:1488`) as the **last write** of a live or model-only commit:
  after every record push, target publication and `commit_owner`. For a rebuild it sets the
  reservation's revision before `reservation.commit()`.
- **D3. Render reads, then drains.** In `render_contiguous` and `render`, after `enter_block` and
  before `render_inner`, render loads the active epoch's cell with `Acquire`. Every drain inside
  `render_inner` then sees every record released before that store. If the loaded revision is above
  the published one, render publishes, after `render_inner` returns `Ok`, `(revision, this block's
  first absolute sample, flags)` and adds the covered revision count to the counters. Otherwise it
  writes nothing. A block that errors publishes nothing. "In effect" means a live value's ramp has
  started at that sample, and a rebuilt plan renders from it.
- **D4. Record and reads.** New module `crates/engine/src/realtime/watermark.rs`, the
  `observe.rs` seqlock pattern in safe Rust: one writer (render), words `revision`, `first_sample`,
  `flags`, `exact`, `preroll_fallback`, `transition_fallback`, `superseded`. A read retries at most
  `observe.rs`'s 64 attempts and then reports "busy". Reader handle: `PlanWatermarkReader`, cloned
  from `PlanPublisher::watermark_reader()`.
- **D5. Flags and counters.** Bits: `EXACT = 1`, `PREROLL_FALLBACK = 2`, `TRANSITION_FALLBACK = 4`,
  `SUPERSEDED = 8`. An advance's flags are the OR over the revisions it covers. Counters are
  saturating counts of revisions completed per outcome; an advance from `a` to `b` adds `b - a`
  in total. In this slice every revision completes `EXACT`, so an advance publishes `EXACT` and adds
  `b - a` to `exact`. The other bits and counters are set by their producers: `SUPERSEDED` by
  *Supersede an unadopted candidate plan by compare-and-swap* (#1310) and *Supersede a running
  catch-up by a structural edit* (#1357); `PREROLL_FALLBACK` and `TRANSITION_FALLBACK` only by the
  catch-up fallback path, *Fall back from a missed catch-up deadline: bounded render-thread
  pre-roll, then the transition* (#1358). A planned D15-9 transition (*Fade in a strip that a swap
  adds during playback* #1288, *Duck-swap a strip whose state cannot continue across a plan swap*
  #1324, *Remove a strip in two phases: ramp out, then a scheduled swap* #1325) is not a fallback
  and sets no flag: its revision completes as `EXACT`. Each producer attributes its revisions to
  its counter and `exact` takes the rest of `b - a`.
- **D6. C ABI.** `uint32_t miso_engine_v1_plan_watermark(const miso_engine_v1_plan *plan,
  miso_engine_v1_watermark *out)`, thread: any, concurrent with render, like
  `miso_engine_v1_plan_resources`. Struct (96 bytes, `MISO_ENGINE_V1_WATERMARK_SIZE`):
  `uint32_t struct_size; uint32_t reserved0; uint64_t revision; uint64_t first_sample;
  uint64_t outcome_flags; uint64_t exact_count; uint64_t preroll_fallback_count;
  uint64_t transition_fallback_count; uint64_t superseded_count; uint64_t reserved[4];`.
  Wrong `struct_size` or nonzero reserved words: `MISO_ENGINE_V1_INVALID_ARGUMENT`. A read that
  gives up: `MISO_ENGINE_V1_BACKPRESSURE`, out untouched; the host retries. The query writes
  nothing through the plan handle, its diagnostic word included: like
  `miso_engine_v1_plan_resources` it is pure (`miso_engine_v1.h:26-30`). Flag macros `MISO_ENGINE_V1_OUTCOME_EXACT` ... `_SUPERSEDED`.
  Feature bit `MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK = 64` (bit 6; if another addition takes bit 6
  first, the next free bit), mask grows to include it. 16 frozen symbols.
- **D7. Header contract text** (beside the live-edit paragraph): submit is synchronous only for
  what can fail (validate, classify, prepare a rebuild and reserve its credits, commit), then
  returns; every committed revision is pending until the watermark covers it; a paused host's
  revisions stay pending until render resumes; poll the watermark, never the event lane.
- **D8. Acked-batch question.** No queue is added. The watermark is a level: render overwrites it,
  never waits, never drops a command. An ack still precedes nothing that can be dropped, because
  D2's store comes after the last infallible write of the commit.

## Deliverables

1. D1-D5 in `crates/engine/src/realtime/` (`watermark.rs`, `plan_exchange.rs`, `mod.rs` exports).
2. D2 call sites in the C ABI control path; the reader in `SharedPlanState` / `PlanQueries`.
3. D6 entry point, struct, constants, feature bit; header, `abi_smoke.c`, `header_smoke.cpp`,
   `check-capi-abi.sh` frozen list; D7 text; the `C_ABI_V1_QUALIFICATION.md` amendment paragraph.

## Authorized paths

- `crates/engine/src/realtime/watermark.rs` (new), `plan_exchange.rs`, `mod.rs`
- `crates/capi/src/abi.rs`, `ffi.rs`, `lib.rs`, `runtime/plan.rs`, `runtime/compile.rs`,
  `runtime/control.rs` (or its successor file in the crate that #1309 creates), `runtime/tests.rs`,
  `runtime/live_tests.rs`, `include/miso_engine_v1.h`, `tests/c/abi_smoke.c`,
  `tests/c/header_smoke.cpp`
- `scripts/check-capi-abi.sh` (the frozen symbol list only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- *Add miso_engine_v1_service for bounded control work between edits* (#1348): the session-handle
  entry point, the shared service step and the host's duty text.
- *Publish the applied-revision watermark in the browser status* (#1349): the status words and the
  engine-side hook the Worker of *Run the browser control plane in a Worker and keep the
  AudioWorklet render-only* (#1332) calls.
- Any producer of the non-`EXACT` flags (#1310, #1357, #1358).

## Hazards

- D2's ordering is the whole correctness argument and no deterministic test can see a reordering.
  The reviewer checks that the store is the last write of each commit path, including the EQ
  `commit_owner` loop and (after #1312) the cell writes.
- The new table and record change the exchange's retained bytes. Add them as rows of
  `plan_exchange_resource_report`, sized with checked arithmetic (`publication_capacity + 1` must
  keep returning `SpscError::CapacityOverflow` at `usize::MAX`, `crates/engine/src/realtime/mod.rs:235-240`),
  and update any test that pins the exchange's or the C ABI plan's retained bytes with the new rows,
  never a looser bound.

## Objective gates

1. **Adoption, not publication.** Engine unit test: exchange with retirement capacity 1; publish a
   candidate at revision 7 while the retirement queue is full; render two blocks: the watermark
   stays at the initial revision (deferred). Reclaim, render block `k`: watermark is
   `(7, k * quantum, EXACT)`.
2. **Live edit on a pending candidate.** C ABI test: commit a rebuild (revision r1), then a live
   fader edit (r2) before any render; render the swap block `k`: the watermark jumps from the
   initial revision straight to `(r2, k * 128, EXACT)` and `exact_count` grows by 2. Before the
   swap block it never reports r1 or r2.
3. **Live edit timing.** C ABI test, one thread: render blocks 0..3, commit a live edit (r), render
   block 4: watermark `(r, 512, EXACT)`; render block 5: unchanged.
4. **Torn reads.** Stress test in `watermark.rs` modelled on `observe.rs`'s: a writer thread
   publishes records whose words are all derived from one counter; a reader never returns a record
   whose words disagree, and a busy result is the only alternative.
5. **Render stays clean.** `bench_support::alloc` or the C ABI audit: render with watermark
   publication performs 0 allocations, 0 deallocations (`./target/release/audit capi` keeps
   `"total_violations":0`).
6. **ABI.** `abi_smoke.c` checks the bit with `&`, calls the query on a live plan with a valid
   struct (OK) and with a wrong `struct_size` (`INVALID_ARGUMENT`); `header_smoke.cpp` static-asserts
   the size 96 and the field offsets; the frozen list has 16 symbols.
7. Commands:
   - `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p capi`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-cross-targets.sh`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if render publishes at candidate publication or pop instead of at adoption, or if a
  deferred swap advances the watermark.
- Gate 2: red if the revision cell is one session-wide word instead of one per epoch (it would
  report r2 while the old plan still renders), or if counters count advances instead of revisions.
- Gate 3: red if render loads the cell after `render_inner` (an edit committed during a render would
  be claimed one block early) or publishes on every block.
- Gate 4: red if a field store escapes the odd/even window or the reader skips the second sequence
  check.
- Gate 6: red if the header, the Rust struct and the export drift apart, or the bit is missing
  from the mask.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309): the D2 call
  sites live in the control path it moves.
