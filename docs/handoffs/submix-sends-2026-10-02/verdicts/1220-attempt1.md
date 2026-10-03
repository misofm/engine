# #1220 attempt 1 verdict: Ramp live send coefficients on the render plane

**Verdict: PASS**, with three MINORs to land before close and no BLOCKER or MAJOR.

- Implementation: commit `d405abb37`, on parent `0268a1c74`, branch `codex/batch-submix-k3`.
- Reviewed on an exported copy (`git archive`), never in the worktree.
- Host: x86-64-v3 AVX2, AMD EPYC 7313P. Node v22.23.2.
- Probes: `1220-attempt1-verifier-scratch.rs` in this directory. They reuse `live_routes.rs`'s helpers.

The render core holds under adversarial probing:

- The queue hands a full record back, and nothing drops after a push.
- A record carries gain, mute and matrix as one value, so it applies atomically.
- Activity is decided once per block, after the drain and before the destination reads it.
- The block in which a mute ramp ends is mixed whole.
- A delayed route never goes inactive: its line holds the fade and then zeros.
- A retarget starts from the exact current coefficients.
- A settled route equals a fresh plan.
- A live route never folds.
- Render allocates nothing.

The findings are about accounting claims, one test-value overclaim and evidence wording.

## Gates re-run (head `d405abb37`)

| Gate | Result |
|---|---|
| test-debug-a (DESIGN 7, `--no-fail-fast`) | rc 0. **1205 passed, 0 failed, 9 ignored**, 107 binaries. `live_routes` 10/10 and `route_mute` 13/13. |
| `cargo test --locked --release -p audit -p bench -p console-workload` | rc 0. 110 passed, 2 ignored. |
| `graph_fixture -- --check` | rc 0 |
| `check-graph-determinism.sh` | PASS (100/100) at head and at parent. `fresh-process-determinism.json` is `cmp`-identical to the parent's. |
| Realtime, graph, builtins and workspace policy (check and test) | All ok. Realtime reports 54 regions in 15 files. The drain and the activity write are inside regions (`runtime.rs:321-933`, `:3272-3629`). |
| `cargo fmt --all -- --check` | rc 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | rc 0 |
| `build-web-audioworklet.sh --named-twin` | Shipped module `c21d647a…`, 2,740,212 B. Parent `1900ef1b…`, 2,726,858 B. **+13,354 B (+0.49%)**, as recorded. |
| `check-web-audioworklet.sh <A> <B>/…named.wasm` | rc 0. Render closure `closure=8 traps=5`, and the only trap owner is `PreparedRenderPlan::render_inner`. That is identical to the parent run, so **no extra trap owner**. |
| Kernel shape | Parent: **12** (`f32x4_arith=9350`). It is green at min 12 and red at min 13. Head: **13** (`f32x4_arith=9395`). It is green at 13 and red at 14. |
| `check-browser-expected-resources.py --artifacts` | rc 0. The digests agree, and the self-test shows 32 red mutations. |
| `check-scalar-oracle-absent.py --wasm` | rc 0 |
| `test-web-audioworklet.sh` | rc 0 |
| V8 spill gate (toolchain, self-test, module) | ok |
| `run-aarch64-tests.sh debug` | **Not run here**: no arm64 host, no qemu and no aarch64-linux target. It runs at batch push (CI `aarch64-debug`). Width independence rests on #1219's `Backend::Simd4` lane tests and on the simd128 instantiation's shape. No 4-lane run exercises the `LiveRoute` glue before then. |

### The kernel floor, 11 to 13: legitimate

The 13th qualifying function is `lane::kernels::route_mix_ramp_block<f32x4>`, at 22 vector and 0 scalar instructions. The other 12 are the parent's set, unchanged. The parent's floor was 11 with one kernel of slack. D9 prescribes "base plus one", and 13 spends the slack, so losing any kernel is now red. The raise is legitimate and does not weaken the floor.

## My probes (all green at head)

