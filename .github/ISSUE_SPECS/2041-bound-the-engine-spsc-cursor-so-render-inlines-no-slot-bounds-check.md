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
