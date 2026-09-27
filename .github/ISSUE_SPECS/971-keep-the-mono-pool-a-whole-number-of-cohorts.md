# Keep the mono pool a whole number of cohorts


Filed from the dual-mono research (`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md`). Class A (regroups lanes; no lane's arithmetic changes). Base
`6ca203f8`. Depends on: "Arm the mono collapse only on chains that gather the track input".

## Problem

The cohort planner pools collapse-eligible tracks apart from the rest (`CohortPoolClass`,
`crates/rack-compiler/src/lib.rs:48-102`; derived once in
`crates/graph-compiler/src/compile.rs:433-444`). Effect banks bind only **full** groups (#96 F7),
so each pool whose size is not a multiple of the lane width strands its tail: those tracks' effects
render per node, in scalar code. `a_single_odd_track_strands_both_pools_remainders`
(`crates/graph-compiler/src/lib.rs:8307-8331`) documents it.

Measured on the dogfood first-listen session (81 tracks, 18 dual mono folded to `(0, 0)`, every
track given the standing EQ + compressor + limiter strip; `DUAL-MONO.md` §4.3-4.4): today's
pooling makes the mix **33% slower at 8 lanes** (169 to 225 µs) and 1% slower at 4 lanes, almost
all of it stranding (about 65 µs at 8 lanes). A prototype that keeps the mono pool a whole number
of cohorts makes the same session **5.3% faster at 8 lanes and 7.6% faster at 4 lanes**, with
digests unchanged.

## Outcome

Within each group of tracks that would share a cohort program **with at least one effect slot**,
keep `floor(n_mono / W) * W` mono-class tracks (the first in normalized track order) in the mono
pool and move the rest to the stereo pool, where they render dual. Groups whose program is builtins
only are left alone: builtin banks pad a partial cohort, so the mono remainder still collapses
there (measured: on the builtins-only dogfood mix at 4 lanes, demoting costs 0.7 µs).

## Read first

* `crates/builtins-compiler/src/lib.rs:3996-4060`: `SessionPoolClasses` (`from_session`,
  `conjoin`, `class_of`). Both planners read this one object; the demotion must happen in it,
  before either planner runs, or the two planners' lane sets diverge and every chain merge declines
  silently (`CohortPoolClass::of_prepare_witness` doc).
* `crates/graph-compiler/src/compile.rs:424-455` and `bind_rack_banks_indexed`
  (`crates/graph-compiler/src/banks.rs`) for how candidates are grouped (level, rack, program).
* The prototype (global rather than per group, so only valid for a one-program session):
  `research_demote_mono_remainder` in `docs/handoffs/dual-mono-2026-09-27/dual-mono-prototypes.patch`.

## Authorized paths

`crates/builtins-compiler/src/lib.rs` (`SessionPoolClasses`), `crates/graph-compiler/src/compile.rs`,
`crates/graph-compiler/src/lib.rs` tests, this issue's spec.

## Design notes

* Demote by clearing the class only (for example `SOURCE` in the pool map); never touch the
  structural join in `host-core` (`prepare.rs:1641-1646`). A chain holding a demoted track together
  with stereo tracks is not armed; a chain of only demoted tracks is armed and collapses correctly.
* The group key must be what makes two tracks share a cohort in both planners (dependency level and
  the ordered effect program of `simd1`, `dynamic`, `simd2`).

## Gates

* New test: the 64-track mono fixture (`fixtures/session/v1/console-sixty-four-track-mono.json`)
  cloned to 81 tracks, with `right_source_channel = 1` on every track except positions
  0, 4, 10, 18, 19, 20, 22, 35, 36, 38, 40, 42, 43, 44, 53, 58, 60, 66 (the dogfood layout). At the
  host width: `prepared_bank_count()` equals the all-stereo version's (30 at 8 lanes, 60 at 4);
  rendered digest equals today's pooling's; `bank_collapse_counters()[1]` is 2 at 8 lanes (4 at 4).
* Update `a_single_odd_track_strands_both_pools_remainders`: with 63 mono and 1 stereo track the
  pools become 56 and 8, so nothing strands and the bank count equals the unsplit session's. Keep
  its digest assertion.
* Coordinate with #969 ("Pin the input-section symmetry witness in the pool-class grouping"): its
  gate counts stranded banks (21 against 24) on exactly this one-odd-track shape, which this issue
  removes. Land #969 first and re-point its assertion at the class map, or fold it in here.
* Unchanged: `cargo test -p console-workload --test chain_shape`, `-p graph-compiler`,
  `-p host-core --test symmetry_witness`.
* `cargo fmt --check`, clippy `-D warnings` on the touched crates.

## Non-goals

Partial effect banks (#96 F7), the route-fold conflict (separate ruling), detection.


## Attempt 1 evidence

Implementer attempt 1, 2026-09-27. Branch `codex/971-whole-mono-cohorts`, base `8eebf17b` (the
local optimisation batch, carrying #970's arming and #966's cross-level unbinding); implementation
commit `6a0689cd`. Host x86_64 (`x86-64-v3`), `CARGO_INCREMENTAL=0`. The verification amendment
overrides the body and this record follows it. Four-lane results come from the research's
`--cfg miso_native_simd4` lane hunk, applied only for measurement in its own target directory
(`target/simd4`) and then reverted.

### The rule

* `bind_rack_banks_indexed` (`crates/graph-compiler/src/banks.rs`) forms the rack plan as before;
  that plan is the trial. `stranded_mono_tracks` returns each track that sits in at least one
  effect group when every group it sits in is a **partial group of the mono pool**. A track in no
  effect group (builtins only, or every chain on the per-node path) is never moved, so a
  builtins-only remainder keeps its padded, collapsing builtin bank.
* If any track is stranded, a clone of the class map moves those tracks with the new
  `SessionPoolClasses::pool_as_stereo`, every candidate's class is re-read, and
  `plan_bank_groups` runs again.
* **The move is kept only if the re-plan binds more effect slots than the trial** (deviation 1).
  Only then is the map replaced (`*classes = demoted`), and that happens before `compile.rs` hands
  the same map to the builtin-stage planner, so both planners pool a moved track alike.
* `bindable_slot_members` is the binder's own slot test (full group, not identity everywhere,
  every lane active, every member at one level: #96 F7 and #966), extracted unchanged. The binder
  and the guard's `bindable_slot_count` both call it.
* `SessionPoolClasses` (`crates/builtins-compiler/src/lib.rs`) records the move as a
  `pooled_as_stereo` set beside the witness, and `class_of` answers `Stereo` for those tracks
  (deviation 2). `classes()` reports through `class_of`. host-core's structural arming join
  (`prepare.rs`) is untouched, and so is the render path.

### Deviations

1. **Bank-gain guard** (not in the brief or the amendment). The amendment's rule alone can cost a
   bank. In `the_mono_remainder_stays_when_moving_it_binds_no_more_banks`, `2W` stereo tracks carry
   a one-slot EQ or compressor chain and `W/2` mono tracks carry both. Once moved, the mono
   tracks' two-slot program leads a single stereo cohort over all of them, the EQ and compressor
   lanes mix, and the session binds **1 bank instead of 2** (mutation `noguard`, below). The guard
   only ever cancels a move, so it is strictly more conservative than the amendment's rule. On the
   dogfood, odd-track and 16-track sessions it cancels nothing.
2. The brief says "for example `SOURCE`". I kept the move out of the witness and in a separate set
   instead, so the witness stays a statement of fact about the source.
3. `a_single_odd_track_strands_both_pools_remainders` is renamed
   `a_single_odd_track_no_longer_strands_a_pool_remainder`, because the old name states the
   opposite of what the test now asserts. #969's spec cites the old name.
4. "Rendered digest equals today's pooling's" is asserted in the test as bit equality with the
   all-stereo session (same feed) and with the bank-free registry. Equality with today's pooling
   is measured directly: it holds in the A/B (`58a7dc2477b23056`, every arm) and in the scratch
   probe (`f138f2c92c06cae6` before and after).

### Gates

The new and updated tests are in `crates/graph-compiler/src/lib.rs`. Each is width-generic and
runs at the host width.

| test | 8 lanes, after (before) | 4 lanes (scratch cfg), after (before) |
|---|---|---|
| `the_mono_pool_keeps_whole_cohorts_on_the_dogfood_layout`: mono fixture cloned to 81, stereo except the 18 dogfood positions | banks **30** = all-stereo 30 (27); `bank_shape` **[12,63]** = all-stereo (**[13,60]**); mono pool = first 16 in track order; `collapse` [24,**2**] over 12 blocks ([36,3]); PCM = all-stereo = bank-free registry; builtins-only arm: 9 mono builtin banks, `collapse[1]` 3 | banks **60** = all-stereo (57); shape **[22,123]** ([23,120]); `collapse[1]` **4** (5); builtins-only `collapse[1]` 5 |
| `a_mono_track_that_fills_a_cohort_is_not_pooled_as_stereo`: the amendment's 16-track session, generalised to `2W` tracks | every mono track stays mono; banks 5; shape [4,13]; `collapse` [12,1]; PCM = bank-free | same: banks 5, shape [4,13], `collapse[1]` 1 |
| `the_mono_remainder_stays_when_moving_it_binds_no_more_banks` (the guard) | mono tracks stay mono; banks 2; builtin chain armed; PCM = bank-free | same |
| `a_single_odd_track_no_longer_strands_a_pool_remainder` (updated) | banks **24** = unsplit (21); pools 56 + 8; chain shape [8,48] = unsplit ([11,48]); both digest assertions kept | banks **48** = unsplit (45); pools 60 + 4 |

"Before" is the same test binary with the move disabled (scratch switch, reverted). The two new
gates that assert current behaviour (the 16-track and guard sessions) are green before as well:
they gate a regression the move could introduce, not a change it makes.

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| `cargo test --locked -p graph-compiler` | 113 passed |
| `cargo test --locked -p rack-compiler` | 13 passed |
| `cargo test --locked -p graph` / `--features test-support` | 110 / 117 passed |
| `cargo test --locked -p builtins-compiler --features test-support` | 79 passed |
| `cargo test --locked -p host-core --all-features` (includes `symmetry_witness`) | 233 passed |
| `cargo test --locked -p console-workload` (includes `chain_shape`, 23) | 39 passed |
| `cargo test --locked -p capi` | 36 passed |
| `bash scripts/check-graph-policy.sh .` | PASS |
| `bash scripts/check-graph-determinism.sh` | PASS (100/100) |
| at 4 lanes: the 4 tests above plus `the_two_planners_agree_on_every_track_class` and `class_pooling_forfeits_the_route_fold_only_on_an_interleaved_session` | 6 passed |

**Class A, standing rows.** All 17 `native_session_rows()` were rendered for 64 blocks each at
`Simd8` and at `Simd4` (scratch example over `build_with_dispatch`, not committed), on base
`8eebf17b` and on `6a0689cd`. `bank_shape`, `bank_transposes`, `bank_collapse_counters`,
`bank_route_folds`, `bank_scatter_redirects` and the output SHA-256 are **identical row for row at
both widths**. No standing row has a stranded mono track: its mono counts are 0, 32 or 64.

### Mutations

Each mutation was applied to `banks.rs`, `cargo test --locked -p graph-compiler --lib` was run
(84 tests), and the mutation was reverted. Four-lane runs used the four #971 tests.

| mutation | 8 lanes | 4 lanes |
|---|---|---|
| `today`: no move at all | red: dogfood gate (banks 27 vs 30), odd-track (21 vs 24) | red: same (57 vs 60, 45 vs 48) |
| `brief`: the brief's rule (floor(n/W)·W per `(simd1, dynamic, simd2)` program, no guard) | red: **16-track gate** ("every mono track stays in the mono pool": left `{}`), guard gate | red: same two |
| `global`: the prototype (global floor(n_mono/W)·W, no guard) | red: guard gate; dogfood builtins-only arm (6 mono builtin banks vs 9) | red: same two |
| `noguard`: the rule without the guard | red: guard gate (mono tracks moved; 1 bank) | red: same |
| `rackonly`: re-plan the rack chains, but leave the map the builtin planner reads unchanged | red: dogfood gate on **`bank_shape`** ([18,63] vs [12,63]) while its bank count and pools stay green; odd-track chain shape ([12,51] vs [8,48]) | red: same two |
| `any + noguard`: move when *any* group is partial mono | red: 16-track gate (`ch00`, `ch02`, `ch04`, `ch06` moved), guard gate | red: same |
| `vacuous + noguard`: also move mono tracks that sit in no effect group | red: dogfood builtins-only arm (0 mono builtin banks vs 9), guard gate | red: same |
| `brief + guard`, `global + guard`, `any` (guard kept), `vacuous` (guard kept) | **green**: the guard cancels each of these moves on every gate session, because none binds more banks there | not run |

The last row is a finding, not a pass. With the guard present, the gate sessions do not tell the
amendment's every-group rule apart from looser rules: the guard makes each looser rule's move on
those sessions a no-op. Each rule is discriminated only once the guard is removed. See the
verifier notes.

### A/B on the dogfood first-listen session (the research's harness)

`research_build` from `dual-mono-prototypes.patch`, driven by a three-arm scratch example with a
compile-time switch that disables the move; not committed. The patch is kept at
`<scratchpad>/971/ab971-harness.patch`. The session is the real first-listen `session.json` (81
tracks; the 18 bit-identical dual-mono stems come from `identical-sha.txt`) with the mono
fixture's EQ + compressor + limiter strip cycled onto every track. The 18 are fed identical
planes in every arm. Arms:

* `as_is`: the 18 declared stereo;
* `folded_today`: the 18 mapped `(0, 0)`, move disabled;
* `folded_971`: the 18 mapped `(0, 0)`, this change.

Method: one invocation per width under `flock -w 7200 <scratchpad>/timing.lock` with
`taskset -c 31`, built first and outside the lock. One warmup of 500 blocks per arm, then two
measured rounds of 3000 observations, with arms alternating in rotating order. Figures are p50 of
per-block `render` time. Host load was 7.8 to 8.2 and uncontrolled, so read the relative numbers.

| strip | width | as_is | folded_today | folded_971 | 971 vs today |
|---|---|---:|---:|---:|---:|
| mixing strip | 8 | 135.9 / 135.9 µs | 164.1 / 164.1 (**+20.8%**) | **130.1 / 130.1 (-4.3%)** | -20.7% |
| mixing strip | 4 | 238.5 / 238.5 | 234.3 / 234.2 (-1.8%) | **221.4 / 221.5 (-7.2% / -7.1%)** | -5.5% |
| builtins only | 8 | 31.1 / 31.0 | 33.6 / 33.5 (+7.8%) | 34.0 / 33.9 (+9.2%) | +1.2%* |
| builtins only | 4 | 48.7 / 49.3 | 48.1 / 49.1 | 48.1 / 48.6 | -0.1% / -1.1%* |

\* On the builtins-only strip, `folded_today` and `folded_971` are the **same plan**: nothing is
moved, and shape, collapse and digest are identical. The 0.4 µs gap is noise and placement, the
same size as the ~1.5 µs noise floor the verification measured on this row.

Every arm printed the same digest: strip `58a7dc2477b23056` (the verification's value) and
builtins `a3bb6a6a84917779`, at both widths. Shapes: 8 lanes `[12,63]` as is and after, `[13,60]`
today; 4 lanes `[22,123]` as is and after, `[23,120]` today. Armed chains: 2 after and 3 today at
8 lanes; 4 after and 5 today at 4 lanes.

Against the brief's targets (about -5% and -7%): -4.3% at 8 lanes and -7.2% at 4. Today's penalty
reads +20.8% here against the verification's +33.6%. Absolute costs are lower on this batch (135.9
µs against 168 µs as is), which also carries the EQ (#976) and compressor (#981-#985) work. I did
not isolate which change accounts for the difference.

### Route folds (ruling 07 is the owner's; no reordering was implemented)

| session | as is / all stereo | folded today | folded + #971 |
|---|---:|---:|---:|
| dogfood first-listen, mixing strip, 8 and 4 lanes (A/B) | 81 | 0 | 0 |
| dogfood first-listen, builtins only, 8 and 4 lanes (A/B) | 81 | 0 | 0 |
| gate: mono fixture on the dogfood layout, strip and builtins only | 81 | 0 | 0 |
| gate: one odd track (`ch07`) | | 0 | 0 |
| gate: 16-track mixed | | 0 | 0 |
| probe, not a gate: 18 mono tracks first (`ch00`-`ch17`), then 63 stereo | 81 | **0** | **81** |

On interleaved sessions #971 neither loses nor restores the fold. On a contiguous mono-first
session it restores the fold at both widths, because the stranded remainder chains no longer
render out of track order. This confirms the verification's note that #971 is a prerequisite for
the ruling's option 3.

### For #969

The one-odd-track shape now binds 24 banks against 24 (48 against 48 at 4 lanes), and `ch07`'s
builtin banks hold 8 members (4 at 4 lanes), not 1: the mono remainder joins it. Both assertions
in #969's slice would be red on this branch. #969 must pin the class map instead, which the plan
exposes as `report().rack_cohorts.plan.groups[*].class`, the helper `pooled_tracks` in this
change reads. It must also cite the renamed test.

### Notes for the verifier

* The guard is my addition, and it masks the rule mutations (see the table above). A session
  where a looser rule's move *gains* a bank would separate the two. One example: `1.5W` stereo
  tracks and `W` mono tracks, all with EQ + compressor, where half the mono tracks also carry the
  limiter and the stereo tracks all carry it. Under the brief's rule or an "any" rule that
  session binds 6 banks instead of 5, but breaks the collapsing mono cohort. I did not pin it:
  which of the two plans renders faster there is unmeasured, and a gate would fix an unmeasured
  preference.
* `CohortPoolClass::of_prepare_witness`'s doc (`rack-compiler`, not authorized) calls it "the only
  constructor". That remains true of the mono class; a move only ever yields `Stereo`, through
  the same `SessionPoolClasses` object both planners read.
* No CI leg runs these tests at 4 lanes. The 4-lane numbers above come only from the scratch cfg
  build.
* The cost falls on the control plane only: one extra `plan_bank_groups` run and two slot counts,
  and only when some mono track is stranded.


## Sol attempt 1 verdict: FAIL

Sol, 2026-09-27, on `9b1a3ce2` (base `8eebf17b`), x86-64-v3 host, `CARGO_INCREMENTAL=0`. The
four-lane runs use the research `--cfg miso_native_simd4` lane hunk in `target/simd4`, applied for
measurement only and then reverted. The worktree was left clean. The scratch probe, the tests and
the mutation driver are in the session scratchpad (`971v/`) and are not committed.

The implementation is class A and does what the evidence says: rendered bits are unchanged, the
A/B reproduces, and every named gate passes. It fails on two claims that no gate checks. With the
bank-gain guard in place, the rule this issue exists to ship is untested. And the guard itself can
keep a move that binds no more banks, or fewer, which its own documentation says it never does.
Both fixes are bounded; see "Required for attempt 2".

### Findings, by severity

1. **High (gate gap). No gate pins the rule while the guard is present.**
   `crates/graph-compiler/src/banks.rs:233-251` (`stranded_mono_tracks`, then the guard at 246).
   Reproduced at 8 and 4 lanes: the brief's per-program rule, the global prototype, the "any
   group" rule and the vacuous rule, each with the guard kept, pass all four #971 gates and
   `the_two_planners_agree_on_every_track_class`. The reason is that in every gate session where a
   wrong rule moves a different set of tracks from the right rule, the right rule moves nothing.
   A cancelled move therefore looks exactly like the correct outcome.
   `a_mono_track_that_fills_a_cohort_is_not_pooled_as_stereo` (`lib.rs:8689`) is red only under
   the brief's rule *without* the guard. Its own doc says so (`lib.rs:8685`). The fix is the test
   below, which turns red under all four wrong rules with the guard kept.
2. **Medium (the guard is unsound on banks). It counts planned slots, not banks a factory will
   bind.** `bindable_slot_count` (`banks.rs:358-359`) says that consent "is the same for every
   plan of one session". That holds for each slot key, but the count adds up slots across keys.
   `miso.delay` always returns `Ok(None)` (`crates/delay/src/lib.rs:609`). At a non-native width,
   EQ and soft-clip decline too. Measured, class A in every case:
   * `dynamic: [delay]` only, `ch00..=ch{W}` mono and the rest stereo: `ch{W}` moves to the stereo
     pool, and the effect bank count is 0 before and after. The move gains nothing and loses
     `ch{W}`'s collapse.
   * The implementer's guard shape plus one mono track with `dynamic: [delay]` and `simd2: [delay]`
     beside `W-1` stereo tracks with the same chains: the effect bank count drops from **2 to 1**
     at `Simd8`. On the native four-lane build it drops from 2 to 1, and from 4 to 3 on the
     `W=8` layout. The guard accepted it because the planned slots rose from 2 to 3.
   * In the randomized probe at native four lanes, seed 380 (template strips with a dynamic delay)
     moves `ch07` and `ch09` for 3 → 3 banks.

   The type's documentation repeats the false invariant ("binds more effect banks than the trial",
   `crates/builtins-compiler/src/lib.rs:4003`).
3. **Low (follow-up, not required).** The guard accepts or refuses the whole move set at once. A
   harmful move bundled with a profitable one is kept whenever the profitable one gains more
   banks than the harmful one loses. When it gains less, both are cancelled. Evaluating moves per
   independent component is an optimisation, so it belongs in its own issue.

### The questions asked

* **Is the guard sound?** Renders: yes. Every accepted and cancelled plan rendered the bits of
  base and of the `Backend::Scalar` oracle, armed and unarmed (see "Class A" below).
  Deterministic: yes. It is a pure function of the trial plan with BTreeMap order,
  `check-graph-determinism.sh` gave 100/100, and a fresh-process rerun of the named probe was
  byte-identical, pools included. Never binds fewer banks: **no**, see finding 2.
* **Is it the right design, or a crutch?** The rule needs an acceptance check. The planner's
  greedy leader choice is not monotone: a longer program that joins a pool re-leads its cohort,
  so `the_mono_remainder_stays_when_moving_it_binds_no_more_banks` loses a bank without the
  check. "Propose with the rule, keep only a measured gain" is therefore sound in shape. It is
  only a crutch in the sense of finding 1, where it hides which rule proposes, and its measure is
  wrong (finding 2).
* **Compile-time cost.** The re-plan is one extra `plan_bank_groups` call, one clone of the class
  map, and two slot counts, and it runs only when some mono track strands. #962's scale session
  has no effects, so it never takes this path. Measured on the same 65,537 tracks, all mono, each
  with a `simd1: [comp]` (debug, `compile_with_builtins` only, two runs each):
  * one stranded track, where the guard refuses: 49.2 / 47.7 s against base 48.5 / 47.6 s;
  * one track in 9 stereo, so 7 move and one bank is gained (8192 against 8191): 49.8 / 48.2 s
    against 49.2 / 47.5 s.

  That is at most about 1.5%, inside run-to-run noise. Peak RSS is 2.1 GB.

### Class A (randomized probe, base against head)

A scratch integration test drove a generator derived from #966's `bank_levels.rs` probe. It
builds 1 to 40, 64, 65 or 81 tracks, with a mono share of 10 to 90% on either the symmetric mono
strip or the asymmetric intended one. It uses template or free strips with dropped and inserted
slots, sidechains from every tap, sends from every tap into submixes, input delays, and bypass.
The generator ran over 400 seeds. It also ran 22 named shapes:
* the dogfood layout, as a strip, builtins only, contiguous and all stereo;
* the 16-track and guard sessions at `W` = 4 and 8;
* the discriminating and phantom sessions;
* the odd-track session;
* #966's shapes with mixed mono: the mono console less EQs (3 and 9 of them), the ragged
  soft-clip session and the realigning strips;
* #970's reduced console, as is and mixed.

Each model was compiled at `Scalar`, `Simd4` and `Simd8`, then rendered unarmed and armed for 16
to 24 blocks, on base and on head.

| build | rows | digest differences base→head | rows ≠ scalar oracle | models with a #971 move | effect banks fewer |
|---|---:|---:|---:|---:|---:|
| native 8 lanes | 2,110 | 0 | 0 | 44 | 1 (phantom-loss) |
| native 4 lanes | 2,110 | 0 | 0 | 44 | 2 (phantom-loss) |

One seed, 46, renders silence at every width, so it compares nothing. The 17 standing rows are
identical, base against after, at both widths: I reran the implementer's four row binaries, and
their output matched the recorded files.

### A/B reproduced

These ran under the timing lock, uncontended, with `taskset -c 31` and host load 4 to 5. The
driver was the implementer's `ab971` harness: one warmup and two measured rounds of 3000
observations. Figures are p50 per-block render times in microseconds.

| strip | width | as is | folded today | folded + #971 |
|---|---|---:|---:|---:|
| EQ + compressor + limiter | 8 | 136.0 / 136.2 | 164.2 / 164.5 (+20.8%) | **130.1 / 130.3 (−4.3%)** |
| EQ + compressor + limiter | 4 | 240.1 / 240.2 | 235.6 / 235.8 (−1.8%) | **223.0 / 223.3 (−7.1%)** |
| builtins only | 8 | 30.8 / 30.8 | 32.9 / 32.9 | 32.9 / 32.9 (same plan) |
| builtins only | 4 | 48.8 / 48.7 | 48.0 / 47.9 | 47.8 / 47.7 (same plan) |

Every arm printed the same digests: `58a7dc2477b23056` for the strip and `a3bb6a6a84917779` for
builtins only, at both widths. The shapes, collapse and fold counts match the evidence.

### Route folds and ruling 07

The dogfood layout folds 0 before and after the change, as the evidence says. The contiguous
mono-first layout goes from 0 to 81 at both widths. With builtins only it folds 81 both before
and after, because nothing moves. This agrees with ruling 07: pooling is what forfeits the fold
on interleaved ids, and #971 is the prerequisite for option 3 (mono ids sorted first at
authoring). The phantom-loss session also regains its folds (0 → 25), because nothing stays
partial.

### Gates and scope

These all pass at head:
* `cargo fmt --all --check`, and clippy on the workspace with `--all-targets --all-features -D
  warnings`;
* `RUSTDOCFLAGS='-D warnings' cargo doc`;
* `-p graph-compiler` (113), `-p rack-compiler`, and `-p graph` with and without `test-support`;
* `-p builtins-compiler --features test-support`;
* `-p host-core --all-features`, which includes `symmetry_witness` and `collapse_arming`;
* `-p console-workload`, which includes `chain_shape` (23);
* `-p capi`;
* both graph scripts, and `cargo check` on `wasm32-unknown-unknown` with `+simd128`.

On the four-lane build, the four #971 tests and the discriminating test pass. Three unrelated
lib tests fail on that build at base as well: `launch_soft_clip_fixture_*`,
`misaligned_lane_sets_decline_the_merge` and `frozen_issue_037_*`. They are artefacts of the
scratch cfg, not of this change.

The other scope checks:
* `bank_shape` is asserted, and it catches the rack-only mutation ([18,63] against [12,63]).
* The 16-track gate is red under the brief's rule without the guard. It is green under the
  prototype, which leaves that session alone by construction.
* The note for #969 is present.
* The diff stays inside the authorized paths, and the only changes to `lib.rs` are in its test
  module.

### Required for attempt 2

1. **Add this test** (`crates/graph-compiler/src/lib.rs` tests, width-generic; it uses the
   implementer's `mono_fixture_with_tracks`, `pooled_tracks` and `render_armed_console_blocks`).
   Validated in scratch: green at head at both widths. It is red at both widths under five
   mutations:
   * `today` (no move);
   * the brief's rule with the guard: T's mono tracks move and the whole move is accepted;
   * the global rule with the guard: the move is cancelled and `ch{W+1}` stays mono;
   * the "any" rule with the guard: the move is cancelled;
   * the vacuous rule with the guard: V moves.

   The implementer's guard test still covers the rule without the guard.

```rust
/// Issue #971: the rule, not the bank-gain check, decides which tracks move. A session that
/// hands the check one move it must accept beside the moves a looser rule adds: accepted
/// wholesale they move tracks the rule keeps, refused wholesale they cancel the move it makes.
/// V (`ch00`): mono, no effect. P (`ch01..=ch{2W}`): `W + 1` mono then `W - 1` stereo, only
/// `dynamic: [comp]`; the last mono one strands and completes the stereo cohort. T (next `2W`):
/// the over-demotion session (even mono, odd stereo, the limiter on the stereo tracks and the
/// lower half's mono ones); the rule moves none of it.
#[test]
fn the_rule_not_the_bank_gain_check_picks_the_moved_tracks() {
    const BLOCKS: u64 = 12;
    let Some(width) = BankWidth::for_backend(host_dispatch()) else { return; };
    let lanes = width.lanes() as usize;
    let registry = launch_native_effect_registry().expect("launch registry");
    let (p, t) = (2 * lanes, 2 * lanes);
    let mut model = mono_fixture_with_tracks(1 + p + t);
    let comp = model.tracks[1].simd1.effects[1].clone();
    for (index, track) in model.tracks.iter_mut().enumerate() {
        if index == 0 {
            track.simd1.effects.clear();
            track.dynamic.effects.clear();
            track.simd2.effects.clear();
        } else if index <= p {
            if index > lanes + 1 { track.right_source_channel = 1; }
            track.simd1.effects.clear();
            track.simd2.effects.clear();
            track.dynamic.effects = vec![comp.clone()];
        } else {
            let local = index - 1 - p;
            if !local.is_multiple_of(2) { track.right_source_channel = 1; }
            else if local >= lanes { track.simd2.effects.clear(); }
        }
    }
    let name = |index: usize| format!("ch{index:02}");
    let mono: BTreeSet<String> = model.tracks.iter()
        .filter(|track| track.left_source_channel == track.right_source_channel)
        .map(|track| track.id.as_str().to_owned()).collect();
    let stranded = name(lanes + 1);
    let artifact = compile_console_model_with_builtins(&model, 2_086, &[], &registry);
    let pooled = pooled_tracks(&artifact);
    assert_eq!(pooled[0], (1..=lanes).chain((1 + p..1 + p + t).step_by(2)).map(name)
        .collect::<BTreeSet<_>>());
    assert_eq!(pooled[1], (lanes + 1..=p).chain((2 + p..1 + p + t).step_by(2)).map(name)
        .collect::<BTreeSet<_>>(),
        "exactly P's stranded track ({stranded}) joined the stereo pool");
    assert_eq!(artifact.graph().prepared_bank_count(), 7);
    // V is in no effect group, so it stays mono: its full post-input bank holds only mono tracks.
    let v_banks: Vec<Vec<String>> = artifact.prepared_builtin_banks()
        .filter(|bank| bank.stage == TrackStage::PostInputBuiltins)
        .map(|bank| bank.members.iter().map(|node| match node {
            GraphNodeId::TrackStage { track_id, .. } => track_id.as_str().to_owned(),
            other => panic!("a builtin bank named {other:?}"),
        }).collect::<Vec<_>>())
        .filter(|members| members.contains(&name(0))).collect();
    assert_eq!(v_banks.len(), 1);
    assert_eq!(v_banks[0].len(), lanes);
    assert!(v_banks[0].iter().all(|track| mono.contains(track) && *track != stranded));
    let render = render_armed_console_blocks(artifact, BLOCKS, &mono, &mono);
    assert!(render.collapse[0] > 0);
    let scalar = compile_console_model_with_builtins(&model, 2_087, &[], &scalar_console_registry());
    let scalar_render = render_armed_console_blocks(scalar, BLOCKS, &mono, &mono);
    assert_pcm_bits_equal(&render.pcm, &scalar_render.pcm, "discriminating session");
}
```

2. **Make the guard count banks the factories will bind.** For launch factories, whether a bank
   binds depends on the factory, the width and the program key. The planner already guarantees
   that key is equal across a group. So one approach is to learn each slot key's consent from
   one `bind_homogeneous_bank` call, cached per key, and count only the consenting slots. Then
   add two gates (width-generic, and red at head today). Both start from
   `mono_fixture_with_tracks`, with `simd1` and `simd2` cleared unless stated:
   * `W + 1` mono and `W - 1` stereo tracks, each with only `dynamic: [delay]`. Assert that
     `ch{W}` stays in the mono pool, and 0 effect banks.
   * The guard session's `2W` stereo tracks (`simd1: [eq]` or `[comp]`); one mono track with
     `simd1: [eq, comp]`; one mono track with `dynamic: [delay]` and `simd2: [delay]`; and `W - 1`
     stereo tracks with the same two delay chains. Assert 2 effect banks (the no-move plan's
     count) and that both mono tracks stay mono.
3. **Correct the two false statements.** `banks.rs:358` says consent is "the same for every plan";
   `crates/builtins-compiler/src/lib.rs:4003` says the move is kept only when the plan "binds more
   effect banks". Correct both, or make them true with item 2.

## Attempt 2 evidence

Implementer attempt 2, 2026-09-27. The branch first merged the batch head `077fb31e` (merge
`ef7e5510`), so every gate below ran against it. The implementation is commit `8ff8ce65`. Host
x86_64 (`x86-64-v3`), `CARGO_INCREMENTAL=0`. The four-lane legs use the research `--cfg
miso_native_simd4` lane hunk in their own target directories, applied for measurement only and
reverted. The scratch harnesses are not committed and live under `<scratchpad>/971/a2/`.

### Finding 2 (medium): the guard now counts banks the factories bind

* `bind_rack_banks_indexed` binds the trial plan through `bind_planned_banks`, which is the old
  bind loop, moved unchanged. If some mono track strands, it re-plans, binds the re-plan the
  same way, and keeps the move only when `replan_banks.len() > banks.len()`. Both counts are
  banks a factory actually returned, so a full group the factory declines counts for nothing.
  That covers the delay's `Ok(None)` and a width the artefact was not built for. The rejected
  plan's banks are dropped on the control plane. `bindable_slot_count` is gone.
* The two false statements are corrected. The comment above the guard in `banks.rs`, formerly
  "binds more effect slots", now says the check compares bound banks and why a planned slot
  proves nothing. The `SessionPoolClasses` doc in `builtins-compiler/src/lib.rs` now says both
  plans are bound and the move is kept only when the factories bound more banks. It also says
  the delay never banks and that the decision is one for the whole move set.
* Error semantics: binding the re-plan is a real `bind_homogeneous_bank` call, so a factory
  error on it fails the compile, exactly as it would if that plan were the one kept. Launch
  factories return errors only for malformed requests, which preparation already refuses.
* The gates are Sol's two phantom sessions, both width-generic, in
  `crates/graph-compiler/src/lib.rs`:
  * `a_move_that_binds_no_bank_is_not_kept` asserts that `ch{W}` stays mono and that there are
    0 effect banks;
  * `a_move_that_binds_fewer_banks_is_not_kept` asserts that both mono tracks stay mono and that
    there are 2 effect banks.

  Both also compare the rendered PCM with the bank-free registry, which now includes `miso.delay`
  so the oracle can prepare these sessions. The delay never banks, so this changes no other test.

### Finding 1 (high): a gate for the rule with the guard in place

`the_rule_not_the_bank_gain_check_picks_the_moved_tracks` is Sol's test, with its assertions
unchanged. Only its formatting differs: `!local.is_multiple_of(2)` for clippy, and the doc was
edited. The doc of `a_mono_track_that_fills_a_cohort_is_not_pooled_as_stereo` now says it gates
the rule only without the check, and names this test and the guard gate.

### Mutations

Each mutation was applied to `banks.rs` (the driver is `<scratchpad>/971/mut2/`) and reverted.
At 8 lanes the whole `cargo test --locked -p graph-compiler --lib` suite ran (86 tests). At 4
lanes the scratch-cfg build ran the seven #971 tests plus `the_two_planners_agree_on_every_track_class`
and `class_pooling_forfeits_the_route_fold_only_on_an_interleaved_session`. The red sets are
identical at both widths.

| mutation (the guard is kept unless named) | red, at 8 and 4 lanes |
|---|---|
| `today`: no move | `the_rule_not_…` (mono pool includes the stranded `ch09`), dogfood gate, odd-track |
| `brief`: the per-program rule | **`the_rule_not_…`**: mono pool = P only (every T mono track moved; the whole set accepted) |
| `global`: the prototype | **`the_rule_not_…`**: the set is cancelled, so `ch09` stays mono |
| `any`: move if any group is partial mono | **`the_rule_not_…`**: cancelled, `ch09` stays mono |
| `vacuous`: also move tracks in no effect group | **`the_rule_not_…`**: "V stays in the mono pool", with V's post-input bank holding `ch09`..`ch15` |
| `slotguard`: attempt 1's planned-slot count | **`a_move_that_binds_no_bank_is_not_kept`** (`ch08` moved), **`a_move_that_binds_fewer_banks_is_not_kept`** (both mono tracks moved) |
| `noguard` | both phantom gates, `the_mono_remainder_stays_when_moving_it_binds_no_more_banks` |
| `rackonly`: the builtin planner's map is not updated | dogfood gate (`bank_shape` [18,63] vs [12,63]), odd-track (chain shape) |
| `brief`, `global`, `any`, `vacuous`, each with `noguard` | 5 tests each: the discriminating test plus the no-guard reds, and the 16-track gate under `brief` and `any`, or the dogfood builtins-only arm under `global` and `vacuous` |

The failure messages quoted come from the 8-lane run, with P's stranded track at `ch09`; at 4
lanes the names shift with `W`.

### Gates

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| `cargo test --locked -p graph-compiler` | 116 passed (lib 86) |
| `cargo test --locked -p rack-compiler` | 13 passed |
| `cargo test --locked -p graph` / `--features test-support` | 110 / 117 passed |
| `cargo test --locked -p builtins-compiler --features test-support` | 79 passed |
| `cargo test --locked -p host-core --all-features` | 234 passed |
| `cargo test --locked -p console-workload` (includes `chain_shape`) | 39 passed |
| `cargo test --locked -p capi` | 36 passed |
| `bash scripts/check-graph-policy.sh .` / `bash scripts/check-graph-determinism.sh` | PASS / PASS (100/100) |
| `bash scripts/check-env-vocabulary.sh .` | ok |
| `cargo check -p graph-compiler --target wasm32-unknown-unknown` (`+simd128`) | pass |
| 4 lanes (scratch cfg): the seven #971 tests plus the two class tests | 9 passed |

### Class A, digests

* **Standing rows.** All 17 `native_session_rows()`, 64 blocks each, at `Simd8` and at `Simd4`,
  base `077fb31e` against head `8ff8ce65`: `bank_shape`, `bank_transposes`, the collapse
  counters, the route folds, the scatter redirects and the output SHA-256 are identical row for
  row at both widths.
* **Sol's randomized probe** (`971v/probe971_final.rs`, run unchanged as a scratch integration
  test, 400 seeds plus 22 named shapes, rendered at `Scalar`, `Simd4` and `Simd8`, armed and
  unarmed), base `077fb31e` against head, on the 8-lane and 4-lane builds: **4,220 rows**.
  * **0 digest differences.**
  * **0 rows that differ from the scalar oracle.** The only row the probe reports is seed 46,
    which is silent at every width and so compares nothing, as Sol found.
  * **0 rows with fewer effect banks at head.** Attempt 1 had 1 such row at 8 lanes and 2 at 4.
  * Every row with a #971 move gains at least one bank: 20 + 48 moved rows at 8 lanes, 12 + 46 at
    4 lanes, none without a gain.
  * The four phantom sessions (`phantom-{4,8}`, `phantom-loss-{4,8}`) now match base in pools and
    bank counts at every dispatch.
  * Folds: the contiguous mono-first strip goes from 0 to 81; nothing else changes.

### A/B, dogfood first-listen session (short re-run)

Attempt 1's `ab971` harness was rebuilt on the merged tree, with its compile-time switch disabling
the move for `folded_today`. The method is the same: one invocation per width under `flock -w
7200 <scratchpad>/timing.lock` with `taskset -c 31`, one warmup and two rounds of 3000
observations, built outside the lock. Host load was 7.3 and uncontrolled. Figures are p50 per
block, in µs.

| strip | width | as_is | folded_today | folded + #971 | #971 vs today |
|---|---|---:|---:|---:|---:|
| mixing strip | 8 | 127.9 / 127.9 | 152.3 / 152.4 (+19.1%) | **123.9 / 124.0 (-3.1%)** | -18.6% |
| mixing strip | 4 | 222.6 / 222.6 | 219.7 / 219.7 (-1.3%) | **210.2 / 210.2 (-5.6%)** | -4.3% |
| builtins only | 8 | 30.7 / 30.9 | 33.8 / 34.0 | 32.8 / 33.1 (same plan) | noise |
| builtins only | 4 | 48.5 / 48.4 | 48.5 / 48.4 | 48.7 / 48.6 (same plan) | noise |

The digests are `58a7dc2477b23056` for the strip and `a3bb6a6a84917779` for builtins only, in
every arm at both widths. The shapes and collapse counters equal attempt 1's: `[12,63]` with 2
armed chains at 8 lanes, and `[22,123]` with 4 at 4 lanes. The route folds are 81 as is, 0
today and 0 with #971.

The #971 plan on this session is attempt 1's. The smaller relative gain than attempt 1's
(-4.3% and -7.1%) comes with the merged batch, where every arm is cheaper: as is was 135.9 µs at
8 lanes and 238.5 µs at 4, and is now 127.9 and 222.6. The merge brought the EQ elision (#980)
and the linked limiter pair (#990). I did not isolate which accounts for the difference.

### Compile-time cost

The session is Sol's #962-scale case: 65,537 tracks, each with `simd1: [comp]`. The figures are
`compile_with_builtins` time in a debug build, one repetition each, under the timing lock, with
host load 5 to 7.

| case | base | head |
|---|---:|---:|
| one stranded track, move refused | 48.9 s | 50.5 s (+3.2%) |
| one track in 9 stereo, move kept (8191 → 8192 banks) | 48.7 s | 50.6 s (+3.9%) |

Binding the re-plan costs about 2 s more than attempt 1's planned-slot count did (Sol measured
about 1.5% for attempt 1). The cost is paid only when some mono track strands, and only on the
control plane.

### Follow-up (low finding 3, not implemented)

The guard keeps or refuses the whole move set at once. A harmful move bundled with a profitable
one is kept whenever the profitable one gains more banks than the harmful one loses, and when it
gains less, both are cancelled. Evaluating moves per independent component, meaning tracks whose
groups share no pool, would recover the profitable part. That is an optimisation, and it belongs
in its own issue.
