# Counters: `BankContractDefects` (23) puts the lane-swap defect and carry-refusal totals on the wire

Successor of #2425, whose ruling this issue implements: `docs/rulings/bank-contract-defect-counter-2026-11-09.md` (root under the owner's delegation of 2026-11-09). Independent of #2424's rebuild; the two may land in either order, and the one shared edge is named below.

## Problem
`graph::lane_swap_defect_count(plan)` (#2414, #2419) and `graph::carry_refusal_count(plan)` (#1279 D6) are per-plan, in-process reads over `&mut PreparedRenderPlan`. The browser Worker holds no plan and a C-ABI app reads counters only through `protocol::CounterId`, so neither shipping host can see that a bank broke its lane contract in production. The ruling settles that one accumulating wire counter carries both totals, and rejects two IDs, a fold into `StripsRestarted`, a watermark flag and a per-plan value.

## Contract
- **One frozen ID.** `protocol::CounterId::BankContractDefects = 23` (`crates/protocol/src/message_wire.rs`), with its decode row and SDK fixture name (`bankContractDefects`). IDs 2 and 11 to 14 stay retired. Doc: the bank-contract defects the engine contained over every plan the session adopted; one count is one lane-swap recovery as #2419 defines it (a strand, a split slot or a split row's reset) or one refused carry restore; 0 in a correct build; non-resetting.
- **Exact and accumulating.** The counter is the sum over every plan the session adopted of the two per-plan totals, each plan counted once. No defect is double-counted between a plan's running value and its reclaim, and none is lost when a plan is reclaimed, discarded unadopted (an unadopted plan has no render-time defects: assert it, do not count it) or alive at session destroy (nothing to report after destroy).
- **No render cost on a healthy block.** The ruling prefers a mirror written on the defect path only, on the `live_block_errors` (#1521 D3) pattern: the graph's recovery paths (`recover_lane`/`reset_swapped_lanes` arms, the chain's and row's split recoveries, the refused-restore branch of the carry) already run inside `REALTIME_POLICY` regions and already count into the stage; the plan-side total reaches a shared `AtomicU64` with one `Release` store on those paths and nowhere else. A healthy block executes no new instruction; the implementation shows this with the existing no-bit-moved digest and the realtime-policy gate, and states the mirror's lag (current by the first control call after the defect's block). If the implementer instead reads at reclaim in `synchronize_plan_epochs`, the spec's evidence must say why and record the lag (the active plan's defects wait for its retirement).
- **The shared edge with #2424.** #2424 raises a render-side flag on the lane-swap defect path for its rebuild. Whichever lands second reuses the first's defect-path signal rather than adding a parallel one: one atomic (or one pair) on the defect path carries both the flag and the running total. Neither issue adds a second `CounterId`.
- **Wired through every host the same way.** Host-core: the ID joins `SERVED_COUNTERS` (`crates/host-core/src/control_provider.rs`, now 10) and gets its `set_bank_contract_defects`; the control plane adds it to `refresh_provider_counters` (`crates/control-plane/src/control.rs`, which already rules that a new counter joins this refresh and never a single control call). C ABI: one documented line in `crates/capi/include/miso_engine_v1.h` ("protocol counter 23, read with COUNTERS_GET or a configured COUNTER_SNAPSHOT"); no new symbol, struct or feature bit. Browser: the render half's status carries the mirror to the Worker as `live_block_errors` does, and the Worker's counters read reports it.
- **The in-process functions stay.** `graph::carry_refusal_count` and `graph::lane_swap_defect_count` keep their signatures; their docs gain one sentence each pointing at the ruling and the counter. The #1286 cost record keeps reading them.

## Non-goals
- Changing what either total counts.
- The rebuild on a nonzero lane-swap total (#2424).
- Telling the two totals apart on the wire (the ruling's reopening condition).

## Gates
- `protocol`: the frozen-ID tests extended for 23 (encode, decode, the retired set unchanged, the SDK fixture name).
- `host-core`: one test that strands a lane swap (as #2419's `SwapRig`/`FaultyStage` tests do, built off the render thread) and one that refuses a carry restore, each reading the counter through the provider after the defect's block, with the exact count the in-process function reports for the same plan. Test value: a counter that lags past the next control call, double-counts across reclaim, or drops a plan's defects when it retires.
- `capi` and `host-web`: one parity row each through COUNTERS_GET and the Worker's counters read.
- Realtime: the realtime-policy gate; allocation counters zero on render; the `silent_bank_skip` gate-3 digest once against base as no-bit-moved evidence (PR evidence, not a committed digest).
- Mutation evidence: the store dropped, the store on the healthy path (caught by the digest or the policy gate), the sum missing one term, the ID decode row missing.
- Gates for every touched crate: `cargo test --locked`, clippy `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings`, `scripts/check-workspace-policy.sh`; the call-graph gate (`scripts/check-web-audioworklet.sh`) if render dispatch changes.

## Evidence / attempt record
(empty)
