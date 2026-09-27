# Enforce the paired-span window bound in the effect process-block constructors

## Product outcome

#1004 keeps the mono collapse when a both-channel `Parameter` edit arrives as twin Left and Right spans. Its exactness rests on one precondition: a lane's staging window is never larger than the effect's `automation_capacity`, so a twin pair can never straddle the cut-off. Today both constructors size the window to the capacity, but nothing enforces it, and the `debug_assert!` meant to guard it can never fire. A future caller that sized the window one larger would render the left channel's state on the right and lose an acknowledged right-channel write. The #1004 Sol verification (attempt 1, PASS) raised this and two smaller documentation findings, quoted below from `.github/ISSUE_SPECS/1004-*.md`.

## Findings to close

1. **Low: the A3 debug assertion is vacuous.**
   * `debug_assert!(staged <= staging.len())` can never fire, because `staged` is incremented
     only while it is below `staging.len()`.
   * So the precondition the pair rests on, `staging.len() <= automation_capacity`, is enforced
     nowhere. Neither `EffectBankProcessBlock::new` nor `EffectProcessBlock::new` checks a lane's
     span count against capacity.
   * Today it holds, because both constructors size the window to the capacity.
   * **Failure scenario.** A future caller sizes its window to capacity + 1. A drain stages
     capacity + 1 spans, and the last twin straddles the cut-off: the effect applies `Left p v` at
     index `capacity − 1` and refuses `Right p v` at index `capacity`. `LIVE` is kept, the
     collapse renders the left channel's state for the right channel, and the right-channel write
     is lost when the collapse disengages.
   * **Fix.** Assert `window == automation_capacity` where both are known, in the two
     constructors.
2. **Low: stale prose in `host-core/tests/symmetry_witness.rs`.**
   `two_per_lane_writes_that_agree_still_decline_the_lane` says:
   * that a `Left` then a `Right` write to the same value "is how the ABI addresses a `PerLane`
     parameter";
   * that the witness "cannot see two writes cancel".

   Both statements are now false for `Parameter` spans. The test still passes only because it
   drives EQ targets.
   * **Failure scenario.** A maintainer reads the test as the contract and "restores" the decline
     for twin spans.
   * **Fix.** Correct it in a follow-up, since the file is outside this slice's paths.
4. **Info: an undocumented change on an invariant-failure path.** A lone one-channel `Parameter`
   record that reaches a target owner now keeps `LIVE`, because the window is empty and so pairs.
   Base cleared `LIVE` there.
   * This is sound: the record is refused and never applied, and `stage` returns `target_error`,
     so the block's render fails.
   * No fix is needed. It is noted so that nobody reads it as the pairing rule firing.

## Objective gates

1. `EffectBankProcessBlock::new` and `EffectProcessBlock::new` refuse (with a typed error at preparation, never on the render thread) a window whose span count differs from `automation_capacity`; the vacuous `debug_assert!` is removed or replaced by an assertion that can fire.
2. A test builds a window of capacity + 1 and shows it refused; a mutation that removes the check turns the test red.
3. `two_per_lane_writes_that_agree_still_decline_the_lane` in `crates/host-core/tests/symmetry_witness.rs` states the contract as it is after #1004: twin `Parameter` spans keep the collapse; one-channel EQ targets still decline it. A test for twin spans keeping `LIVE` sits beside it.
4. The lone one-channel record that now keeps `LIVE` on the invariant-failure path (finding 4) is documented where it happens.
5. All #1004 gates, every standing digest and the `console_mixing_automation` preflight are unchanged; clippy, fmt and the policy scripts pass. Render stays allocation-free.

## Attempt 1 evidence

Implementer: Terra (Claude Opus 5.5), 2026-09-27, branch `codex/1012-paired-span-window-bound` on
`b2f312c2` (the batch head with #1004 merged at `f8556aaf`). Code commit `0f638168`. Host AMD EPYC
7313P (Zen 3), rustc 1.97.1, `CARGO_INCREMENTAL=0`, every target directory under scratch (deleted
afterwards). No timing (not required). Nothing pushed, no GitHub state touched.

### Where the check lives, and why not inside `new`

`EffectBankProcessBlock::new` and `EffectProcessBlock::new` run once per block on the render
thread, receive only the staged prefix of the window, and never see the window or the capacity; they
have 237 call sites. A check inside them could only refuse on the render thread, which gate 1
forbids. So the bound is stated once in the contract, as a typed function on each of the two block
types, and called at preparation where each window is allocated and both numbers are known:

- `effect-contract`: `ProcessBlockError::AutomationWindow` (new variant) and
  `EffectProcessBlock::check_automation_window(window, &PreparedEffectMetadata)` /
  `EffectBankProcessBlock::check_automation_window(window, &PreparedBankMetadata)`, both
  `window.len() == automation_capacity` through one private `check_window`. The docs state the
  #1004 reason: a larger window can stage a twin across the effects' per-span
  `span_index < automation_capacity` cut-off; a smaller one drops admitted records.
- `rack::ConsoleEffectBankStage::new`: checks its window at bind and returns the typed
  `RackError::AutomationWindow` (new variant). Its window is sized from one `metadata()` read and
  checked against a fresh read.
- `graph::runtime::ConsoleEffect::new` (per-node console effect): checks its window at bind. The
  graph node builder is infallible by design, so a violation is a bind-time `expect`, the same
  treatment `stage_for` gives the rack's `validated width` (whose message now names the window too).
  Never on the render thread.
- `live.rs`: the vacuous `debug_assert!(staged <= staging.len())` is removed and replaced by a
  comment that names the preparation checks; `stage`'s docs state the window precondition.

### Findings 2 and 4

- `host-core/tests/symmetry_witness.rs`: `two_per_lane_writes_that_agree_still_decline_the_lane`
  now states the post-#1004 contract -- it drives EQ **targets**, which still decline (with the
  `[Left A, Both C, Right A]` reason), and says in bold that twin `Parameter` spans keep `LIVE` and
  must not be "restored" to a decline. Beside it, `twin_parameter_spans_keep_the_lane_and_a_lone_half_declines_it`
  (mono `compressor-bank-observation` fixture, raw `Parameter` records): three twin rides keep the
  census, a twin one ulp apart declines exactly its lane, a lone `Left` declines and its `Right` in
  the next drain does not re-earn it.
