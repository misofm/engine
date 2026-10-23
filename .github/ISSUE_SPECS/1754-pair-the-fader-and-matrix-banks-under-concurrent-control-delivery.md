# Pair the fader and matrix banks under concurrent control delivery

Found by the #1750 implementer (2026-10-22), checked by root.

## Problem

The fused fader and matrix stage (#466; its settled pass #881 and its ramping pass #890) forms only when both banks use `BuiltinControlDelivery::BetweenRenderCalls` (`make_fader_matrix`, `crates/builtins-compiler/src/lib.rs` near `:5505-5525`). No product host prepares that delivery:

- the C ABI prepares through `host_core::prepare_host_runtime_with_live_lanes(_successor)` (`crates/capi/src/runtime/mod.rs` near `:103-106`), which passes `between_render_calls = false` (`crates/host-core/src/prepare.rs` near `:1666`);
- the browser's `prepare_web_host` calls no `_between_render_calls` entry since #1400b (13fd9b491; `hosts/host-web/src/lib.rs` near `:5545-5550`).

So every product plan keeps the fader and the matrix as two stages, and #881's and #890's fused passes run only in tests and in the bench's metered row. The metered row's doc (`tools/bench/src/console.rs` near `:103-118`) and its prepare comment (`tools/console-workload/src/lib.rs` near `:1376-1383`) still say the browser fuses; that is stale.

The rule came from #444: under the September control queues, a fader or matrix record could arrive between the two stages' drains (`Consumer::try_pop` refreshed the producer cursor while draining), so one traversal could apply a record to one stage and not the other. Decision 15 replaced that delivery: live values come from one committed snapshot per block (`apply_latest(snapshot, ...)`), and every live value of one committed transaction takes effect in the same block (D15-2).

## Smallest closable slice

1. **Diagnose (record in this spec first).** Under today's concurrent delivery, can a fused traversal observe a different set of fader, mute or matrix values than the two separate stages would in the same block? Trace both stages' control reads for one block on both hosts: what they read, when, and from which snapshot or queue. Answer with file:line evidence. Also answer the acked-batch question: can an ack ever precede a drop on the fused path?
2. **If the answer is "no":** let `make_fader_matrix` pair `Concurrent` banks, with the control reads of both banks taken at the same block boundary the separate stages use today. Move the bench's metered row to the product preparation and correct the stale comments.
3. **If the answer is "yes":** stop and record the hazard. Root then decides between a delivery change and removing the BetweenRenderCalls-only fused stage.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (pairing and its tests), `crates/host-core/src/prepare.rs` (only if a prepare entry must change)
- `tools/console-workload/src/lib.rs`, `tools/bench/src/console.rs` (the metered row's preparation and docs)
- this spec

## Objective gates

1. The diagnosis, with evidence, recorded before any code change.
2. A builtins-compiler test: on a Concurrent plan, the paired stage renders PCM and state bit-identical to the separate stages, with live fader, mute and matrix values committed between blocks, across a ramp start, middle and end.
3. host-core and host-web control parity tests pass unchanged (`hosts/host-web/tests/control_parity.rs`); render allocates nothing; `tools/realtime-policy` passes.
4. One descriptive run of the fader-ride row (#1750) before and after.

## Dependencies

#1750 (the baseline row on the product preparation).

## Evidence and decision record

### Step 1 diagnosis (attempt 1, 2026-10-23, at e9c1dc40c)

**Q1. Can a fused traversal read a different set of fader, mute or matrix values than the two separate stages in the same block, on either host? No.**

- One snapshot per block, per plan, on both hosts. Both hosts render through `RealtimePlanOwner::render` (`crates/host-core/src/lib.rs:64-65`) to `render_active` (`crates/engine/src/realtime/plan_exchange.rs:879-883`) to `PreparedRenderPlan::render_inner`, which takes `S = self.gate.snapshot()` once (`crates/engine/src/realtime/plan.rs:1328`) and hands it to the executor (`:1353`). The executor keeps it and takes it once per block (`crates/graph/src/lib.rs:5654`, `:5714`).
- The fused stage forms only inside one bank chain, from two adjacent slots with the same tracks (`crates/graph/src/runtime.rs:7872-7905`). That chain drains every slot with the same `S` before any slot processes (`crates/rack/src/lib.rs:3055-3065`). Split: fader slot `begin_block(S)`, then matrix slot `begin_block(S)`, then both process. Fused: `begin_block(S)` runs the fader drain, then the matrix drain (`crates/builtins-compiler/src/lib.rs:5543-5547`, `:5641-5648`), then it processes. The order and the snapshot are the same.
- A read under `S` is a function of the cell's publications `<= S` and of the reader's own state. It is not a function of when the read runs: "every read of that block applies, per cell, the newest value whose publication is `<= S`. It never applies a value newer than `S`" (`crates/engine/src/realtime/latest_cell.rs:7-11`). Control can publish only numbers `> S` after render took `S`. So a write between the fader drain and the matrix drain is invisible to both stages, fused or split. The readers are `FaderReader::apply_latest` (`crates/builtins-compiler/src/lib.rs:598`) and `MatrixReader::apply_latest` (`:667`), called by `drain_fader_controls` (`:5395`) and `drain_matrix_controls` (`:5478`). The fused stage calls the same functions.
- The #444 hazard needed a queue whose read moved with time (`try_pop` refreshed the producer cursor). Live values no longer use a queue (D15-2). Delivery mode changes no reader, no drain and no kernel: `control_delivery` is read only by the pairing (`:5520-5521`), the test-only scalar oracle factories (`:9388`, `:9393`, `:9722`, `:9761`) and the scalar resource estimate (`:7166`, `:7175`).
- Ramp state and timed operations: the fused passes read and write the same `fader.bank` and `matrix.bank` state that the split stages own. Stored fader automation is plan data, not delivered data. The fused stage builds the same `FaderEventDriver` from it (`:5676-5683`), and a block with a timed operation falls back to the split order, `bank.process_with(driver)` then `matrix.bank.process` (`:5728-5741`). Block time goes to the fader half only, as in the split plan (`:5567-5569`). None of this depends on delivery. #881 and #890 prove the fused passes bit-identical to the split order.

**Q2. The acked-batch question: can an ack precede a drop on the fused path? No.** A live-value submit writes cells (infallible), commits, then acks (#1053 D6). A cell write cannot fail and never drops: a value that a later one replaces before render reads it is counted in `live_values_superseded` (D15-2). A drain consumes a dirty bit only when it reads the cell (`DirtyReader::pending`, `crates/engine/src/realtime/latest_cell.rs:673-675`). One path differs and it is not a drop. If a drain refuses a value (unreachable in product, because control refuses every value the setter refuses), the split chain stops at the first refusing slot (`?` at `crates/rack/src/lib.rs:3063`), and the matrix value stays pending for a later block. The fused drain runs both halves and then returns the first refusal (`crates/builtins-compiler/src/lib.rs:5543-5547`). A failed block publishes no watermark (`crates/engine/src/realtime/plan_exchange.rs:876-877`).

**Q3. A hazard that step 2 would add to product: a plan swap restarts strips when an edit changes a bank between fused and split.** This is not a read-set hazard. It concerns the carry of state across a swap.

- Whether a bank fuses depends on the graph. `chains_into` keeps the fader and matrix banks in one chain only if no lane's fader output has a second reader and no observer is on the post-fader node (`crates/graph/src/runtime.rs:10912`, `:10916-10923`). It is all lanes or none. So a post-fader send or a post-fader meter on one strip splits the whole bank. Removing it fuses the bank again.
- A swap carries a strip stage only when both plans hold it in a bank of the same kind: `own.stage == before.stage` (`crates/host-core/src/prepare.rs:6263`), where a split bank reports `Fader`/`Matrix` and a fused bank reports `FaderMatrix` (`crates/builtins-compiler/src/lib.rs:4917-4919`, `:5149-5151`, `:5571-5576`). The fused stage adopts only from a fused predecessor (`:5602-5625`), and the split fader stage adopts only from a split one (`:4938-4952`). A strip that does not carry starts the stage at rest and is listed in `restarted` (`crates/host-core/src/prepare.rs:6289-6291`). No product code reads that list.
- Also, a fused bank carries both owners or neither (`:6265-6274`). In a split bank, the fader and the matrix carry separately.
- Today every product plan is split, so a kind change never occurs. With step 2, adding or removing a post-fader send or meter on one strip would restart the fader, mute and matrix ramps of every strip in that bank (up to 8 on AVX2). A ramp in flight at the swap (a fader ride, a mute or a pan move) would jump, also on strips that the edit did not touch. Decision 15 requires seamless swaps. Banking must not move a rendered bit.

**Verdict for root.** Q1: no. Q2: no. Step 2 as the spec gives it is not safe until the strip carry accepts a fused/split kind change. Root decides between these options: (a) extend #1277 D9 so that a carry moves state between fused and split banks (builtins-compiler `adopt_strip_lane` for both kinds, plus `resolve_strip_carry` in `crates/host-core/src/prepare.rs`; that function is not a prepare entry, so it is outside this spec's paths), (b) a different pairing rule, or (c) remove the fused stage. A second out-of-scope edit is necessary for any version of step 2: `crates/host-core/tests/live_lanes.rs:218-233` asserts "Concurrent delivery forms no fused fader-matrix bank" (#1053 D5), and step 2 makes that assertion false. No code has been changed.

**Q3 measured (scratch worktree at e9c1dc40c, removed after the run, not committed).** The patch removed the two delivery conditions in `make_fader_matrix`. The test prepared session A, `plain_fixture` from `live_lanes.rs` with 9 tracks and a submix, through `prepare_host_runtime_with_live_lanes(FADER_AND_MATRIX)`. Session B was A plus one post-fader send, from eq0 into the submix, prepared as A's successor through `..._with_live_lanes_successor`. On AVX2 the bank width is 8.
- Main: no bank fuses in A or in B. `restarted_strips()` is empty for A to B and for B to A.
- Patched: all strips fuse in A. In B, eq0's 8-lane bank splits. `restarted_strips()` is `[eq0..eq7]` in both directions, and an A to A swap restarts nothing.
- A live fader move to -24 dB with a 4096-sample ramp ran on eq3, which the edit does not touch. At the swap block, main continues the ramp smoothly (last sample before the swap -0.497, first after -0.560). Patched, the strip jumps to its target in one sample (-0.497, then -0.043).

### Step 2a: measure only (root ruling, 2026-10-23)

Root set the decision rule before any timing. Implement (a) only if, in one arm, the fused plan is at least 2 % faster at p50 in both measured rounds, and product plans do fuse. Otherwise stop. Do not implement (c) without the owner.

**Do product plans fuse with the patch? Yes, unless a strip in the bank has a post-fader send.**
- The fader-ride plan (`build_with_product_lanes(SixtyFourTrackConsole)`: 64 tracks, no submixes, only `post_pan` routes) forms 8 fused banks with 64 members on AVX2. The scratch witness gives `factory_calls=8`, `factory_members=64`. In 20 riding blocks, all 160 bank passes are ramp-fused (`ramp_fused=160`, `fallback=0`).
- The only product tap that splits a bank is a session route at `SendTap::PostFader`. This is because `chains_into` refuses a second reader of the fader output, or an observer on the post-fader node (`crates/graph/src/runtime.rs:10912`, `:10916-10923`). Product meters do not split a bank. They are `PostMatrix` on every plan: the host-core default `meter_tap` (`crates/host-core/src/prepare.rs:412`), and the browser at `hosts/host-web/src/lib.rs:5638` and `:6884`. The #881 metered row fused with them. Spectrum captures observe `PostInputBuiltins` and `PostMatrix` (`crates/host-core/src/spectrum.rs:220`, `:224`). Effect observation observes effects. Pre-fader and post-pan sends read other nodes.
- So a mix with post-fader aux sends fuses nothing in the banks that hold those strips. `fixtures/session/v1/console-sixty-four-track-sends.json` has a post-fader send on every track.

**Timing.** I ran `scripts/operator/run-console-benchmark.sh` once per arm, with one warmup and two measured rounds. Main e9c1dc40c ran as step `opt3-1754-main`. The scratch commit 59f5ec8ab (main plus the pairing patch only, never on this branch) ran as `opt3-1754-patch`. The release build ran under the shared lock and the timing under the exclusive lock, on cpu 31. `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` was set, because the load average at lock was 5.0 and 4.1 (`measurement_control: uncontrolled`). The records are preserved at `/tmp/claude-1002/-home-bl-misofm-engine/258223fb-6f51-4ebb-b550-04d18c009589/scratchpad/opt/records-1754/`. `console_fader_ride` p50, ns per block, rounds 1 / 2:

| arm | main | patch | change |
|---|---|---|---|
| quiet | 111442 / 111212 | 111472 / 110701 | +0.03 % / -0.46 % |
| riding | 117233 / 116582 | 115109 / 115170 | -1.81 % / -1.21 % |
| paired riding - quiet | 5682 / 5359 | 3636 / 4409 | |

Bit identity: every output digest of all 64 records is equal between the two runs. The fader-ride digests are quiet `21d04f12...` and riding `11b0f9b5...`.

**Decision by the rule: stop.** Neither arm is 2 % faster in both rounds; riding is 1.81 % and 1.21 % faster. So (a) is not implemented, and (c) goes to the owner if anyone proposes it. The step-2 patch is not on this branch. Steps 2 and 3 of the slice, the row move and the stale comments, remain open for root.
