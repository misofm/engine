FAIL

# #1345 attempt 1: adversarial verdict

Under review: `git diff 710f77df7 1d6e12f93` on `codex/d15-stream-b3` (commits `09190ea55`, `122bdbbc0`,
`ba14de159`, `f99f6cd7d`, `1d6e12f93`), as one attempt. I exported `1d6e12f93` with `git archive` to
`/tmp/claude-1002/v1345/tree`, git-initialized it, and built it with a private `CARGO_TARGET_DIR`. I did
not read, build or edit anything in the worktree. Three read-only helpers worked in their own exports
(render-side cells; admission on both hosts; docs, re-pins and ported tests). I checked their main
findings against the code before I used them here.

**Summary.** The cell design is correct and complete for D1, D3, D4 with Amendment 2, D5, D7, D8 and
D9. Every gate job is green. The bypass-order claim is true. The attempt fails on two MAJOR findings:
1. a shipped SDK operation now always fails, because the observation queue depth is D2 read literally;
2. the resource estimate that this attempt rewrote charges less than it retains for an observed bank.

Both need a fix before merge. M1 also needs a root ruling on D2.

## MAJOR

**M1. The observation queue at one slot per tap makes the SDK's managed `update()` fail on every try,
and the refusal invites a retry that cannot succeed (spec level: D2).**
- The queue depth is `descriptor.observations.len()` (`crates/effect-compiler/src/prepare.rs:1747-1751`).
  Every launch effect has one tap, so the depth is 1.
- At `710f77df7` an `Observe` record rode the shared effect queue, `min(records, automation_capacity)`
  deep. In the browser that is 64 by default.
- `sdk/src/core/observation-subscriptions.ts:617-625`: when only `windowBlocks` changes on a tap that one
  handle owns, `update()` sends a disarm (old key) and an arm (new key) for that one tap in **one**
  managed submit (`#submit`, `:849-862`). `submitManaged` (`sdk/src/core/live-controls.ts:1030-1053`)
  does not split the batch.
- The browser room pass (`hosts/host-web/src/lib.rs:5855-5872`) then gets `needed = 2 > available <= 1`
  and returns `COMMAND_REASON_BACKPRESSURE`. `#submit` throws "observation transaction was refused".
- The batch can never fit a one-slot queue, so the failure is permanent, not one block long. BACKPRESSURE
  tells the host to retry after a render. Here a retry never succeeds. This is the endless-backpressure
  shape that #1264 D3 removed for effect queues.
- Helper probe (`/tmp/claude-1002/v1345/mB/mut/`), one submit with unsubscribe(window 2) then
  subscribe(window 4) on one tap:

  | commit | result |
  |---|---|
  | `1d6e12f93` | `result=6, reason=8` |
  | `710f77df7` | `result=0` |

  The same two records as two submits with no render between them: `0` then `6` here, `0` then `0` at
  the parent.
- The acked-batch question: no. The refusal is typed, admits 0 records and writes no cell. So this is a
  product regression, not a lost value.
- Tests were fitted to D2 and nothing that shows the regression was added:
  - `sdk/test/observation-evals.mjs:172-187` splits the one-submit re-arm into two submits with a render
    between them.
  - `tools/parameter-metadata/tests/round_trip.rs:597-599` renders between a subscribe and its
    unsubscribe.
  - `hosts/host-web/src/tests.rs:6346` drops the `observe(false)` of the re-arm.
  - `hosts/host-web/src/tests.rs:6803-6841` now asserts BACKPRESSURE for a double subscribe.
  - No SDK eval calls `update()` with a window change on the real engine, so the SDK gates stay green.
- The implementer's open item 1 reports the idiom but not the broken API.
- D2 says "Its depth is the number of the effect's observation taps", so the implementer followed the
  text. But the product outcome and the owner principle do not allow a shipped API that always fails.
- Fix: root rules on D2. The depth must hold the largest valid single submission per tap; disarm plus arm
  is 2 per tap, the implementer's own alternative. A batch larger than the queue's whole capacity should
  get a refusal that does not invite a retry. Add a real-engine SDK eval for `update()` with a window
  change, and restore the one-submit re-arm in `observation-evals`.