- Finding 4 is documented where it happens: the target-owner refusal arm of `stage` (a deferred
  one-channel record is never staged, the empty window pairs, `LIVE` survives; sound because the
  record reaches neither channel and `target_error` fails the block's render).

### Gate 2 tests

- `rack/tests/console_bank.rs`, `a_staging_window_larger_than_the_capacity_is_refused_at_bind`: a
  bank whose reported capacity falls by one per `metadata()` read hands the stage a window of
  exactly capacity + 1 against the capacity it enforces -> `Err(RackError::AutomationWindow)`; a
  stable bank binds. This exercises the production call site.
- `console-workload/tests/paired_spans.rs`,
  `every_launch_effect_refuses_a_staging_window_that_is_not_its_capacity`: all 8 registry effects'
  scalar metadata, and every one that banks at the native width (7), accept a window of exactly
  the capacity and refuse capacity + 1 and capacity - 1 with `ProcessBlockError::AutomationWindow`.

### Mutations (each applied, run with `--no-fail-fast` over the rack, console-workload and
host-core suites in dev, reverted with `git checkout`)

| # | mutation | result |
|---|---|---|
| K1 | `check_window` always `Ok` | RED: the registry-wide refusal test and the rack bind test |
| K2 | the check call deleted from `ConsoleEffectBankStage::new` | RED: `a_staging_window_larger_than_the_capacity_is_refused_at_bind` |
| K3 | rack window sized `capacity + 1` (check kept) | RED at bind: every console-workload bank/console test, 10 of 12 host-core `symmetry_witness` tests, 10 of 11 rack `console_bank` tests (typed `RackError::AutomationWindow`) |
| K4 | graph per-node window sized `capacity + 1` (check kept) | RED at bind: the per-node console arms (`the_scalar_console_effect_arm_maintains_its_own_live_terms`, `a_prepare_time_bypass_seeds_...`, the pinned ride's `Simd4` leg) |
| K5 | #1004's deferral removed (the pre-#1004 drain) | RED: `twin_parameter_spans_keep_the_lane_and_a_lone_half_declines_it` and 9 console-workload tests |
| K6 | `spans_pair` always `true` | RED: the host-core twin test (lone and near-twin halves) and 5 console-workload tests |

### Gates

- `cargo clippy --workspace --all-targets -- -D warnings`: clean. `cargo fmt --all -- --check`: clean.
- `cargo test --no-fail-fast -p effect-contract -p effect-compiler -p host-core -p rack -p graph
  -p console-workload`, dev and release: 546 passed, 0 failed, 5 ignored in each (52 binaries). All
  #1004 gates are in it (`effect-contract` and `console-workload` `paired_spans`, the pinned mixed
  ride `9242f149…`).
- `bench console --preflight` (`console_mixing_automation`), base `b2f312c2` and change: PASS on
  both, output **byte-identical** (8 controls, 7 arms, every digest and every
  `bank_collapse_counters` equal).
- Standing digests: the 64-block digest of all 14 native session rows (`gain_pan_profile`
  `digests`) identical on base and change.
- `scripts/run-wasm-gates.sh`: ok (native + wasm scalar + wasm simd128 + V8 EQ loops; 142 cases,
  358 comparisons, 0 mismatches).
- `check-realtime-policy.sh` (57 marked regions), `check-env-vocabulary.sh`,
  `check-workspace-policy.sh`, `check-effect-contract.sh`, `check-rack-policy.sh`,
  `check-graph-policy.sh`, `check-host-core-policy.sh`: all ok.
- Render stays allocation-free: `check-web-audioworklet-callgraph.py` on delivery-recipe
  `host_web.wasm` builds of base and change gives **identical reports** (render closure 8, one trap
  owner `render_inner`, kernel roster unchanged). Artifact 3,400,108 -> 3,400,981 bytes (+873, the
  bind-time checks and their messages).

