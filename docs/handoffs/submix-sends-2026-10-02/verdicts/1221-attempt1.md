# #1221 attempt 1 verdict: Produce live send records from host-core

**Verdict: PASS.** There is no BLOCKER and no MAJOR. Two MINORs are to be fixed before close or routed as stated. The NITs are optional.

- Implementation: commit `1c5ce5d02` on parent `d405abb37`, branch `codex/batch-submix-k3`.
- I reviewed an exported copy (`git archive`) and never touched the worktree.
- Host: x86-64-v3 AVX2, AMD EPYC 7313P.
- Probes: `1221-attempt1-verifier-scratch.rs` in this directory. They append to `live_routes.rs` and reuse its helpers.

What holds up under probing:

- **Validate before push.** `record` is pure. It refuses `Domain` first, from `route_coefficients`, which is the compiler's own function, and `Length` second.
- **No partial set.** `set` is `record` then `push`.
- **Room before push.** `push` decides `Full` from `free()` before `try_push`.
- **Single-producer safety.** The SPSC producer is unique and `!Sync`. `push` takes `&mut self`, and only the consumer moves concurrently, which can only add room. So `free() > 0` means `try_push` cannot fail. The pre-check is redundant (my V3 is an equivalent mutant) but harmless, and the spec asks for it.
- **Ack and drop.** No `Ok` is returned without a record queued, and `RouteControlRecord` is `Copy`, so a refused record is never lost to the caller. In this slice no host acks a route record: host-web drops `handles.route_controls`, and the C ABI requests no depth. So no ack can precede a drop here.
- **The mute rule.** Record `mute` is `RouteGate { mute, follow_zeroed }.silences()`, which is `mute || [true, true]`. It is the predicate the prepared route binds, and `route_coefficients` zeroes the per-lane columns.
  - When one source lane is muted, the route is *not* silenced: the other lane's column still mixes. `mute = false` with one zeroed column is exactly what a fresh plan prepares with `follows_mute` and that lane mute.
- **Exhaustive probe.** It covers 4 lane combinations × route mute × open or prepared-follow-silenced initial state × a silent or live co-contributor × length {0, 300}, 64 cases in all. Each case is bit-identical to a fresh plan from the snap block's end. The live route mixes in exactly the blocks a fresh plan's route does, and 0 when silenced.

## Gates re-run (head `1c5ce5d02`)