**M2. For an observed bank the estimate charges less than bind retains. The attempt record says the
estimate is exact, which is false.**
- Where: `crates/graph-compiler/src/estimate.rs:303-310`, and the same formula in the graph-compiler lib
  test at `crates/graph-compiler/src/lib.rs:3193-3199`.
- The estimate charges `lanes × size_of::<Option<Consumer<EffectControlRecord>>>()`, which is 40 B for
  every bank lane.
- Bind allocates `Box<[(usize, Consumer<..>)]>` only for the lanes that hold a queue
  (`crates/rack/src/lib.rs:1119-1127`). That is 48 B for each queued lane.
- When no lane is observed, an 8-lane bank is charged 320 B too much. So "estimate equals prepared bytes
  exactly (four cases checked)" is false; only the per-node cases are exact.
- When every lane is observed, an 8-lane bank is charged 64 B too little. That breaks the estimate's
  ceiling, `charged >= retained`. Helper probe: `bypass_resources` with observation taps 1 on a console
  without the EQ gave retained 166,224 B and charged 166,096 B, and the assertion went red.
- In the committed three-bank case the untapped EQ bank's +320 B hides this. No committed test runs a
  bank with taps. The graph-compiler lib test copies the estimate's formula, so it enshrines the error
  (see m8).
- On an observation-capable plan, each lane of a bank whose effect declares a tap has a queue. So a
  session whose banks are only tapped effects, such as compressors and limiters, is under-charged.
- Fix: charge `queued_lanes × size_of::<(usize, Consumer<EffectControlRecord>)>()`, and add an
  allocator-measured test of a tapped bank.

## MINOR

- **m1. Open item 4 is wrong for the browser, and the number is a dead configuration surface.**
  - In host-core, `control_queue_depth` (`crates/host-core/src/prepare.rs:331`) and `STRIP_LIVE_QUEUE_DEPTH`
    are read only as `Some`/`None`. The number is dead.
  - In the browser, `live_control_command_queue_records` (`hosts/host-web/src/lib.rs:1232`) still changes
    behaviour:
    - `lib.rs:6764` uses `.max(records)` to raise `maximum_automation_spans_per_block`. On
      `command-session.json`, records 256 against 16 costs +8.5 KB of graph plan bytes and +8.5 KB of
      metadata. That coupling existed only to make the old queue `records` deep, and D4 now sizes the
      window.
    - The JS in-flight bound (`worklet-host.js:1185`, `:1759`) still uses it, and its comment says
      "worklet-side queue depth".
  - The browser word is a frozen boot-option field (offset 32 in the ABI layout), so removing the word is a
    contract change for a successor issue.
  - Removing the `.max` coupling and correcting the option docs (`lib.rs:1226-1244`) are inside this
    issue's paths and should be done here.