### For the verifier

- Gate 1 names the block constructors; the check is on the block types but not inside `new`, for
  the reason above. The two production window owners call it at bind.
- The graph arm refuses by bind-time `expect`, not a `Result`: threading a `Result` through
  `node_kind`/`build_op` would change the infallible graph bind API, which this slice does not own.
- `ProcessBlockError` and `RackError` each gain a variant; no exhaustive `match` on either exists in
  the tree.

## Sol attempt 1 verdict: PASS

Sol, 2026-09-27. I judged `6c3e29e9` merged onto the current batch head `f4cc12db`, which adds
#1002 and its renumbering gate after the branch's base `b2f312c2`. The merge is clean and was made
in scratch only. The base for the artifact and digest comparisons is `f4cc12db`, built from a
`git archive`. Host AMD EPYC 7313P, `CARGO_INCREMENTAL=0`. Nothing was timed or pushed, and all
scratch and target directories were deleted.

### The questions

1. **The panic is unreachable, and I accept it.**
   * **It cannot fire.** `graph::runtime::ConsoleEffect::new` allocates the window as
     `vec![_; effect.metadata.automation_capacity as usize]` and checks it against that same
     field, three lines later, with nothing in between. So no session, host, restore or re-plan
     can make it fire. Only an edit to the sizing line can, which is K4.
   * **It never runs on the render thread.** It runs in `RuntimeParts::node_kind`, during plan
     construction (`graph/src/lib.rs:2559`). On the web host that is boot, on the worklet thread
     but outside `process()`.
   * **A panic is the right tool here.** The same builder already uses
     `.expect("validated width")` for a bind invariant. Making the infallible graph bind API
     return a `Result` would add an error path that cannot be taken.
2. **Coverage.** The window has exactly two production owners, the rack's
   `ConsoleEffectBankStage::new` and graph's `ConsoleEffect::new`, and both now check it.
   * **Hosts.** Every host that attaches an effect console reaches those owners through host-core
     `prepare`, then `effect-compiler`, then `graph-compiler`. That covers the web host
     (`control_queue_depth`), the builtin batch endpoint and the linked-limiter session. The C ABI,
     the native host and the mobile host attach no console.
   * **Re-plans.** A plan swap or re-plan binds a changed cohort through `stage_for`, which is the
     rack constructor. A reused bank keeps a stage that was already checked.
   * **Restore.** A restore writes state payloads only, and never touches a window.
3. **The finding-1 scenario is now impossible, in two independent ways.**
   * **The bind check.** A window of capacity + 1 is refused at bind by either owner. I reproduced
     K2 as red myself.
   * **The queue bound.** A drain stages at most the number of records it pops. Preparation sizes
     every production queue to `depth.min(capacity)` (`effect-compiler/src/prepare.rs:2066`). So
     `staged <= automation_capacity` holds even for an oversized window.
4. **Gates, on the merge.**
   * **Tests.** `effect-contract`, `effect-compiler`, `host-core`, `rack`, `graph`,
     `console-workload` and `host-web` all pass: 752 passed, 0 failed and 7 ignored in both dev
     and release.
   * **Lint.** Clippy (workspace, all targets, `-D warnings`) and fmt are clean.
   * **Policy scripts.** Ten pass: realtime (57 regions), workspace, effect-contract, rack,
     env-vocabulary, host-core, graph, effect-runtime, lane and session.
   * **`run-wasm-gates.sh`.** It passes, including the V8 spill gate.
   * **Web artifact.** `check-web-audioworklet.sh` reports are identical for base and change
     (render closure 8, one trap owner `render_inner`). The artifact grows by +873 bytes, the same
     delta as the evidence reports.
   * **Standing digests.** All 17 64-block console digests are identical on base and change.
   * **My fuzzes.**
     * Drain model: 1.5M runs in release, including windows up to queue + 7, all sound.
     * Rendered bank differential: 2,000 scenarios against forced dual, 946 keeping the collapse
       throughout and 1,054 retiring it, every block bit-exact.
   * **Findings 2 and 4** are closed as the evidence says.

### Findings

1. **Info: both checks guard an edit, not an input.**
   * Each check compares a window against the capacity it was just sized from. The rack reads
     `metadata()` a second time for the check, so only a non-deterministic `metadata()` or an edit
     to the sizing line can make it fire.
   * `EffectControlLane::stage` still accepts any slice, so a future third owner of a window is
     bound only by the new doc precondition.
   * **Failure scenario.** A new drain site allocates its own window and its own queue deeper than
     the capacity, and skips the check. Nothing in the tree does this today.
   * **Structural fix, if wanted.** Let the lane own its window, sized from the metadata at
     construction.
2. **Info: gate 1's wording was not followable, and the deviation is correct.** The
   `EffectProcessBlock::new` and `EffectBankProcessBlock::new` constructors run per block, on the
   render thread, and never see the window. The typed check at bind meets the gate's intent.
