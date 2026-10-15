# Pair carried meter and observation readers on the C ABI

Successor found by the batch-12 spec check, decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `ea336c039`.
Issue number: assigned at filing (written here as TBD24). Feature sprint mode (owner, 2026-10-13):
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

- `crates/capi/src/runtime/**` (the attachment and its tests), `crates/control-plane/src/{compile.rs,control.rs,plan.rs}`
  (the attachment at preparation and its read path), `crates/capi/tests/resource_lifecycle.rs`,
  and this spec's decision record.

## Dependencies

- Whichever issue first gives the C ABI meter or observation readers: #1581 (D6, the meter
  consumers) or #1584 (D7, the observation handles). The second of them adds its readers to the
  same attachment, and its gate (1 or 2) moves to it. Lane K.

## Size

Half a day.
