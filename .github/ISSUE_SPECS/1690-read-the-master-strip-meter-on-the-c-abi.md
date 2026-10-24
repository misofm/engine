# Read the master strip's meter on the C ABI

Successor of #1683 (the master strip is a core `master` monitor row), decision 17 C1 and C13
(`docs/rulings/one-control-abstraction-2026-10-13.md`: a behaviour that exists on one host only is
a defect). Code anchors verified on branch `codex/controls-batch-14aw` at `5ef2c81d8` (the #1683
tree). The specs of #1581, #1582, #1583 and #1587 are read as frozen text: none of their code is
on that tree yet. Issue number: assigned at filing (written here as #1690). Feature sprint mode
(owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A mobile app that designates a master strip with the `master` row reads that strip's gain
reduction, the reading the browser reports as `masterGrDb`, without naming the strip again. The
app gets it from the same meter batch as its other meters. Both hosts compute the reading with one
core function, from the same windows, so the same session gives the same bits. When no strip is
the master, or the master has no fresh window, the reading is absent on both hosts (`null` in the
browser, an invalid record on the C ABI), never `0`.

## Context

- **Where the browser computes it.** In `AudioWorkletEngineHost::poll_meters`
  (`hosts/host-web/src/lib.rs:3601`), from the gain-reduction fold that runs after the track
  fold:
  - each armed observation tap's latest window since its arm (`window_since_arm`, `:4465`)
    becomes a decibel magnitude with `observed_decibels` (`:4559`) and folds, as a maximum, into
    the frame's per-strip gain word at `gain_base + strip` (`gain_base = 2 * strips + 2`);
  - the master slot at `gain_base + strips` copies the designated strip's word, but only when that
    strip is `observation_present` (`:3969-3995`); then `master_gr_present` is set in the header
    (`WebMeterHeader::master_gr_present`, `:977`; read as the header's word at byte 44 by
    `direct-oracle.mjs:443` and `sdk/src/core/boundary.ts:1272`), so absence is `null`, not `0`.
  - The designated strip is `PlanShape::master_track` (`:1398`), resolved after every control
    command by `ReadyOwnership::sync_master` (`:1655-1675`) from the core's master
    (`SessionState::master_strip`, `crates/control-plane/src/control.rs:2966`).
- **The C ABI has no master reading.** It has no meter-batch strip or gain-reduction entry yet
  (see the dependencies), and no way to name the master in a meter table.
- **What the planned slices already move.**
  - #1582 moves the whole fold above into `host_core::MeterDelivery` in
    `crates/host-core/src/meters.rs`, including `observed_decibels` (as `reduction_db`),
    `window_since_arm`'s predicate and the master slot: its `MeterFrame` carries
    `strip_gain_reduction_db`, `strip_gain_reduction_present` and
    `designated_gain_reduction_db`. So the computation itself is **not** moved by this slice.
  - #1583 gives the C ABI a meter table of `MeterEntry { target: Output | Strip(id), metric:
    Peak | GainReduction }` and refuses a gain-reduction entry `UNAVAILABLE` until #1587.
  - #1587 delivers `{strip, gain reduction}` records on the C ABI from `MeterDelivery`
    (`{h, Aggregate, VALID, db}` when the strip's reading is present, else
    `{h, Aggregate, 0, 0.0}`).
- **#1581 is not the home.** #1581 gives every plan the strip `SAMPLE_PEAK` meters and the output
  peak (`meter_selection`, `OutputPeakReader`). Those are peaks. `masterGrDb` is gain reduction
  from the observation taps, which #1584 and #1585 arm and publish. The C ABI delivery of both
  is #1587's, so #1587 is the home, and this slice builds on it.
- **What is still missing after those slices.**
  - `MeterDelivery::new(.., designated_strip: Option<u32>)` (#1582 D1) fixes the designated strip
    by index at construction. #1683 made the designation a row the app writes at any time, held
    by strip ID, so a constructor argument cannot follow it. The browser works around that with
    `sync_master`; the C ABI has no equivalent.
  - A C ABI table entry can name a strip by ID, but an app that wants "the master" would have to
    read the control list, find the strip whose `master` row is on, configure `{strip, gain
    reduction}`, and reconfigure on every move. The entry does not follow the row.

## Decisions frozen for this slice

- **D1. One core function.** `host_core::MeterFrame::master_gain_reduction(&self, master:
  Option<u32>) -> Option<f32>` in `crates/host-core/src/meters.rs`: `Some(db)` exactly when
  `master` names a strip of the frame whose `strip_gain_reduction_present` is true, with that
  strip's `strip_gain_reduction_db`; else `None`. `MeterFrame::designated_gain_reduction_db` and
  the constructor's `designated_strip` argument (#1582 D1-D2) are deleted. The browser's master
  slot and the C ABI's master record both come from this function. No decibel logic is written
  twice.
- **D2. The master is read from the core row, by strip ID, at each plan.** The control half
  resolves the master's strip index against the adopted plan's strip list at every adoption and
  after every `SET_CONTROL master` write, from `SessionState::master_strip()`, as #1683's
  `sync_master` does for the browser. The browser keeps `PlanShape::master_track` as the resolved
  index it passes to D1; it stops owning any gain-reduction logic (its block at `lib.rs:3969-3995`
  is deleted by #1582 and replaced by one call). The C ABI keeps the resolved index with its
  table. A removed master strip ends the designation (#1683), so the reading is absent.
- **D3. A master table entry on the C ABI.** The meter table (#1583 D1) gains the target
  `Master`: `MeterTarget::{Output, Strip(Box<str>), Master}`, wire `target` code 3 with no
  `strip_id`. It is valid with the metric `GainReduction` only; `{master, peak}` and
  `{output, gain reduction}` are refused `INVALID_FIELD`. A `{master, gain reduction}` entry's
  handle is its 1-based table index, like any entry, and never moves. Its record follows #1587
  D3: `{h, Aggregate, VALID, db}` when D1 returns `Some`, else `{h, Aggregate, 0, 0.0}`; in a
  fold of `k` frames the value is the last frame's reading that is present, as for a strip. An
  entry stays in the table when no strip is the master.
- **D4. The browser meter frame is unchanged.** Its master word, `master_gr_present`,
  `masterGrDb` and the SDK's `null` stay bit for bit what #1683 leaves.
- **D5. The wire and the docs.** The schema `meter_entry` field `target` gains code 3; the
  conformance corpus and `COMPLETE_SCHEMA_HASH` move with it. `docs/CONTROL_PROTOCOL_REGISTRY.md`
  (the meter-enum rows), `docs/C_ABI_V1_QUALIFICATION.md` and the header prose
  (`crates/capi/include/miso_engine_v1.h`) state D3. No C symbol or struct changes.
- **D6. The acked-batch question.** A configure is checked whole before it is stored (#1583 D8).
  A master record is lossy telemetry like any other: a window that never reaches a batch is
  counted by `MeterWindowsLost`.

## Deliverables

1. D1 in `crates/host-core/src/meters.rs` (and its re-export), D2 in
   `crates/control-plane/src/control.rs` and `hosts/host-web/src/lib.rs` (the call and the
   resolved index only).
2. D3 and D5 in `crates/protocol` (schema, codec checks, corpus, hash), the meter staging of
   `crates/control-plane`, and the docs.
3. The tests below.

## Authorized paths

- `crates/host-core/src/meters.rs`, `crates/host-core/src/lib.rs` (re-export),
  `crates/host-core/src/control_provider.rs`, `crates/host-core/tests/meters.rs`.
- `crates/control-plane/src/{control.rs,compile.rs}`.
- `crates/protocol/**` (schema, message wire, controller checks, conformance corpus, schema hash).
- `crates/capi/src/runtime/{tests.rs,live_tests.rs}`, `crates/capi/tests/resource_lifecycle.rs`,
  `crates/capi/include/miso_engine_v1.h` (prose), `docs/C_ABI_V1_QUALIFICATION.md`,
  `docs/CONTROL_PROTOCOL_REGISTRY.md`.
- `hosts/host-web/src/{lib.rs,tests.rs}` (the master call and the resolved index),
  `hosts/host-web/tests/control_parity.rs`.
- The regenerated SDK files, only as `scripts/check-sdk-generated.sh` requires.

## Non-goals

- No change to the observation taps, their arm rule (#1534, #1584, #1585) or the reduction
  formula (`reduction_db`).
- No master peak. The master has no structural bus; its peak is the strip's own entry.
- No change to #1683's row, the boot word (#1689) or the browser's lease frame bytes.
- No SDK API beyond the generated table type; a later issue gives the SDK a `masterGainReduction`
  call on the batch.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p protocol`,
`cargo test --locked -p capi`, `cargo test --locked -p host-web --all-features`,
`cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-sdk-generated.sh <artifacts>`,
`bash scripts/check-capi-abi.sh`.

1. **Parity row.** In `hosts/host-web/tests/control_parity.rs`, one session with a compressor
   insert on `t1` and a submix `bus`, played through both hosts: write `master` on `t1` with
   `SET_CONTROL`, arm the compressor's reduction tap, feed the same PCM, and take the same tap
   windows. The browser leg reads `masterGrDb` from its lease frame; the C ABI leg configures
   `[{master, gain reduction}]` with a period equal to the meter window and reads the record. Per
   window the two values are equal bit for bit, and present together. The row then moves the
   master to `bus` (a strip with no armed tap) and both read absent in the same window, then
   clears it and both read absent. The row asserts the first value is nonzero, so it is not
   vacuous.
2. **Absent is not zero.** Core unit test: `master_gain_reduction` returns `None` for `master:
   None`, for a master strip with no fresh window (before its first window since its arm, and
   after a disarm), and `Some(0.0)` for a master whose tap is armed and reducing nothing. The C
   ABI record flags are `0` for the first group and `VALID` with `0.0` for the last.
3. **The entry follows the row.** A capi test configures the table once, then writes `master` on
   `t1`, on `t2` and on `t1` again, adds a track before the master (a rebuild), and removes the
   master strip. The handle never moves; the records read the master's own reduction at each
   step, and read absent from the removal on.
4. **Refusals.** `{master, peak}` and `{output, gain reduction}` refuse `INVALID_FIELD` and keep
   the stored configuration; a `{master, gain reduction}` entry on a session with no master is
   accepted.
5. **Realtime.** `bench_support::alloc` shows `allocations == 0 && frees == 0` around render and
   the delivery's poll on both hosts with a master designated and an armed tap, after warm-up
   (the capi race test of `crates/capi/tests/resource_lifecycle.rs`, extended with this table).
6. **Browser bytes unchanged.** The existing master tests pass unchanged, among them
   `structural_apply::a_master_designated_after_an_empty_boot_reads_as_a_boot_designation` and
   the `masterGrDb` cases of `scripts/test-web-audioworklet.mjs`; the browser qualification's
   `masterMatchesTrack` holds.
7. **Schema.** The protocol corpus and `COMPLETE_SCHEMA_HASH` are regenerated and the conformance
   suite passes.

## Test value

- Gate 1 turns red if either host keeps a master computation of its own, reads another window, or
  reports `0` where the other reports absent.
- Gate 2 turns red if absence collapses to `0`, so an app cannot tell "not reducing" from "no
  reading".
- Gate 3 turns red if the entry is bound to an index or a strip that was master once, so it
  reports a stale strip after a move, an add or a removal.
- Gate 4 turns red if a meaningless entry is stored and then silently reports nothing.
- Gate 5 turns red if the master read allocates on the render thread.
- Gate 6 turns red if moving the master slot into the core changes a published browser byte.

## Dependencies

- #1587 (*Report strip peaks and gain reduction to C ABI apps from the core's meter fold*): the
  home of the C ABI's gain-reduction record and of the `MeterDelivery` adoption step. This slice
  lands after it. It brings its own dependencies: #1583 (the meter table and batch), #1582
  (`MeterDelivery` and the browser's move onto it), #1584 and #1585 (observation capacity on C ABI
  plans and the arm state in the slot).
- #1581 is a dependency of the above, not of this slice: it supplies the strip and output peaks
  but no gain reduction, so it does not host this reading.
- #1683 (the `master` row and `master_strip()`).
- #1588 serves the same batches in the browser; this slice does not need it (gate 1 reads the
  browser's lease frame), and after #1588 the browser reads the same batch record as well.

## Size

Half a day.

## Split and decision record (2026-10-24, scoped against main `068742aba`)

The issue splits in two slices.

- **#1690a (done on `codex/batch-b13-1690a`): D1 and the browser half of D2.**
  `MeterFrame::master_gain_reduction(master)` replaces `MeterFrame::designated_gain_reduction_db`.
  Amendment to D1: #1582 landed the designation as a `designated: Option<u32>` argument of
  `fold_gain_reduction` and `MeterDelivery::poll`, not as a constructor argument; #1690a deletes
  that argument and the field's copy in `MeterDelivery::adopt` and its clear in the reset. The
  browser half of D2 is already `sync_master`: `publish_web_meter_frame` takes
  `PlanShape::master_track` (the index the fold received before, read at the same point) and calls
  the function; its lease-transition call passes the same index, on a frame whose presence the
  reset cleared, so it reads absent as before. No wire or C ABI change.
  Evidence: the existing browser tests defend the claim (no new test). Mutants, each red in
  `cargo test -p host-web --all-features --lib`: presence ignored and absence read as `Some(0.0)`
  turn `a_resubscribed_tap_leaves_the_meter_frame_until_its_first_new_window` red; no master read
  as strip 0 turns `the_meter_frame_carries_the_app_shaped_gain_reduction` red. Gate 6 (browser
  bytes unchanged): a one-time uncommitted digest of every meter-frame word and header field after
  every poll and lease transition (four master choices, a silence/loud run, master moves, a
  disarm/re-arm and a lease toggle; 132 records, 70 with the master present) is the same on the
  base and on the branch (PR evidence, not a committed test).
- **#1690b (open, blocked by #1587b, so by #1583 and #1585): D2's C ABI half, D3, D5, D6.**
  The C ABI keeps the resolved master index (`Option<u32>`) beside #1587's entry-resolution vector
  in `SessionState`, recomputed at each promotion, after every command and after a `SET_CONTROL` of
  the `master` row; the `Master` target is wire code 3 of #1583's `meter_entry.target`, with its
  record `frame.master_gain_reduction(index)` through #1587's gain row rule; the corpus and
  `COMPLETE_SCHEMA_HASH` re-pin once, in the order #1595b -> #1583 -> #1690b. Gate 1 extends the
  existing parity row `both_hosts_designate_the_master_strip_after_an_empty_boot_alike`
  (`hosts/host-web/tests/control_parity.rs`) rather than adding one. Gates 1, 3, 4, 5 and 7 are
  #1690b's; gate 2's C ABI record half also moves there.
  #1690b also gives `master_gain_reduction` its single host-core test (gate 2, in the owning
  crate) and removes that role from the host-web tests.
