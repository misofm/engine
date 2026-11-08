# Effect contract: a render-time lane swap trusts its defaults rotation and its undo to `debug_assert!`

Successor of #2392. It came out of Sol's review of #2392 attempt 1 as an out-of-scope note. #2392 put a bank's prepared per-lane defaults, their exchange and its restore's verdict into one `effect_contract::LaneSwap` type. The swap's handling of an exchange that refuses is unchanged, and this issue owns it.

## Problem
`effect_contract::swap_bank_lanes` (`crates/effect-contract/src/live.rs` lines 3401-3522 at `05b58129c`; every line cite below is at that commit) runs on the render thread (`Runtime::pack_lanes`). It moves two lanes' prepared defaults with `rotate_lane_defaults`: three in-place `LaneSwap::exchange_lane_defaults` calls through `spare`. It then restores each lane from the other's payload. Four results are checked only by `debug_assert!`:

- **Step 1** (line 3489): `debug_assert!(rotated, ..)`. Call the first lane's defaults `X`, the second's `Y`, and `spare`'s prior bytes `A`. The rotation is three exchanges:
  1. first lane with `spare`;
  2. second lane with `spare`;
  3. first lane with `spare` again.
  It folds their results with `taken &= ..` (lines 3334 and 3339), which does not short-circuit. So a refused exchange changes nothing and the later exchanges still run. In a release build the swap then restores both payloads and returns `Ok(())`. The outcome depends on which exchange refuses:
  - **Exchange 1 refuses:** the first lane ends with `Y` (correct), and the second lane ends with `A`, which is garbage.
  - **Exchange 2 refuses:** exchange 3 hands the first lane its own bytes back. Both lanes keep their own defaults, `X` and `Y`, while the payloads cross. No byte is garbage, but each moved strip has left its defaults behind.
  - **Exchange 3 refuses:** the second lane ends with `X` (correct), and the first lane ends with `A`, which is garbage.
  A moved strip then re-seeds (a `FullToDefaults` reset, a D7 non-finite recovery) from bytes that are not its own defaults, and the next restore's validation reads them.
- **The undo after a refused step 2** (line 3497) and **after a refused step 3** (line 3509): `debug_assert!(undone, ..)`. The undo is the same three exchanges from `(Y, X)` with `spare` holding `A`. In release, a refused undo still returns `Err(LaneSwapRefusal::Restore)`, whose doc promises "both lanes are then as they were". By the same arithmetic:
  - its first exchange refusing leaves `(X, A)`;
  - its second leaves `(Y, X)`, so the defaults stay crossed;
  - its third leaves `(A, Y)`.
