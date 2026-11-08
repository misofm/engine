# Effect contract: a render-time lane swap trusts its defaults rotation and its undo to `debug_assert!`

Successor of #2392. It came out of Sol's review of #2392 attempt 1 as an out-of-scope note. #2392 put a bank's prepared per-lane defaults, their exchange and its restore's verdict into one `effect_contract::LaneSwap` type. The swap's handling of an exchange that refuses is unchanged, and this issue owns it.

## Problem
`effect_contract::swap_bank_lanes` (`crates/effect-contract/src/live.rs`, about lines 3401-3520 at #2392's head) runs on the render thread (`Runtime::pack_lanes`). It moves two lanes' prepared defaults with `rotate_lane_defaults`: three in-place `LaneSwap::exchange_lane_defaults` calls through `spare`. It then restores each lane from the other's payload. Three results are checked only by `debug_assert!`:

- **Step 1** (about line 3489): `debug_assert!(rotated, ..)`. In a release build, an exchange that refuses partway leaves the defaults half-rotated. For example, the second exchange refuses after the first took. The first lane then holds `spare`'s bytes, and the second holds its own. The swap goes on to restore both payloads and returns `Ok(())`. A moved strip then re-seeds (a `FullToDefaults` reset, a D7 non-finite recovery) from bytes that are neither lane's defaults. Also, the next restore's validation reads those bytes.
- **The undo after a refused step 2** (about line 3497) and **after a refused step 3** (about line 3509): `debug_assert!(undone, ..)`. In release, a refused undo still returns `Err(LaneSwapRefusal::Restore)`, whose doc promises "both lanes are then as they were".
- **The step-3 payload revert** (about line 3518): `debug_assert!(reverted, ..)`. In release, if that restore refuses, the first lane is left holding the second lane's payload, under a refusal that says nothing moved.

The precondition argument is sound for a bank that keeps the exchange contract: both lanes exist because their snapshots took, and `spare` has the declared length, the only two reasons an exchange may refuse. But the argument is only the bank's promise. `LaneSwap::exchange_lane_defaults` returns `bool` because it can refuse. Every caller treats a refusal of the whole swap as "both lanes unchanged":
- `rack`'s two `can_swap_lanes`/`swap_lanes` paths (`crates/rack/src/lib.rs` about 1436-1480 and 2490-2550);
- `BankChain::swap_lanes` (`ChainLaneSwapRefusal`);
- the graph's `Runtime::swap_bank_lanes` (`crates/graph/src/runtime.rs` about 11491) and its home walk.

So a release build turns a bank defect into a silent misplacement that the swap reports as success or as a clean refusal. This is the "can an ack ever precede a drop?" question (AGENTS.md) applied to a lane swap.

## Contract
- **No `Ok` after a refused exchange.** `swap_bank_lanes` checks each exchange of the step-1 rotation. If one refuses, the swap undoes the exchanges that took (an exchange is its own inverse through the same `spare`) and returns a refusal before any payload write. The refusal is a new or existing `LaneSwapRefusal` variant, decided by the implementer and documented. Every variant except a defect variant keeps "both lanes unchanged".
- **A refused undo is reported, not hidden.** A refusal of the undo rotation, or of the step-3 payload revert, leaves the lanes in a state the swap cannot restore. It returns a distinct defect outcome that says so, never `Restore`. The callers act on it as they act on `ChainLaneSwapRefusal::Defect` today: the rack and graph callers reset or quarantine the two lanes, or re-seed them from the carry, rather than assume nothing moved. Which one is the implementer's choice, recorded with its reason. The graph caller is in `crates/graph` and is touched only for this mapping.
- **Each `debug_assert!` on these four results becomes a checked branch.** A `debug_assert!` may remain only beside a check, never instead of one.
- **Render stays realtime-safe.** No allocation, lock or syscall. The checks are bounded by the defaults size, and every new branch is inside `REALTIME_POLICY` regions.
- **No behaviour change for a bank that keeps the contract.** No rendered bit moves on the launch effects.

## Non-goals
- Making an exchange infallible by type (for example, an exchange over a checked lane token). That is worth a note, but it is a wider change to `LaneSwap`.
- The limiter's history/main-ring tie (#2393).
- `rack::lane_swap_scratch_fits`'s pre-check. #2392 recorded that its peer comparison is equivalent under the callers' program-key check.

## Gates
- **Red on base, in release and debug.** `effect-contract`'s `lane_swap_tests` gains a test bank whose exchange refuses on its *n*-th call, on the model of `Words::refuse_restore_call`. The cases are:
  - each of the three step-1 exchanges;
  - an undo exchange after a refused step 2, and one after a refused step 3;
  - the step-3 payload revert.
  On base, a release run returns `Ok` with half-rotated defaults (or `Restore` with misplaced lanes), and a debug run panics in the `debug_assert!`. Record both. The fix's run is green and asserts the documented outcome and lane state.
- **Caller mapping.** A `rack` test drives the defect outcome through `BankChain::swap_lanes` and asserts the chosen recovery.
- **Mutation evidence.** Reverting each new check to its `debug_assert!` turns its case red in release.
- **No bit moved.** A one-time comparison against base of a lane-swapping console render, as #2392 recorded with `silent_bank_skip`'s gate 3. This is PR evidence, not a committed digest.
- **Gates for every touched crate:**
  - `cargo test --locked` (release as well as debug for `effect-contract`'s lane-swap tests);
  - clippy `-D warnings`, `cargo fmt --check` and `cargo doc -D warnings`;
  - `scripts/check-workspace-policy.sh` and `realtime_policy`;
  - the call-graph gate (`scripts/check-web-audioworklet.sh`) on a rebuilt artifact.

## Evidence / attempt record
(empty)
