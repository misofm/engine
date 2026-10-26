# Silence as a propagated fact: no scan on the render thread

Draft slice 4 (#2133) of the banking design. Successor of #1981 (merged first, owner-accepted). Class A: no
rendered word changes; the facts the packer reads change source.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

#1981 learns which lanes' inputs were exact `+0.0` by ORing the bits of every word the gather
moves (`tile_gather`, `AoSoaScratch::gather_lane`). That is a per-word operation on every bank of
every block. The fact exists for free elsewhere: the host's chunk is copied into the source ring
on the control thread, a skipped chain is known to have emitted `+0.0`, and a submix's input is
the fold of routes whose silence is known.

**Measurement note (not a claim).** #1981 attempt 3 recorded V8 all-active at +3.0/+3.2 % and
native `dispatch_only` at +0.3 to +0.7 us against its twin; the twin ran at loadavg 3.4 and the
candidate at 6.8, so the attribution of that cost to the OR is a hypothesis this slice measures,
not a finding it rests on. The slice is justified by the design rule that silence is propagated,
whatever the number comes out as.

## Smallest closable slice

- **Monotone last-non-zero frame.** The submit path (C ABI source submit and the Worker
  instance's submit, both on the control thread) advances a per-source `last_nonzero: u64` (ring
  frame coordinates) while it copies: for every word whose bits are not `+0.0` (a `-0.0` word
  counts as non-zero, conservative, cost only) it sets `last_nonzero = frame`. A block `[a, b)` is
  silent iff `last_nonzero < a` and every frame past the written end is a `+0.0` the ring emits
  itself (underrun, held seek, zeroed tail: `source/src/lib.rs:423-427`, `:2426`). One compare per
  block, `O(1)`; a seek resets the counter with the generation. No ABI change.
- **Chain flag.** `RuntimeUnit::Bank` sets `output_silent` for every populated lane when the chain
  skipped (#1107 proves every lane emitted `+0.0`), and clears it otherwise. A per-node effect op
  sets it when it skipped and clears it otherwise.
- **Route and fold flag.** A route's output is silent iff its source is silent or the route is
  muted and settled. A submix `Input` is silent iff every route into it is silent: one AND per
  route in the fold. A fold whose inputs are all `-0.0` is not flagged (the flag is conservative).
- `BankChain::input_silent_lanes` is set from the lanes' input flags before the gather; the
  gather's OR is removed and `the_gather_reports_exactly_the_silent_lanes` is deleted in the same
  PR (its claim moves to gate 1).
- The silence skip's own head test (`block_is_positive_zero` on the gathered block) is unchanged.

## Gates (one test per claim, red first)

1. `crates/source` unit test: a block after a chunk of `+0.0` is silent; one `-0.0` word in the
   last frame of the right plane makes its block not silent; a block spanning a silent chunk and
   an underrun tail is silent; a seek resets the counter. Red on a counter that ignores the right
   plane or survives a seek. Test value: red when a block with a non-zero word is reported silent.
2. `crates/graph` unit test on a two-track-one-submix plan with the test double: the submix's
   input flag is set iff both routes' sources are silent or muted-settled; a ramping mute clears
   it. Red on a flag that ignores the mute's settle. Test value: red when a submix is reported
   silent while a route still carries signal.
3. #1981's packing gate unchanged and green with its skip count asserted unchanged (474 against
   335). Red on a flag that never sets.
4. Allocation: the existing render gate; the submit path's work is on the control thread (the
   realtime-policy tool's region list, count recorded).
5. CPU, descriptive: the V8 and native all-active console rows and `dispatch_only`, before and
   after, under one lock and comparable loadavg; recorded in the spec with the loadavg of each run.

## Paths

`crates/source/src/lib.rs`, `crates/capi/src/` (submit), `hosts/host-web/src/` (the Worker
instance's submit), `crates/graph/src/runtime.rs` (flags, route AND, chain flag),
`crates/rack/src/lib.rs` (the gather OR removed; `input_silent_lanes` set from outside),
`crates/graph-compiler/tests/silent_bank_skip.rs` (gate 3 unchanged), this spec.

## Dependencies

#1981 merged. Recommended before slice 5 (#2135) so slice 5 (#2135)'s V8 baseline is the propagated one.
