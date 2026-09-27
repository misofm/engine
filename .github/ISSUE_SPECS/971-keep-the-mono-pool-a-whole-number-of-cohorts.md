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
