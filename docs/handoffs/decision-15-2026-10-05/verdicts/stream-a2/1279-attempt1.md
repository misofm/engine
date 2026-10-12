FAIL

# #1279 attempt 1 -- adversarial slice verdict

Range: the #1277 follow-ups `1080ccbc0` (parent `14d15fed6`) and #1279 attempt 1 `1080ccbc0..fab031fdb` (f5f61af3b,
8f897e6ab, bc267c2a5, 287510412, 3e1f74c2c, 7852f0a25, fab031fdb). I reviewed an export (`git archive fab031fdb`) under
`/tmp/claude-1002/v1279/`. I did not edit, build or commit in `/home/bl/misofm/wt-d15-a2`. Small evidence (step
summaries, mutation summary, probes, capi audit record): `/home/bl/misofm/submix-verdicts/1279-attempt1-evidence/`.

One MAJOR, two MINOR, five NIT. The MAJOR fails the attempt. The fix is small (a test feed and one oracle); the
implementation itself passed every gate and every mutation I applied to it.

## #1277 follow-ups (`1080ccbc0`): all three are fixed correctly

- **MINOR 1 (fused clamp).** `a_followed_mute_change_restarts_the_fused_stage_whole_at_{four,eight}_lanes` exists.
  Mutation M9 (`carries = [both, both]` replaced by `let _ = both;`): both new tests are red, the other 63 are green.
  Reverted: green.
- **MINOR 2 (record).** The attempt 3 record now states the per-family behaviour (fader/mute stage restarts with no
  mute retarget; split matrix stage carries and is retargeted; fused stage restarts whole). This agrees with the code.
- **NIT 3 (line width).** The two doc lines are rewrapped. No added line of `1080ccbc0` is over 100 columns.

## Findings

### MAJOR 1 -- gate 1 does not defend the gate/expander's carry, and the "does work" test claims it does

