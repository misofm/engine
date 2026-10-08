FAIL

# #1457 attempt 1: adversarial verdict

Commits reviewed: `3e8ee9b49` (Amendment 1, filing of #1470), `30a355d5d` (implementation,
Amendment 2, filing of #1471) and `f5c98fa8d` (gate record), against the parent `5dc6f96e9`.
I exported `f5c98fa8d` and built it in `/tmp/claude-1002/v1457`. I did not edit, build or check out
anything in the worktree.

## Answers to the six questions

1. **Is the D3 fallback certified, and is the result a pure function of the session? Yes.** A
   design past the budget reports `input_section_live_bound_table(rate)`. That bound covers every
   history of trim (up to +24 dB), polarity and filter targets in `[10 Hz, maximum]`, so it covers
   every constant design. The fixed designs use the same `SvfSection::design` words, the same
   domain (`prepare_input_track` refuses anything outside it), and the same kernel with constant
   words. The live bound has `tail == tail_every_peak`, so the `P*` caveat does not apply. As
   evidence (not proof), a grid of 860 designs at each of the four rates found no fixed exact bound
   above the live bound in any of the four fields, and none unbounded. The grid covered cutoffs
   from 10 Hz to the maximum, trims from -144 dB to +24 dB, and L != R pairs.

   The result is a pure function of the session:
   - The cache key carries everything the walk reads: the rate, the words, and |trim|. A disabled
     section's words are `IDENTITY` (`k = 0`), so the enabled flag is implied.
   - A `Bound` hit charges its stored frames under the same `<=` test that a walk would pass.
   - `ChargeAbove(h)` records a true fact (charge > h). It is honoured only when `remaining <= h`.
   - Clear-on-full drops entries but never changes a value.

   Gate 5 tests caches warmed under budgets 0, MAX and budget +/- 1. My probe asserts that the
   values are identical first, warm and without a cache, for 65,537 strips at each rate. A cache
   hit or a miss cannot change a bit.
2. **Is the horizon cap sound? Yes.** A stopped walk returns `None` before any result exists, and
   the strip takes the fallback. A walk that finishes runs the same computation, so its result is
   the same under every horizon at or above its frames. The cap is present in all three
   frame-by-frame loops: the majorant pass, the replay, and the deviation walk. `rest_frames`, the
   exponential search and the bisection are closed form. Their cost is not charged (see MJ1), but
   they do not walk frames. The one gap is in the tests, not the code (see MJ2).
3. **Is the memoryless shortcut correct? Yes.** The zero bound is the same value
   `fixed_input_bound` already returned: each channel with `count == 0` returns `ZERO`. A section of
   trim and polarity only is memoryless, so `ZERO` is certified. The shortcut runs before the budget
   check, so a memoryless design after exhaustion still reports zero. A walk under a zero horizon
   gives the same result. The count changes are right: 6 -> 5 and 4 -> 3. My mutation X6 (shortcut
   disabled) turns only that test red.
4. **Are the budget figures honest? No.** The near-top figure reproduces: the 64-design family
   takes 18.6-19.7 ms here. The stated worst case of "about 32 ms" does not hold (MJ1).
5. **Is the cache control-side, bounded, and away from render? Yes, with a qualification.** The
   cache is passed by `&mut` and never stored in `PreparedBuiltinsSession` or anything render owns.
   Its entries are capped, with clear-on-full; M9 turns red. No render code changed. `audit capi`
   reports 0 allocations, 0 syscalls and an unchanged `pcm_digest`. The memory and rebuild claims
   in the cap's doc are overstated (m3).
6. **Is #1471 self-contained and ordered after #1309? Yes.** Its STREAMS row is "after #1457, after
   B #1309", its paths are named, and its gates are concrete. Its gate 1 has an unstated condition
   (m3).

## BLOCKER

None.

## MAJOR

**MJ1. The stated worst case (about 32 ms) is wrong by about 2x. The issue's "stated, gated
figure" is false.**

Locations:
- `crates/builtins/src/tail.rs:229-237`: "the cheapest, a 1 kHz LPF of 513 frames, about 47,500
  [frames/ms] ... can spend about 32 ms".
- Spec Amendment 2, D1, lines 289-293.
- Attempt record, lines 367-379.

The 1 kHz low-pass is the cheapest one-section design. A two-section design with the same 513-
(or 769-) frame minimal walk does about half as many frames per ms. The charge counts frames only,
and the fixed cost of each design is not charged. That cost is about 14 us for a two-section design
(prepare, section constants, three `rest_frames`, powers, vectors), which is about 70 % of a cheap
design's cost.

Measured on this box (pinned core 7, release, x86-64-v3, the same conditions as the record), on a
session of 4,096 distinct designs at the production budget (`input_section_bounds`, no cache,
rounds 1 and 2). Each run walks exactly 1,510,000 frames:

| family (HPF / LPF, trim) | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz |
|---|---|---|---|---|
| 0 / 1 kHz (the record's "cheap"), 0 dB | 31.4-32.1 ms | 31.4-31.5 | 27.2-27.4 | 27.3 |
| 1 kHz / 1.28-1.48 kHz, +24 dB | **61.7-63.7** | 48.3-48.6 | 53.8 | 44.5-44.7 |
| 1.28 kHz / 5.12 kHz, +12 dB | 59.6-59.7 | **58.8-62.8** | 50.1-50.6 | 50.2-50.6 |
| 2.56 kHz / 5.12 kHz, +24 dB | 54.0-54.3 | 53.3-53.4 | **59.7** | **58.5-58.7** |

The record's own family gives 27-32 ms here, so this box matches the box the worker measured on.
The worst case of the budget is therefore at least about 60-64 ms at every launch rate. That is
3x the 20 ms headline, not about 32 ms. The scan of single designs agrees: the lowest rate is
26,655-28,508 frames/ms, for HPF-into-LPF designs that walk 513 frames.

Root's D1 ruling restates the 32 ms figure, and the product outcome requires a correct worst-case
figure. Root must decide one of these:
- (a) restate the worst case at about 64 ms, measured with the two-section family;
- (b) charge each design a per-design (or per-section) constant as well as its frames, so that the
  frames budget bounds the time;
- (c) lower the budget.

Then re-measure gate 2 with the two-section family included.

**MJ2. Gate 7's "a preparation walks at most the budget" is not measured, so a walk that overruns
the budget stays green.**

Location: `crates/builtins/src/tail.rs:174-178`. For a stopped walk, the test counter adds
`horizon`: "A stopped walk took its whole horizon". This is an assumption, not a count, and
`fixed_cascade_within` returns nothing about a stopped walk's real frames.

Mutation X3 removes the cap from the majorant pass and only counts there
(`*walked += 1; majorants.step(..)`). With X3, every test stays green: `tail_contract` 12/12 and
the builtins-compiler and host-core lib suites. The replay or deviation cap still returns `None`,
so no value moves, but the stopped design walks its whole majorant pass past the budget. In the
domain that is up to about 0.58 M frames (about 7 ms). A future uncapped pass could walk up to
`HORIZON_LIMIT = 2^26` frames.

The record (line 393) says "Design-bound walking stays within the budget (gate 7 holds the frame
count exactly)". That claim is stronger than what the test checks. This matters now:
#1465-#1468 change this walk (#1465 F5: "a decade law that extends the horizon"), and gate 7 is
the gate that is supposed to keep D1 binding.

The code shipped today is correct: I read all three loops, and each one is capped. The fix:
- `fixed_cascade_within` reports the frames it actually walked on a stop as well, for example
  `CascadeWalk { result: Option<..>, frames }`;
- the counter adds those frames;
- gate 7 asserts `walked <= budget` with a real count, and X3 must turn it red.

## MINOR

- **m1. The host-core test's test-value sentence names a defect that other tests already catch.**
  Location: spec line 418, mutation row M11.
  - The named defect is "a cached design's charge differs from its computed one" (M11, which is the
    same mutation as M1). M1 also turns gate 5 and gate 7 red.
  - The test's own catch is X1: the policy function does not pass the cache to builtins-compiler
    (`prepare.rs:1541` given `None`). X1 turns only this test red, through
    `assert!(!cache.is_empty())`.
  - Restate the sentence and the evidence row.
- **m2. Doc comments contradict the new behaviour.**
  - `crates/builtins-compiler/src/lib.rs:387-390`, the prepared `tails` field, says a non-live strip
    "carries ... its own design's bound". Past the budget it now carries the live bound. A reader
    such as #1379, which builds on this field, will rely on that sentence.
  - `crates/builtins/src/tail.rs:4-7`, the module doc, says the compiler calls
    `input_section_live_bound` (it now reads the table) and that `fixed_input_bound` runs "once per
    distinct design" (it now runs at most once, within the budget, unless cached).
- **m3. Cache-cap claims are overstated, and #1471's gate 1 inherits one of them.**
  - `tail.rs:239-246` says "the cap holds a whole preparation's designs, so a rebuild of any
    unchanged session is served entirely from the cache". That is true only when nothing else
    shares the cache. With #1471's engine cache, clear-on-full can fire in the middle of a compile.
    The designs inserted before the clear are then lost, so the first rebuild computes them again.
  - #1471 seeds each session with "a copy of the engine's cache after that compile". So #1471
    gate 1 ("a rebuild of an unchanged session ... computes no design bound") fails for an engine
    cache near its cap. Each copy can also be about 2.3 MiB.
  - #1471 should seed with the session's own designs (at most 5,876 entries), or state the
    condition.
  - Memory: 8,192 entries measured 2,366,400 B (2.26 MiB, 289 B per entry with BTreeMap nodes).
    The doc says "under about 2 MiB".
- **m4. Successors are told to re-run "#1457's gates 2 and 4", and that is no longer possible.**
  - Amendment 1 (spec line 197) still says "#1464-#1468 rerun gates 2 and 4". Amendment 2 moved
    gate 4 to #1471 and #1470. Root's ruling is gates 2 and 8.
  - #1465 (lines 103, 185-191), #1466 (132-136), #1467 (118) and #1468 (148-152) still cite
    "#1457's gates 2 and 4" and "#1457's gate 2 and gate 4 commands".
  - Gate 2's probe is "not committed" (line 368), so no gate-2 command exists for a successor to
    re-run.
  - Record root's "gates 2 and 8" and either commit the frozen gate-2 workload or specify it
    exactly. After MJ1, it must include the two-section family.
- **m5. `builtins::input_section_bounds_within` is a second public entry point with a caller-chosen
  budget.** Location: `crates/builtins/src/lib.rs:3544`. It is public and not gated, and only
  tests call it. Ruling (i) asks for one entry point. Put it behind `test-support`; `tail_contract`
  already builds with `builtins/test-support`. D1's budget then cannot be bypassed in production.

## NIT

- **n1.** `crates/host-core/src/prepare.rs:2229`: "The second design's live bound reaches the
  report". It is the third design (index 2).
- **n2.** `tail.rs:231-235`: the rates in the doc differ from the record. The doc says typical
  57,000-73,000 frames/ms; the record says 64,700-71,400. The doc says near-top 75,500-81,000; the
  record goes up to 81,900. The doc also says "the CI-class runner" and does not say that this box
  was the stand-in. MJ1 rewrites this paragraph anyway.
- **n3.** `crates/math/src/tail.rs:2118`: the 10-line `fixed_cascade_within` does not need
  `#[allow(clippy::too_many_lines)]`.
- **n4.** Evidence rows M10, M12 and M13 (spec lines 437, 439, 440) show catches that other tests
  share:
  - M10 changes the 48 kHz entry, which the existing
    `live_input_lane_reports_the_live_bound_and_plain_input_its_own` also catches.
  - M12 (950,000) and M13 are also caught by the host-core test's premise.
  - The tests' own catches are M10b (the 96 kHz entry: only gate 6 turns red) and M12b
    (budget 1,200,000: only gate 8 turns red).
- **n5.** `prepare.rs:1515-1531`: the policy function passes `bound_cache` only on its default
  branch. It silently drops the cache on the selected-meters branch and the between-render-calls
  branch. The browser prepares through
  `prepare_host_runtime_with_selected_meters_between_render_calls`, so #1470's brief should say
  that its cache must reach a branch that ignores the cache today.

## Observations for root (not findings)

- **Headroom of the budget's purpose (gate 8).** `console-sixty-four-track` at 96 kHz walks
  1,254,016 of 1,510,000 frames (83 %). Distinct designs of the 20 Hz class (HPF 20.00-32.75 Hz
  into a 20 kHz LPF) that fit within the budget:

  | rate | designs that fit |
  |---|---|
  | 44.1 kHz | 134 |
  | 48 kHz | 125 |
  | 88.2 kHz | 70 |
  | 96 kHz | 64 |

  So a 65-track session at 96 kHz with distinct designs of this class already puts strips on the
  live bound: a tail of 704k samples and a rest of 2.38M samples.
- **Gate 2, re-measured.**
  - 65,537 distinct near-top designs: exactly 1,510,000 frames, 2 strips exact. First run
    40.5-57 ms, warm 23.5-40 ms (the O(strips) keying), no cache 41.6-58 ms.
  - The 64-design family: exactly 1,510,000 frames, 3 exact. First run 18.6-19.7 ms, warm
    0.018 ms.
  - Both agree with the record.

## Test value (one sentence per new or changed test)

- `tail_contract::the_live_bound_is_taken_exactly_when_the_budget_is_exhausted`: red when
  designs are charged out of strip order, or when the hit or walk boundary is off by one. M7, M3
  and M5 turn only this test red. M1 and M6 also turn gate 7 red; M2 also turns the existing count
  test red.
- `tail_contract::a_full_bound_cache_is_cleared_and_reports_the_same_values`: red when the entry
  cap is not enforced (M9, unique).
- `tail_contract::live_bound_table_is_the_computed_live_bound_at_every_launch_rate`: red when a
  44.1, 88.2 or 96 kHz table entry differs from the derivation (M10b, unique). An existing
  builtins-compiler test also catches the 48 kHz entry.
- `builtins_compiler::tests::a_preparation_walks_at_most_the_budget_and_a_warm_cache_walks_nothing`:
  red when a stopped walk's horizon is not cached (M8) or is honoured with `<` (M4), so that a warm
  rebuild walks again. Both are unique. It is blind to an overrunning walk (MJ2).
- `builtins_compiler::tests::every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`:
  red when the budget drops below a 64-track document's walk while the host-core premise still
  holds (M12b at 1,200,000, unique).
- `host_core::prepare::tests::a_design_bound_cache_changes_no_prepared_value`: red when the policy
  function does not pass its cache to builtins-compiler (X1, unique). The recorded sentence (M11)
  is not unique (m1).
- Changed: `design_bounds_are_computed_only_for_strips_without_a_live_input_lane` (6 -> 5,
  4 -> 3): red when the memoryless shortcut is lost (X6, unique).

## Mutation runs

Each mutation was applied alone to the export and run with `--no-fail-fast` on
`tail_contract` (release) and on the builtins-compiler and host-core lib suites (release,
test-support). The export was restored and confirmed byte-identical after each run.

| id | mutation | red |
|---|---|---|
| M1 | a hit charges 0 | gate 5, gate 7, host-core test |
| M2 | a repeated design is charged again | gate 5, existing count test |
| M3 | a hit fits only when `frames < remaining` | gate 5 only |
| M4 | `ChargeAbove` honoured only when `remaining < above` | gate 7 only |
| M5 | `take_frame` stops one frame early | gate 5 only |
| M6 | a stopped walk leaves the remaining budget | gate 5, gate 7 |
| M7 | charged in reverse strip order | gate 5 only |
| M8 | a stopped walk's horizon is not cached | gate 7 only |
| M9 | no clear-on-full | cache-cap test only |
| M10 | 48 kHz table entry + 1 | gate 6, existing `live_input_lane_reports_the_live_bound_and_plain_input_its_own` |
| M10b | 96 kHz table entry + 1 | gate 6 only |
| M11 (= M1) | a hit charges 0 | host-core test (with gate 5 and gate 7) |
| M12 | budget 950,000 | gate 8, host-core test (premise) |
| M12b | budget 1,200,000 | gate 8 only |
| M13 | each frame charged twice | gate 5, gate 8, host-core test |
| X1 | host-core policy passes `None` to builtins-compiler | host-core test only |
| X2 | builtins-compiler passes `None` to `input_section_bounds` | gate 7, host-core test |
| X3 | majorant pass counts but does not stop | **none (MJ2)** |
| X4 | fallback is `UNBOUNDED` | gate 5, gate 7, host-core test |
| X5 | deviation loop counts but does not stop | gate 5 |
| X6 | memoryless shortcut disabled | changed count test only |

## Gates run

All gates ran on the `f5c98fa8d` export, x86-64, `CARGO_INCREMENTAL=0`.

| gate | result |
|---|---|
| test-debug-a workspace step (its exact `--exclude` and `--features`), 1,465 tests | ok |
| test-debug-a workspace doctests | ok |
| test-debug-b DSP step and its doctests | ok |
| Release `-p lane -p math -p wasm-gates --features math/lane` | ok |
| Release `filter_liveness` | 14 passed |
| Release `tail_contract` | 12 passed |
| `cargo fmt --check` | clean |
| `cargo clippy --workspace --all-targets --all-features -D warnings` | clean |
| `cargo doc -D warnings` | fails only in `gate-expander/src/corpus.rs:64` (from #1459, already on the base, untouched here); clean with `--exclude gate-expander` |
| `audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations; `pcm_digest` `cb10fbface44a3a4` (unchanged) |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-sdk-headless.sh`, `test-web-audioworklet.sh` (empty `TMPDIR` afterwards) | all ok |
| `run-wasm-gates.sh --without-v8-spill --without-native` | ok |
| Every `scripts/check-*.sh` with the CI arguments | all exit 0 except the two below |
| `check-host-core-policy.sh` | fails with the same message on the base `5dc6f96e9`; the script is unchanged (#1469, not counted) |
| `check-sdk-types.sh` | first run failed because `sdk/node_modules` was missing; after `npm ci` in the export it passes |

The `scripts/check-*.sh` scripts that exited 0 were: artifact-evidence-leak, bench-policy,
bench-preconditions, builtins-fixtures (with `target/release/audit`), builtins-listening,
builtins-policy, capi-abi and `--self-test`, conformance-boundaries, console-benchmark-fixture,
console-fixtures, cross-targets, dsp-research, effect-contract, effect-runtime-fixtures,
effect-runtime-policy, env-vocabulary, graph-determinism, graph-policy, lane-policy,
parametric-eq-render-contract, protocol-control-policy, protocol-wasm-parity, rack-policy,
realtime-audit-leak, realtime-policy, sdk-generated, session-policy, unfused-seal and
`--self-test`, and workspace-policy.

Gate 2, the 65,537-design worst case, the 64-design family, the cheap families and the grid were
re-run with my own probe (`probe1457.rs`, not committed). AArch64 runs only in CI.

GitHub (read only): #1457, #1470 and #1471 are open, and their titles match the specs.
