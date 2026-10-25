# Report strip peaks and gain reduction to C ABI apps from the core's meter fold

Slice S24b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Second part of the planner's S24, after #1583. Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

A C ABI app configures `{strip, peak}` and `{strip, gain reduction}` entries and receives, in each
`METER_BATCH`, every configured strip's left and right peak and its gain reduction in decibels,
over the same window span as the output peak. The readings continue across a plan swap. A mobile
app gets the strip meters and the gain-reduction meter the browser shows today.

## Context

- **The meters exist on every C ABI plan.** #1581 D6 keeps `HostLiveControlHandles::meters` in
  `PreparedRuntime` (`crates/control-plane/src/compile.rs:33`; today dropped at `:646-651`). #1584
  keeps `effect_observations` there too and attaches observation capacity on every C ABI plan.
- **The fold.** #1582's `MeterDelivery` folds strip windows, the output window and gain reduction
  into one `MeterFrame`; its `GainReductionTaps` input reads the tap's arm state from the slot
  since #1585. #1583's `MeterBatchFold` and staging turn frames into batches, and refuse strip and
  gain-reduction entries with `UNAVAILABLE` until this slice.
- **Adoption on the control thread.** `SessionState::synchronize_plan_epochs`
  (`crates/control-plane/src/control.rs:974`) promotes the pending provider epoch when render has
  adopted it (`:976-978`). That is where the control half learns that a successor renders.
- **The carry.** *Carry meter and effect observation state across a plan swap* (#1327) moves a
  carried meter's whole state, producer included, into the successor (its D2), gives the
  successor's table an `ObserverCarry` record (D3), and adds `pair_carried_meters` and
  `pair_carried_observations` (D3) in `crates/host-core/src/prepare.rs`. A fresh meter of the same
  configuration joins the carried window grid (D6). Its D5 says the C ABI has nothing to carry;
  that is no longer true after #1581.
- **The browser's rule for a strip's gain word.** The maximum over the strip's armed taps' latest
  fresh windows, `0` and absent when none (`hosts/host-web/src/lib.rs:3836-3890`, with #1534's
  predicate). #1582 moved it into `MeterDelivery` unchanged.

## Decisions frozen for this slice

- **D1. Strip consumers in the C ABI delivery.** The `MeterDelivery` of the C ABI's `SessionState`
  takes the current plan's strip consumers at boot. In `synchronize_plan_epochs`, after
  `promote_pending_provider`, `MeterDelivery::adopt(successor: Vec<MeterConsumer>, carries:
  &[ObserverCarry])` pairs its present consumers with the successor's through
  `pair_carried_meters`, installs the successor vector, and resets the pending state of every
  consumer that did not carry. The displaced vector drops on the control thread. The same step
  pairs the effect observation handles through `pair_carried_observations`. A swap that carries
  every meter changes no generation and counts no loss.
- **D2. Entry resolution.** On each configure and each adoption, the control half resolves each
  table entry to a strip index of the adopted plan (`None` when the ID is absent), into a vector
  sized to the table at configure time. A batch never searches by string.
- **D3. Records.** In table order, for each entry:
  - `{strip, peak}` with a strip present: `{h, Left}` and `{h, Right}` from the frame's
    `strip_peaks`, `VALID`, plus `CLIPPED` above `1.0`;
  - `{strip, gain reduction}`: `{h, Aggregate, VALID, db}` when the strip's reading is present,
    else `{h, Aggregate, 0, 0.0}`;
  - an entry whose strip is absent: each of its components with flags `0` and value `0.0`.
  - In a fold of `k` frames, a peak is the maximum of the `k` peaks and a gain reduction is the
    last frame's reading that is present (`VALID` when any frame's is).
- **D4. Refusals removed.** #1583's `meter.strip.unavailable` and `meter.gain_reduction.unavailable`
  are deleted. `meter.strip.unknown` stays: it checks the committed model at the configure.
