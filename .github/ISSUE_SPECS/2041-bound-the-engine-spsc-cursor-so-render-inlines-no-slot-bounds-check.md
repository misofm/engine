# Bound the engine SPSC cursor so render inlines no slot bounds check

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: #1577 (its verifier) and #1381a D4, #1581 D4, #1588b.

## Problem
The engine's bounded SPSC ring wraps its cursor by a compare (`wrap_increment`), and its slots are
indexed with a checked `slots[cursor]`. The optimiser cannot prove the cursor is below `slots.len()`, so
every inlined push and pop carries a `panic_bounds_check` that cannot fire. The browser render gate
admits each one by name and count (trap owners), and each new render-thread SPSC user adds a pin.

## Evidence (main `c1a218022`)
- `crates/engine/src/realtime/spsc.rs:67-77`: `wrap_increment` (compare, not `%`; `slot_count = capacity + 1`
  is never a power of two, #84 F6).
- `spsc.rs:205`, `:482`, `:556`, `:629`, `:653`: the checked `slots[...]` sites.
- `scripts/check-web-audioworklet.sh:574-603`: the render callgraph's trap owners
  `RealtimePlanOwner::enter_block`, `OutputPeakObserver::observe=1`, `ResponseCaptureServer::after_block=2`,
  `MeterDelivery::poll=3`, `MeterDelivery::begin_generation=1`, `MeterFrameProducer::publish_frame=2`;
  `:626-627` the same for `meter_poll`.

## Smallest slice
Give the ring a cursor the optimiser can bound without `unsafe`, for example a power-of-two physical slot
count with a mask (the logical capacity stays exact; `available_capacity` and backpressure stay as they
are), or an index type whose range the compiler can prove. Measure the hot path once (native and the
browser shape) before and after under the AGENTS.md benchmark rules. Remove every trap-owner pin the
change makes unneeded; a pin that stays gets a named reason.

## Gates
- `cargo test --locked -p engine` (with the loom SPSC tests where they run today) pass; queue capacity and
  backpressure semantics unchanged.
- `bash scripts/check-web-audioworklet.sh` passes with the SPSC trap-owner pins removed (or each kept pin
  named); no rendered bit moves (browser native-digest gate unchanged).
- One descriptive benchmark of push/pop, recorded; no regression beyond the 2 % allowance.
- `cargo clippy --locked --all-targets -p engine -- -D warnings`, AArch64 check.

## Paths
`crates/engine/src/realtime/spsc.rs`, `scripts/check-web-audioworklet.sh` (the trap-owner lines only),
this spec.

## Decision record
- D1 (root, binding, 2026-10-25): `get_unchecked` under `spsc.rs`'s unsafe charter, not the
  power-of-two mask the slice above proposes. LLVM cannot prove `cursor & mask < slots.len()` from
  two independent fields, the mask reverses #84 F6, and it grows rings up to 2x. `wrap_increment`,
  capacities and backpressure are unchanged.
- D2: `Ring::slot` and `Mailbox::cell` are the two unchecked lookups (unsafe fns, each with a
  `debug_assert!` of its bound); every call site's `SAFETY` states the invariant (a cursor advances
  only through `wrap_increment(cursor, slots_len)`; a cell index comes only from
  `MailboxWord::cell_in`). The lookup sits inside the `sync::with`/`with_mut` argument, so loom
  still sees every access. `docs/REALTIME_DEPENDENCY_POLICY.md`'s `spsc.rs` rows name them.
- D3: `scripts/check-web-audioworklet.sh` drops every SPSC trap-owner pin (render: `enter_block`,
  `observe=1`, `after_block=2`, `poll=3`, `begin_generation=1`, `publish_frame=2`; meter_poll:
  `poll=3`, `publish_frame=2`). No pin stays.

## Evidence (attempt 1, PR evidence only)
- Gate with the pins removed: render closure `traps=5`, owners only the allow-listed
  `PreparedRenderPlan::render_inner`; meter_poll `traps=0`. A mutant that restores
  `&self.slots[cursor]` in `Ring::slot` is red (six unexpected owners, `traps=16`). A mutant that
  restores `&self.cells[cell]` in `Mailbox::cell` stays green: on wasm the optimiser already
  proves the inlined `cell_in` index below 2, so the mailbox change removes no browser site.
- Push/pop, throwaway harness (capacity 255, `u64`, x86-64-v3, one invocation, one warmup, two
  rounds), ns per push+pop pair, before `4448e90b3` / after: fill-drain 2.2835, 2.3119 /
  2.1950, 2.1955; one-push-one-pop 2.4865, 2.3800 / 2.2256, 2.2251.