1. **`verifier_tap_sweep_round_trip`** covers 28 configurations:
   - **taps:** all seven (`input`, `post_input`, `insert_send`, `insert_return`, `pre_fader`, `post_fader`, `post_pan`);
   - **source:** track to bus, and **bus to bus** (`c -> y`, then `y -> b` from `y`'s tap);
   - **delay:** delayed (486, through a silent limiter track) and undelayed.

   Every configuration has banked strips (`bank_shape()[0] > 0`). Each delayed case has one scatter redirect into the live route's buffer, so the route mixes in place.

   The live sequence:
   - a mute (480);
   - 10 idle blocks;
   - an unmute (480);
   - a mid-ramp retarget to R1 (300);
   - a step mute and a step unmute in one drain.

   Every output sample equals an independent scalar model of D3/D4. A delayed route mixes 30/30 blocks, and an undelayed one 20/30. The settled tail equals a fresh plan. Across every tap shape there is no stale audio and no truncated fade.
2. **`verifier_per_block_steps_match_fresh_plans`.** Eight banked tracks feed one bus, with step records every block. They include toggling `t0`, the store owner, and one block where all eight are muted. Each block is bit-identical to a fresh plan prepared with that block's values.
3. **`verifier_retarget_and_multi_record_drains`.** It runs 24 blocks of mid-ramp retargets and multi-record drains (a step then a ramp, a ramp then a mute, three records in one drain). Lengths are 1, 37, 77, 127, 128, 129, 200, 333, 1000, 4000 and 4800. Every sample equals the independent model.
4. **`verifier_follows_mute_interplay`.** A fully follow-muted send idles inactive, exactly as prepared. A gain edit, with target `route_coefficients(.., [true, true])` and `mute = true`, keeps it inactive. A half-follow-muted send (`[true, false]`) settles on a fresh plan's bits.
5. **`verifier_attached_state_charge`.** This is MINOR-1's evidence.
6. **`verifier_gate3_audio_only`.** It is gate 3 with the mix-count assertion removed. The audio comparison alone is red under 1220-4: `sample 512: 0.0 != 0.3514467`, the fade cut in the line.

## Mutations (mine; each applied alone, then reverted)

| # | Mutation | Result |
|---|---|---|
| M1 | The drain ramps from `self.ramp.target` instead of `coefficients_at(position)` | **RED.** `the_drain_is_bounded_and_lossless` fails, plus probes 1 and 3. |
| M2 | Activity is decided before the drain | **RED.** Gate 5 and `a_bus_whose_every_send…` fail, plus probes 1-3. |
| M3 | The table bit is written only when inactive, so it is stale after an unmute | **RED.** Gate 5 fails, plus probes 1-3. |
| M4 | `position` is not reset on a record | **RED.** Gate 3 fails, plus probes 1 and 3. |
| M5 | D1 also makes routes into the output live | **RED.** Gates 1, 7 and 8 and `route_controls_attach_once` fail. |
| M6 | `live_route_owner_bytes()` returns 0 | **RED.** Gate 8 fails, but only in the muted case and by 16 B: "charge 1300 covers 1316". See NIT-2. |
| M9 | The table bit is always written `true`, so an inactive route's buffer is read | **RED.** Gates 1, 2 and 4 and the NIT-1 test fail, plus probes 1-3. |
| M10 | The drain is unbounded (`for _ in 0..usize::MAX`, stopping only when empty) | **GREEN on every test.** See MINOR-2. |
| M14a | `BorrowedPlanningMetadata::has_route_control` returns `false` | **RED.** Gate 7 fails, plus probe 2. |
| M14b | `RuntimeParts::has_route_control` returns `false` | GREEN, and equivalent. That impl is reached only by the `#[cfg(test)]` `route_folds_over_program` (`runtime.rs:7609`), which builds `RuntimeParts` with no lanes. Production fold planning uses the borrowed impl (`preflight_sequential`). |

Re-confirmed from the record:

- **1220-3** is RED on gate 2.
- **1220-4** is RED on gates 1 and 3 (also audio-only, probe 6).
- **1220-7** is RED: SIGABRT from the armed render audit.
- **1220-10** is RED on the NIT-1 test.
- **1220-11 (MC)** is RED on `a_middle_inactive_route_keeps_every_active_contribution`, and only there among the `route_mute` tests.

## Findings

### MINOR-1: the D8 charge does not bound the attached state, as deviation 1 claims

The record says the charge covers "the plan's `GraphRouteControlBinding` entry, which is held from attach until bind… So the charge bounds both the attached and the bound state." The docs say `route_control_resources` "states exactly what the attach and the bind add."

- Each binding holds `node: node.clone()`, a `GraphNodeId::Route` whose `StableGraphId(String)` is a second heap copy of the route ID. It can be up to 127 B. Only the producer's copy is charged (`route_id_bytes`).
- Probe 5 used a muted plan (no activity charge) with two 127-byte route IDs. After attach and before bind it retains **1804 B**, against a charge of **1694 B**. With short IDs it retains 1304 against 1444.
- The bound state is covered: gate 8 holds, because the binding is freed at bind. The pre-bind state is not covered once route IDs are longer than `size_of::<LiveRoute>()` (72 B here).
- "Exactly" is also inaccurate:
  - `activity_bytes` is the `route_activity_bound_bytes` upper bound;
  - the binding entries are transient after bind;
  - the measured gap is 200 B and 128 B.

**Fix (one line plus wording).** Either:

- charge the binding's node-ID bytes as well, by counting `route_id_bytes` twice or adding a `binding_id_bytes` field, and keep the claim; or
- retract "bounds the attached state" in the record and say "an upper bound on what the bound plan retains" in the rustdoc and in `docs/BUILTINS_AND_METERING_V1.md`.

I recommend the first. #1221 admits this number while it holds the attached artifact.

### MINOR-2: gate 5's test value claims "red if the drain is unbounded"; mutation M10 is green

`the_drain_is_bounded_and_lossless`'s doc and the spec's gate 5 sentence both claim it. Single-threaded, an unbounded drain applies exactly what a bounded one does. The only concurrent test, gate 6, asserts at least one applied record per block, never a maximum. Row 1220-6a is the opposite defect: it applies too few.

**Fix:** drop "unbounded" from the test-value sentence and from the record. The bound is structural: the loop runs over `available_at_entry()`, which is at most the capacity. Alternatively, add to gate 6 an assertion that a block's applied count per route never exceeds `DEPTH`. That catch is weak and nondeterministic, so rewording is the honest fix.

### MINOR-3: an evidence-wording error, and a pre-existing failure it hides

The record says "clippy without features on `graph` and `builtins-compiler` is clean." The following fails:

```
cargo clippy --locked -p graph -p builtins-compiler --all-targets -- -D warnings
```

- It reports two dead-code errors in `builtins-compiler`'s lib test target: `initial_matrix_state` (`lib.rs:911`), and `BoundaryVariant::{Nonadjacent, NonadjacentOutputConflict}` (`lib.rs:5876`).
- The failure is **identical at parent `0268a1c74`**, so #1220 did not cause it.
- The spec's gate 10 command, workspace `--all-features`, passes.

**Fix:**

- state the exact command that was clean (presumably without `--all-targets`);
- root files the pre-existing no-feature dead code as a follow-up.

### NIT-1: no roster row for the new kernel

`route_mix_ramp_block<f32x4>` is in `--kernel-min`'s count but not in `KERNEL_ROSTER`. The generic "vector > scalar" rule would not catch the regression `kernels.rs` documents: an inlined settled tail is 18 scalar against 22 vector. A roster row with ceiling 0.1 in `check-web-audioworklet-callgraph.py` would. That file is outside this slice's paths, so this goes to root's follow-up ledger.

### NIT-2: gate 8 is weaker than it reads

- Its equality checks reuse the implementation's own constants (`LIVE_ROUTE_OWNER_BYTES`, `size_of`), so they are partly tautological.
- The load-bearing check is "covers retained", and it catches an omitted owner box only in the muted case, by a 16 B margin (M6).
- Making `LIVE_ROUTE_OWNER_BYTES` public is acceptable API. It is a resource-model constant, like the public `route_activity_bound_bytes`, and nothing else exposes the owner's size. `#[doc(hidden)]` is optional.

### NIT-3: where the +0.49% browser growth comes from

The +13,354 B is mostly bind-time monomorphizations of `BTreeMap<GraphNodeId, Box<RouteControlLane>>`:

- sort, remove, balance and drop glue, about 5k wasm ops;
- plus growth in `GraphExecutor::new` and `build_sequential`.

`execute_op` grows by 305 ops. The browser never attaches in this slice. D6's "keyed exactly as effect controls" mandates this shape, so it is acceptable. A sorted `Vec` would avoid the new instantiations if module size becomes a budget question.

## INFO for #1221's brief (no change in #1220)

- **The acked-batch question across a plan's life.**
  - `try_push` returns `Ok` on a lane whose consumer is gone (bind failed, or the plan was retired) until the ring fills.
  - Records still queued at a structural plan replacement are discarded with the old plan.
  - host-core must retire producers with their plan. It must also define what a swap does with records it has acked but the plan has not drained: re-derive from committed state, or carry them over.
- **`mute` must be `gate.silences()`, including follow-mute.** If a producer sends `mute = false` with an all-zero follow-silenced target, the route stays active, mixing zeros. That costs CPU, and its signed-zero/NaN bits differ from a fresh plan's inactive route.
- Records to different routes in one batch can apply one block apart: there is no cross-queue fence.
- `RouteControlRecord::new` does not refuse a non-finite target. That domain check belongs to the producer (`route_coefficients`).
- Inside the block in which a mute ramp ends, frames after the snap can carry `-0.0` at a bus's `input` tap where a fresh plan stores `+0.0`. This applies only when every other contributor is silent. Mixing that block whole is by design (VERIFY-1 MAJOR-1), and the gates compare from the snap block's end. Recorded so nobody reads it as a defect later.

## Deviations

All six are acceptable. Only deviation 1's claim is wrong, and MINOR-1 covers it.

- **2:** `BoundRoute` and `QueueCapacity` are sound defensive refusals. A user binding cannot target a route: bind requires the supplied bindings to equal the required set.
- **3:** setting the activity index and `delayed` after the table walk is sound. `bind_live_routes` `expect`s at bind, never at render.
- **4:** using `input` taps for gate 2 is acceptable.
- **5:** the gate 6 design is acceptable.
- **6:** the gate 4 trial design is acceptable.

## Test value (one sentence per new test)

- **`idle_attached_lanes_change_nothing`:** red if an idle lane binds anything but its prepared, gated coefficients and gate silence (1220-1, 1220-2, M5).
- **`the_block_in_which_a_mute_ramp_ends_is_mixed_whole`:** red if activity is decided after the mix, which drops frames 0-94 of block 3 (1220-3, M9).
- **`a_bus_whose_every_send_goes_inactive_is_refilled_with_positive_zero`:** red if the arena-form sum skips an all-inactive destination, leaving a stale sum (1220-10, M2).
- **`a_delayed_send_round_trips_through_mute_without_a_cut_or_stale_audio`:** red if a delayed muted live route goes inactive, cutting the fade in its line (1220-4, also by audio alone), or if a record does not reset the ramp position (M4).
- **`a_settled_live_edit_equals_a_fresh_plan`:** red if a record reaches the wrong route (1220-5a), or a mute keeps the old target (1220-5b).
- **`the_drain_is_bounded_and_lossless`:**
  - red if a drain applies too few records (1220-6a), skips while inactive (1220-6b), ramps a retarget from the wrong start (M1), leaves a stale activity bit after an unmute (M3), or decides activity before draining (M2);
  - **not** red if the drain is unbounded (MINOR-2).
- **`live_routes_render_without_allocating`:** red if the drain, a record's application or the activity write allocates on the render thread (1220-7).
- **`no_live_route_folds`:** red if fold planning folds a live route (1220-8, M14a).
- **`route_control_resources_cover_the_allocation`:** red if a queue, a route ID, the activity table, or (muted case only) the owner box leaves the charge (1220-9a, 1220-9b, M6).
- **`route_controls_attach_once`:** red if a second attach replaces the first's lanes (1220-12).
- **`route_mute.rs::a_middle_inactive_route_keeps_every_active_contribution`:** red if `route_segments` forgets that an opening run stored (MC), and red in this test alone.

## Required before close

1. MINOR-1: charge the binding's node-ID bytes (or retract the attached-state claim), and fix the "exactly" wording.
2. MINOR-2: reword gate 5's test value.
3. MINOR-3: correct the clippy sentence in the record. Root takes the pre-existing dead-code item as a follow-up.

NIT-1 goes to root's follow-up ledger. The INFO items go into #1221's brief.