- **D5. Docs.** The header prose and `docs/C_ABI_V1_QUALIFICATION.md` state D3's record rule,
  that gain reduction reports only taps an `Observe` op armed (#1584), and that readings continue
  across a swap that carries the strip.

## Deliverables

1. D1-D3 in `crates/control-plane/src/{control.rs,compile.rs}` and `crates/host-core/src/meters.rs`
   (`MeterDelivery::adopt`, the `k`-frame gain rule of `MeterBatchFold`).
2. D4 in `crates/host-core/src/control_provider.rs`; D5.
3. Tests below.

## Authorized paths

- `crates/control-plane/src/{control.rs,compile.rs}`, `crates/host-core/src/{meters.rs,control_provider.rs}`,
  `crates/host-core/tests/meters.rs`.
- `crates/capi/src/runtime/{tests.rs,live_tests.rs}`, `crates/capi/tests/resource_lifecycle.rs`,
  `crates/capi/include/miso_engine_v1.h` (prose), `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No browser change (#1588). No change to the meter kernels or to the carry rules of #1327.
- No render-side gate for the strip meters (#882, amended).

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`.

1. **Strip peaks equal an offline fold.** Session: tracks `a` and `b`, submix `bus`, `b` sends
   `post_fader` into `bus`. Configure `[{strip a, peak}, {strip bus, peak}, {output, peak}]` with
   period equal to the window. Feed known PCM; every batch's values equal the maxima of a
   reference plan's post-matrix planes over the batch's span, computed offline from the same PCM.
2. **Gain reduction.** Add a compressor insert on `a` and arm its tap with an `Observe` op (#1584).
   The `{strip a, gain reduction}` record is `VALID` once a window opened at or after its arm
   sample closes, and equals `reduction_db` of that window; before that, and after a disarm, it is
   `{h, Aggregate, 0, 0.0}`.
3. **Across a swap.** With the table of gate 1, commit `UpsertTrack` of a muted track whose ID sorts
   first (a rebuild). The batches before and after the adoption have consecutive
   `window_sequence`s, `MeterWindowsLost` does not move, and every value equals the reference plan
   of gate 1 fed from frame 0.
4. **Removed and returned strip.** Remove `b` and then add it back: while it is absent its entry's
   records have flags `0`; after the add they are `VALID` again under the same handle.
5. **Realtime.** The capi race test (`crates/capi/tests/resource_lifecycle.rs:3364`) runs with
   gate 1's table and one armed tap: `allocations == 0 && frees == 0` around every render call
   after warm-up, and zero `INTERNAL` results.

## Test value

- Gate 1 turns red if a strip's record carries another strip's peak or a span other than the
  batch's.
- Gate 2 turns red if gain reduction leaks a window from before the arm or reports a disarmed tap.
- Gate 3 turns red if adoption drops the carried meters' windows or reads a retired plan's queue.
- Gate 4 turns red if a handle moves or an absent strip reports a stale value as valid.
- Gate 5 turns red if the strip meters or the adoption step allocate on render.

## Dependencies

- #1583, #1582, #1584 (observation capacity on C ABI plans), #1585 (the arm state in the slot).
- *Carry meter and effect observation state across a plan swap* (#1327): the move-mode carry and
  the two pairing functions; its D5 is amended for the C ABI.
- `crates/control-plane` and `crates/capi` are stream B's files; this slice rebases after them.

## Supersedes and shrinks

- With #1581 and #1583, supersedes *Report each configured meter handle's own meter in the C ABI
  meter batch* (#1352); it closes when this slice merges.

## Decision record (#1587a, 2026-10-24)

- **Split.** #1587a is strip peaks only; gain reduction is #1587b (after #1585a). D1's "(new)
  `MeterDelivery::adopt`" is read as "call the existing one" (scope R5).
- **R1' (root, binding; replaces R1).** The adoption sample of epoch E is the retired
  predecessor plan's `next_absolute_sample()`, taken on the reclaim path of
  `synchronize_plan_epochs`, where `MeterDelivery::adopt` runs (`adopt_meters`). Reasons R1 (a
  render-side `active_epoch_sample` atomic) was wrong: render retires the predecessor inside
  `render_contiguous`, before `PlanState::render` could store a sample, so the reclaim path can
  promote before the sample exists; two single-writer words read as a pair tear; and a second
  adoption before the control thread promotes overwrites the first one's sample, which a carried
  swap after an uncarried one needs. `synchronize_plan_epochs` loads `active_epoch` (Acquire)
  before it drains, so outside the test hold every promotion is a reclaim's; with the hold on the
  atomic still promotes and the delivery waits (`SessionMeters::epoch`; no poll until it is the
  running epoch), catching up one epoch per reclaim. B-1's drain-first rule is unchanged.
- **Gate.** At a block that adopts a successor whose strip meters were not carried,
  `PlanState::render` calls `OutputPeakGateRelay::restart_at(block start)`: an active gate moves
  there and `applied` is unchanged. That is the rule of the control thread's `align` at the
  adoption sample, so its later request asks for the value the gate holds. The relay stays the
  one writer.
- **R2.** `ProviderEpoch::meter_tables`, built in `prepare_runtime` for a `METER_DELIVERY`
  adapter, charged as an epoch row. The running epoch's tables live in the delivery; a parked
  epoch gets its own back at the adoption that displaced it.
- **R4.** `SessionMeters::{resolution, folds}`, reserved at boot for the most table entries,
  rebuilt at each accepted configure and each meter adoption against the strip meters of the
  epoch whose frames the delivery folds (not the committed model: a pending rebuild never
  relabels a record).
- **Records bound.** A batch carries at most 256 records, so the provider refuses the 129th peak
  entry: `INVALID_FIELD`, `meter.records`, at that entry.
- **Root ruling 1 (lane).** The meter lane holds one whole batch, 256 slots. The protocol queue
  config sizes the counter-snapshot lane apart with no wire change (`ProtocolQueueConfig::
  counter_telemetry_slots`, kept at 2), and `telemetry_slots` reports the meter lane. So the two
  counter tests that relied on a two-slot lane are unchanged. The C ABI capability and event
  vectors are re-pinned (field 15: 256, was 2); the conformance corpus passes, so
  `COMPLETE_SCHEMA_HASH` did not move. `reference_session_retained_rows`: `capi_retained_bytes`
  measured 259,582 after attempt 2's 128-entry bound (and #1325a's duck overlay from main),
  ceiling 285,568 (+10 %, to 64), for the 32,512 B strip-ID reserve and the meter lane.
  Attempt 2 (verifier m2): the lane is sized by the adapter, 256 for a `METER_DELIVERY` adapter
  (the C ABI) and 2 otherwise, so the browser's capability record does not change; #1588 sizes
  the browser's lane.
- **Root ruling 2 (depth).** The C ABI's strip meter queues are as deep as the output meter's,
  `METER_QUEUE_DEPTH` (8, was 1): at depth 1 every frame after a service lag of more than one
  window was lost. Caps now charge 8 meter items per strip, so a session near
  `maximum_meter_items` is refused earlier; a host can raise the cap. This moved the reference
  session's `builtin_meter_payload_bytes` (20,538) and `builtin_retained_payload_bytes` (56,997)
  past their ceilings, which are raised by the same rule (22,592 and 62,720); the ruling's "no
  other budget" did not foresee these two, so they are flagged for root.
- **Charge.** The strip-ID text of the two stored tables is charged at the reachable bound, 128
  entries (the records bound) x `STABLE_ID_MAXIMUM_BYTES` x 2 = 32,512 B (`meter_text_reserve_bytes`),
  with the resolution and fold vectors reserved for 128 entries (attempt 2, verifier m3; attempt 1
  charged 256).
- **Root ruling 3 (snap).** The relay keeps the strip meters' grid on render (origin 0, moved by
  `restart_at`; the window) and `apply(block_start)` writes only gates on that grid at or after
  the block: a request at or after the block opens at the grid's first boundary at or after it; a
  request behind the block keeps a gate already open at or before the block, and otherwise opens
  at the first boundary at or after the block. So a configure that races render, or a stale-grid
  request after an uncarried swap, never opens an output window off the strip grid. Integer
  arithmetic only, no allocation.
- **Root ruling 4.** The re-exports in `host-core/src/lib.rs` and `control-plane/src/lib.rs` are
  accepted.
- **Tests (#1587a).** One per claim:
  - `strip_and_output_meter_batches_are_each_entrys_own_peak_across_rebuilds` (capi; rewrites
    #1583's gate-3 test, gates 1, 3, 4): red if a record carries another strip's or the output's
    peak, a strip resolves by position or against a retired epoch, adoption drops carried windows
    or restarts the sequence, an absent strip reports a stale value, or a returned strip moves its
    handle. Attempt 2 (verifier B1): the carried rebuild is adopted in the middle of a batch; red
    on attempt 1, where `resolve` cleared the folds at every adoption (eq0 Left 0.25 flags 1, want
    1.5 flags 3). The folds now keep their values across a re-resolution. The reference is offline and exact (power-of-two gains). A returned strip fades in
    over its first window, so that batch reads its second window.
  - `an_uncarried_rebuild_restarts_the_meter_grid_at_its_adoption_block` (capi): red if render
    does not restart the gate at an uncarried swap or the control thread's adoption asks for
    another value.
  - `a_gate_request_behind_its_block_opens_on_the_next_grid_boundary` (host-core): red if the
    relay copies a request behind its block into the gate (root ruling 3).
  - `a_meter_table_past_one_batch_of_records_is_refused_at_its_first_extra_entry` (host-core):
    red if the records bound is missing or off by one.
  - Gate 5: the render race's table gains two strip entries; red if strip meters or adoption
    allocate on render. (Root's condition 1 asked for a withdrawn-candidate assertion; the
    verifier found no defect it catches before "A's epoch is dropped" does, so it was removed in
    attempt 2.)
  - Deleted: the `meter.strip.unavailable` row of the capi refusal test (the refusal is gone).
