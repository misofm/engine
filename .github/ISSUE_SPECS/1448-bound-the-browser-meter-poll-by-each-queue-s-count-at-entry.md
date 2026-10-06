# Bound the browser meter poll by each queue's count at entry

Stream H follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-10).
Root ruling R1 (2026-10-05), on the second review of the realtime-policy tool drafts: the meter poll
is a render-thread drain whose bound is the summed queue capacity, not the queues' counts at
entry, so the realtime-policy gate refuses it. The fix belongs to stream H, which owns
`hosts/host-web`. Its sibling *Bound every drain the AudioWorklet runs on its audio thread*
(stream H, #1449) bounds the spectrum drains.

No allocation on the render path, lock or syscall is added. The rendered PCM does not change.
Root's rulings on the fourth review (2026-10-05):
- **Ruling 1: the per-queue design.** Each meter's remaining count is read at entry. Each pop is
  guarded by that meter's own count and decrements it. The passes are bounded. There is no minimum
  over the queues. On today's thread layout this is identical to today's code, poll for poll.
- **Ruling 2: option (C).** This issue lands its code and tests unmarked. That is no worse than
  `main` today, where `poll_meters` is unmarked. Its marker commit (D5) lands in stream J's tool
  batch after tool slice C2 (#1446), named in STREAMS as "H #1448's guard, landed by J after C2". Its
  spec and gates live in the J slice that carries it (tool slice B2b-1 (#1443), section "#1448 guard"),
  because this issue closes at its own PASS, long before the J batch.

## Problem (verified on `origin/main` at `6d28a80ec`)

`hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs` and the worklet module are the same on
`6b9067ede`, on `6d28a80ec` and on `codex/d15-stream-b` at `b8392df66` (stream B batch 1), so this
issue lands on `main` as it is.

- **Where it runs.** `AudioWorkletEngineHost::poll_meters` (`hosts/host-web/src/lib.rs:3430-3758`)
  is the `meter_poll` export. The worklet calls it once per `process()`, after the render export
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1758-1759`, `:1859`; the comment at
  `hosts/host-web/src/lib.rs:3653-3656`). `scripts/check-web-audioworklet.sh`'s call-graph gate
  covers `meter_poll` with `render` and `command_submit`.
- **The loop.** `drain_budget` is 1 plus the sum of every meter queue's capacity (`:3453-3456`).
  `while popped < drain_budget` (`:3462`) holds `for index in 0..meter_count` (`:3463`), which pops
  one snapshot into each empty pending slot (`meter_pending`, `:3464-3472`, pop `:3466`). Then, in
  the same pass:
  - if a slot is still empty, the loop ends (`:3474-3476`);
  - if the pending snapshots disagree, it drops the older heads (same generation, different
    sequence) or all of them (`:3501-3535`), counts the loss, and `continue`s, which pops again;
  - if the span is invalid, it drops all and `continue`s (`:3536-3545`);
  - if the producer generation changed, it resets delivery and returns 0 (`:3546-3559`);
  - otherwise it folds the window and `break`s (`:3560-3573`). "The frozen adapter policy is
    bounded per-window delivery. Further complete windows remain queued for the next poll"
    (`:3570-3571`).

  So a call folds at most one window. It pops a queue more than once only to recover from a gap or
  a discontinuity, inside the same call. The test
  `meter_delayed_poll_delivers_each_queued_window_with_its_own_peak`
  (`hosts/host-web/src/tests.rs:5174`) pins one window per poll.
- **The gate refuses it.** On an `origin/main` export with markers in place of the blank lines
  `:3422` and `:3759`, `bash scripts/check-realtime-policy.sh` reports
  `marked realtime unbounded try_pop drain` at `:3466` (second review, BLOCKER-1). The innermost
  loop around the pop is `for index in 0..meter_count`, not a count read at entry.
- **Bounded in fact today, by the thread layout.** The meter producers are render, on the same
  thread, and `poll_meters` holds `&mut self`, so no snapshot is published during the call. Today's
  code therefore pops no queue past its own count at entry. That stops being true once meter
  polling runs on a thread other than render's (H #1332's successors): drop-newest on a full queue
  can then race a pop in the middle of a pass. The shape also cannot be marked, which tool slice
  B2b-1 requires, and root ruled that this site does not go on the control-side allowlist.
- **Two designs that were rejected, and why.** The fourth review measured both. Its probes are
  `mtr-review4-probes.rs` in the probes folder, and the mode switch it compiled into `poll_meters`
  is `mtr-review4-probe-modes.diff`.
  - **Next-poll recovery (option (a)).** A poll that drops an inconsistent candidate and returns,
    leaving recovery to the next poll, leaves the leading meters one window ahead. At
    `live_control_meter_blocks = 1` with one poll per `process()`, no poll finds no new window, so
    the backlog never clears.
  - **A minimum over the queues' counts** (one count, the minimum over meters of the queued count
    plus a held slot, bounding the passes). After a loss on one meter, the poll runs out of passes
    before it can publish. It then publishes window N-1 after render N for as long as the stream
    runs. Measured: in 8000 polls with random per-meter losses, 7925 polls ended with a one-window
    backlog. No design inside the minimum form meets the bound, one window per poll and no lasting
    lag together.
  - The per-queue form (D2) meets all three. The fourth review measured it (its mode 2): identical
    to today's code poll for poll on every meter test and on every probe.

## Decisions

- **D1. The cap, named once.** Add `const METER_QUEUE_DEPTH: usize = 8;` in
  `hosts/host-web/src/lib.rs`. The live-control request builds each meter queue with
  `NonZeroUsize::new(METER_QUEUE_DEPTH)` (`:7239`, today the literal `8`), and each meter's count
  at entry is capped by it.
- **D2. One remaining count per meter, read at entry; each pop guarded by its own count.**
  - **State.** Add `meter_remaining: Box<[usize]>` to the ready state, beside `meter_pending`
    (`:1632`). Build it where `meter_pending` is built (`:7200-7203`), with one `0` per meter. It
    is allocated at preparation, never on the render path, and its length is the meter count:
    there is no compiled maximum. Its values are scratch for one call: each poll sets every entry
    before it pops, so `reset_meter_delivery` (`:1666`) does not touch it.
  - **Its bytes are charged** with `pending_meter_bytes`, in the bridge resource report
    (`:7111-7135`): `remaining_meter_bytes = meter_count * size_of::<usize>()`, with checked
    arithmetic and the `web.resource.arithmetic` diagnostic as its neighbours, is added to
    `meter_delivery_bytes` (so to `bridge_metadata_bytes` and `bridge_retained_bytes`), and the
    largest-allocation row takes `.max(remaining_meter_bytes)`. It is a retained allocation
    beside `meter_pending`, so it is reported as one (fifth review, MINOR-4). The exact-bytes
    tests run with meters off and the browser resource ceilings have headroom, so no pin should
    move; if a resource pin or an exact-bytes test moves, stop and report.
  - **The loop.** The `drain_budget` lines (`:3453-3456`), `let mut popped = 0_usize;`
    (`:3457`), the `while popped < drain_budget` loop and its inner `for index in 0..meter_count`
    (with its `popped` increment) become:

    ```rust
    for (meter, remaining) in ready.meters.iter().zip(&mut ready.meter_remaining) {
        *remaining = meter.consumer.available_at_entry().min(METER_QUEUE_DEPTH);
    }
    let mut passes = 1_usize;
    for remaining in ready.meter_remaining.iter() {
        passes = passes.saturating_add(*remaining);
    }
    for _ in 0..passes {
        for ((meter, pending), remaining) in ready
            .meters
            .iter_mut()
            .zip(&mut ready.meter_pending)
            .zip(&mut ready.meter_remaining)
        {
            if pending.is_some() || *remaining == 0 {
                continue;
            }
            *remaining -= 1;
            if let Ok(snapshot) = meter.consumer.try_pop() {
                *pending = Some(snapshot);
            }
        }
        // today's checks, unchanged (D3)
    }
    ```

  - **The bound.** Each queue pops at most its own count at entry, capped at `METER_QUEUE_DEPTH`,
    whatever a producer on another thread does. That is #1426's per-queue rule, stated directly.
  - **The passes.** A pass that does not end the loop has cleared at least one slot. The next pass
    either refills it (a pop, which spends one count) or ends the loop at the empty-slot check. So
    `1 + Σ remaining` passes always suffice, and the `for` never cuts a recovery short.
  - **The decrement comes before the pop.** On a single-consumer queue, a count read at entry is a
    lower bound on what the consumer can pop, so a pop under a count above 0 never fails. The
    fourth review's probe decremented only on `Ok`; this form behaves the same, and it is the
    form the tool accepts (B1a-D8b).
  - **The tool form.** This is tool slice B1a-D8b, the per-queue counted pop: the set loop at
    entry, the guard and the decrement as the pass body's first two statements, the pop in the
    third, and the counters written nowhere else. #1418 and #1426 (each Amendment 1) accept it.
  - The names are illustrative; the shape is not. No `[]` index (the comment at `:3438-3441`).
- **D3. The checks are unchanged.** After the meter loop, each pass runs today's code
  (`:3474-3573`) in today's order: a slot still empty ends the loop; a gap or a discontinuity
  drops heads, counts the loss and `continue`s to the next candidate; an invalid span drops all,
  counts the loss and `continue`s; a producer reset returns 0; a consistent set folds one window
  and `break`s. One window is published per poll (`:3570-3571`), and everything after the loop
  (`:3574-3757`) does not change.
  - **What changes.** Only the bound: a queue can no longer pop past its count at entry. On today's
    thread layout no producer runs during the call, so the same snapshots are popped, the same
    losses are counted, and the same windows are published at the same polls (gate 3).
- **D4. Realtime and call-graph rules.** As today: no `[]` index, `.expect(..)`, `.unwrap()`, panic
  macro, allocation, lock or syscall in `poll_meters`. `check-web-audioworklet.sh`'s call-graph gate
  on `meter_poll` must still pass.
- **D5. The marker, landed by stream J after C2 (root ruling 2, option (C)).** This issue adds no
  marker and edits no floor. `poll_meters` stays unmarked on `main` until the guard commit, as it
  is today. The guard commit, "H #1448's guard, landed by J after C2", is specified, with its own
  gates, in tool slice B2b-1's section "#1448 guard" (#1443), the J slice whose first
  commit it is. It lands after tool slice C2 and before the rest of B2b-1 (STREAMS).
  The spec lives there because this issue closes at its PASS and its spec leaves
  `.github/ISSUE_SPECS/` at the next batch, long before the J batch (fifth review, MINOR-5). In
  short: two markers replace the blank lines around `poll_meters` (no line moves), the region
  floor in `Policy::workspace()` rises by one, and the tool must accept this issue's
  `poll_meters` and refuse today's and the minimum-bound mutant.
- **D6. Tests.**
  - Every existing meter test passes unchanged, among them
    `meter_delayed_poll_delivers_each_queued_window_with_its_own_peak` (`tests.rs:5174`),
    `meter_invalid_master_rejects_transactionally_then_recovers_with_loss` (`:5207`),
    `meter_producer_reset_starts_a_fresh_delivery_epoch` (`:5240`; its snapshots sit in the
    pending slots with every queue empty, so every count is 0 and the first pass reaches the
    reset check), `meter_queue_saturation_recovers_with_explicit_loss` (`:5269`) and
    `bus_meters_render_and_poll_without_allocating` (`:5834`). If one fails, stop and report; do
    not change it.
  - **The host builder.** `bus_meter_host` (`:5698-5748`) becomes a call of a new
    `bus_meter_host_with(quantum, buses, meter_blocks, source_quanta)` with `(2, 64)`, today's
    values (`:5739`, `:5734`). No existing caller changes.
  - **One new test: no backlog after a loss on one meter.**
    `meter_gap_on_one_meter_leaves_no_backlog_at_one_block_windows`:
    - **Host:** `bus_meter_host_with(128, true, 1, 64)`, lease on (five meters, one window per
      block).
    - **Run:** for each block from 0 to 39: `feed_and_render_channels(host, block, 0.5, 0.25)`;
      at blocks 10, 20 and 30, after that render, pop and drop the head of one or two queues
      through the private fields (as `:5250-5255` reaches the queues), asserting that each pop
      succeeds: meter 2 at block 10, meter 0 at block 20, meters 1 and 3 at block 30; then
      `poll_meters()` once.
    - **Backlog** after a poll is the minimum, over the meters, of
      `consumer.available_at_entry() + usize::from(pending.is_some())`: the complete candidate
      windows left queued.
    - **Assert, with N = 1.** For every block b in 11-19, 21-29 and 31-39 (every poll from the
      first poll after each injection's own poll), the poll returns 1,
      `meter_header().first_sample` is `b * 128` (the window just rendered), and the backlog is
      0. At blocks 11, 21 and 31 the loss flag (`reserved[1] & METER_VALID_LOSS`) is set.
    - **Test value:** red if the passes are bounded by a minimum over the queues: that leaves a
      lasting one-window lag at meter blocks 1, and no existing test catches it (fifth review,
      gate 2's mutant (min): only this test goes red). It is also red on next-poll recovery
      (option (a)), but that defect is not unique to it:
      `meter_lease_reacquisition_waits_for_a_clean_boundary` (red at `tests.rs:5364`) and `meter_reacquisition_rejects_a_full_stale_queue_then_recovers`
      (red at `:5400`) also catch it (fifth review, MINOR-3).
- **D7. The equivalence driver (gate 3).** A scratch test file outside the repository, the same
  file at the parent and at the change, copied into `hosts/host-web/src/` as `vconfirm.rs` and
  declared from `hosts/host-web/src/tests.rs` by one line, `#[path = "vconfirm.rs"] mod vconfirm;`
  (the probes call `mod tests`'s private helpers `feed_and_render_channels`, `boot_options` and
  `observe`, so the module must be a child of `tests`, not of `lib.rs`). It is run with
  `cargo test --locked -p host-web --features test-support -- --nocapture` and removed again. It
  is not committed. For every poll, in order, it records the poll's index, its
  return value, the bytes of `meter_header()` and `meter_frame()`, and the backlog (D6's
  definition). It runs:
  - **The fourth review's probes** (`mtr-review4-probes.rs`), edited once before either run:
    `set_mode` and `VCONFIRM_MODE` removed, and each poll recorded as above:
    - `vc_trace_gap_backlog`: meter blocks 1, 90 blocks, the gaps `(10,2)`, `(25,2)`, `(40,0)`,
      `(41,4)`, `(60,1)`, `(61,1)` and `(62,3)`;
    - `vc_precise_trace`;
    - `vc_random_backlog`: 8000 blocks, with its xorshift seed and drop schedule;
    - the scenario of `vc_d6_new_test`: meter blocks 2, four renders, meter 1's head dropped,
      three polls.
  - **The injection driver**, at `live_control_meter_blocks` 1, 2 and 12 in turn:
    - **Host.** `bus_meter_host_with`'s model with track 0's inserts kept (not cleared) and
      `live_control_observation_taps: 1`, with one observation tap armed on track 0's compressor
      as the observation tests arm one (`observation_host`, `tests.rs:5408-5423`, and the
      `observe` helper), so the gain-reduction words carry data. `sources[0].frames` is
      `128 * 4096`, more than any run renders. The meter lease is on. If the gain-reduction words
      are all zero over the run, stop and report.
    - **Blocks.** `feed_and_render_channels(host, block, 0.5, 0.25)`, then `poll_meters()` once,
      per block, except where an injection or the saturation says otherwise. The run has
      `max(96, 80 + 10 * meter_blocks + 16)` blocks: 106, 116 and 216.
    - **Injections**, each at the first block N at or after 10, 30, 50 and 70 that closes a window
      (`(N + 1) % meter_blocks == 0`), in this order: after block N-1's poll, poll without
      rendering until a poll returns 0; render block N; apply the injection through the private
      fields, asserting that every pop it makes succeeds; poll.
      - **gap:** pop and drop one snapshot from meter 2's queue;
      - **discontinuity:** pop one snapshot from each queue, move `start_sample` of meter 0's by one
        quantum, and put each in its pending slot;
      - **invalid span:** as above, with `end_sample = start_sample` on every snapshot;
      - **producer reset:** as `meter_producer_reset_starts_a_fresh_delivery_epoch` does
        (`:5248-5255`).
    - **Saturation.** At block 80: poll until a poll returns 0, render `10 * meter_blocks` blocks
      with no poll (each queue then holds 8 windows and has dropped 2), then resume one poll per
      block.
    - **End.** After the last block, poll without rendering until a poll returns 0. Record
      `ready.meter_loss_count` and the published loss count (`reserved[1]`).
  - **Pass:** at the parent and at the change, every recorded sequence is identical, poll for poll
    (the same indices, return values, header and frame bytes and backlogs), and the final loss
    counts are equal. **Stop rule:** if anything differs, stop and report the first difference. Do
    not change the driver or the code to make them agree.

## Authorized paths

- `hosts/host-web/src/lib.rs`: `poll_meters`, the `meter_remaining` field and its construction,
  D1's constant and its use at `:7239`, and the bridge resource charge for `meter_remaining`
  (`:7111-7135` on `6d28a80ec`, beside `pending_meter_bytes`).
- `hosts/host-web/src/tests.rs`: `bus_meter_host_with` and D6's new test.
- This spec

The guard commit's paths (D5) are stream J's, by the exception in STREAMS, and its
spec is tool slice B2b-1's "#1448 guard" section.

## Non-goals

- The region markers and floors (D5: the guard commit).
- Moving meter polling to the Worker (H #1332's successors).
- Any change to the meter frame layout, the header, loss counting, the one-window-per-poll policy
  or the gain-reduction section.

## Hazards

- **Hot file.** `hosts/host-web/src/lib.rs` is H's. B (#1312, #1345-#1347, #1399), D #1326, E #1364
  and J #1423 land before H #1332 and touch other functions; the later slice rebases. Find
  `poll_meters` by name.
- **The guard comes later.** Between this issue and the guard commit, no gate refuses a change
  that restores the capacity bound. D6's test and gate 3 hold the behaviour; the guard commit holds
  the shape. That gap is the state of `main` today.

## Objective gates

1. **Tests.** `cargo test --locked -p host-web --features test-support` passes, with every meter
   test unchanged and D6's new test.
2. **The new test is red on both rejected designs (PR evidence).** Apply each mutant alone to this
   change, run D6's new test, and record the failing assertion and its block:
   - **(a) next-poll recovery:** each `continue` in the gap, discontinuity and invalid-span
     branches (`:3501-3545` today) becomes `return 0`, after the loss is counted. Record also
     that `meter_lease_reacquisition_waits_for_a_clean_boundary` and
     `meter_reacquisition_rejects_a_full_stale_queue_then_recovers` go red (`tests.rs:5364`,
     `:5400`; fifth review, measured);
   - **(min) the minimum bound:** the set loop, the `passes` loop and the guard are replaced by the
     fourth review's mode 1 (`mtr-review4-probe-modes.diff`): `let mut available =
     METER_QUEUE_DEPTH;`, folded by
     `available.min(meter.consumer.available_at_entry() + usize::from(pending.is_some()))` over
     every meter, then `for _ in 0..available` around a pass that skips a full slot and pops.

   Each mutant must turn the test red. Also run the same test on the parent: it passes (it pins
   today's behaviour; it is not a regression reproducer). If a mutant stays green, stop and
   report.
3. **Poll for poll equal to today (PR evidence).** D7's driver at the parent and at the change:
   identical sequences on every probe and at meter blocks 1, 2 and 12. Record each run's digest of
   its sequence, its final loss counts and its poll count.
4. **Worklet chain gates.** The batch's `qualification` run passes `artifact`, `artifact-identity`,
   `artifact-gates` (`check-web-audioworklet.sh` with its `meter_poll` call graph,
   `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`, the V8 spill gate) and the three
   browser legs.
   - `artifact-identity` reports ARTIFACT CHANGED. The new 8-byte field (`Box<[usize]>` on
     wasm32) moves `ReadyOwnership`'s later fields, so every function that reads them changes
     (fifth review, MAJOR-2, measured on `6d28a80ec` with D1 and D2 as written; D2's resource
     charge, added after that measurement, is in `compile_ready` and adds no render-path code).
   - **The method (PR evidence).** Build the base and head named twins and run
     `mtr-closure-offset-cmp.py` (in the probes folder; copy it into the PR evidence) on them
     with the exports `miso_engine_web_v1_render`, `miso_engine_web_v1_command_submit` and
     `miso_engine_web_v1_meter_poll`. It disassembles both (`wasm-objdump -d`), maps call targets
     to names with crate hashes normalised, takes each export's call-graph closure, and compares
     each function in both closures; a function whose only differences are offset immediates (the
     offset of a load or store, or an `i32.const` added to a base address) with the same
     instruction count is `offset-only`, with the deltas printed. Record its full output.
     (Coordinator decision, 2026-10-06: `offset-only` covers wasm load/store memarg offsets and
     `i32.const` immediates that are struct-field offsets moved by the new field. Every delta must
     be +8. Any other difference fails the gate.)
   - **The render closure and the `command_submit` closure:** the same functions on both sides,
     and every differing function is `offset-only`. Any other difference (a `BODY` line, a
     function on one side only) fails the gate: stop and report. Expected, as measured:
     - render closure (25 functions): `miso_engine_web_v1_render`,
       `AudioWorkletEngineHost::render_next` (its one change is an `i32.const`, 1296 → 1304) and
       `ReadyOwnership::push_master_window`;
     - `command_submit` closure (81 functions): `admit_commands`, `prepared_queue_address`,
       `ReadyOwnership::push` and `AudioWorkletEngineHost::submit_commands_inner`;
     - every delta +8.

     An `offset-only` function not on these lists is named in the evidence with the field it
     addresses; it does not fail the gate. A delta other than +8 fails the gate (coordinator
     decision above).
   - **The `meter_poll` closure:** `poll_meters` differs in body; `pop_master_window` and
     `reset_meter_delivery` are `offset-only`. The functions that build or drop the ready state
     (`compile_ready`, `ffi::boot_staged`, the `ReadyOwnership` drop glue, the new `Vec<usize>`
     collect and `into_boxed_slice` instances, and `project_buffers`, whose size constant grows
     by 8) differ in body; they run at boot or teardown, never on the render path.
   - **No PCM moved.** No fixture, pin or expected digest changes, and every rendered-digest gate
     passes with the base's pins (`check-browser-expected-resources.py --artifacts`, the browser
     legs' native-digest gates, `wasm-gates` G5 and G6).
5. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p host-web --all-targets --features test-support -- -D warnings`,
   `bash scripts/check-realtime-policy.sh` (unchanged: `poll_meters` is not marked) and
   `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- D6's new test is red if the passes are bounded by a minimum over the queues, a lasting
  one-window lag at meter blocks 1 that no existing test reaches (gate 2's mutant (min)). It is
  also red if a per-meter loss is recovered one poll late (mutant (a)), but two existing tests
  catch that too (`tests.rs:5364` and `:5400`).
- The existing meter tests, the per-window-peak test first, are red if a poll publishes more than
  one window, or a window other than the first good one.
- The bound itself (no queue pops past its count at entry) cannot be red on today's thread layout,
  because no producer runs during the call. Its committed guard is D5's marker, which the tool
  judges. Gate 3 shows that the change is invisible today.

## Evidence

- Gates 1-5 output; gate 2's two red runs and the parent's green run; gate 3's digests, loss
  counts and poll counts.
- The guard commit's evidence (D5) is recorded in tool slice B2b-1's issue, which carries the
  guard's spec and gates. This issue is closed by then; nothing is added to it.

## Dependencies

- After (same stream): none.
- After (other streams): none.
- Before (other streams): the J tool batch, which carries D5's guard commit after C2 and before
  B2b-1.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: under half a day.