- **The step-3 payload revert** (line 3518): `debug_assert!(reverted, ..)`. In release, if that restore refuses, the first lane is left holding the second lane's payload, under a refusal that says nothing moved.

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
  The test bank's restore ignores the defaults, as `Words`' does, so a step 2 or 3 refuses only where a case makes it. On base, each case is red as follows. Record both builds for every case.
  - **Debug build:** every case panics in its `debug_assert!`. The rotation cases panic with "a checked lane's defaults exchange", the revert case with "a restore its own validation passed".
  - **Release build, step-1 exchange 1 refuses:** `Ok`, and the second lane holds `spare`'s prior bytes.
  - **Release, exchange 2:** `Ok`, and both lanes keep their own defaults under the crossed payloads.
  - **Release, exchange 3:** `Ok`, and the first lane holds `spare`'s prior bytes.
  - **Release, an undo exchange refuses** (one case for each of the undo's three exchanges, after a step-2 refusal and after a step-3 refusal): `Err(Restore)` with the defaults left `(X, A)`, `(Y, X)` or `(A, Y)` as above. After step 3 the payload revert has run as well.
  - **Release, the step-3 revert refuses:** `Err(Restore)` with the first lane holding the second lane's payload.
  The fix's run is green and asserts the documented outcome and lane state for each case.
- **Caller mapping.** A `rack` test drives the defect outcome through `BankChain::swap_lanes` and asserts the chosen recovery.
- **Mutation evidence.** Reverting each new check to its `debug_assert!` turns its case red in release.
- **No bit moved.** A one-time comparison against base of a lane-swapping console render, as #2392 recorded with `silent_bank_skip`'s gate 3. This is PR evidence, not a committed digest.
- **Gates for every touched crate:**
  - `cargo test --locked` (release as well as debug for `effect-contract`'s lane-swap tests);
  - clippy `-D warnings`, `cargo fmt --check` and `cargo doc -D warnings`;
  - `scripts/check-workspace-policy.sh` and `realtime_policy`;
  - the call-graph gate (`scripts/check-web-audioworklet.sh`) on a rebuilt artifact.

## Evidence / attempt record
### Attempt 1 (Terra), branch `codex/issue-2414-lane-swap-checked-undo`, base `3bf66dd46`

**Claims re-verified at base `3bf66dd46`.** Every claim holds; none is dropped. `swap_bank_lanes` (`crates/effect-contract/src/live.rs` 3401-3522 at base) folds the three exchanges with a non-short-circuiting `taken &=` (3334, 3339) and checks the four results only with `debug_assert!` (3489, 3497, 3509, 3518). `rack`'s two stage `swap_lanes` (`crates/rack/src/lib.rs` 1435-1480, 2480-2600) map any `Err` to `false`; `BankChain::swap_lanes` (5198-5270) maps a `false` after the check to `Defect`, after a `debug_assert!(false, ..)` that made the defect path untestable in debug; the graph's `Runtime::swap_bank_lanes` (`crates/graph/src/runtime.rs` 11491-11590) trades back the rows already traded and returns the refusal, which the packer takes as a refused trade.

**Red on base, both builds.** A one-time probe over the ten spec cases (one bank, lanes 1 and 2, a scratch prefilled with `0xA5` so the spare's prior bytes `A` are recognisable; `X`, `Y` the lanes' defaults, `P1`, `P2` their payloads), against base's `swap_bank_lanes` with the new test bank (`Words::refuse_exchange_call`, `refuse_restore_again`):

| Case | Debug build | Release build |
| --- | --- | --- |
| step-1 exchange 1 | panic "a checked lane's defaults exchange" | `Ok(())`, defaults `(Y, A)`, payloads `(P2, P1)` |
| step-1 exchange 2 | panic, same | `Ok(())`, defaults `(X, Y)`, payloads `(P2, P1)` |
| step-1 exchange 3 | panic, same | `Ok(())`, defaults `(A, X)`, payloads `(P2, P1)` |
| undo exchange 1 after step 2 | panic, same | `Err(Restore)`, defaults `(X, A)`, payloads `(P1, P2)` |
| undo exchange 2 after step 2 | panic, same | `Err(Restore)`, defaults `(Y, X)` |
| undo exchange 3 after step 2 | panic, same | `Err(Restore)`, defaults `(A, Y)` |
| undo exchange 1 after step 3 | panic, same | `Err(Restore)`, defaults `(X, A)`, payloads `(P1, P2)` (revert ran) |
| undo exchange 2 after step 3 | panic, same | `Err(Restore)`, defaults `(Y, X)` |
| undo exchange 3 after step 3 | panic, same | `Err(Restore)`, defaults `(A, Y)` |
| step-3 revert refuses | panic "a restore its own validation passed" | `Err(Restore)`, defaults `(X, Y)`, payloads `(P2, P2)` |

Every row is the spec's prediction. The same probe on the branch: rows 1-3 `Err(Exchange)` with `(X, Y)`, `(P1, P2)`; rows 4-6 `Err(Stranded)` with `(Y, X)`, `(P1, P2)`; rows 7-9 `Err(Stranded)` with `(Y, X)`, `(P2, P2)`; row 10 `Err(Stranded)` with `(X, Y)`, `(P2, P2)`; identical in debug and release.

**Fix.**
- `effect-contract`: `rotate_lane_defaults` checks each exchange and returns a `Rotation` (`Rotated`, `Unwound`, `Stranded`). On a refusal it undoes the exchanges that took, last first; an exchange through the same spare is its own inverse on the lane, which gets back the bytes the bank wrote itself, exactly. Two new `LaneSwapRefusal` variants:
  - `Exchange`: a step-1 exchange refused and was unwound, both lanes as they were.
  - `Stranded`: the defect outcome. An unwind exchange, an undo rotation after a refused step 2 or 3, or the step-3 payload revert refused. The swap returns at once with no further write.

  The four `debug_assert!`s are now checked branches; none remains. Every other variant still means "both lanes unchanged", and the enum doc says so. Cost: at most five exchanges per rotation, eight passes over one lane's defaults on a refusal; no allocation, lock or syscall, and all of it inside the existing `REALTIME_POLICY` region.
- `rack` (the recovery): both stages' `swap_lanes` go through one helper, `swap_stage_bank_lanes`. On `Stranded` the helper resets both lanes with `reset_lane(lane, FullToDefaults)`, each in its own bank. This is the D7 lane recovery that every launch effect already takes for a non-finite lane. The stage then:
  - refreshes the per-lane symmetry cache, with the `RESTORED` term withheld (`EffectBankStage::witness`; `LiveControlEffectBankStage::designed`/`unrestored`);
  - counts the event in a new saturating `lane_swap_stranded()` on each stage;
  - returns `false`, so `BankChain::swap_lanes` swaps back the slots it already swapped and returns `ChainLaneSwapRefusal::Defect`.

  `Defect`'s docs now say it may have reset the two lanes of the refusing slot. The chain's two `debug_assert!(false, "a checked slot swaps its lanes")` are removed: the defect is now a recovered and counted outcome, and its test runs in debug.

  *Why reset, not quarantine or carry re-seed.* A stranded swap cannot say whose defaults each lane holds: the other strip's, or the spare's bytes. A reset makes each lane's whole state a function of its defaults again, so the state agrees with them for the next reset, D7 recovery or restore validation, and the bank's rest claim is true. Quarantine would need a new per-lane mechanism that `EffectBankStage` lacks (its bypass is per bank). A carry re-seed needs a plan rebuild, which render cannot start. The defect stays visible through the counter, and a plan rebuild prepares fresh defaults.
- `graph`: docs only (`BankLaneMoveRefusal`, `Runtime::swap_bank_lanes`). The existing mapping, trading back the rows already traded and refusing the trade, is correct now that the rack recovers the slot: both strips stay where they were and the map is untouched.

**Tests.**
- `effect-contract` `lane_swap_tests::a_refused_exchange_or_undo_is_never_reported_as_unmoved`: the ten cases plus a two-bank step-1 case (the second bank's exchange refuses, and the unwind must reach the first bank). It asserts each outcome and the exact lanes, in debug and release. Test value: a swap that folds its exchanges unchecked (`Ok` with a half-rotated pair), skips or misorders the unwind, unwinds in the wrong bank, or reports a refused undo or revert as `Restore`.
- `rack` `lane_swap::a_stranded_slot_resets_its_two_lanes_and_the_chain_reports_a_defect`: an effect slot in a cross-chain swap and the live slot in an in-chain swap. Each bank's exchange refuses from its second call on. It asserts `Defect`, the stage's count of 1, a reset of exactly the two stranded lanes in their own banks, and the slots before the stranded one swapped back. Test value: a stage that takes a stranded swap for a clean refusal (no reset, no count), resets the peer's lane in the wrong bank, or a chain that does not swap back.
- `StripBank` gains a `Fault`: an armable exchange refusal and per-lane `FullToDefaults` reset counters. Its other behaviour is unchanged, and the existing `every_lane_swap_refusal_leaves_both_lanes_unchanged` is kept: it covers the other refusals.

**Mutation evidence** (each reverted, focused test red, restored):
- `effect-contract`, release:
  - step-1 check back to `debug_assert!`: `Ok(())` vs `Err(Exchange)`;
  - step-2 undo check back to `debug_assert!`: `Err(Restore)` vs `Err(Stranded)`;
  - step-3 undo check back to `debug_assert!`: `Err(Restore)` vs `Err(Stranded)`;
  - revert check back to `debug_assert!`: `Err(Restore)` vs `Err(Stranded)`;
  - unwind skipped: lane 1 left holding `A`;
  - step 1 sent to the first bank: both tests red.
- `rack`, debug:
  - no reset: the reset counts `[0, ..]` vs `[0, 1, ..]`;
  - the peer reset in the calling bank: `[0, 1, 1, ..]`;
  - the effect stage's count dropped, and the live stage's count dropped: `0` vs `1` each;
  - the chain's swap-back skipped: slot 0's defaults not swapped back;
  - `debug_assert!(false)` restored: the debug run panics.

**No bit moved.** The #2392 instrumentation: an FNV-1a digest over the moved plan's host PCM words for every block and both plans' observed words, in `graph-compiler`'s `silent_bank_skip::a_moved_strip_renders_the_bits_of_an_unmoved_one` (gate 3, 13 forced render-time lane swaps across a plan swap, the limiter on the payload path), `--release`. Base `3bf66dd46`: `inserts=false 7d31fea2b6652670`, `inserts=true 8e4dfb8b8dc1d599` (the values #2392 recorded); branch identical. PR evidence only; not committed.

**Gates.** Debug and release runs of `effect-contract`'s lane-swap tests, plus the crates below, one at a time with `-j 4`, each in debug unless marked:
- `cargo test --locked` passed in every crate, with none failing:
  - `effect-contract`: debug, and `--release --lib lane_swap` (2);
  - `rack`;
  - `graph`, `graph-compiler` and `host-core`, for the struct-size change (91 result lines, all ok).
- `cargo clippy --locked -p effect-contract -p rack -p graph --all-targets -- -D warnings`: clean.
- `cargo fmt --all --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p effect-contract -p rack -p graph`: clean.
- `scripts/check-workspace-policy.sh`: ok.
- `realtime_policy`: ok (312 marked regions in 48 files).
- `scripts/check-web-audioworklet.sh --without-metadata-regeneration`, on a rebuilt artifact and named twin with wabt 1.0.34: passed ("web AudioWorklet static/object checks passed"; `miso_engine_web_v1_render` closure=38, allocation-free, atomic_waits=0).

**Follow-ups (not filed).**
- The chain's `debug_assert!(undone, "a swapped slot swaps back")` (rack) and the graph's `debug_assert!(undone.is_ok(), "a traded row trades back")` and `debug_assert!(false, "a checked row trades its lanes")` are the same pattern one level up: an undo whose result is only asserted. A slot that refuses its swap-back leaves that slot traded under a `Defect` that says the others were swapped back.
- The packer retries a strip whose trade hit `Defect` at the next episode, so a persistently broken bank would be reset at every episode. Pinning both strips on `Chain(Defect)` (the #2135 pin) would quarantine them for the plan's life; it is a graph change outside this issue's mapping.
- `lane_swap_stranded` is read per stage only; no host-facing total such as `carry_refusal_count` sums it yet.

**Sol's verdict on attempt 1 (`9794148d4`): FAIL**, with one MAJOR and four MINOR findings. The effect-contract fix itself was correct.
1. MAJOR: nothing tested the rack recovery's symmetry refresh. Deleting all four refresh blocks kept `cargo test -p rack` green, yet a stale witness could mono-collapse a lane whose defaults now differ.
2. MINOR: no effect-contract case refused two exchanges. Changing the unwind's `return Rotation::Stranded` to `continue` was caught only in rack.
3. MINOR: on a bank with a detector link, an unwound exchange snaps an in-flight link glide to rest (`rest_link` inside `exchange_lane_defaults`). Either restore each lane's own payload after the undo, or narrow the "both lanes as they were" doc.
4. MINOR: keep a debug tripwire at the chain for refusals that no stage counted as stranded.
5. Write a successor for the chain and graph swap-backs, the graph's `debug_assert!(false)`, a host-facing total with a #2135 pin, and the trait-doc mismatch from item 3.

### Attempt 2 (Sol's findings folded), same branch, origin/main `fe332ff80` merged

- **Item 1.** `a_stranded_slot_resets_its_two_lanes_and_the_chain_reports_a_defect` now runs four cases: the effect slot and the live slot, each in a cross-chain and an in-chain swap. Every arm of both stages' stranded branch is therefore exercised.
  - Before the swap, it asserts that both lanes hold `ChannelSymmetryWitness::RESTORED`, through `BankChain::lane_symmetry`.
  - After the `Defect`, it asserts that both stranded lanes withhold `RESTORED`.
  - Red on mutation: deleting each of the four refresh blocks alone turns the test red at the `RESTORED` assertion. This holds for the effect stage in the same chain and across chains, and for the live stage in both.
- **Item 2.** Words gains `refuse_exchange_again`. New case: "step-1 exchange 2 and its unwind" (exchanges 2 and 3 refuse). It returns `Err(Stranded)`, the first lane holds the spare's bytes as its defaults, and the payloads are unchanged.
  - Red on mutation: changing the unwind's `return Rotation::Stranded` to `continue` makes this case return `Err(Exchange)` instead of `Err(Stranded)`.
- **Item 3 (restore, which is bounded).** `OwnPayloads::restore` runs after every undo that completes:
  - step 1's unwind;
  - the undo after a refused step 2;
  - the undo after a refused step 3, which replaces the old revert of the first lane alone.

  It restores the first lane's own payload, then the second's. Both payloads validated against their own lanes before the first write, and the cost is at most two extra restores. A restore that refuses returns `Stranded`.
  - The docs of `Exchange`, `Restore` and the swap now say why: an exchange may touch a state word that a payload holds.
  - Words gains `rests_on_exchange`: an exchange that takes sets the lane's right word to `REST`, standing in for `rest_link`. It is on for every bank in `every_lane_swap_refusal_leaves_both_lanes_unchanged` (accepted swaps included) and for the `Exchange` cases of the new test.
  - Red on mutation, each dropping one own-payload restore:
    - after the unwind: the `Exchange` cases leave `REST` in the payload;
    - after step 2: the `Restore` cases leave `REST`, `[11, 12, 32343]`;
    - after step 3: the `Restore` cases leave `REST` and `Stranded` turns into `Restore`.
  - The trait-doc mismatch (`LaneSwap::exchange_lane_defaults`, "It changes no state word", `crates/effect-contract/src/lib.rs` 2526) is item (d) of the successor.
- **Item 4.** `BankStage` gains `lane_swap_stranded()`, with default `0`. It replaces the two inherent accessors, so there is one source. `BankChain::swap_lanes` reads it before each slot's swap and, on a refusal, asserts `debug_assert!(stranded count moved, "a checked slot swaps its lanes, or strands and counts it")` in both arms.
  - A counted strand passes in debug.
  - Dropping the effect stage's count, or the live stage's, trips the assertion (`lib.rs` 5356 and 5321 under the mutation).
- **Item 5.** The successor spec is written, ready to file. This spec names it as #SUCCESSOR. It covers:
  - the checked chain and graph swap-backs, reported as a split strip;
  - the graph's `debug_assert!(false)`;
  - a host-facing `lane_swap_stranded` total, and a #2135 pin on `Chain(Defect)`;
  - the trait-doc mismatch.

  These replace the attempt-1 follow-ups above.
- **Mutation evidence, attempt 2** (each reverted, focused test red, tree restored):
  - `effect-contract`, release:
    - each of the four checks back to its `debug_assert!`;
    - the unwind skipped;
    - the unwind's `return Stranded` changed to `continue`;
    - the unwind in the wrong bank;
    - each of the three own-payload restores dropped.
  - `rack`, debug:
    - no reset;
    - the peer reset in the wrong bank;
    - each stage's count dropped (the tripwire fires);
    - each of the four symmetry refreshes deleted;
    - the chain's swap-back skipped.
- **No bit moved.** `silent_bank_skip` gate-3 digest on the branch: `inserts=false 7d31fea2b6652670`, `inserts=true 8e4dfb8b8dc1d599`, identical to base `3bf66dd46` and to attempt 1. The healthy path is unchanged.
- **Gates.** Debug and release runs of the effect-contract lane-swap tests, plus the crates below, one crate at a time with `-j 4`:
  - `cargo test --locked`: every run below passed, with none failing.
    - `effect-contract`;
    - `effect-contract --release --lib lane_swap`;
    - `rack`;
    - `graph`;
    - `graph-compiler --test silent_bank_skip --release` (14 tests).
  - Clippy `-D warnings` on `effect-contract`, `rack` and `graph`: clean.
  - `cargo doc -D warnings`: clean.
  - `cargo fmt --all --check`: clean.
  - `check-workspace-policy.sh`: ok.
  - `realtime_policy`: ok (312 regions).
  - `check-web-audioworklet.sh --without-metadata-regeneration` on a rebuilt artifact and named twin: passed (`miso_engine_web_v1_render` closure=38, atomic_waits=0).
