# Pair carried meter and observation readers on the C ABI

Successor found by the batch-12 spec check, decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `ea336c039`.
Issue number: assigned at filing (written here as #1674). Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review.

## Product outcome

A C ABI app reads a strip's meter and an effect's observation tap across a structural edit with no
restart. When the swap block carries a meter or a tap, the app's reader follows the state that
continues on the successor. The reader does not stay on the retiring plan.

## Context

- The carry moves the observer state, producer included, into the successor in the swap block
  (#1327a meters, #1327b taps). Then it writes `ObserverCarry` records
  (`crates/engine/src/realtime/plan.rs:548-560`). A host whose readers live in the plan's
  attachment pairs them in the same block through `HostAttachment::adopt_predecessor`, which
  `PreparedRenderPlan::carry_from` calls (`plan.rs:1237`). The browser does this
  (`RenderCompanions`, `hosts/host-web/src/lib.rs:5190`, #1381b) with
  `host_core::pair_carried_meters` and `pair_carried_observations`
  (`crates/host-core/src/prepare.rs:3293`, `:3263`), both allocation-free and inside realtime-policy
  regions.
- The C ABI has no reader of either kind today. `CapiPreparer` asks for no meters and no observation
  capacity (`crates/capi/src/runtime/mod.rs:97`), and `prepare_runtime` drops the handles it does
  not keep (`crates/control-plane/src/compile.rs:713-719`). No `HostAttachment` exists outside
  host-web and the tests.
- #1581 D6 keeps the meter consumers in `PreparedRuntime` (read by #1587), and #1584 D7 keeps the
  observation handles there (read by #1586). Those fields are on the control side, and the swap
  block cannot reach them. After a carry, they would read the retiring plan's producers, which
  then go silent.

## Decisions frozen for this slice

D1-D3 below are replaced by R1 (root, 2026-10-24); they stay for the record.

- **R1. The readers pair on the control thread at promotion (#1593 D5 precedent).** host-core's
  `PlanStateInventory` holds `meter_pairing: Box<[engine::realtime::ObserverCarry]>`, built at
  preparation from the meter moves alone (in the swap block's record order), and charged in the inventory's retained bytes beside `spectrum_pairing`. `promote_pending_provider`
  calls `host_core::pair_carried_meters(&mut next.meters, &mut self.providers.meters,
  next.inventory.meter_pairing())` before the replace; the parked epoch keeps the fresh
  consumers and drops them with its plan. Why D1 cannot hold: `active_attachment_mut` is on
  `PlanState`, which is the render thread's alone (`control-plane/src/plan.rs`), and the C ABI's
  readers are read on the control thread (`SessionState`, where `take_output_meter` is and where
  #1583/#1587 stage `METER_BATCH`), which cannot reach the attachment. The meter rows equal the
  swap block's record's meter entries: the observer carry is all or nothing and runs every meter
  move, and a promoted candidate was prepared against the plan it displaced (#1310 D3). The list
  is not the whole record: the record also holds spectrum entries (paired by `spectrum_pairing`)
  and tap entries, and a tap enters the record only when its effect restore succeeded at the swap
  (`carry_taps` records taken taps whose `swap_state` succeeded), which preparation cannot know. The rule's one assumption, that
  promotion follows a carried adoption, is the one spectrum pairing makes (#1593 D5). No attachment,
  no render-thread code. **#1587 D1's `carries` source is `inventory.meter_pairing()` at
  promotion, not an attachment's record.** The observation half lands with #1584 D7, when the
  epoch keeps `effect_observations`; gate 2 moves there. **Constraint on #1584 D7 and #1587 D1:**
  observation pairing must be taken from the swap record's tap entries (what actually carried),
  not from a preparation-time list; `meter_pairing` holds meter rows only.
- **D1. The readers live in the plan's attachment.** The C ABI's plan carries one attachment that
  holds the meter consumers and the observation handles, in host-core's order. The giving issue's
  `PreparedRuntime` fields move into this attachment. The C ABI reads them through the plan owner's
  `active_attachment_mut` (`crates/control-plane/src/plan.rs:244`), on the thread that reads them
  today.
- **D2. The hook.** The attachment implements `HostAttachment::adopt_predecessor`. It downcasts the
  predecessor through `as_any_mut` and calls `pair_carried_meters` and `pair_carried_observations`
  with the record. Another attachment type, or an empty record, pairs nothing.
- **D3. Realtime.** The hook makes no allocation, takes no lock and makes no syscall. Its
  bookkeeping is sized at preparation and charged to the plan's retained rows.
- **D4. No engine edit.** `adopt_predecessor` and the record exist (#1327a).

## Objective gates

Under R1, gates 1-3 are: (1) a capi test, `a_carried_strip_meter_keeps_its_reader_across_a_c_abi_rebuild`
(`crates/capi/src/runtime/tests.rs`); (2) one assertion in host-core's `Paired` rig test,
`meter_pairing()` equals the record's meter entries; (3) no new render-thread code, so `audit capi` is unchanged
evidence. The observation gate waits for #1584 D7.

1. **A meter window continues.** On the C ABI, meters on, add a muted track whose ID sorts first
   (a rebuild). The strip that was meter 0 and is now meter 1 continues its window sequence through
   the reader that the app already holds, with no counted loss. Its first window after the swap
   equals the reference window field by field.
2. **An acknowledged subscription continues.** With a compressor tap subscribed and its window
   open, the same rebuild keeps the window sequence on the app's reader, and the next window
   equals the window that the predecessor would have published.
3. **Realtime.** The swap block, the hook included, makes zero allocations and frees.
   `target/release/audit capi` reports 0 allocations, locks and syscalls.
4. Commands: `cargo test --locked -p capi`, `cargo test --locked -p control-plane --features
   test-support`, `cargo build --locked --release -p audit -p capi && target/release/audit capi`,
   `cargo run --locked --release -q -p realtime-policy`, clippy, `cargo fmt --all -- --check`.

## Test value

- Gates 1 and 2 turn red if a carried meter or tap leaves the C ABI's reader on the retiring plan.
- Gate 3 turns red if the pairing allocates or frees on the render thread.

## Authorized paths

Under R1: `crates/host-core/src/prepare.rs`, `crates/control-plane/src/control.rs`
(`promote_pending_provider`, the `meters` doc), `crates/capi/src/runtime/tests.rs` (one test),
`crates/host-core/tests/successor_swap.rs` (one assertion), this spec.

- `crates/capi/src/runtime/**` (the attachment and its tests), `crates/control-plane/src/{compile.rs,control.rs,plan.rs}`
  (the attachment at preparation and its read path), `crates/capi/tests/resource_lifecycle.rs`,
  and this spec's decision record.

## Dependencies

- Whichever issue first gives the C ABI meter or observation readers: #1581 (D6, the meter
  consumers) or #1584 (D7, the observation handles). The second of them adds its readers to the
  same attachment, and its gate (1 or 2) moves to it. Lane K.

## Size

Half a day.

## Evidence (attempt 1, 2026-10-24)

- Gate 1 is red on main: the running epoch's `test_meters()[1]` is empty after the rebuild. A
  strip queue holds one window (capacity 1) and nothing pops it, so the test rebuilds at half a
  window and reads the window that opened on the predecessor at sample 0 and closes on the
  successor: `(0, 0, 1536)` at 48 kHz with the fix.
- Gate 1 also asserts no counted loss (`cumulative_dropped_snapshots` and
  `cumulative_discontinuities` are 0 on the carried window). Not tested: that the parked epoch's
  entry holds the fresh consumer. The test API cannot tell the two consumers apart (the swap
  exchanges only the ring, both rings are empty at the rebuild, and the parked epoch drops with
  its reclaimed plan).
- `target/release/audit capi`: 0 allocations, deallocations, locks and syscalls over 100,000
  calls (`total_violations` 0). `cargo run --locked --release -q -p realtime-policy`: ok, 212
  marked regions in 38 files. Both unchanged: no render-thread code is new.
- Mutants: passing `&[]` as the pairing in `promote_pending_provider` turns gate 1 red; swapping
  `successor`/`predecessor` in the meter rows of `meter_pairing` turns gate 2 red.
- R1's charge moved one existing claim: `a_plan_with_spectrum_charges_the_service_once`
  (`crates/control-plane/tests/spectrum_monitor.rs`, one line, root-authorized) expected the third
  plan to cost the boot plan's bytes, but that plan carries nine meters, whose pairing rows R1
  charges (216 bytes). It now expects the boot plan's bytes plus the size of
  `meter_pairing()`, read from the accessor; the claim (the service is charged once) stands.
