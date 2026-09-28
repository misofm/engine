# Bind only the changed cohorts when the whole-mono-cohort guard compares plans

## Product outcome

#971's guard binds both the trial plan and the re-plan to compare the banks they actually produce, so both plans' banks are alive at once. The #971 attempt-2 verification measured the cost at #962's scale (debug `compile_with_builtins`, 65,537 mono tracks, one strand, under the timing lock):

| effect per track | case | base | #971 | change |
|---|---|---:|---:|---:|
| `simd1: [comp]` | refused | 48.6 s | 50.3 s | +3.4 % |
| `simd1: [comp]` | one track in 9 stereo, kept | 48.7 s | 50.8 s | +4.3 % |
| `simd2: [limiter]` | refused | 48.5 s | 53.4 s | +10.2 % |

With the limiter, peak RSS rises from 3.21 GB to 3.99 GB (+24 %). #962's scale gates never take this path because their session has no effects. The verifier found a cheaper exact count: groups the move does not change bind to the same banks in both plans, so only the changed groups need binding for the comparison.

## Smallest closable slice

Bind the re-plan's changed groups only, reuse the trial plan's banks for unchanged groups, and compare the bound counts over the changed groups. The accepted plan must be exactly the plan #971 accepts today, with the same banks.

**Out of scope:** evaluating moves per independent component instead of all at once (#971 attempt-1 low finding 3), and changing what the guard decides.

## Objective gates

1. **Same decision.** On the #971 randomized probe (the verifier's 4,220 renders: 400 random mixed mono and stereo sessions plus the 22 named shapes) and the #971 gate sessions, the accepted plan, its pools and its bank count equal #971's at 8 and 4 lanes; rendered digests are unchanged.
2. **Scale.** The three table rows above, re-measured under the timing lock against the batch head: compile time within 1 % of base (pre-#971), and limiter peak RSS within 5 % of base.
3. **Mutations** (each alone, red): reusing a trial bank for a changed group; skipping a changed group in the count.
4. Every standing console digest unchanged; clippy and fmt pass.

## Attempt 1 evidence

Implementer attempt 1, 2026-09-27. The branch `codex/1002-replan-binds-changed-cohorts` starts
from the #1001 branch.
* Code commit: `1a5cc881`.
* Then the batch head `486e62a0` (which carries #1001's merge `4cf2a104` and Sol's verdict) was
  merged in (`a050694f`).
* Then Sol's #1001 doc finding was fixed in the doc-only commit `13b9ae27`. It comes after the
  code commit because the code was already committed.

Host x86_64 (`x86-64-v3`), `CARGO_INCREMENTAL=0`. The four-lane legs use the research `--cfg
miso_native_simd4` lane hunk in their own target directories, applied for measurement only and
reverted.

### The change (`crates/graph-compiler/src/banks.rs`)

* `bind_planned_banks` becomes `bind_group_banks`, which binds one group into a `BoundGroup`: its
  banks, each paired with its report slot. The trial plan is bound group by group, as before.
* For the re-plan, each group finds the one trial group that could equal it, through its first
  member (a chain sits in exactly one group). It counts as unchanged only if it is fully equal to
  that group: `BankGroup` equality over level, rack, class, leader program, members in lane order,
  active mask and active slots.
* Only changed re-plan groups are bound. The move is kept iff
  `gained (banks of the changed re-plan groups) > lost (banks of the trial groups no re-plan group
  equals)`. Unchanged groups bind identically in both plans: bind reads only the group, the
  chains, the levels, the prepared entries, the width and the dispatch. So this is exactly
  #971's `replan_banks.len() > banks.len()`.
* On acceptance, the trial's banks for unchanged groups move into the re-plan's order, with
  `cohort.group` and the slot's `group` renumbered, and the fresh banks fill the changed groups.
  This gives the same bank order that binding the whole re-plan gave.
* #1001 is intact: an `Err` binding any changed group keeps the trial plan, its banks and the
  unmoved map. The fresh banks are dropped. The trial's banks are only taken once the move is
  accepted.

### Gate 1: the same decision

Sol's #971 probe source was no longer in the scratchpad, so I reconstructed it on the same design.
The reconstruction is a scratch integration test: `bank_levels.rs` plus a #971-flavoured generator
and 23 named shapes. It is not committed, and the scratchpad was deleted as instructed.

* **Random sessions.** 400 seeds:
  * 1-40, 64, 65 or 81 tracks;
  * a mono share of 10, 25, 50, 75 or 90%;
  * the symmetric mono strip in 3 of 4 seeds, otherwise the asymmetric intended strip;
  * template racks with dropped and inserted slots, or free racks;
  * per-side input delays, bypass, sidechains at every tap, and sends into submixes.
* **Named shapes:**
  * the dogfood layout: as a strip, all stereo, builtins only, and contiguous;
  * the odd-track session;
  * at W = 4 and 8: the amendment's 2W session, the guard session, the new as-many session,
    Sol's discriminating session, and both phantom sessions;
  * #966's mono consoles less 3 and less 9 EQs, the ragged soft-clip session and the realigning
    strips, each made mixed;
  * #970's reduced console, as is and mixed.
* **Rows.** Each model is compiled at Scalar, Simd4 and Simd8, then rendered unarmed and armed
  for 16 blocks: 2,115 rows per build. Each row records:
  * a hash of the rack plan's groups and of its bound slots;
  * a hash of every effect bank's ordered members;
  * the effect and builtin bank counts, the mono pool, `bank_shape`, the collapse counters and
    the route folds;
  * the rendered-bits digest.

| build | rows | #1002 against #971 (the batch's `banks.rs`, #1001 included) | rows where #971 moves tracks (against pre-#971) | digests against pre-#971 | models off their scalar row |
|---|---:|---:|---:|---:|---:|
| native 8 lanes | 2,115 | **0 differing rows** (every field) | 156 rows in 62 models | 0 differ | 0 |
| native 4 lanes | 2,115 | **0 differing rows** | 104 rows in 52 models | 0 differ | 0 |

One seed, `seed269:t81:m500:asym:free`, renders silence at every width, so its digests compare
nothing; its plan fields still compare. Every #971 and #1001 lib test also passes on #971's own
`banks.rs` (the batch before this change), so the gate sessions' decisions are #971's.

### Gate 2: scale

Debug `compile_with_builtins` on 65,537 tracks (Sol's harness, rewritten in scratch). Base is
`b90ce6f2`'s compile logic: the head tree with that commit's `banks.rs`, `compile.rs` and
`builtins-compiler/src/lib.rs`, which are the only files #971, #1001 and #1002 changed outside
tests and docs. Head is this branch.

Two holds of `flock -w 7200 <timing.lock>`, 12 minutes each. In each hold the three rows ran base
then head (round 1) or head then base (round 2), each process with `/usr/bin/time -v taskset -c
31` and two repetitions. Load average was 15 to 6.6 in round 1 and 8.8 to 4.2 in round 2,
uncontrolled; individual repetitions scatter by ±3%.

| effect per track | case | base, 4 reps (s) | head, 4 reps (s) | mean | min | peak RSS base → head |
|---|---|---|---|---:|---:|---|
| `simd1: [comp]` | one strand, refused | 49.82, 51.09, 48.27, 47.35 | 49.18, 47.84, 53.49, 47.56 | +0.78% | +0.43% | 2.082 → 2.127 GB (+2.1%) |
| `simd1: [comp]` | 1 in 9 stereo, kept (8191 → 8192 banks) | 49.35, 47.85, 48.48, 47.90 | 49.24, 48.30, 49.02, 47.83 | +0.42% | −0.05% | 2.089 → 2.129 GB (+1.9%) |
| `simd2: [limiter]` | one strand, refused | 48.36, 51.09, 48.27, 46.47 | 49.31, 48.12, 49.43, 46.66 | −0.33% | +0.43% | **3.400 → 3.370 GB (−0.9%)** |

Compile time is within 1% of base by mean and by minimum. The medians are −1.1%, +1.0% and
+0.8%. Limiter peak RSS is within 5% of base (−0.9%, against #971's measured +24%). The kept row
binds 8192 banks and the refused rows keep every mono track, as #971 does. #971's own figures
from its attempt-2 verdict (+3.4%, +4.3%, +10.2%) were not re-measured.

### Gate 3: mutations

Each mutation was applied alone to `banks.rs`, `cargo test -p graph-compiler --lib` was run at 8
lanes (89 tests), and the #971 and #1001 tests were run at 4 lanes (12 tests). The red sets were
the same at both widths.

| mutation | red |
|---|---|
| **reuse a trial bank for a changed group** (match on first member only) | `a_replan_that_fails_part_way_drops_its_banks`, `a_single_odd_track_no_longer_strands_a_pool_remainder`, `the_rule_not_the_bank_gain_check_picks_the_moved_tracks`, `the_mono_pool_keeps_whole_cohorts_on_the_dogfood_layout` |
| **skip a changed group in the count**: the first changed re-plan group in `gained` | `the_rule_not_the_bank_gain_check_picks_the_moved_tracks` |
| **skip a changed group in the count**: the last vanished trial group in `lost` | `a_move_that_binds_as_many_banks_as_it_loses_is_not_kept` (new) |
| skip the *first* vanished trial group in `lost` | **green: an equivalent mutant.** The first vanished trial group is always a stranded track's partial mono group, which binds nothing. The earliest `(level, rack)` the move changes is a moved track's earliest chain, and there the mono pool (class order first) holds that track's partial group, while the pool's earlier groups are untouched by its removal. So skipping it never changes the count. |
| `gained >= lost` | the new as-many gate, `a_move_that_binds_no_bank_is_not_kept`, `a_replan_that_fails_to_bind_keeps_the_unmoved_plan` |
| restore `?` on the changed-group bind (#1001) | both #1001 gates |

### Gate 4 and the rest

* **Standing digests.** All 17 `native_session_rows()`, 64 blocks, at `Simd8` and `Simd4`,
  compared on the batch's `banks.rs` (`4cf2a104`) and on head: shape, transposes, collapse,
  folds, redirects and SHA-256 are identical row for row at both widths.
* **Command results:**
  * `cargo fmt --all --check` and `cargo clippy --locked --workspace --all-targets -- -D warnings`
    (also with `--all-features`) pass, and so does rustdoc with `-D warnings`.
  * `cargo test -p graph-compiler` passes in dev (119) and in release (119) with
    `CARGO_PROFILE_RELEASE_PANIC=unwind` in its own target directory. That is the workaround for
    the effect-package collision, now #1008.
  * `cargo test -p console-workload` passes (39), and so does `scripts/check-env-vocabulary.sh`.
  * The whole lib at 4 lanes has 86 passed and 3 failed. The three are the scratch-cfg artefacts
    recorded at base in #971 and #1001.

### Deviations

1. **New gate:** `a_move_that_binds_as_many_banks_as_it_loses_is_not_kept`. It uses `W/2` mono
   `[eq, comp]`, `W` stereo `[eq]` and `1.5W` stereo `[comp]`, so 2 banks are gained against 2
   lost. No existing gate put the lost banks outside the first vanished group, so without it the
   `lost` count and the strict comparison were ungated. It passes on #971's code too.
2. **#1001's part-way session changed.** In `a_replan_that_fails_part_way_drops_its_banks`, the
   first re-plan group was identical to the trial's, so it is no longer rebound and `bound` would
   read 1. The mono track now sits at `ch{W-1}` and the marker on the last stereo track, so both
   re-plan groups change: the first binds before the second errors. The test still asserts the
   original claims: 2 bound, 1 alive, the trial's bank and pools, the mono track's lone builtin
   bank, and 0 alive after the drop. It passes on #971+#1001's code as well, and it is red under
   restored `?`.
3. `bind_group_banks` takes eight arguments under `#[allow(clippy::too_many_arguments)]`, with a
   reason; there is precedent in `parametric-eq` and `gate-expander`.
4. The #1001 doc finding (the #95 table cell and the #95 citation at the re-plan bind) is fixed
   in `13b9ae27`, a doc-only commit, as the coordinator asked.


## Sol attempt 1 verdict: PASS

Sol, 2026-09-27. I judged `d50eb9cd` merged onto the batch head `b03edde4`, as a detached
scratch merge `2408d63d` that is not kept. Host x86-64-v3, `CARGO_INCREMENTAL=0`. The four-lane
legs used the research `--cfg miso_native_simd4` hunk, applied for measurement only. The worktree
was left clean.

The change is sound. It makes exactly #971's decisions, with #971's banks and plan, and removes
#971's extra peak memory. The one finding is Low: renumbering the reused banks' group index is
correct but no test gates it.

### The questions

1. **Reuse soundness: sound.** The trial bank reused for a group has no state or configuration
   that a fresh bind of the same group in the re-plan would set differently. The reasons:
   * **What a bind reads.** `bind_group_banks` reads only the group, `chains`, `level_by_node`,
     the prepared entries, the width and the dispatch. None of them depends on the class map or on
     the rest of the plan.
   * **What "unchanged" means.** A group counts as unchanged only under full `BankGroup` equality
     (`banks.rs:296`): level, rack, class, the leader's program keys, the members in lane order,
     the active mask and the active slots. So a group whose members are the same but sit in
     different lanes, or that changed class or program, is rebound.
   * **Nothing else can differ.**
     * Scratch is `(width, quantum)` for every bank.
     * The `native_id` and the snapshot flag come from the factory.
     * The processor is fresh prepared state that has not rendered yet.
     * Collapse arming happens at runtime and is structural. It never reads the bank objects.
   * **The only group-dependent fields are renumbered.** They are `cohort.group` and
     `slot.group` (`:335-336`). Both are report fields, and the runtime no longer reads
     `cohort` (`crates/graph/src/lib.rs:924`).
   * **The probe confirms it.** Across 4,260 rows at 8 and 4 lanes, head equals #971 (the batch's
     `banks.rs`) in every field. The fields are:
     * hashes of the plan's `Debug`, `bound_slots`, the effect banks' ordered members and the
       builtin banks;
     * the effect and builtin bank counts, and the mono pool;
     * `bank_shape`, the collapse counters, the route folds, the transposes and the scatter
       redirects;
     * the digest.
2. **The equivalent mutant: the argument holds.** Pools are planned independently, and plan
   order is level, then rack, then class, with the mono pool first. So the earliest pool a move
   touches is the mono pool at the earliest `(level, rack)` that holds a moved track's chain.
   * Leaders chosen before the moved track's cohort are unchanged, and so are the chunks before
     its position. The first vanished trial group is therefore that track's group, which is
     partial (the track is stranded). A partial group binds nothing (`bindable_slot_members`
     refuses `!is_full()`), so skipping it never changes `lost`.
   * Measured: it is green in the lib suite at both widths, and matches head on every probe row.
   * A related weakening, "unchanged when the members alone are equal", is also green and matches
     on every probe row. Full equality is the conservative choice; keep it.
3. **The #1001 test change does not weaken coverage.** With the old layout, the re-plan's first
   group was identical to the trial's. Under reuse it is not rebound, so the error would come
   before any fresh bank existed, and the "part way" case would go unexercised.
   * The new layout (`ch{W-1}` mono, the marker on `ch{2W-1}`) changes both re-plan groups. The
     first binds (`bound` 2), then the second errors.
   * The test still asserts that 1 bank is alive, the trial's bank and pools, the mono track's
     lone post-input bank, and 0 banks alive after the drop.
   * The fallback frees every bank:
     * On an error, the collected `Result<Vec<_>>` drops what it had bound.
     * On a refusal, `fresh` drops at the end of its scope.
     * On an acceptance, the vanished trial groups drop with `trial`.
     * The trial's banks are moved out (`mem::take`) only after `gained > lost`.
   * Letting the error propagate turns both #1001 gates red at both widths.
4. **Reproduced.** My probe was deleted, so I rebuilt it. It covers 400 mixed sessions, the 22
   #971 shapes, the as-many session and a new multi-level `multi-{4,8}` session, rendered at
   Scalar, Simd4 and Simd8, unarmed and armed.

   | build | rows | head against #971 | head against pre-#971 (`b90ce6f2`'s three files) |
   |---|---:|---|---|
   | 8 lanes | 2,130 | **0 differing fields** | 70 rows moved in 28 models, 0 fewer banks, 0 moves without a gain, 0 digest differences |
   | 4 lanes | 2,130 | **0 differing fields** | 60 rows moved in 30 models, the same zeros |

   No render differs from the Scalar oracle.
5. **Scale: the limiter row (refused, 65,537 tracks, debug), under the lock.** Peak RSS
   reproduces: pre-#971 3.214 GB, **#1002 3.218 GB (+0.1%)**, #971 3.988 GB (+24%).
   * The compile times are pre-#971 59.5 / 56.8 s, #1002 53.3 / 73.3 s and #971 55.7 s.
   * They are noise: other builds held the load at 17 to 28, so I cannot confirm or refute
     the implementer's ±1% from this run.
   * On this row the only changed groups are the stranded one-member mono group and a
     one-member stereo group. Neither binds, so the second bind does no factory work.
6. **The doc commit is accurate.** The #95 table cell (`crates/effect-contract/src/lib.rs:1513`)
   now names the speculative re-plan exception. The `banks.rs` comment no longer credits #95 with
   the `Ok(None)` rule, and rests the exception on #95's own reasoning that a planner bug must not
   cost the user their session. That closes Sol's #1001 finding 1.

### Mutations (my driver; the whole lib suite at 8 lanes, the #971 and #1001 set at 4)

| mutation | red at 8 and 4 lanes |
|---|---|
| reuse by first member only | the part-way test, the odd-track test, the discriminating test and the dogfood test |
| skip the first changed group in `gained` | the discriminating test |
| skip the last vanished group in `lost` | the as-many gate |
| `gained >= lost` | the as-many gate, the no-bank gate, the #1001 keep-plan gate |
| let the re-plan error propagate | both #1001 gates |
| no move | the part-way test, the odd-track test, the discriminating test and the dogfood test |
| skip the first vanished group in `lost` | **green**: an equivalent mutant (question 2) |
| do not renumber `slot.group` | **green**, see the finding |
| do not renumber `cohort.group` | **green**, see the finding |

This matches the attempt-1 evidence on every row it reports.

### Gates

On the merged tree, these all pass:
* fmt;
* clippy `-D warnings` on the workspace, all targets and features;
* rustdoc `-D warnings`;
* `graph-compiler` 119, `effect-contract` 50, `host-core --all-features` 237, `console-workload`
  39, `capi` 36;
* the graph policy and determinism scripts (100/100) and the env-vocabulary script.

### Finding

**Low (gate gap): the renumbering of reused banks is ungated.** Dropping either assignment
(`banks.rs:335` or `:336`) passes every committed test.

* **It matters, because the index does shift.** My probe shows `bound_slots` changing on the
  dogfood layout at both widths, and on 22 of 2,000 random rows at 8 lanes.
* **The runtime is unaffected.** It never reads `cohort`.
* **The report is not.** `bound_slots_in`, `scalar_in` and `bound_groups_in` index
  `plan.groups[bound.group]`, and `tools/audit` and `tools/bench` read the report. A regression
  would therefore silently misattribute racks in the report, or index out of bounds.
* **Recommended gate** (validated in scratch: green at head, red under the `slot.group` mutation
  at 8 and 4 lanes):
  * **Where:** in `the_mono_pool_keeps_whole_cohorts_on_the_dogfood_layout` (`lib.rs:8550`), on
    the folded artifact, assert that every `bound` in `report().rack_cohorts.bound_slots` has
    `bound.members` (track ids, in order) equal to
    `plan.groups[bound.group].members.iter().flatten()`, with the same rack.
  * **For `cohort.group`, which has no public reader:** add a
    `debug_assert_eq!(bank.cohort.group as usize, slot.group)` over the final `(bank, slot)`
    pairs before the `unzip` (`banks.rs:345`). That ties it to the gated field.

  This can ride the batch merge or a one-line follow-up. It does not block.

## Follow-up: renumbering gate

This closes Sol's Low finding, that nothing checked the renumbering of reused banks. Branch
`codex/1002-renumber-gate`, from the batch head `16c66f73`.

* **Test.** `the_mono_pool_keeps_whole_cohorts_on_the_dogfood_layout` now asserts, on the folded
  artifact, that every `report().rack_cohorts.bound_slots` entry's members (track id and rack, in
  lane order) equal the members of `plan.groups[bound.group]`. The move shifts the group
  indices on this layout, so a stale index points at another group.
* **Debug assertion.** `bind_rack_banks_indexed` now has a `debug_assert!` over the final bound
  groups, before the `unzip`: every `slot.group` and every bank's `cohort.group` equals the
  group's final index. `cohort.group` has no public reader, so this is its gate.

Mutations, each alone, in `banks.rs`:

| mutation | dev, 8 lanes | dev, 4 lanes (scratch cfg) | release (`panic = "unwind"`) |
|---|---|---|---|
| drop `slot.group = index` | red: the dogfood, odd-track and discriminating tests, on the debug assertion | red, the same three | red: the dogfood test, "bound slot 0 of group 3 names that group's lanes" |
| drop `bank.cohort.group = cohort_group` | red, the same three | red, the same three | **green**: a debug assertion does not run in release, and nothing reads the field there |

Gates: `cargo fmt --all --check`; clippy `--workspace --all-targets -- -D warnings`, also with
`--all-features`; `cargo test -p graph-compiler` in dev (119) and in release (119,
`CARGO_PROFILE_RELEASE_PANIC=unwind`, the #1008 workaround). No rendered bit or plan changes:
the change adds one test assertion and one debug assertion.