| Gate | Result |
|---|---|
| `live_routes` (focused) | 7/7 |
| test-debug-a (DESIGN 7, `--no-fail-fast`) | rc 0. **1212 passed, 0 failed, 9 ignored** over 108 binaries, as recorded. |
| `check-host-core-policy.sh` / `test-host-core-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (54 regions in 15 files) / ok |
| `check-workspace-policy.sh` | ok |
| `cargo fmt --all -- --check` | rc 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | rc 0 |
| `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | rc 0 |
| `./target/release/audit capi` | rc 0. 0 violations, `pcm_digest` `ff6cdcb96cdcdad5`, as recorded. Nothing moved. |
| `build-web-audioworklet.sh --named-twin` | Shipped `551f943b…`, 2,755,262 B, as recorded |
| `check-web-audioworklet.sh <A> <named>` | rc 0. Kernels 13 with `f32x4_arith=9395`. Render `closure=8 traps=5`, sole trap owner `PreparedRenderPlan::render_inner` (same as #1220). |
| `check-browser-expected-resources.py --artifacts` | rc 0. Digests and exact rows agree; the self-test shows 32 red mutations. |
| `check-scalar-oracle-absent.py --wasm <named>` | rc 0 |
| `test-web-audioworklet.sh` | rc 0 |
| `check-sdk-headless.sh <A>` (extra) | 352/352 pass |
| `run-aarch64-tests.sh debug` | **Not run.** There is no arm64 host, so it runs at batch push (CI `aarch64-debug`). |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` (extra; CI lint job "Documentation") | **rc 101, but not from #1221.** See BATCH-1. With `--exclude graph-compiler` it is rc 0, so #1221's new intra-doc links are clean. |

## Mutations

The implementer's rows that I re-ran:

- **1221-M5** is RED on gate 3, line 624.
- **1221-M10** is RED on gates 5 and 6.

My own mutations, each applied alone and then reverted:

| # | Mutation | Result |
|---|---|---|
| V1 | `record`'s mute ignores follow (`follow_zeroed: [false; 2]` in the gate) | **RED.** `the_live_target_is_the_prepared_constant` fails at its record-level mute assertion (`:446`). |
| V1b | V1 with that record-level assertion removed | **GREEN.** Gate 1's rendered half alone does not see it (NIT-1). My probe's activity check is red. |
| V2 | The named-allocation join (`.max(route_control_resources.largest_allocation_bytes)`) is dropped | **GREEN on all host-core tests.** The join is dominated (NIT-2). |
| V3 | `push` without the `free()` pre-check | GREEN. It is an equivalent mutant: SPSC `try_push` already refuses without pushing. |
| V7 | `Length` decided before `Domain` | **RED.** Gate 3 fails at `:646` (NaN with a too-long length must be `Domain`). |
| V9 | `source_lane_muted` lanes swapped before `route_coefficients` | **RED.** Gate 1 fails at `:441`. With the record-level asserts removed it is still RED on the rendered half: trial 0, `[true, false]`, `-0.0338 != -0.0297`. The rendered comparison defends the per-lane columns. |

## Findings

### MINOR-1: the browser's own exact-retained budget does not count the route lanes this slice makes it attach

The defect:

- host-web boots with `control_queue_depth = Some(..)` whenever the command queue is nonzero (the producer's mixer uses 64). As of this commit, every route into a submix in a browser session gets a lane.
- The plan retains each lane's queue, owner box and activity table. host-web drops the producers.
- host-core charges those bytes against its own caps (D4, correct).
- The browser's `exact_retained_bytes` (`hosts/host-web/src/lib.rs:5720`) does not charge them. Its `host.budget.retained_exact` check (`:2012`) sums `bridge_retained + graph_session_plus_plan + source_total`.
  - `graph_session_plus_plan_bytes` is the compile-time estimate. The attach adds to it, and the attach's bytes are deliberately excluded from it.
  - So the browser's "exact" report undercounts. On gate 6's session, two routes come to 1,659 B at depth 8 and 4,347 B at depth 64.
- The effect channels' transferred payload *is* added to `bridge_retained` (`:6089-6104`), so this is an omission, not a convention.

Who owns the fix:

- It is outside #1221's authorized paths.
- #1222's brief puts `route_controls` into host-web but has no resource row for them (its spec mentions resources only for `check-browser-expected-resources`).

**Fix:** root amends #1222's brief, or authorizes `hosts/host-web/src/lib.rs` here, to:

- add `engine.route_control_resources.total_bytes` to `bridge_metadata_bytes` and `bridge_retained_bytes`;
- fold `largest_allocation_bytes` into `largest_bridge_allocation_bytes` and `largest_named_allocation_bytes`;
- add a host-web test on a session with a submix send, where `exact_retained` grows by exactly `route_control_resources.total_bytes` between depth 0 and depth > 0;
- avoid double counting once #1222 retains the producer `Vec` (`producer_table_bytes` is already inside the total).

It does not block #1221's close if it is routed to #1222 explicitly.

### MINOR-2: the record's gate 6 numbers are the depth-1 numbers from the M10 run, not depth 8

The record says "two sends, depth 8: 1,323 bytes … 608 queue". At head, `live_send_lanes_are_charged_against_the_graph_cap`'s session gives:

| Depth | Queue | Owner | Producer table | ID | Activity | Total | Largest allocation |
|---|---|---|---|---|---|---|---|
| 8 | 944 | 352 | 144 | 4 | 215 | **1,659** | 256 |
| 4 | 752 | 352 | 144 | 4 | 215 | 1,467 | 256 |

The queue grows by 48 B a depth step, so depth 1 gives 608 queue and 1,323 total. That is the M10 row's "1323 bytes", which attaches at depth 1. The number is the same with and without `engine/realtime-audit`.

**Fix:** correct the Attempt 1 record to the depth-8 row above.

### NIT-1: gate 1's rendered half does not defend the record's `mute` flag

A record with `mute = false` and an all-zero follow-silenced target leaves the route active, mixing zeros. That renders the same bits while `u-b` is live (V1b green). Only the record-level assertion catches it. The cost is CPU, plus `-0.0` against `+0.0` when the route is the bus's only contributor.

**Fix:** pick one.

- Assert `graph::test_only_route_mix_counts()[index]` over the settled blocks: 0 when `mute || lanes == [true, true]`, else the block count.
- Or add trials with `u` muted, compared from the snap block's end (not from `settled`: #1220's by-design snap-block `-0.0`). My probe does both.

### NIT-2: D4's named-allocation join is untested, and is unreachable today

V2 is green. The probe shows why: at a given depth, the builtin strip queues are larger than a route queue.

| Depth | Route largest | Engine largest |
|---|---|---|
| 4,096 | 98,328 B | 163,880 B |
| 65,536 | 1,572,888 B | 2,621,480 B |

So the builtins compile refuses first (`builtin.resource.limit`), or the compile caps do at small depths. The join is correct defense in depth, and D4 asks for it.

**Fix:** one sentence in the record: the join cannot be reached while a route record is no larger than a strip record, so no test exercises it.

### NIT-3: the lifetime contract of `route_controls` is unstated

The rustdoc should say three things:

- `Ok` means queued, not applied.
- A producer belongs to the `PreparedHost` plan it was prepared with, and a host must drop or replace it with that plan.
- A record still queued when the plan is replaced is discarded with it, so the replacement must be prepared from committed state.

The strip and effect producers have the same unstated contract, so this is pre-existing in kind.

## The coordinator's three #1220 items

1. **The acked-batch question across a plan's life.**
   - #1221 does **not** tie producers to their plan. host-core returns `PreparedHost` and `HostLiveControlHandles` as separate values, as it already does for strip and effect producers. A host that drops the plan and keeps a producer gets `Ok` until the ring fills, because the ring is `Arc`-shared.
   - Nobody acks in #1221: host-web drops the producers, and the C ABI attaches none.
   - **#1222 (browser):** there is one preparation site (boot, `lib.rs:5835-5869`), no plan swap, and `dispose` drops all of `ready`. The fix is to store `route_controls` in `ReadyOwnership` beside the plan. Then no ack can outlive its consumer.
   - **#1225 (C ABI):** plans swap there. A record that was acked but not drained is discarded with the retiring plan. That is correct only if:
     - the committed model already holds the acked value, so the replacement prepares it as a constant and the edit survives as a step at the swap boundary, with its ramp lost;
     - the producers are swapped atomically with the plan at the L0 boundary.

     #1225's "L0 window" hazard covers the second point. I recommend #1225 add an explicit gate: an edit acked, then a structural swap before the drain, then the new plan renders the edited value.
   - For #1221 itself this is NIT-3 only.
2. **Record mute equals `RouteGate::silences()`, including follow-mute.** Verified.
   - The code computes it from the gate.
   - Gate 1 asserts it for 2,000 draws.
   - My 64-case probe checks it rendered and by activity, from both an open and a prepared-follow-silenced start.
   - One muted source lane correctly gives `mute = false` with that lane's column zeroed, which is the prepared semantics of #1218.
3. **No cross-route fence.** This is not a defect for #1221 or #1222.
   - D5 requires only "check every `free()` before any `push`", and `set` never partially pushes.
   - In the browser, `miso_engine_web_v1_command_submit` runs from `port.onmessage` on the AudioWorklet thread (`miso-engine-v1-audio-worklet.js:266`, `:1518`). That is the thread `process()` runs on, so one batch's pushes all land between two quanta and apply in the same block.
   - In the C ABI, control and render are different threads, so a batch can straddle one quantum. This is the existing property of #1053's strip and effect queues. Every record still applies, and nothing drops.
   - #1225 should state it, and either accept a skew of at most one quantum or add a fence. It is not #1221's to fix.

## BATCH-1 (outside #1221; root must fix before the K3 push)

`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` is the `qualification.yml:460` "Documentation" step in the required lint job. It fails with:

```
error: public documentation for `route_coefficients` links to private item `route_transform`
```

- The source is `crates/graph-compiler/src/ids.rs:318`, from #1215 (`6701cd55e`). That commit is on this batch branch and not on `main`.
- The K3 push will be red.

**Fix:** write `` `route_transform` `` as a plain code span instead of an intra-doc link.

The other error I saw, host-core's unresolved `SessionControlProvider`, occurs only with `-p host-core` alone. Under workspace feature unification the workspace run excluding graph-compiler is clean.

## Deviation: `route_id()` method instead of D2's `pub route_id: Box<str>` field. Accepted.

- The wrapper is `#[repr(transparent)]`, so the retained producer table is exactly what `graph::route_control_resources` charges. The in-place `collect` keeps the same allocation.
- A second `Box<str>` would be 16 B per route, uncharged, and a duplicate.
- A read-only accessor is also safer than a `pub` mutable ID.
- #1222 to #1225 never spell the field. They address "by route ID through `route_controls`" (#1225 D1, Hazards) and "copy the route ID" (#1223 D1), which `route_id()` serves directly.
- Optionally, root updates D2's code block in the spec to the method spelling.

## Inherited from #1220 (INFO)

#1220 MINOR-1 is still open at this head: the pre-bind `GraphRouteControlBinding.node` ID copies are uncharged. host-core admits the charge while it holds the attached, unbound artifact, so up to one route ID's bytes per route are briefly uncounted. When #1220 fixes `graph::route_control_resources`, host-core's charge and gate 6's oracle (the same function) follow without change here.

## Test value (one sentence per new test)

- **`the_live_target_is_the_prepared_constant`:** red if a record's target or per-lane follow columns come from anywhere but `route_coefficients` (M1, M2, V9, the last also on the rendered half alone), or if its `mute` is not the gate's silence (V1; record-level assertion only, NIT-1).
- **`a_settled_live_edit_through_host_core_equals_a_fresh_plan`:** red if host-core hands producers out in the wrong order or attached to the wrong routes, or `set` pushes other values than it was given (M3, M4).
- **`a_refused_send_record_pushes_nothing`:**
  - red if a refused call pushes (M7), a full queue acks (M5), an over-long ramp is clamped (M6), or `Length` is decided before `Domain` (V7);
  - not red without the `free()` pre-check (V3), which is equivalent.
- **`live_controls_cost_the_standing_sessions_no_fold`:** red if attaching live controls takes output routes and loses the master fold (M8). The fold counts are 64 and 64; this is the only fold test through host-core.
- **`live_send_handles_are_in_canonical_route_order`:** red if producers are listed in declaration or any non-lane order (M3), or attached without a depth (M10).
- **`live_send_lanes_are_charged_against_the_graph_cap`:**
  - red if the route total leaves `admitted_graph_and_model` (M9) or is charged on a live-control-free plan (M10);
  - not red if the named-allocation join is dropped (V2), which is dominated (NIT-2).
- **`live_sends_render_without_allocating`:** red if a lane's drain allocates or frees on the render thread (M11, SIGABRT under the armed audit).

## Required before close

1. MINOR-2: correct the gate 6 numbers in the Attempt 1 record (1,659 B at depth 8).
2. MINOR-1: root routes the browser exact-retained row to #1222's brief, with the fix and the test above, or authorizes it here.
3. BATCH-1, not #1221's: fix the graph-compiler doc link before the K3 push.

NIT-1 to NIT-3 are optional.