- **Where:** `crates/host-core/tests/successor_swap.rs:3658` (`bursty_feed`) and `:3825-3827`
  (`every_console_effect_does_work`, whose doc says "Each of the six effects changes the test signal, so a lane it
  fails to carry is heard").
- **Evidence (mutation M8, applied in the join so one console effect's lanes get no move):**

  | Effect whose lanes are not carried | `console_effect_state_survives_a_swap_at_eight_lanes` |
  | --- | --- |
  | parametric EQ | bits differ from block 9 |
  | compressor | bits differ from block 8 |
  | **gate/expander** | **bits identical in all 12 blocks**; only the restart-set assertion fails, because M8 also lists the strips |
  | transient shaper | bits differ from block 8 |
  | soft-clip | bits differ from block 8 |
  | true-peak limiter | bits differ from block 6 |

- **Why it matters:**
  - The spec requires each effect to do work, "the gate opening and closing", so that gate 1 can catch "an effect
    whose restore misses a derived word" (Test value). For the gate/expander, the swap block (frame 768) falls in
    the loud burst `500..1000`. The gate is fully open, and its state at the swap renders the same as its state at
    rest. A gate lane that is not moved gives the same bits (measured). A gate lane moved to the wrong lane, or
    restored with a missing word, very probably gives the same bits too, because every strip's gate is fully open
    at the swap (not measured). Gate 1 cannot see the first of these, and very probably not the others.
  - `every_console_effect_does_work` compares each one-slot session with a session with no console. This proves
    only that the effect changes the signal at some time. It does not prove that the effect's state at the swap is
    different from rest. The test's doc and the attempt record ("a session whose effects leave the signal alone
    would pass gate 1 vacuously") claim more than the test shows.
- **Mitigation (why this is not a BLOCKER):** the generic carry is the same code for all six effects, and the other
  five catch the shared defects. `crates/gate-expander/tests/state.rs`
  (`active_state_restore_continues_an_uninterrupted_donor`) covers the gate's own payload continuation. The gap is
  the engine-level acceptance gate for one of the six effects.
- **Fix (measured):**
  - Put the gate in release at the swap block. With the envelope loud over `0..300`, `500..740` and from `1100`,
    gate 1 stays green, and M8 for the gate goes red at block 10. The EQ also stays red (block 9). See
    `gate-release-feed-probe.txt` in the evidence directory.
  - Make the "does work" test prove that each effect's state matters at the swap: for each one-slot session,
    compare the swapped run with a successor that starts the slot at rest on the same PCM. The spliced-feed oracle
    of `a_restarted_slot_starts_at_rest_at_its_new_value` does this. Then a future feed change cannot make one
    effect vacuous again without a red test.
  - Do the same at four lanes. That leg runs only on AArch64 CI.

### MINOR 1 -- the restart set leaves out strips whose inserts restart; record it for #1324

- **Where:** `crates/host-core/src/prepare.rs:2878` (`join_chains`), `:2956` (`resolve_effect_carry`), and the rustdoc
  of `restarted_strips` (`:565-577`).
- **What happens:** an insert is a chain member and not an owner. Its lane, banked or per node, starts at rest on
  every swap until #1281/#1282. A strip whose only non-carried state is an insert is therefore not in
  `restarted_strips()`. On the C ABI, a console slot with no row (a `LiveControlEffectBankStage`) restarts its strip
  and lists it.
- **Ruling:** this is consistent with #1277 D6 ("the later carry slices add their families' restarts"). The rustdoc
  names exactly the owners it covers, so nothing misreports by its own definition.
- **What to change:** the record should state the asymmetry, and #1324 must know it: until #1281/#1282, a strip with
  a stateful insert restarts that insert at rest and is not ducked. Root to confirm the reading. Not a fix to the
  code.

### MINOR 2 -- `crates/capi/tests/resource_lifecycle.rs` is outside the authorized paths (deviation 1)

- The edit is correct (see the deviation rulings below). It needs root's named exception in the spec and in STREAMS
  before the batch merges.
- This is a process item. I do not fail the attempt on it.

### NIT 1 -- five added doc lines are over 100 columns

`crates/effect-contract/src/lib.rs:2535` (103), `:2592` (117), `crates/host-core/src/prepare.rs:574` (136),
`crates/host-core/tests/successor_swap.rs:4009` (101), and `effect-contract/src/lib.rs:2538` (101). rustfmt does not
wrap comments. The #1277 verdict raised the same NIT (NIT 3).

### NIT 2 -- the RESTORED comparison duplicates its single authority

- **Where:** `crates/rack/src/lib.rs:972` (`Some(left == right)`).
- **What to change:** call `effect_contract::payload_sections_agree(left, right)`. Its rule is the same, and it is the
  documented authority for the term (`symmetry.rs:374-386`). Its doc says "Off the render thread, at restore time".
  Now that a restore runs in the swap block, say that in its doc too.

### NIT 3 -- four-lane cases use `#[cfg]`, not `#[cfg_attr(..., ignore = "reason")]`

- The header of `scripts/run-aarch64-tests.sh` names the convention: a width-specific test "`#[ignore]`s the other
  width with its reason, which the test output reports". With `#[cfg]`, an x86 run does not list the four-lane
  cases at all.
- The file had the precedent before this slice (`#[cfg(target_feature = "avx2")]` on the eight-lane cases), so this
  is a NIT.

### NIT 4 -- nothing in production reads `carry_refusal_count`

- `graph::carry_refusal_count` is correct (see deviation 4). But the active plan lives in `RealtimePlanOwner`, and no
  host or owner surfaces the sum, so a refusal in production is not visible.
- An own payload is never refused (#1278), so the risk is low. Name the successor that will surface the sum.

### NIT 5 -- spec premise: a stale `DESIGNED` term cannot be reached through a session (for root)

- The spec's test value for gate 2 says that a stale `designed` cache turns "one of gates 1-2" red. Mutations M11
  (witness not refreshed) and M12 (`DESIGNED` kept from bind) leave all 65 `successor_swap` tests green.
- **Why:** by D1, a carried lane's parameters are bit-equal, so the restored designed words equal the bound ones.
  The `RESTORED` term is also covered by the agreement flag (M6 is red).
- The rack unit tests are the only defence, and they catch both mutations. This is correct: the attempt's design
  holds, and only the spec sentence is wrong.

## Rulings on the implementer's deviations

1. **The capi oracle line (`7852f0a25`): correct. Acceptable with root's named exception (MINOR 2).**
   - `capi_epoch_terms` adds `prepared.report.inventory_retained_bytes` once (`resource_lifecycle.rs:759`), taken
     from a fresh preparation. The C ABI's live prospective epoch keeps the successor's inventory, so replacing
     one term with `successor.inventory_retained_bytes` is the exact formula. Before this slice it held only
     because the two were equal.
   - The restart set is right for that session. `scratch_session` gives every track a console soft-clip, and capi
     prepares with `HostLiveLanes { effects: true }`. Every console lane is then a `LiveControlEffectBankStage`
     with no inventory row. By D1 the slot does not carry and each strip present in both plans restarts. Its
     console state really does restart at rest. The added muted track is not listed.
   - `cargo test -p capi`: 103 passed.
2. **Four-lane cases on `neon`/`simd128` only: the claim is true, and CI covers it natively.**
   - `compressor/src/lib.rs:1008-1010` (the same rule holds for every banked effect): a bank whose width is not
     `Backend::current().width()` returns `Ok(None)`. A console slot must bank, so preparation fails with
     `console.slot.unbanked` (`graph-compiler/src/tests/console_banking.rs:737`).
   - `qualification.yml`'s `aarch64-debug` job runs `scripts/run-aarch64-tests.sh debug` on `ubuntu-24.04-arm`. It
     runs `cargo test --all-targets` over the product crates, and `host-core` is one of them (`product-crates.sh`).
     The features include `host-core/test-support` and `engine/realtime-audit`. No known-defect row skips these
     tests (the debug leg has no rows).
   - So gate 1 and the other four-lane cases run natively in CI on the full route. I checked that they compile for
     `aarch64-unknown-linux-gnu` with `-D warnings` (`cargo check --tests -p host-core -p rack -p graph`, rc 0).
     As the rules require, I did not run them under emulation. See NIT 3 for the form of the gating.
3. **Gate 2 on EQ, compressor and limiter only: the claim is true. This is a spec premise error for root.**
   - Gate/expander, soft-clip and transient shaper do not override `supports_mono_collapse` or
     `lane_channel_symmetry`, so they get the trait defaults `false` (`effect-contract/src/lib.rs:2457-2474`).
   - `BankChain::collapse_prefix_of` (`rack/src/lib.rs:2324-2344`) needs every upstream slot to support collapse.
     If one slot does not, the prefix is 0 and the chain never collapses.
   - So "the same console" of gate 2 cannot be met. The subset is the strongest form of the gate that is possible.
4. **`carry_refusal_count` as a free function: acceptable, and it meets D6.**
   - `PreparedRenderPlan` is an `engine` type, so `graph` cannot give it an inherent method. A free function over
     the plan follows `plan_identity` and `carry_program_retained_bytes`.
   - It is control-side: it needs `&mut PreparedRenderPlan`, and it sums with `saturating_add`
     (`graph/src/runtime.rs:4385`).
   - The render-side `carry_lane_from` cannot panic. Every index is checked by `can_carry_lane_from` inside the
     closure, the splits use `split_at_mut_checked` and `get_mut`, and the count uses `saturating_add` (M13 is red).
   - NIT 4 covers the missing production reader.
5. **C ABI: nothing carries until #1280, and every strip with a console slot restarts: consistent, and it does not
   misreport.**
   - D2 says that this slice emits no retarget and that live-controlled lanes are #1280's. That is a non-goal here.
   - D1's "otherwise" branch (no matching layout, because there is no row) restarts the owner and lists its strip.
     This is the truth: those lanes start at rest.
   - `a_console_slot_with_live_controls_restarts_its_strip_*` covers it (M15 is red).
   - The carry program still exists, because `may_carry` reads only the model. It installs no effect move and
     `adopt_predecessor` reports `Carried`. This is harmless, and the double-live oracle charges its bytes.
6. **`RESTORED` from the byte equality of L/R: correct against `ChannelSymmetryWitness`.**
   - `symmetry.rs:36,171` defines `RESTORED` as "set by restore: the left and right payload sections compared
     byte-equal; cleared by a restore whose sections differ". `lane_witness(designed, restored)` sets exactly that
     term, and `DESIGNED` is re-read from the bank after the restore.
   - D4 holds: the payload is taken after `disengage_for_carry`, and `disengage_collapse` desymmetrizes every prefix
     stage, so a collapsed predecessor's sections are equal and the successor keeps its collapse. Gate 2 shows
     this: the successor collapsed on every block after the swap with 0 agreement proofs, and it was bit-identical
     both free and forced dual.
   - D5 holds: the cache's rustdoc and the two "no engine path snapshots a bound bank" docs are updated, and no
     stale statement remains.
   - Only the rack tests see a defect here (NIT 5, NIT 2).

## Other checks

- **D3 move mode:**
  - At the swap block, `can_carry_effect_lanes` checks every move first (all or nothing).
  - `carry_effect_lanes` calls `disengage_for_carry` once per predecessor unit (the moves are sorted by predecessor
    location, and the call is idempotent after the input carry). It snapshots into one scratch and restores into the
    successor lane.
  - `install_effect_carry` preallocates the scratch at the largest carried `payload_bytes()` (`state_sizes.total()`).
    It is charged against the named-allocation cap and, with the move table, against the graph cap, and both
    checks come before the install.
  - Gate 4 measured 0 allocations, reallocations and frees on the allocator counters and on the render audit. M16
    (an allocation in `carry_effect_lanes`) aborts the test binary under the render audit, and gate 4 alone also
    aborts.
- **The strip's new lane:** gate 1 adds `a-muted`, which sorts first. M2 (successor location = predecessor location)
  is red in gates 1, 2, 3, 4, the insert case, the removed-slot case, and #1277's
  `a_strip_behind_an_effect_still_carries`.
- **D1 chain-change rule:** a session slot added, removed or moved, or an insert added, removed, reordered or changed
  in identity, restarts the strip. M4 is red in the removed-slot and added-insert cases. M3 (carry despite a changed
  value) is red in gate 3. M14 (layout guard dropped) is red in `a_lane_of_another_prepared_layout_is_not_moved`.
- **D6:** the render path never panics; see deviation 4.
- **Realtime floors:**
  - `check-realtime-policy.sh` measured exactly 112 regions in 27 files. The one new region is in `rack`. The new
    `graph` functions sit inside regions that already existed, and the control-side functions
    (`bank_effect_lanes`, `effect_lane_layout`, `carry_refusal_count`) are outside them.
  - The self-test pad (`1 x 30 + 13 x 3` = 69) and the floor message match. This is the STREAMS standing exception.
- **Submix strips (probe, not committed):**
  - A submix `bus` with the same console carries its console lanes: 60 rows, the restart set empty, bit-identical,
    for `[eq, comp]`, `[eq]`, `[comp]` and `[eq, comp, gate, transient]`. A changed bus parameter lists exactly
    `["bus"]`.
  - With a latent slot (soft-clip 31 samples, limiter), the route through the submix needs compensation lines, and
    they restart. That is #1283/#1284 scope, not this slice.
- **The acked-batch question: can an ack ever precede a drop?** No. This slice adds no queue. The effect-carry
  install and its cap checks come before the retarget writes and the stamp (`prepare.rs:2170-2176` before `:2261`).
  A refused restore at the swap block is D6's counted defect path, not a dropped command.
- **Person-role wording:** the slice adds none (I searched the added lines for producer, fan, listener and similar).
- **Version suffixes:** the new names (`EffectLaneLayout`, `GraphEffectMove`, `GraphEffectLane`,
  `EffectLaneInventoryRow`) have no version suffix.

## Test value (new tests)

- `console_effect_state_survives_a_swap_at_*` (gate 1): catches a console lane restored to its old lane index or not
  moved, and a join that skips a slot, for EQ, compressor, transient shaper, soft-clip and limiter. **It does not
  catch these for the gate/expander (MAJOR 1).**
- `every_console_effect_does_work_at_*`: catches a console whose effects leave the signal unchanged. It does not
  show that each effect's state at the swap differs from rest (MAJOR 1).
- `the_effect_carry_allocates_and_frees_nothing_at_*` (gate 4): catches an allocation or a free in the swap block's
  effect carry (M16 aborts it).
- `console_chains_keep_collapsing_across_a_swap_at_*` (gate 2): catches a carry that does not disengage a collapsed
  predecessor before it snapshots (M5: red only in the inputs-restart case, which no other test catches), and a
  successor that loses its collapse.
- `a_chain_that_could_not_collapse_hands_over_no_agreement_at_*`: catches a successor chain that does not AND in the
  predecessor chain's agreement through the effect carry (M6: the only red test).
- `a_prepared_change_to_one_slot_restarts_only_its_strip_at_*` (gate 3, first case, with the exact at-rest oracle):
  catches a carry of a slot whose prepared value changed (M3: the only red test), and a restart set that misses or
  over-lists.
- `an_added_insert_restarts_its_strip_and_its_console_slots_carry_at_*` and
  `a_removed_console_slot_restarts_every_strip_at_*`: catch a join that ignores a chain change (M4: these two only).
- `a_console_slot_with_live_controls_restarts_its_strip_at_*`: catches a join that does not restart a slot with no
  row (M15: red, together with gate 3).
- `prepare::tests::a_lane_of_another_prepared_layout_is_not_moved`: catches a join without the layout guard (M14:
  the only red test; no session can reach it).
- rack `effect_lane_carry_tests` (6): catch a witness that is not recomputed (M10, M11, M12), a refusal that is not
  counted or that wraps (M13), and a lane-index or layout guard defect.
- `a_followed_mute_change_restarts_the_fused_stage_whole_at_*` (#1277 follow-up): catches a fused stage whose
  matrix half carries while its fader half restarts (M9: these two only).

## Mutations re-done (scratch copy `/tmp/claude-1002/v1279/gittree`, each reverted; summary in evidence)

| Mutation | Red |
| --- | --- |
| M1 effect carry moves nothing at the swap block | gates 1, 2, 3 (first case), insert case, no-agreement case |
| M2 successor location = predecessor location | 7 tests (gates 1-4, insert, removed slot, #1277 strip-behind-effect) |
| M3 carry despite a changed value | `a_prepared_change_to_one_slot_*` only |
| M4 chain change ignored | removed-slot case and insert case only |
| M5 effect carry does not disengage the predecessor chain | gate 2 (inputs-restart case) only |
| M6 successor chain does not inherit agreement | `a_chain_that_could_not_collapse_*` only |
| M8 one effect's lanes not moved (×6) | five effects red on bits; **gate/expander green on bits** (MAJOR 1) |
| M9 fused clamp removed (#1277) | the two fused followed-mute tests only |
| M10 `RESTORED` always true | rack: two witness tests; successor_swap green |
| M11 witness not refreshed | rack: two witness tests; successor_swap green (NIT 5) |
| M12 `DESIGNED` kept from bind | rack: `recomputes_the_designed_term`; successor_swap green (NIT 5) |
| M13 refusal `wrapping_add` | rack: `a_refused_restore_*` |
| M14 layout guard dropped from the join | `a_lane_of_another_prepared_layout_is_not_moved` only |
| M15 missing row does not restart | live-controls case and gate 3 |
| M16 allocation in `carry_effect_lanes` | binary aborts (render audit); gate 4 alone aborts |

## Gates run (all from the export, toolchain 1.97.1, x86-64-v3; all pass)

- **`lint` job, all steps in workflow order:**
  - fetch, fmt, clippy `--all-targets --all-features -D warnings`, Documentation
    (`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`).
  - Workspace policy and its mutations, session policy, env vocabulary, bench policy.
  - Test-support CI policy and its mutations, the scalar-oracle self-test.
  - Script reachability and its mutations: the plain export has no `.git`, so I ran this step in a git-initialised
    copy of the same export (120 files, 20 cases).
  - The console benchmark fixture, the builtins fixture-manifest mutation tests, the bench preconditions.
  - Host-core and protocol policies. Realtime policy (112 regions in 27 files), audit-leak and artifact-evidence-leak.
  - The trace validator, lane policy, the unfused seal, rack/builtins/graph policy (including `check-rack-policy`),
    effect-runtime policy and fixtures.
  - Conformance boundaries, the PEQ render contract, the release-shape self-test, npm publish modes, the stem store,
    stem identity.
  - The two sub-v3 refusals and the AVX2+FMA probe.
- **`test-release` job, all steps:** lane, math and wasm-gates in release, `filter_liveness`, `tail_contract`,
  `ramp_endpoint` (8 crates), `designer_total --ignored`, M3's FMA cfg, the loom leg
  (`RUSTFLAGS='--cfg loom' cargo test --release -p engine --lib spsc_loom`), and M1 and F1 exhaustive.
- **Spec gate 5:**
  - `cargo test --locked -p rack -p graph -p host-core` with the test-support features: 459 passed.
  - `cargo build --locked --release -p audit` and `trace-graph-audit.sh` (1,000,000 blocks).
  - `cargo test --locked -p console-workload`.
  - The policy, clippy and doc steps above.
- **Also run:**
  - `cargo test --locked -p capi`: 103 passed.
  - `check-capi-abi.sh` and `--self-test`.
  - `audit capi`: 100000 calls, with 0 allocations, deallocations, locks, syscalls and violations.
  - `trace-builtins-audit.sh` and `trace-builtins-graph-audit.sh`.
  - The builtins manifest digest (`check-builtins-fixtures.sh . target/release/audit`) and
    `check-console-fixtures.sh`.
  - `check-cross-targets.sh`.
  - The CI workspace debug test command (test-debug-a, with `engine/realtime-audit`): 1589 passed, 0 failed.
- **Worklet chain:**
  - `build-web-audioworklet.sh --named-twin` with the strip-names check.
  - `check-web-audioworklet.sh --without-metadata-regeneration` and `check-browser-expected-resources.py --artifacts`.
  - `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh` with a private TMPDIR, which was empty afterwards.
  - The V8 spill gate and its self-test.
- **Not run:** AArch64 tests (CI only, as the rules require). The four-lane cases compile for
  `aarch64-unknown-linux-gnu` with `-D warnings`.
