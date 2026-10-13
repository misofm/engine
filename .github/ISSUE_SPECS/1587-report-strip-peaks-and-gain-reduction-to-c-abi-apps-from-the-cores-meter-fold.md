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
