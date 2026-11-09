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

### Attempt 1 (Terra, 2026-11-09), branch `codex/issue-2427-bank-contract-defects`
Started on #2424's branch (head `847855857`), which shipped `GraphLaneSwapDefectWatch`. Rebased onto `origin/main` `ca6d112ee` after #2424 merged as PR #2429. The rebase was a no-op: the two trees are identical.

**Claims re-verified.** Every claim holds, so none is dropped:
- `graph::lane_swap_defect_count` and `graph::carry_refusal_count` (`crates/graph/src/lib.rs`) are per-plan reads over `&mut PreparedRenderPlan`.
- `SERVED_COUNTERS` had 9 entries, not the 10 the spec says. It now has 10.
- No `CounterId` carried either total.

**What landed.**
- **Wire.** `protocol::CounterId::BankContractDefects = 23` has its enum arm and its decode row (`crates/protocol/src/message_wire.rs`). The frozen-ID test now lists 23, and the first ID it refuses is 24. IDs 2 and 11 to 14 stay retired.
- **SDK.** The name is `bankContractDefects` (`sdk/src/core/protocol-messages.ts`). The fixture was re-blessed with `MISO_ENGINE_BLESS_FIXTURES=1`, and its only change is the `counterIds` row.
- **Docs and header.** One sentence was added to `miso_engine_v1.h` ("protocol counter 23, read with COUNTERS_GET or a configured COUNTER_SNAPSHOT"). The ID's row is in `CONTROL_PROTOCOL_REGISTRY.md`. `C_ABI_V1_QUALIFICATION.md` now says ten served counters and a ten-slot minimum. There is no new symbol, struct or feature bit.
- **One defect-path signal (the shared edge with #2424).** The same watch now holds a second total, `carry_refusals: AtomicU64`, beside #2424's `recoveries`. `GraphLaneSwapDefectWatch::bank_contract_defects()` returns their sum.
  - The watch's table is now made for every graph plan, so a plan with no pack group can still report a refused carry.
  - `lane_swap_defect_watch_bytes` charges the table's header with no strips. The projection in `prepare.rs` charges no strip-ID text below two strips, so the projected bytes stay equal to the actual ones.
- **Render, on the refused-restore path only.** The three refusing arms of `Runtime::carry_effect_lanes` are lane to lane, per-node instance to lane, and lane to per-node instance. Each arm's `else` branch now calls `mirror_carry_refusals`. That function makes one release store of the plan's total: the banks' `carry_refused`, through `bank_carry_refusals` (which `Runtime::carry_refusal_count` now delegates to), plus this carry's per-node refusals. A plan takes its predecessor once, so the executor's `node_carry_refused` is 0 before the carry, and the stored total equals `carry_refusal_count`. A carry that refuses nothing runs no new instruction. The lane-swap total travels on #2424's existing `raise` store, unchanged.
- **Control plane (the read: a mirror, not a read at reclaim).**
  - `SessionState::bank_contract_defects()` sums `bank_contract_defects_reclaimed` with the watches of the running epoch and each parked epoch. `refresh_provider_counters` sets the counter from that sum.
  - `synchronize_plan_epochs` adds a reclaimed epoch's final totals when it removes the epoch. A plan therefore moves from the held sum to the reclaimed sum in a single step: never counted twice, never lost.
  - Pending epochs are not read. One render has not adopted holds no defect: a `debug_assert_eq!` checks this where a withdrawn candidate drops, and it is never counted. One render has adopted is promoted by the service step of the first control call after its adoption block, before the refresh runs.
  - **Lag:** current by the first control call after the defect's block. Render's release store happens before that block publishes `active_epoch`, and every session call runs the service step.
- **Host-core.** `BankContractDefects` joins `SERVED_COUNTERS`, which now has 10 entries, and gets `set_bank_contract_defects`. The in-process functions keep their signatures, and each doc gained one sentence pointing at the ruling and the counter.
- **Test seam.** `graph::test_only_refuse_carry_restores(n)` (test-support only, thread-local) hands the carry's next `n` restores an empty payload scratch. The stage's own check then refuses each one and counts it, so tests drive a real refused restore.

**Where the tests sit (a deviation from the spec's gate list).** The spec asks for both host-core tests to read the counter "through the provider". But the summing (lag, reclaim, parked plans) is control-plane logic, and the one-test-per-claim rule puts a test in the crate that owns its logic. So:
- host-core holds the render-side claim;
- control-plane holds the accumulation claims;
- the provider's own test covers the served slot.

Tests:
- `host-core` `successor_swap::a_refused_carry_restore_reaches_the_defect_watch`: on each of the three refusing paths, with two restores refused, the watch holds `carry_refusal_count`'s total (2) and the hand-over still carries. Test value: a refusing path that stores nothing, a store of one refusal instead of the plan's total, or a total that leaves out the banks' or the per-node refusals.
- `control-plane` `lane_swap_residue::every_lane_swap_recovery_is_counted_once_over_its_plan_life`: a strand counts 1 at the next service call. It stays 1 while the displaced plan is parked under `test_hold_reclamation`, and stays 1 after the reclaim. A strand on the successor makes it 2. Test value: a counter that lags past the next control call, double-counts or drops a plan at its reclaim, or skips a parked plan.
- `control-plane` `lane_swap_residue::a_refused_carry_restore_is_counted_once_over_its_plan_life`: two refused restores at an adoption read 2, equal to `carry_refusal_count`. The count stays 2 after a later edit displaces the plan and its reclaim drops it. Test value: a counter that misses the carry total, takes one refusal for the total, or loses a plan's refusals at reclaim.
- Parity rows, one per host:
  - `capi` `counters_get_serves_a_refused_carry_as_bank_contract_defects` (through `COUNTERS_GET`);
  - `host-web` `the_counters_get_reply_counts_a_refused_carry_restore` (through the Worker's counters read). This row needs a new dev-only `graph` (test-support) dependency in `hosts/host-web/Cargo.toml`. The browser artifact does not link it.
- Extended:
  - `protocol` `every_counter_id_round_trips_and_the_next_is_refused` (23 round-trips, 24 is refused);
  - host-core `clock_transport_counters_and_diagnostics_are_session_backed` (slot 9);
  - the capi `COUNTERS_GET all` pinned frame, re-pinned: length 0x1c0 to 0x1f0, TLV count 10 to 11, one more 40-byte value message.

**Red on base.** On the base (`ca6d112ee`) every new test fails to compile: there is no `CounterId::BankContractDefects`, no `graph::test_only_refuse_carry_restores` and no `GraphLaneSwapDefectWatch::carry_refusals`. The behaviour itself is pinned by the mutation sweep below. Each mutant puts back a base-equivalent gap, such as no store, no sum term or no decode row, and turns its tests red.

**Mutation sweep.** Each mutant was applied to the committed tree, its tests were run in debug and in release, and the file was restored from `HEAD`. Every row was red in both profiles, and every debug red is an assertion failure, not a compile error. The release rows ran their test binaries and failed.

| # | Mutant | Red tests (debug and release) |
|---|---|---|
| M1 | lane-to-lane refusal stores nothing | host-core watch test, control-plane carry test, capi row, host-web row |
| M2 | per-node-to-lane refusal stores nothing | host-core watch test |
| M3 | lane-to-per-node refusal stores nothing | host-core watch test |
| M4 | mirror leaves out the banks' refusals | host-core watch test |
| M5 | mirror leaves out the per-node refusals | host-core watch test |
| M6 | mirror stores 1, not the total | host-core watch test, control-plane carry test |
| M7 | watch sum leaves out the carry refusals | control-plane carry test |
| M8 | watch sum leaves out the recoveries | control-plane lane-swap test |
| M9 | session sum leaves out the reclaimed plans | both control-plane tests |
| M10 | session sum leaves out the parked plans | control-plane lane-swap test |
| M11 | session sum leaves out the running plan | control-plane lane-swap test |
| M12 | reclaim adds nothing (a retired plan is dropped) | both control-plane tests |
| M13 | reclaim adds twice (double count) | both control-plane tests |
| M14 | refresh never sets the counter | control-plane lane-swap test, capi row, host-web row |
| M15 | decode row for 23 missing | protocol frozen-ID test, SDK fixture test |
| M16 | provider does not serve the ID | host-core provider test |
| M17 | setter writes another ID | host-core provider test |

**The healthy-path store.** No gate can see a mirror store moved to the taken arm. It would store the plan's total, which is 0 on a healthy carry, so the value is unchanged and the audio is unchanged. Neither the digest (which covers audio and observed words) nor the realtime policy (which polices spellings, not instruction counts) can tell, so this is an equivalent mutant; it was not run. The rule against it is that the store sits only in the `else` arms, which inspection shows. A healthy-path *raise* on the lane-swap side is caught by #2424's existing `only_a_defect_raises_the_defect_watch`.

**No bit moved.** The #2392 instrumentation was used: `graph-compiler` `silent_bank_skip` gate 3, an FNV-1a digest over the moved plan's PCM and both plans' observed words, run in `--release`. The scratch `digest2392.patch` was not committed. Gate 3 includes a carried plan swap. Results:
- head: `inserts=false 7d31fea2b6652670`, `inserts=true 8e4dfb8b8dc1d599`;
- base (`ca6d112ee` graph sources): identical.

These are also the values #2424 recorded.

**Gates.** One private target directory was used, with `-j 4`, one crate at a time.
- `cargo test --locked` in debug, every touched crate, with its test-support feature: `graph`, `host-core`, `control-plane`, `protocol`, `capi` and `host-web`. All passed. `host-core --lib` was run again with `control-provider`.
- `cargo test --locked --release` for the new and extended tests passed. The runs were: `host-core` `successor_swap` (the watch test) and the provider test, all 12 `control-plane` `lane_swap_residue` tests, the `capi` counter tests, the `host-web` counter tests, and `protocol` in full.
- `cargo fmt --all --check`: passed.
- clippy `-D warnings` with `--all-targets` on the six crates, with and without test-support: passed.
- Builds without test-support features: passed.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` on the six crates: passed.
- `check-workspace-policy.sh`, `check-host-core-policy.sh`, `check-graph-policy.sh` and `check-protocol-control-policy.sh`: passed.
- `realtime_policy`: ok, 315 marked regions in 48 files. The first run found the helpers' own markers nested inside the enclosing region; they were removed.
- Web: `build-web-audioworklet.sh --named-twin` passed. `check-web-audioworklet.sh --without-metadata-regeneration` passed, using wabt for the call graph. The render closure is 38, the same as #2424, with 5 traps all owned by `render_inner` and `atomic_waits=0`.
- Wire and ABI gates: `check-capi-abi.sh` passed. That covers the frozen symbol set, the header consumers built at `-Werror`, and shared and static linkage. The SDK checks also passed: `check-sdk-generated.sh` against the built artifact, `check-sdk-deletions.py`, `check-sdk-types.sh` (after `npm ci`), and `check-sdk-headless.sh`, whose evals include "the counter names are the codec's counter IDs". The protocol corpus hash does not move: no corpus frame names counter 23.
