# #1816 Drain carry_effect_lanes's pending Observe queues in a shape the drain rule can select

Prerequisite of #1418 (root decision, 2026-10-24, from #1418's scoping).

## Problem (verified on `main` at `661c96a5e`)

#1418 D1 requires every loop around a realtime drain to select a different queue on each pass, in a
form the realtime-policy tool proves from the written code. The second loop of `carry_effect_lanes`
(`crates/graph/src/runtime.rs`, inside its marked region) bound its queue by a struct `let` from
`observe_target_at(&mut predecessor.units, step.predecessor)`: a keyed lookup (`effect_lane_mut` ->
`effect_stage_mut` downcast -> `LiveControlEffectBankStage::observe_target_mut`, a `position` search,
or `LiveControlEffect::observe_target_mut`), none of D1's forms. It passed only because the outer
rule checks finiteness.

## Smallest closable slice

Reach the pending Observe queues through one call outside any loop, into the owner's own drain walk
(the shape of the rack's `drain`). Keep #1444's call rule. No behaviour change.

## Decisions

- **D1 (root R1, accepting the scope note).** A graph walk cannot reach a bank stage's queues by
  fields (`Box<dyn BankStage>`), and #1444 rule 1 refuses a popping method called in a loop. So the
  carry uses the architecture the block path already has: a non-popping hook that hands off to the
  owner's one drain loop.
  - rack: `LiveControlEffectBankStage` gets `carry_lanes: u8` (bit n = lane n) and
    `mark_observe_carry(lane)`. `drain`'s second loop is now the shared
    `drain_observe_queues(first_sample, lanes)`, with `if lanes & (1 << *index) == 0 { continue; }`
    first; `drain` passes `u8::MAX`. New `BankStage::carry_observes(first_sample)` (default no-op;
    the live impl takes the mask and runs the shared loop). `BankChain::carry_observes` calls it on
    every slot (no active-lane guard: the mask selects).
  - graph: `LiveControlEffect` gets `observe_carry: bool` and `carry_observes`, which runs
    `EffectControlLane::drain_observations` when marked. The first loop marks each predecessor as
    soon as it resolves, before any successor-side `continue`. The second loop is replaced by one
    call, `predecessor.carry_observes(predecessor_next_sample)`, a walk over the units.
  - Deleted: `rack::ObserveTarget`, both `observe_target_mut`, `into_observe_target`,
    `observe_target_at`.
- **D2.** This is the second instance of #1444 D4's stated limit ("a helper two calls deep"); the
  first is `BankChain::run` -> `begin_block` -> `drain`. The hook is the owner's carry entry, not a
  wrapper written to pass. Recorded in one line in #1418's spec.
- **D3. Equivalence.** A predecessor site was drained iff `observe_target_at` was `Some`; the mark
  is set under the same resolution, and a mark on a lane with no queue selects nothing. Each queue
  pops `available_at_entry()` times, records go to the same tap at `predecessor_next_sample`, and
  unbound counts sum into the same `self.unbound` (per-node: discarded, as before). Only the
  cross-queue order changes (was `moves` order; now unit, slot, lane), which is unobservable:
  disjoint queues, one sample, saturating sums. A duplicated predecessor site drains once instead
  of once plus an empty pass. No audio arithmetic is touched.
- **D4. Tests.** One tool twin, `twin-carry-observe-lanes` in
  `tools/realtime-policy/tests/policy/drain/twins.rs`. No new production test: host-core
  `successor_swap`'s `pending_observe_*` tests are red if a mark is missed or the hook is not
  reached (mutant below).

## Authorized paths

`crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs` (the named regions),
`tools/realtime-policy/tests/policy/drain/twins.rs`, this spec, one line in #1418's spec.

## Gates

1. `cargo run --locked --release -q -p realtime-policy`: `ok`, region and file counts unchanged.
2. `cargo test -p realtime-policy`; `rack --test live_control_bank`; host-core (`test-support`)
   `successor_swap` and `browser_successor`; `cargo test -p graph`.
3. No bit moves: `scripts/check-graph-determinism.sh` JSON identical at base and head (one-time PR
   evidence), `scripts/test-builtins-fixtures.sh`, `scripts/check-console-fixtures.sh`.
4. clippy `-D warnings`, fmt, aarch64 `--tests` check, rustdoc `-D warnings`.

## Evidence (attempt 1)

- Recorded in the implementer's report and the PR.