- **m2. The shipped browser declaration is still false.**
  - Where: `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, and its twin
    `sdk/src/browser/shipped-host.d.ts`.
    - `:63-67` says effect bypass and parameters keep bounded queues and refuse when full.
    - `:71-73` says "the other stages drain their queue".
    - `:343-344` lists the cell kinds without effect values.
  - #1312, #1346 and #1347 each updated these lines.
  - The files are outside this issue's paths, so root must authorize the edit, as for #1399/#1349.
- **m3. `docs/C_ABI_V1_QUALIFICATION.md` still has stale #1264/#1265 text. D6 says this document follows the
  header.**
  - `:415`: "#1264 pushes live effect parameter edits through them".
  - `:494-498`: "The engine pushes one parameter record ...".
  - `:529-531`: "through the EQ's prepared-target staging". D1 removed that staging.
- **m4. Two header sentences are now too general** (`crates/capi/include/miso_engine_v1.h`).
  - `:56-57`: "A later value for the same lane ... replaces the earlier one". It should say "same cell":
    `:63-64` now calls all of an effect's cells "that effect's lane".
  - `:60`: "A Both record writes two cells and counts in each". This is false for a `Shared` Block-rate
    parameter, which has one `Both` cell (D1). An example is the delay's cross feedback, which is live
    on the C ABI.
- **m5. The second bullet of gate 3 is met only in part.**
  - `crates/effect-contract/tests/live_window.rs:197-216`: `one_block_applies_every_live_cell_of_one_transaction`
    builds its lane with `Rig::new`, whose window is the cell count. No `automation_capacity` and no
    preparation are involved.
  - Its doc says the window-sized-to-`automation_capacity` mutation turns it red. It does not: under that
    mutation, and under `check_window` always `Ok`, the test stays green.
  - What it really catches: dirty-word aliasing past 64 cells. That catch is unique and useful.
  - The spec asks for a synthetic descriptor with more cells than `automation_capacity`, through
    preparation. Only `native_session` covers the preparation sizing, and only for the launch effects.
  - Fix: make the test go through preparation, or correct its doc and the attempt record.
- **m6. The per-node effect path's snapshot read is not tested.**
  - With `crates/graph/src/runtime.rs:1252` mutated to `stage(.., LiveSnapshot::ALL)`, every test stays
    green: graph 101, capi lib 78, host-core 232, host-web 199.
  - Amendment 2 is defended only at the lane level and on the bank path (the #1504 test). The code is
    correct.
- **m7. `crates/rack/tests/live_control_bank.rs:900`
  `a_revisions_bypass_and_gain_land_together_under_the_blocks_snapshot` has no unique catch.**
  - It uses unity input and latency 0, so a bypassed block looks the same as a wet block at unity.
  - Under the defect it names (stage reads `ALL`), all 60 rack tests are green.
  - It goes red only on "drain skips a bypassed lane", and `un_bypassing_returns_the_current_wet_signal`
    also catches that.
- **m8. The graph-compiler resource lib test is weaker than before** (`crates/graph-compiler/src/lib.rs:3083-3199`,
  `:3445-3466`).
  - Lane bytes now come from `lane.retained_resources()`, the same function the estimate calls. So the
    test is no longer independent, which its own header claims it is.
  - It copies the bank-queue formula that is wrong (M2).
  - An exact payload equality became a lower bound and a vacuous `assert!(size_of::<EffectLaneResources>() > 0)`.
- **m9. `tools/console-workload/tests/automation.rs:350-418` is looser than it needs to be.**
  - It asserts only `apart_kept < kept`.
  - The exact value `kept - 3 * PREROLL_BLOCKS` (320) still holds, and the parent pinned it.
  - The doc's phrase "retires the collapse for that block" is false: the three EQ cohorts retire for the
    whole preroll.
- **m10. One SDK eval lost an assertion although the behaviour is still true.**
  - The eval: `sdk/test/writer-evals.mjs:280-313` ("a batch larger than the queue splits").
  - It no longer asserts that a narrowed batch is admitted while the host is paused. The head record
    blocks every split until a render.
  - With the batch staged `[1, 2, 0]` and `admitted == 2, pending == 1` asserted before rendering, the
    eval would catch a `drain()` that stops at its first refusal.
- **m11. Gate 2's test doc says more than the test proves, and its oracle differs from the spec.**
  - The test: `crates/capi/src/runtime/live_tests.rs:2680-2685` (`a_bypass_lifted_beside_a_parameter_renders_like_the_browsers_lane`).
  - Under a mutation that defers parameters one block after a bypass change, the test stays green. Its
    lane-reference oracle shares the lane code.
  - Its real unique catch: the C ABI dropping a `Bypass` that rides beside parameter records. Only gate 2
    goes red.
  - It compares against host-core's lane reference, not "the twin's rebuild of the committed model". This
    follows the file's #1264/#1265 convention, and a rebuild twin would not catch more. But the attempt
    record must state the deviation.
- **m12. No test shows that one large transaction on one instance stays live.**
  - Reinstating the deleted queue-capacity rebuild at more than 16 records per instance leaves everything
    green (capi 100, host-web 199). So does a variant at more than 3 records.
  - A rebuild renders the same samples, so only an assertion that no plan was replaced can defend the
    product outcome "none is ... turned into a rebuild".
- **m13. Files outside the spec's list need root to ratify them.**
  - Every file is disclosed, and each edit is forced (see "Answers", item 4). But the STREAMS hot-file rows
    do not list #1345 for:
    - graph-compiler (`docs/handoffs/decision-15-2026-10-05/STREAMS.md:95`);
    - `host-core/src/prepare.rs` (`:75`);
    - the callgraph script (`:110`);
    - the ABI fixture (C #1406's exception).
  - STREAMS' "Watermark bound on `main` today" (`:312-317`) goes stale when this merges (open item 2). That
    is root's text.

## NIT

1. Stale queue-era prose in files this change edits or owns:
   - `crates/rack/src/lib.rs:644`, `:664-665`, `:1391`, `:1918`, `:2551`;
   - `crates/graph/src/lib.rs:1488`, `:2511`, and the test name
     `the_live_control_effect_drops_nothing_within_its_prepared_capacity`;
   - `crates/host-core/src/live_delta.rs:95-102`;
   - `crates/host-core/src/prepare.rs:810`, `:1661-1662`;
   - `crates/control-plane/src/control.rs:38-42`;
   - `crates/control-plane/src/compile.rs:15-16`, which describes an observation queue that the C ABI never
     attaches;
   - `tools/audit/src/capi.rs:308`;
   - `tools/console-workload/src/lib.rs:1356`;
   - `sdk/src/core/writer.ts:34-36`, `:102`.
2. `miso_engine_v1.h:50` is new and 121 characters long. The rest of the block is at most 100. The reflow
   at `:66` and `:86` is ragged.
3. D15-2's new parenthetical sits under "Amended 2026-10-11, root" but is this issue's text. Add a #1345
   marker. "Every lane" also depends on #1346 and #1347, which the text does not name.
4. `hosts/host-web/src/lib.rs:5895`: the push loop treats `Effect(PreparedTarget)` as "written above", but
   `write_strip_cells` has no arm for it. `preflight_effect` (`:5419`) refuses a raw target first, so no
   ack can come before a drop. The arm should be `unreachable!`.
5. The EQ's D7 depends on the defaulted `target_slots()` (`crates/effect-contract/src/prepared_target.rs:73`)
   covering every slot that `validate_targets` accepts. Checking each target with `writer.check` in
   `preflight_publication` would make D7 structural.
6. `crates/effect-contract/tests/observation_lane.rs:119-141` asserts only `is_armed(0)`, so its claim
   about order and window is weak. Rack's `each_lanes_subscriptions_reach_that_lanes_taps_in_order` does
   defend order.
7. `paired_spans::a_channel_the_policy_does_not_take_has_no_cell` (`:176`) duplicates
   `live_control::a_record_without_a_cell_is_refused_and_writes_nothing` (`:133`). The batch half of the
   latter tests the rig's preflight loop, not production code.
8. The graph #1504 test's doc (`crates/graph/tests/live_snapshot_drains.rs:768`) also claims a
   "skips the drains of a silent block" catch. A stand-in mutation stays green.
9. `scripts/check-realtime-policy.sh:79-82`: prose outside "the region floor line only" was edited. It is
   prose only.
10. `EffectLaneShape::live_parameter_cells` raises the EQ's window from the descriptor's count, but the EQ
    lane holds no parameter cells.
11. Removing the browser-v1 flood row was correct, but a positive flood that asserts `RESULT_OK` would keep
    a wasm-level "flood admitted" check.

## Answers to the five questions

1. **The spec, the amendments and #1504.**
   - D1, D3, D4 with Amendment 2, D5, D7, D8 and D9 are met.
   - Every cell read goes through `Reads::read` → `CellReader::read_counted(.., snapshot, ..)`
     (`crates/effect-contract/src/live.rs:1070-1097`). `LiveValuesSuperseded::add` is private and its only
     caller is `read_counted`.
   - Every applied parameter is staged in its block. The window is at least the cell count, checked at
     bind (rack `:1110-1118`, graph `runtime.rs:1300`).
   - Pairing keeps `LIVE` only for two cells applied in one block with equal words. This is conservative
     and matches #1004. "Folds from the final cell contents" in D3 could be read more widely, so root may
     want the wording aligned.
   - D8: the bank drain is two loops. The second names the queue only in its binding, its count and its
     pop. Regions went from 104 to 106 in 27 files (+1 `graph/src/runtime.rs`, +1 `rack/src/lib.rs`).
   - D2 is met as written, but see M1.
   - The #1504 deferral is met. `effect_bank::a_live_bypassed_lane_is_drained_every_block_under_the_blocks_snapshot`
     drives a lane bypassed by its cell inside 4- and 8-wide banks. Its results:
     - "rack drain skips a bypassed lane": red;
     - "bank stage reads `ALL`": red, and the only red test, so that is its unique catch.
   - The acked-batch question: no ack can come before a drop.
     - C ABI: every fallible step runs before the stamp. After the stamp, the strip, effect and EQ-target
       writes cannot fail, and the revision is published once (`crates/control-plane/src/control.rs:1344-1621`).
     - C ABI: no `Observe` exists there (taps 0).
     - Browser: observation room is checked for the whole batch before `write_strip_cells` (`lib.rs:5855-5881`).
     - Browser: only `Observe` is pushed afterwards, and its room was already checked.
     - The EQ owner cannot fail after a write. Its slot range equals `EQ_SECTION_COUNT`.
   - Render is free of allocations, locks and syscalls:
     - `audit capi`: 100,000 calls, all violation counts 0, `pcm_digest cb10fbface44a3a4`, the same as
       the parent.
     - The effect trace: 1,000,000 blocks, ok.
     - `check-realtime-policy`: ok.
2. **The doc set.**
   - The header no longer has the effect-before-fader sentence, "up to one quantum apart", the 16-record
     room, the rebuild-when-too-big text or `control.live.backpressure`. It states the exact rule.
   - AGENTS.md changes only that one sentence: the "until then" clause is removed and "early" became
     "early,". No other byte changed.
   - D15-2 and the qualification doc agree with the header.
   - The central claim is true in the code: every live lane is read under the block's one snapshot.
     - fader, matrix and input lanes: `builtins-compiler`;
     - route lanes: `graph/src/runtime.rs:902`;
     - effect lanes: `live.rs`;
     - VCA and solo lower to fader and mute cells in the browser and rebuild on the C ABI.
     - All of #1502, #1503, #1504, #1312, #1346, #1347 and #1399 are below `710f77df7`.
   - Defects: m3 and m4, and the shipped d.ts (m2).
3. **The open items.**
   - (a) It is mandated by D2 as written, but it is a product regression, and wider than reported. Tests
     were fitted, not weakened in a way that hides a defect the spec allowed. See M1.
   - (b) The host-core number is dead. The browser number is not dead: it is a dead coupling that costs
     memory. See m1. Nothing mandates keeping the coupling. The boot word itself is frozen ABI and needs a
     successor issue.
   - (c) `capi_retained_bytes` is confirmed at 277,481 → 281,513 against a ceiling of 282,432 (919 bytes of
     headroom). The growth is justified by D1: the producer now owns the `EffectCellWriter`. Ceilings are
     ceilings, and no budget or digest constant moved. The four-lane (AArch64) rows were not measured here.
   - (d) Each re-pin is justified on its own:
     - ABI-layout fixture: it is red at `710f77df7`
       (`parameter-metadata --test abi_layout the_checked_in_self_test_fixture_is_current`), and the new
       file is the generator's exact output. It has been stale since #1399; root should note this against
       #1399's n5.
     - Callgraph pins: `impl EffectControlProducerHandle` was deleted, so the impl index moved
       `Ms2_` → `Ms1_`. The site counts (4 and 4) and the reasons are unchanged. The pins pass in
       `check-web-audioworklet.sh`.
     - Browser-v1 flood row: it asserted that 5 `bypassOn` records were refused whole. Under cells that
       batch is admitted. Both pinned digests are unchanged.
4. **Files outside the list.** Every edit is forced by a removed API or a new attach signature:
   - graph-compiler estimate (but see M2);
   - host-core attach and helpers;
   - `effect-compiler/src/control.rs` (the `Capacity` variant);
   - `capi/src/ffi.rs` and `resource_lifecycle.rs`;
   - console-workload (`try_push` is gone; `chain_shape`'s old `Both` push moved nothing at the parent,
     so the new `Right` push makes the test stronger);
   - parameter-metadata `round_trip` (D2);
   - the SDK writer prose and evals.

   For the STREAMS rows see m13.
5. **Deleted, rewritten and new tests.**
   - The four gate-6 capi tests and `an_eq_bypass_beside_targets_filling_its_queue_rebuilds` are gone.
   - No `LiveBackpressure`, `live.backpressure`, `effect_room`, `EffectControlPushError`, `try_push_prepared`
     or owner `Capacity` remains in code.
   - The host-web bypass floods that the cells made false were rewritten as observation floods. They still
     defend "a cell family is not written when the batch's observation room fails".
   - The SDK's 17th-flush and 32/33 both-lane evals were deleted because their premise is now false.
   - Weakened: m8, m9, m10.
   - The bypass-before-parameters order is **unobservable: confirmed**.
     - Moving the bypass read after the parameters and targets leaves every suite green: effect-contract
       81, rack 60, graph 101, capi lib 78, host-core 232, host-web 199.
     - The reasons: the witness terms commute, the shunt is applied after `process` on both paths, the wet
       path always runs, and nothing resets on un-bypass.
     - So the spec's test value for gate 2 cannot be reached. The implementer disclosed this.

## Test value (new or rewritten tests)

**capi**
- `forty_effect_edits_of_each_kind_before_one_block_are_all_accepted_and_the_last_is_heard` (gate 1)
  catches a C ABI path that bounds, refuses or rebuilds a run of unrendered effect edits, or miscounts
  superseded target cells (39 vs 117). The miscount is also caught by effect-contract
  `prepared_targets_are_cells_where_the_last_write_wins`.
- `a_bypass_lifted_beside_a_parameter_renders_like_the_browsers_lane` (gate 2) catches the C ABI dropping
  a `Bypass` that rides beside parameter records (unique). It does not catch a deferral (m11).
- `a_refused_eq_transaction_leaves_its_owners_idle_and_writes_no_cell` catches a cell written before the
  owner preflight or fault hook (unique).

**host-web**
- `strip_cells_cross_host::both_hosts_drain_two_parameter_edits_and_one_eq_gain_edit_alike` (gate 5)
  catches the browser writing an effect parameter cell twice under one stamp (unique; superseded 3 vs 1).

**effect-compiler**
- `every_launch_effect_is_prepared_with_a_window_for_all_its_live_cells` catches preparation that does not
  raise the window to the live cell count.
- `an_entry_whose_capacity_is_below...` catches the attach-time window check being removed (unique).

**effect-contract `live_window`**
- The cell-count test catches a count that includes non-live parameters.
- `a_window_shorter_than_...` catches `check_window` accepting a short window.
- `one_block_applies_every_live_cell_of_one_transaction` catches dirty-bit aliasing past 64 cells
  (unique; not the window mutation it names, m5).

**effect-contract `live_control`**
- `prepared_targets_are_cells_where_the_last_write_wins` catches a target L/R swap, uncounted target reads
  and no superseded charge.
- `a_target_owners_lane_refuses_a_bare_parameter_and_resets_its_target_prefix` catches a target prefix
  that is not reset (unique).
- `a_transaction_lands_in_one_block...` and `a_value_above_the_snapshot_waits...` catch a lane read
  without the snapshot.
- `a_repeated_write_collapses_last_wins_and_is_counted` catches a missing superseded charge.
- `a_drain_emits_the_contract_canonical_span_order` catches an L/R cell swap.

**effect-contract `paired_spans`**
- The seven pair tests catch `paired = false`. They overlap each other, as they did at the parent.
- `targets_fold_from_the_final_cell_contents` catches equal target words that are not merged to `Both`.
- `a_channel_the_policy_does_not_take_has_no_cell` duplicates another test (NIT 7).

**effect-contract `observation_lane`**
- The full-queue test catches a full queue that loses or overwrites the refused record.
- The no-queue test catches a queue built at depth 0.

**rack**
- `each_lanes_subscriptions_reach_that_lanes_taps_in_order` catches records drained to the wrong lane
  (unique).
- `a_bank_window_below_a_lanes_live_cells...` catches the bank bind check being removed.
- `a_revisions_bypass_and_gain_land_together...` has no unique catch (m7).

**graph**
- `effect_bank::a_live_bypassed_lane_is_drained_every_block_under_the_blocks_snapshot` catches a bank
  stage that reads its cells without the block's snapshot (unique).

**host-core**
- In `symmetry_witness`, the two tests that replace the EQ per-lane-target test catch equal target words
  in one block that are not merged (red under that mutation).

**console-workload**
- `paired_spans` catches `paired = false` (9 red).
- `the_eq_lowering_keeps_the_collapse_when_a_pair_lands_in_one_block` catches a missing `Both` merge. Its
  first assertion is older, so its "together" half has no unique catch, and its "apart" half is loose
  (m9).
- `chain_shape`'s `Right` push now moves rendered bits, which the old `Both` push never did.

**SDK**
- "A paused host takes any number of effect edits, refuses none, and the last of each lands" catches a
  host or writer that refuses paused effect edits or applies a value other than the last.

**Rewritten host-web tests**
- The observation-flood rewrites still catch a cell family written before the batch's observation room
  check (10-11 host-web tests red when the room pass reads the wrong slot or counts effect values as
  wanting room).

## Gates run (on the export of `1d6e12f93`, this machine, no browsers)

**`lint` job, every step: PASS.**
- Rust checks: fmt; clippy `-D warnings`; rustdoc `-D warnings`.
- Policy scripts and their mutation tests:
  - workspace, session, env vocabulary, bench, host-core and protocol-control;
  - test-support CI coverage;
  - scalar-oracle self-test;
  - script reachability (export git-initialized);
  - realtime policy: `realtime policy: ok (106 marked regions in 27 files)`, plus audit-leak and
    artifact-evidence-leak;
  - lane, unfused seal, check-rack-policy, builtins, graph, effect-runtime policy and fixtures.
- Fixtures and seals: console benchmark fixture; the standing builtins fixture manifest mutation tests;
  bench preconditions; trace validator; conformance boundaries; EQ render contract.
- Other: release-shape self-test, npm publish modes, stem store, stem identity, and the three sub-v3 and
  AVX2+FMA probes.

**`test-debug-a`: PASS.**
- `builtins-compiler --no-run`.
- The workspace `--all-targets` step with the job's exact excludes and features: 1,569 passed, 0 failed.
- Doctests: 24 passed.

**`test-debug-b`: PASS.**
- DSP crates: 914 passed, 26 ignored, 0 failed.
- Doctests and `conformance_fixtures --check`.

**`test-release`: PASS.**
- lane, math and wasm-gates: 133 passed.
- `filter_liveness`, `tail_contract`, `ramp_endpoint`, `designer_total`.
- The FMA cfg check.
- **The loom SPSC leg.**
- M1 exhaustive and F1 exhaustive.

**`audit-native`: PASS.**
- The release build, and audit/bench/console-workload unit tests: 113 passed.
- `audit capi`: all counts 0, and the record validates.
- delay, compressor, EQ and gate audits.
- builtins and builtins-graph traces, the graph audit, the protocol allocation audit.
- Realtime probes and their trace (1,000,000).
- Probe mutations.
- The effect-contract trace (1,000,000).
- `check-capi-abi.sh` and `--self-test`.
- `check-scalar-oracle-absent --native`.
- Graph determinism 100/100.
- `check-builtins-fixtures`, `check-console-fixtures` and `check-effect-contract` (8 factories, 0 failed
  gates).

**Other gates: PASS.**
- `check-cross-targets.sh`.
- `check-release-shape.py`.

**Worklet chain: PASS.**
- `build-web-audioworklet.sh --named-twin`: shipped module `b44cf5b1...1c71825`, the same as the attempt
  record.
- `strip-wasm-names.py --self-test` and `check`.
- `check-web-audioworklet.sh --without-metadata-regeneration`, including the callgraph pins.
- `check-browser-expected-resources.py --artifacts`.
- `check-scalar-oracle-absent.py --wasm`.
- `test-web-audioworklet.sh` with a private TMPDIR: 0 entries left.
- The V8 spill self-test and the gate on Node v22.23.2.

**ABI layout: PASS.** `check-abi-layout-v1.py` on the asset passes, and `--self-test` catches 22
mutations.

**`sdk` job checks (source-bundle mode, `sdk/dist` deleted first): PASS.**
- `check-sdk-generated`, `check-sdk-deletions`, `check-sdk-types`, `sdk-package.sh check`.
- `check-sdk-headless`: 362 passed, 0 failed. The failure that was there before (#1399's verdict) was
  deleted by #1346.

**Spec gates 1-10: PASS as the attempt record states**, with the reservations in m5 and m11. Gate 10 was
re-run with attempt 2's gate: `realtime policy: ok (106 marked regions in 27 files)`.

**Package suites (helper exports): PASS.**

| package | result |
|---|---|
| effect-contract | 81 |
| rack | 60 |
| graph `--features test-support` | 101 |
| graph-compiler | 172 passed, 2 ignored |
| effect-compiler `--features test-support` | 42 |
| capi | 100 |
| control-plane `--features test-support` | 1 |
| host-web `--features test-support` | 199 passed, 2 ignored |
| host-core `--features test-support` | 266 passed, 2 ignored |
| console-workload (release) | 66 passed, 2 ignored |

**Not run:**
- the Playwright `browser` legs (no browsers);
- the AArch64 legs, including the four-lane resource rows.

## Mutation runs (helpers' exports; each reverted)

| mutation | red | green |
|---|---|---|
| bypass read after params and targets | none | ec, rack, graph, capi, hc, hw |
| rack drain skips bypassed lanes | graph #1504 test, two rack tests | rest |
| bank `stage(.., ALL)` | graph #1504 test only | rack 60 |
| per-node `stage(.., ALL)` | none (m6) | all |
| `check_window` always `Ok` | ec `a_window_shorter...`, rack bank-window test | graph, ecomp, gc |
| drop `.max(live cells)` in preparation | ecomp window test, 13 hw | gc, capi, hc |
| `paired = false` | ec 7, hc 1, console-workload 9 | rack, graph |
| equal target words not merged | ec 3, gc 2, hc 2, console-workload 11 | capi, hw |
| parameter L/R cells swapped | ec 3, graph 2, hc 3 | rack |
| target cells read uncounted | ec 1, capi gate 1 (39 vs 117) | hc, hw |
| no superseded charge | ec 3 | rack, graph |
| dirty-word aliasing past 64 cells | ec `one_block_applies...` only | rest |
| C ABI writes bypass before owner preflight | D7 test only | rest |
| C ABI drops a bypass beside parameters | gate 2 only | rest |
| C ABI parameters deferred one block after a bypass change | ec 2, rack 2 | capi gate 2 (m11) |
| browser double parameter write | gate 5 only | rest |
| browser `Both` target as `Left` | gate 5 and 8 hw | — |
| queue-capacity rebuild reinstated (>16, >3) | none (m12) | all |
| observation probe at the parent vs here | parent `0`, here `6` | — |

Evidence logs: `/tmp/claude-1002/v1345/logs/`, `/tmp/claude-1002/v1345/mA/mut/`,
`/tmp/claude-1002/v1345/mB/mut/`.
